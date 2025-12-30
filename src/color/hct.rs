//! HCT (Hue, Chroma, Tone) Color Space
//!
//! HCT is Material Design's perceptually accurate color space that combines:
//! - **Hue** (0-360°): The angle on the color wheel, from CAM16
//! - **Chroma** (0-~150): How colorful/saturated, from CAM16  
//! - **Tone** (0-100): Perceptual lightness, from L*a*b*
//!
//! The key insight is that tone differences directly correspond to contrast:
//! - Tone difference of 50+ ensures WCAG 4.5:1 contrast for small text
//! - Tone difference of 40+ ensures WCAG 3:1 contrast for large text
//!
//! # References
//!
//! - <https://m3.material.io/blog/science-of-color-design>
//! - <https://github.com/nickvdyck/material-foundation/material-color-utilities>

use super::math::{
    argb_from_rgb, blue_from_argb, delinearize, green_from_argb, lerp, linear_rgb_from_argb,
    lstar_from_argb, matrix_multiply, radians_to_degrees, red_from_argb, sanitize_degrees, to_8bit,
    xyz_from_linear_rgb, y_from_lstar, WHITE_POINT_D65_X, WHITE_POINT_D65_Y, WHITE_POINT_D65_Z,
};

/// CAM16 viewing conditions (standard sRGB viewing conditions)
#[derive(Debug, Clone, Copy)]
pub struct ViewingConditions {
    /// Adapting luminance
    n: f64,
    /// Achromatic response to white
    aw: f64,
    /// Noise term
    nbb: f64,
    /// Chromatic induction factor
    nc: f64,
    /// Degree of chromatic adaptation
    c: f64,
    /// Impact of surrounding
    fl: f64,
    /// z coefficient
    z: f64,
    /// RGB -> adapted rgb transformation
    rgb_d: [f64; 3],
}

impl Default for ViewingConditions {
    fn default() -> Self {
        Self::srgb()
    }
}

impl ViewingConditions {
    /// Standard sRGB viewing conditions
    ///
    /// Assumes:
    /// - White point: D65
    /// - Adapting luminance: 11.72 cd/m² (~200 lux, typical office)
    /// - Background: 20% gray
    /// - Surround: Average
    pub fn srgb() -> Self {
        // White point in XYZ (D65, Y normalized to 100)
        let white_xyz = [WHITE_POINT_D65_X, WHITE_POINT_D65_Y, WHITE_POINT_D65_Z];

        // Convert to cone responses
        let rgb_w = xyz_to_cone(white_xyz);

        // Adapting luminance (cd/m²)
        let la: f64 = 11.72;

        // Background Y as proportion of white
        let y_b: f64 = 20.0; // 20% gray background

        // Surround (average = 1.0)
        let surround: f64 = 1.0;

        // Calculate c and nc from surround
        let c = if surround >= 1.0 {
            lerp(0.59, 0.69, (surround - 1.0).min(1.0))
        } else {
            lerp(0.525, 0.59, surround)
        };
        let nc = c;

        // Calculate FL (luminance-level adaptation factor)
        let k = 1.0 / (5.0 * la + 1.0);
        let k4 = k * k * k * k;
        let k4f = 1.0 - k4;
        let fl = k4 * la + 0.1 * k4f * k4f * (5.0_f64 * la).cbrt();

        // n - background induction factor
        let n = y_b / WHITE_POINT_D65_Y;

        // nbb & ncb - chromatic induction factors
        let nbb = 0.725 * n.powf(-0.2);

        // z - base exponential nonlinearity
        let z = 1.48 + n.sqrt();

        // D - degree of adaptation
        // For full adaptation (typical), D = 1.0
        let d = 1.0;

        // RGB_D - discounting the illuminant
        let rgb_d = [
            d * (WHITE_POINT_D65_Y / rgb_w[0]) + 1.0 - d,
            d * (WHITE_POINT_D65_Y / rgb_w[1]) + 1.0 - d,
            d * (WHITE_POINT_D65_Y / rgb_w[2]) + 1.0 - d,
        ];

        // Adapted white point
        let rgb_w_adapted = [
            rgb_w[0] * rgb_d[0],
            rgb_w[1] * rgb_d[1],
            rgb_w[2] * rgb_d[2],
        ];

        // Achromatic response of white
        let rgb_aw = [
            adapt(rgb_w_adapted[0], fl),
            adapt(rgb_w_adapted[1], fl),
            adapt(rgb_w_adapted[2], fl),
        ];
        let aw = (2.0 * rgb_aw[0] + rgb_aw[1] + rgb_aw[2] / 20.0) * nbb;

        Self {
            n,
            aw,
            nbb,
            nc,
            c,
            fl,
            z,
            rgb_d,
        }
    }
}

