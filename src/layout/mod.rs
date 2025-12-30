//! Material Design 3 layout components.
//!
//! Layout components are higher-level building blocks (e.g. Scaffold) that help
//! compose apps from Material UI components while remaining compatible with
//! Bevy UI's flexbox model.

mod permanent_drawer_scaffold;
mod scaffold_types;
mod bottom_navigation_scaffold;
mod navigation_rail_scaffold;
mod modal_drawer_scaffold;
mod list_detail_scaffold;
mod supporting_panes_scaffold;
mod adaptive_navigation_scaffold;

pub use permanent_drawer_scaffold::{
    PermanentDrawerScaffold,
    spawn_permanent_drawer_scaffold,
};

pub use scaffold_types::{PaneEntities, PaneTestIds, ScaffoldEntities, ScaffoldTestIds};

pub use bottom_navigation_scaffold::{
    BottomNavigationScaffold, NavigationBarScaffold, spawn_bottom_navigation_scaffold,
    spawn_navigation_bar_scaffold,
};

pub use navigation_rail_scaffold::{NavigationRailScaffold, spawn_navigation_rail_scaffold};

pub use modal_drawer_scaffold::{ModalDrawerScaffold, spawn_modal_drawer_scaffold};

pub use list_detail_scaffold::{ListDetailScaffold, spawn_list_detail_scaffold};

pub use supporting_panes_scaffold::{SupportingPanesScaffold, spawn_supporting_panes_scaffold};

pub use adaptive_navigation_scaffold::{
    AdaptiveNavigationScaffold, NavigationSuiteScaffold, spawn_adaptive_navigation_scaffold,
    spawn_navigation_suite_scaffold,
};
