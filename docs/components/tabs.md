# Tabs

Material Design 3 tab navigation component.

## Types

| Type | Description |
|------|-------------|
| Primary | Top-level navigation |
| Secondary | Within a content area |

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    MaterialTabs::new()
        .add_tab("Home", None)
        .add_tab("Profile", None)
        .add_tab("Settings", None)
        .spawn(&mut commands, &theme);
}
```

## With Icons

```rust
MaterialTabs::new()
    .add_tab("Home", Some(ICON_HOME))
    .add_tab("Favorites", Some(ICON_FAVORITE))
    .add_tab("Settings", Some(ICON_SETTINGS))
    .spawn(&mut commands, &theme);
```

## Secondary Tabs

```rust
MaterialTabs::new()
    .secondary()
    .add_tab("All", None)
    .add_tab("Active", None)
    .add_tab("Completed", None)
    .spawn(&mut commands, &theme);
```

## Default Selected Tab

```rust
MaterialTabs::new()
    .add_tab("Tab 1", None)
    .add_tab("Tab 2", None)
    .add_tab("Tab 3", None)
    .selected(1)  // Select "Tab 2"
    .spawn(&mut commands, &theme);
```

## Scrollable Tabs

```rust
MaterialTabs::new()
    .scrollable()
    .add_tab("Tab 1", None)
    .add_tab("Tab 2", None)
    // ... many tabs
    .spawn(&mut commands, &theme);
```

## Handling Tab Changes

```rust
use bevy_material_ui::tabs::TabChangeEvent;

fn handle_tab_changes(
    mut reader: EventReader<TabChangeEvent>,
) {
    for event in reader.read() {
        println!("Tab changed to index: {}", event.index);
    }
}
```

## Tab Content Visibility

```rust
#[derive(Component)]
struct TabContent(usize);

fn update_tab_content(
    tabs: Query<&MaterialTabs, Changed<MaterialTabs>>,
    mut content: Query<(&TabContent, &mut Visibility)>,
) {
    for tabs in tabs.iter() {
        for (tab_content, mut visibility) in content.iter_mut() {
            *visibility = if tab_content.0 == tabs.selected {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `tabs` | `Vec<Tab>` | `[]` | Tab definitions |
| `selected` | `usize` | `0` | Selected tab index |
| `tab_type` | `TabType` | `Primary` | Tab style |
| `scrollable` | `bool` | `false` | Enable horizontal scroll |

## Tab Structure

| Field | Type | Description |
|-------|------|-------------|
| `label` | `String` | Tab text |
| `icon` | `Option<String>` | Optional icon |
| `badge` | `Option<String>` | Optional badge text |

## TabChangeEvent

| Field | Type | Description |
|-------|------|-------------|
| `entity` | `Entity` | The tabs container entity |
| `index` | `usize` | New selected tab index |
| `label` | `String` | New selected tab label |