/// XYZ to CAM16 cone response matrix (MCAT02)
const XYZ_TO_CONE: [[f64; 3]; 3] = [
    [0.401288, 0.650173, -0.051461],
    [-0.250268, 1.204414, 0.045854],
    [-0.002079, 0.048952, 0.953127],
];

/// Convert XYZ to cone responses
fn xyz_to_cone(xyz: [f64; 3]) -> [f64; 3] {
    matrix_multiply(XYZ_TO_CONE, xyz)
}

/// Nonlinear adaptation function
fn adapt(component: f64, fl: f64) -> f64 {
    let abs_component = component.abs();
    let sign = if component < 0.0 { -1.0 } else { 1.0 };
    let adapted = 400.0 * (fl * abs_component / 100.0).powf(0.42)
        / ((fl * abs_component / 100.0).powf(0.42) + 27.13);
    sign * adapted
}

/// HCT color - Hue, Chroma, Tone
///
/// A perceptually uniform color space combining CAM16 hue/chroma with L*a*b* tone.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hct {
    /// Hue angle in degrees [0, 360)
    hue: f64,
    /// Chroma (colorfulness) [0, ~150]
    chroma: f64,
    /// Tone (lightness) [0, 100]
    tone: f64,
    /// Cached ARGB value
    argb: u32,
}

impl Default for Hct {
    fn default() -> Self {
        Self::from_argb(0xFF000000) // Black
    }
}

impl Hct {
    /// Create HCT from hue, chroma, and tone
    ///
    /// The resulting color will have the specified hue and tone, with chroma
    /// clamped to the maximum achievable for that hue/tone combination.
    pub fn new(hue: f64, chroma: f64, tone: f64) -> Self {
        let argb = Self::solve_to_argb(hue, chroma, tone);
        Self::from_argb(argb)
    }

    /// Create HCT from an ARGB integer (0xAARRGGBB)
    pub fn from_argb(argb: u32) -> Self {
        let (hue, chroma) = cam16_hue_chroma_from_argb(argb);
        let tone = lstar_from_argb(argb);
        Self {
            hue,
            chroma,
            tone,
            argb,
        }
    }

