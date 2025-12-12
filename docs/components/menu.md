# Menu

Material Design 3 dropdown menu component.

![Menu Example](./screenshots/menus.png)

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    MaterialMenu::new()
        .add_item("Cut", Some(ICON_CONTENT_CUT))
        .add_item("Copy", Some(ICON_CONTENT_COPY))
        .add_item("Paste", Some(ICON_CONTENT_PASTE))
        .add_divider()
        .add_item("Delete", Some(ICON_DELETE))
        .spawn(&mut commands, &theme);
}
```

## With Nested Menus

```rust
MaterialMenu::new()
    .add_item("New", Some(ICON_ADD))
    .add_submenu("Open Recent", |submenu| {
        submenu
            .add_item("Document 1", None)
            .add_item("Document 2", None)
    })
    .add_divider()
    .add_item("Settings", Some(ICON_SETTINGS))
    .spawn(&mut commands, &theme);
```

## Disabled Items

```rust
MaterialMenu::new()
    .add_item("Active", None)
    .add_item_disabled("Disabled", None)
    .spawn(&mut commands, &theme);
```

## Handling Selection

```rust
use bevy_material_ui::menu::MenuItemSelectedEvent;

fn handle_menu_selection(
    mut reader: EventReader<MenuItemSelectedEvent>,
) {
    for event in reader.read() {
        println!("Menu item selected: {}", event.label);
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `items` | `Vec<MenuItem>` | `[]` | Menu items |
| `open` | `bool` | `false` | Visibility state |

## MenuItem Types

| Type | Description |
|------|-------------|
| `Item` | Regular clickable item |
| `Divider` | Visual separator |
| `Submenu` | Nested menu |
