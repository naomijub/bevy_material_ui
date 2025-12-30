use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Re-run build script if the embedded font changes.
    println!("cargo:rerun-if-changed=assets/fonts/MaterialSymbolsOutlined.ttf");

    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let font_path = manifest_dir.join("assets/fonts/MaterialSymbolsOutlined.ttf");
    let font_bytes = fs::read(&font_path).expect("Failed to read MaterialSymbolsOutlined.ttf");

    let face = ttf_parser::Face::parse(&font_bytes, 0)
        .expect("Failed to parse embedded Material Symbols font");

    // Generate a const for every Unicode codepoint in the Private Use Areas that maps to a glyph.
    // This guarantees users can reference *all* icons included in the shipped TTF without needing
    // an external codepoints list.
    let mut codepoints: Vec<u32> = Vec::new();

    // BMP Private Use Area.
    for u in 0xE000u32..=0xF8FFu32 {
        if let Some(ch) = char::from_u32(u) {
            if face.glyph_index(ch).is_some() {
                codepoints.push(u);
            }
        }
    }

    // Supplementary Private Use Area (common range).
    for u in 0xF0000u32..=0xF8FFFu32 {
        if let Some(ch) = char::from_u32(u) {
            if face.glyph_index(ch).is_some() {
                codepoints.push(u);
            }
        }
    }

    // Stable ordering.
    codepoints.sort_unstable();
    codepoints.dedup();

    let mut out = String::new();
    out.push_str("// Generated Material Symbols codepoints.\n");
    out.push_str("//\n");
    out.push_str("// This module is generated at compile-time by `build.rs` by scanning the\n");
    out.push_str("// embedded `MaterialSymbolsOutlined.ttf` for codepoints that map to glyphs.\n");
    out.push_str("//\n");
    out.push_str("// The constants are named `ICON_CP_<HEX>`. For example, `ICON_CP_E88A`.\n\n");

    // Also generate a compact list for iteration.
    out.push_str("/// All icon codepoints discovered in the embedded font (PUA ranges).\n");
    out.push_str("pub const ALL_ICON_CODEPOINTS: &[char] = &[\n");
    for u in &codepoints {
        out.push_str(&format!("    '\\u{{{:X}}}',\n", u));
    }
    out.push_str("];\n\n");

    for u in &codepoints {
        out.push_str(&format!(
            "pub const ICON_CP_{:X}: char = '\\u{{{:X}}}';\n",
            u, u
        ));
    }

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let dest = out_dir.join("material_symbols_codepoints.rs");
    let mut file = fs::File::create(&dest).expect("Failed to create generated codepoints file");
    file.write_all(out.as_bytes())
        .expect("Failed to write generated codepoints file");
}