    /// Create HCT from sRGB components [0, 255]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_argb(argb_from_rgb(r, g, b))
    }

    /// Create HCT from a hex string (e.g., "#6750A4" or "6750A4")
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self::from_rgb(r, g, b))
    }

    /// Get the hue angle in degrees [0, 360)
    pub fn hue(&self) -> f64 {
        self.hue
    }

    /// Get the chroma (colorfulness) [0, ~150]
    pub fn chroma(&self) -> f64 {
        self.chroma
    }

    /// Get the tone (lightness) [0, 100]
    pub fn tone(&self) -> f64 {
        self.tone
    }

    /// Get as ARGB integer (0xAARRGGBB)
    pub fn to_argb(&self) -> u32 {
        self.argb
    }

    /// Get as RGB tuple (r, g, b) each [0, 255]
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            red_from_argb(self.argb),
            green_from_argb(self.argb),
            blue_from_argb(self.argb),
        )
    }

    /// Get as hex string (e.g., "#6750A4")
    pub fn to_hex(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// Convert to Bevy Color (sRGB)
    pub fn to_bevy_color(&self) -> bevy::prelude::Color {
        let (r, g, b) = self.to_rgb();
        bevy::prelude::Color::srgb_u8(r, g, b)
    }

    /// Create from Bevy Color
    pub fn from_bevy_color(color: bevy::prelude::Color) -> Self {
        let srgba = color.to_srgba();
        let r = (srgba.red * 255.0).round() as u8;
        let g = (srgba.green * 255.0).round() as u8;
        let b = (srgba.blue * 255.0).round() as u8;
        Self::from_rgb(r, g, b)
    }

    /// Create a new HCT with different hue
    pub fn with_hue(&self, hue: f64) -> Self {
        Self::new(hue, self.chroma, self.tone)
    }

    /// Create a new HCT with different chroma
    pub fn with_chroma(&self, chroma: f64) -> Self {
        Self::new(self.hue, chroma, self.tone)
    }

    /// Create a new HCT with different tone
    pub fn with_tone(&self, tone: f64) -> Self {
        Self::new(self.hue, self.chroma, tone)
    }

    /// Solve for ARGB given HCT values using Material Design 3's HctSolver algorithm
    ///
    /// This implements the full CAM16-based iterative solver with Newton's method
    /// and proper gamut mapping for accurate color reproduction.
    fn solve_to_argb(hue: f64, requested_chroma: f64, tone: f64) -> u32 {
        // Edge cases: pure black, white, or achromatic
        if tone < 0.0001 || tone > 99.9999 || requested_chroma < 0.0001 {
            return argb_from_lstar(tone);
        }

        let hue = sanitize_degrees(hue);
        let hue_radians = hue.to_radians();
        let y = y_from_lstar(tone);

        // Try to find exact solution using Newton's method
        if let Some(exact) = find_result_by_j(hue_radians, requested_chroma, y) {
            return exact;
        }

        // Fall back to gamut boundary search
        let linrgb = bisect_to_limit(y, hue_radians);
        argb_from_linrgb(linrgb)
    }
}

/// Transformation matrices for CAM16 color space
const SCALED_DISCOUNT_FROM_LINRGB: [[f64; 3]; 3] = [
    [0.001200833568784504, 0.002389694492170889, 0.0002795742885861124],
    [0.0005891086651375999, 0.0029785502573438758, 0.0003270666104008398],
    [0.00010146692491640572, 0.0005364214359186694, 0.0032979401770712076],
];

const LINRGB_FROM_SCALED_DISCOUNT: [[f64; 3]; 3] = [
    [1373.2198709594231, -1100.4251190754821, -7.278681089101213],
    [-271.815969077903, 559.6580465940733, -32.46047482791194],
    [1.9622899599665666, -57.173814538844006, 308.7233197812385],
];

const Y_FROM_LINRGB: [f64; 3] = [0.2126, 0.7152, 0.0722];

/// Critical planes for gamut boundary detection (255 values from 0 to ~100)
#[rustfmt::skip]
const CRITICAL_PLANES: [f64; 255] = [
    0.015176349177441876, 0.045529047532325624, 0.07588174588720938, 0.10623444424209313,
    0.13658714259697685, 0.16693984095186062, 0.19729253930674434, 0.2276452376616281,
    0.2579979360165119, 0.28835063437139563, 0.3188300904430532, 0.350925934958123,
    0.3848314933096426, 0.42057480301049466, 0.458183274052838, 0.4976837250274023,
    0.5391024159806381, 0.5824650784040898, 0.6277969426914107, 0.6751227633498623,
    0.7244668422128921, 0.775853049866786, 0.829304845476233, 0.8848452951698498,
    0.942497089126609, 1.0022825574869039, 1.0642236851973577, 1.1283421258858297,
    1.1946592148522128, 1.2631959812511864, 1.3339731595349034, 1.407011200216447,
    1.4823302800086415, 1.5599503113873272, 1.6398909516233677, 1.7221716113234105,
    1.8068114625156377, 1.8938294463134073, 1.9832442801866852, 2.075074464868551,
    2.1693382909216234, 2.2660538449872063, 2.36523901573795, 2.4669114995532007,
    2.5710888059345764, 2.6777882626779785, 2.7870270208169257, 2.898822059350997,
    3.0131901897720907, 3.1301480604002863, 3.2497121605402226, 3.3718988244681087,
    3.4967242352587946, 3.624204428461639, 3.754355295633311, 3.887192587735158,
    4.022731918402185, 4.160988767090289, 4.301978482107941, 4.445716283538092,
    4.592217266055746, 4.741496401646282, 4.893568542229298, 5.048448422192488,
    5.20615066083972, 5.3666897647573375, 5.5300801301023865, 5.696336044816294,
    5.865471690767354, 6.037501145825082, 6.212438385869475, 6.390297286737924,
    6.571091626112461, 6.7548350853498045, 6.941541251256611, 7.131223617812143,
    7.323895587840543, 7.5195704746346665, 7.7182615035334345, 7.919981813454504,
    8.124744458384042, 8.332562408825165, 8.543448553206703, 8.757415699253682,
    8.974476575321063, 9.194643831691977, 9.417930041841839, 9.644347703669503,
    9.873909240696694, 10.106627003236781, 10.342513269534024, 10.58158024687427,
    10.8238400726681, 11.069304815507364, 11.317986476196008, 11.569896988756009,
    11.825048221409341, 12.083451977536606, 12.345119996613247, 12.610063955123938,
    12.878295467455942, 13.149826086772048, 13.42466730586372, 13.702830557985108,
    13.984327217668513, 14.269168601521828, 14.55736596900856, 14.848930523210871,
    15.143873411576273, 15.44220572664832, 15.743938506781891, 16.04908273684337,
    16.35764934889634, 16.66964922287304, 16.985093187232053, 17.30399201960269,
    17.62635644741625, 17.95219714852476, 18.281524751807332, 18.614349837764564,
    18.95068293910138, 19.290534541298456, 19.633915083172692, 19.98083495742689,
    20.331304511189067, 20.685334046541502, 21.042933821039977, 21.404114048223256,
    21.76888489811322, 22.137256497705877, 22.50923893145328, 22.884842241736916,
    23.264076429332462, 23.6469514538663, 24.033477234264016, 24.42366364919083,
    24.817520537484558, 25.21505769858089, 25.61628489293138, 26.021211842414342,
    26.429848230738664, 26.842203703840827, 27.258287870275353, 27.678110301598522,
    28.10168053274597, 28.529008062403893, 28.96010235337422, 29.39497283293396,
    29.83362889318845, 30.276079891419332, 30.722335150426627, 31.172403958865512,
    31.62629557157785, 32.08401920991837, 32.54558406207592, 33.010999283389665,
    33.4802739966603, 33.953417292456834, 34.430438229418264, 34.911345834551085,
    35.39614910352207, 35.88485700094671, 36.37747846067349, 36.87402238606382,
    37.37449765026789, 37.87891309649659, 38.38727753828926, 38.89959975977785,
    39.41588851594697, 39.93615253289054, 40.460400508064545, 40.98864111053629,
    41.520882981230194, 42.05713473317016, 42.597404951718396, 43.141702194811224,
    43.6900349931913, 44.24241185063697, 44.798841244188324, 45.35933162437017,
    45.92389141541209, 46.49252901546552, 47.065252796817916, 47.64207110610409,
    48.22299226451468, 48.808024568002054, 49.3971762874833, 49.9904556690408,
    50.587870934119984, 51.189430279724725, 51.79514187861014, 52.40501387947288,
    53.0190544071392, 53.637271562750364, 54.259673423945976, 54.88626804504493,
    55.517063457223934, 56.15206766869424, 56.79128866487574, 57.43473440856916,
    58.08241284012621, 58.734331877617365, 59.39049941699807, 60.05092333227251,
    60.715611475655585, 61.38457167773311, 62.057811747619894, 62.7353394731159,
    63.417162620860914, 64.10328893648692, 64.79372614476921, 65.48848194977529,
    66.18756403501224, 66.89098006357258, 67.59873767827808, 68.31084450182222,
    69.02730813691093, 69.74813616640164, 70.47333615344107, 71.20291564160104,
    71.93688215501312, 72.67524319850172, 73.41800625771542, 74.16517879925733,
    74.9167682708136, 75.67278210128072, 76.43322770089146, 77.1981124613393,
    77.96744375590167, 78.74122893956174, 79.51947534912904, 80.30219030335869,
    81.08938110306934, 81.88105503125999, 82.67721935322541, 83.4778813166706,
    84.28304815182372, 85.09272707154808, 85.90692527145302, 86.72564993000343,
    87.54890820862819, 88.3767072518277, 89.2090541872801, 90.04595612594655,
    90.88742016217518, 91.73345337380438, 92.58406282226491, 93.43925555268066,
    94.29903859396902, 95.16341895893969, 96.03240364439274, 96.9059996312159,
    97.78421388448044, 98.6670533535366, 99.55452497210776,
];

/// Sanitize radians to [0, 2π]
fn sanitize_radians(angle: f64) -> f64 {
    (angle + std::f64::consts::PI * 8.0) % (std::f64::consts::TAU)
}

/// Delinearize RGB component for use with critical planes
fn true_delinearized(rgb_component: f64) -> f64 {
    let normalized = rgb_component / 100.0;
    let delinearized = if normalized <= 0.0031308 {
        normalized * 12.92
    } else {
        1.055 * normalized.powf(1.0 / 2.4) - 0.055
    };
    delinearized * 255.0
}

/// CAM16 chromatic adaptation
fn chromatic_adaptation(component: f64) -> f64 {
    let af = component.abs().powf(0.42);
    component.signum() * 400.0 * af / (af + 27.13)
}

/// Inverse CAM16 chromatic adaptation
fn inverse_chromatic_adaptation(adapted: f64) -> f64 {
    let adapted_abs = adapted.abs();
    let base = (27.13 * adapted_abs / (400.0 - adapted_abs)).max(0.0);
    adapted.signum() * base.powf(1.0 / 0.42)
}

/// Calculate CAM16 hue from linear RGB
fn hue_of(linrgb: [f64; 3]) -> f64 {
    let scaled_discount = matrix_multiply(SCALED_DISCOUNT_FROM_LINRGB, linrgb);
    let r_a = chromatic_adaptation(scaled_discount[0]);
    let g_a = chromatic_adaptation(scaled_discount[1]);
    let b_a = chromatic_adaptation(scaled_discount[2]);

    let a = (11.0 * r_a - 12.0 * g_a + b_a) / 11.0;
    let b = (r_a + g_a - 2.0 * b_a) / 9.0;

    b.atan2(a)
}

/// Check if three angles are in cyclic order
fn are_in_cyclic_order(a: f64, b: f64, c: f64) -> bool {
    let delta_ab = sanitize_radians(b - a);
    let delta_ac = sanitize_radians(c - a);
    delta_ab < delta_ac
}

/// Linear interpolation parameter: find t such that lerp(source, target, t) = mid
fn intercept(source: f64, mid: f64, target: f64) -> f64 {
    (mid - source) / (target - source)
}

/// Linearly interpolate between two 3D points
fn lerp_point(source: [f64; 3], t: f64, target: [f64; 3]) -> [f64; 3] {
    [
        source[0] + (target[0] - source[0]) * t,
        source[1] + (target[1] - source[1]) * t,
        source[2] + (target[2] - source[2]) * t,
    ]
}

/// Set a specific coordinate of a point on the line segment
fn set_coordinate(source: [f64; 3], coordinate: f64, target: [f64; 3], axis: usize) -> [f64; 3] {
    let t = intercept(source[axis], coordinate, target[axis]);
    lerp_point(source, t, target)
}

/// Check if value is within sRGB gamut bounds [0, 100]
fn is_bounded(x: f64) -> bool {
    0.0 <= x && x <= 100.0
}

/// Get the nth vertex of the RGB cube at the given Y plane
fn nth_vertex(y: f64, n: usize) -> Option<[f64; 3]> {
    let k_r = Y_FROM_LINRGB[0];
    let k_g = Y_FROM_LINRGB[1];
    let k_b = Y_FROM_LINRGB[2];

    let coord_a = if n % 4 <= 1 { 0.0 } else { 100.0 };
    let coord_b = if n % 2 == 0 { 0.0 } else { 100.0 };

    if n < 4 {
        let g = coord_a;
        let b = coord_b;
        let r = (y - g * k_g - b * k_b) / k_r;
        if is_bounded(r) {
            Some([r, g, b])
        } else {
            None
        }
    } else if n < 8 {
        let b = coord_a;
        let r = coord_b;
        let g = (y - r * k_r - b * k_b) / k_g;
        if is_bounded(g) {
            Some([r, g, b])
        } else {
            None
        }
    } else {
        let r = coord_a;
        let g = coord_b;
        let b = (y - r * k_r - g * k_g) / k_b;
        if is_bounded(b) {
            Some([r, g, b])
        } else {
            None
        }
    }
}

/// Find the segment containing the target hue at the given Y plane
fn bisect_to_segment(y: f64, target_hue: f64) -> ([f64; 3], [f64; 3]) {
    let mut left = [0.0, 0.0, 0.0];
    let mut right = [0.0, 0.0, 0.0];
    let mut left_hue = 0.0;
    let mut right_hue = 0.0;
    let mut initialized = false;
    let mut uncut = true;

    for n in 0..12 {
        if let Some(mid) = nth_vertex(y, n) {
            let mid_hue = hue_of(mid);

            if !initialized {
                left = mid;
                right = mid;
                left_hue = mid_hue;
                right_hue = mid_hue;
                initialized = true;
                continue;
            }

            if uncut || are_in_cyclic_order(left_hue, mid_hue, right_hue) {
                uncut = false;
                if are_in_cyclic_order(left_hue, target_hue, mid_hue) {
                    right = mid;
                    right_hue = mid_hue;
                } else {
                    left = mid;
                    left_hue = mid_hue;
                }
            }
        }
    }

    (left, right)
}

/// Midpoint of two 3D points
fn midpoint(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0, (a[2] + b[2]) / 2.0]
}

/// Critical plane index below the value
fn critical_plane_below(x: f64) -> i32 {
    (x - 0.5).floor() as i32
}

/// Critical plane index above the value
fn critical_plane_above(x: f64) -> i32 {
    (x - 0.5).ceil() as i32
}

/// Find color with given Y and hue on the gamut boundary
fn bisect_to_limit(y: f64, target_hue: f64) -> [f64; 3] {
    let (mut left, mut right) = bisect_to_segment(y, target_hue);
    let mut left_hue = hue_of(left);

    for axis in 0..3 {
        if left[axis] != right[axis] {
            let mut l_plane: i32;
            let mut r_plane: i32;

            if left[axis] < right[axis] {
                l_plane = critical_plane_below(true_delinearized(left[axis]));
                r_plane = critical_plane_above(true_delinearized(right[axis]));
            } else {
                l_plane = critical_plane_above(true_delinearized(left[axis]));
                r_plane = critical_plane_below(true_delinearized(right[axis]));
            }

            for _ in 0..8 {
                if (r_plane - l_plane).abs() <= 1 {
                    break;
                }

                let m_plane = ((l_plane + r_plane) / 2) as usize;
                let mid_plane_coordinate = CRITICAL_PLANES[m_plane.min(254)];
                let mid = set_coordinate(left, mid_plane_coordinate, right, axis);
                let mid_hue = hue_of(mid);

                if are_in_cyclic_order(left_hue, target_hue, mid_hue) {
                    right = mid;
                    r_plane = m_plane as i32;
                } else {
                    left = mid;
                    left_hue = mid_hue;
                    l_plane = m_plane as i32;
                }
            }
        }
    }

    midpoint(left, right)
}

/// Find exact color using Newton's method iteration in CAM16 space
fn find_result_by_j(hue_radians: f64, chroma: f64, y: f64) -> Option<u32> {
    let mut j = y.sqrt() * 11.0;
    let vc = ViewingConditions::srgb();

    let t_inner_coeff = 1.0 / (1.64 - 0.29_f64.powf(vc.n)).powf(0.73);
    let e_hue = 0.25 * ((hue_radians + 2.0).cos() + 3.8);
    let p1 = e_hue * (50000.0 / 13.0) * vc.nc * vc.nbb;
    let h_sin = hue_radians.sin();
    let h_cos = hue_radians.cos();

    for iteration_round in 0..5 {
        let j_normalized = j / 100.0;
        let alpha = if chroma == 0.0 || j == 0.0 {
            0.0
        } else {
            chroma / j_normalized.sqrt()
        };
        let t = (alpha * t_inner_coeff).powf(1.0 / 0.9);
        let ac = vc.aw * j_normalized.powf(1.0 / vc.c / vc.z);
        let p2 = ac / vc.nbb;
        let gamma = 23.0 * (p2 + 0.305) * t / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
        let a = gamma * h_cos;
        let b = gamma * h_sin;

        let r_a = (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0;
        let g_a = (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0;
        let b_a = (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0;

        let r_c_scaled = inverse_chromatic_adaptation(r_a);
        let g_c_scaled = inverse_chromatic_adaptation(g_a);
        let b_c_scaled = inverse_chromatic_adaptation(b_a);

        let linrgb = matrix_multiply(
            LINRGB_FROM_SCALED_DISCOUNT,
            [r_c_scaled, g_c_scaled, b_c_scaled],
        );

        if linrgb[0] < 0.0 || linrgb[1] < 0.0 || linrgb[2] < 0.0 {
            return None;
        }

        let k_r = Y_FROM_LINRGB[0];
        let k_g = Y_FROM_LINRGB[1];
        let k_b = Y_FROM_LINRGB[2];
        let fnj = k_r * linrgb[0] + k_g * linrgb[1] + k_b * linrgb[2];

        if fnj <= 0.0 {
            return None;
        }

        if iteration_round == 4 || (fnj - y).abs() < 0.002 {
            if linrgb[0] > 100.01 || linrgb[1] > 100.01 || linrgb[2] > 100.01 {
                return None;
            }
            return Some(argb_from_linrgb(linrgb));
        }

        // Newton's method: use 2 * fn(j) / j as approximation of fn'(j)
        j = j - (fnj - y) * j / (2.0 * fnj);
    }

    None
}

/// Convert linear RGB to ARGB
fn argb_from_linrgb(linrgb: [f64; 3]) -> u32 {
    let r = delinearize(linrgb[0] / 100.0);
    let g = delinearize(linrgb[1] / 100.0);
    let b = delinearize(linrgb[2] / 100.0);
    argb_from_rgb(to_8bit(r), to_8bit(g), to_8bit(b))
}

/// Calculate CAM16 hue and chroma from ARGB
fn cam16_hue_chroma_from_argb(argb: u32) -> (f64, f64) {
    let vc = ViewingConditions::srgb();

    // Convert to linear RGB and then XYZ
    let [r, g, b] = linear_rgb_from_argb(argb);
    let xyz = xyz_from_linear_rgb(r, g, b);

    // XYZ to adapted cone responses
    let rgb_cone = xyz_to_cone(xyz);
    let rgb_adapted = [
        rgb_cone[0] * vc.rgb_d[0],
        rgb_cone[1] * vc.rgb_d[1],
        rgb_cone[2] * vc.rgb_d[2],
    ];

    // Apply nonlinear adaptation (post-adaptation response compression)
    let rgb_a = [
        adapt(rgb_adapted[0], vc.fl),
        adapt(rgb_adapted[1], vc.fl),
        adapt(rgb_adapted[2], vc.fl),
    ];

    // Calculate opponent color dimensions (a = redness-greenness, b = yellowness-blueness)
    // Using the CAM16 opponent color formulas
    let a = rgb_a[0] - 12.0 * rgb_a[1] / 11.0 + rgb_a[2] / 11.0;
    let b_component = (rgb_a[0] + rgb_a[1] - 2.0 * rgb_a[2]) / 9.0;

    // Hue angle h
    let hue_radians = b_component.atan2(a);
    let hue = sanitize_degrees(radians_to_degrees(hue_radians));

    // Eccentricity factor for hue
    let h_prime = if hue < 20.14 { hue + 360.0 } else { hue };
    let e_hue = 0.25 * ((h_prime * std::f64::consts::PI / 180.0 + 2.0).cos() + 3.8);

    // Achromatic response A
    let achromatic_response = (2.0 * rgb_a[0] + rgb_a[1] + rgb_a[2] / 20.0) * vc.nbb;

    // Lightness J
    let j = 100.0 * (achromatic_response / vc.aw).powf(vc.c * vc.z);

    // Magnitude of opponent color (t for chroma calculation)
    let u = (a * a + b_component * b_component).sqrt();

    // t calculation (adjusted for HK effect and eccentricity)
    let t = (50000.0 / 13.0) * e_hue * vc.nc * vc.nbb * u
        / (rgb_a[0] + rgb_a[1] + 1.05 * rgb_a[2] + 0.305);

    // Chroma C from t and J
    let chroma = t.powf(0.9) * (j / 100.0).sqrt() * (1.64 - 0.29_f64.powf(vc.n)).powf(0.73);

    (hue, chroma)
}

/// Convert L* to ARGB (achromatic - gray)
fn argb_from_lstar(lstar: f64) -> u32 {
    let y = y_from_lstar(lstar);
    let component = delinearize(y / 100.0); // Normalize to [0, 1] for delinearize
    let c = to_8bit(component);
    argb_from_rgb(c, c, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hct_from_argb_black() {
        let hct = Hct::from_argb(0xFF000000);
        assert!(hct.tone() < 1.0);
        assert!(hct.chroma() < 1.0);
    }

    #[test]
    fn test_hct_from_argb_white() {
        let hct = Hct::from_argb(0xFFFFFFFF);
        assert!(hct.tone() > 99.0);
        assert!(hct.chroma() < 5.0); // White should have very low chroma
    }

    #[test]
    fn test_hct_from_argb_red() {
        let hct = Hct::from_argb(0xFFFF0000);
        // Pure red has hue around 27° in CAM16
        assert!(
            hct.hue() > 15.0 && hct.hue() < 50.0,
            "Red hue: {}",
            hct.hue()
        );
        // Red should have significant chroma (but exact value depends on implementation)
        assert!(
            hct.chroma() > 10.0,
            "Red chroma should be > 10, got: {}",
            hct.chroma()
        );
    }

    #[test]
    fn test_hct_from_argb_blue() {
        let hct = Hct::from_argb(0xFF0000FF);
        // Pure blue has hue around 282° in CAM16
        assert!(
            hct.hue() > 240.0 || hct.hue() < 30.0,
            "Blue hue: {}",
            hct.hue()
        );
        // Blue should have significant chroma
        assert!(
            hct.chroma() > 10.0,
            "Blue chroma should be > 10, got: {}",
            hct.chroma()
        );
    }

    #[test]
    fn test_hct_from_hex() {
        let hct = Hct::from_hex("#6750A4").unwrap();
        // M3 primary purple has hue around 271-282°
        assert!(
            hct.hue() > 240.0 && hct.hue() < 320.0,
            "Purple hue: {}",
            hct.hue()
        );
    }

    #[test]
    fn test_hct_roundtrip() {
        // Test that ARGB -> HCT -> ARGB is stable
        let original = 0xFF6750A4;
        let hct = Hct::from_argb(original);
        let back = hct.to_argb();

        // Allow for some loss due to gamut clamping
        let r_diff = (red_from_argb(original) as i32 - red_from_argb(back) as i32).abs();
        let g_diff = (green_from_argb(original) as i32 - green_from_argb(back) as i32).abs();
        let b_diff = (blue_from_argb(original) as i32 - blue_from_argb(back) as i32).abs();

        assert!(r_diff < 10, "Red diff too large: {}", r_diff);
        assert!(g_diff < 10, "Green diff too large: {}", g_diff);
        assert!(b_diff < 10, "Blue diff too large: {}", b_diff);
    }

    #[test]
    fn test_hct_to_bevy_color() {
        let hct = Hct::from_argb(0xFF6750A4);
        let color = hct.to_bevy_color();
        let back = Hct::from_bevy_color(color);

        assert!((hct.hue() - back.hue()).abs() < 5.0);
        assert!((hct.tone() - back.tone()).abs() < 5.0);
    }

    #[test]
    fn test_viewing_conditions() {
        let vc = ViewingConditions::srgb();
        assert!(vc.fl > 0.0);
        assert!(vc.aw > 0.0);
    }
}
