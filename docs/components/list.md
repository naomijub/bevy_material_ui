# List

Material Design 3 list component with selection support.

![List Example](./screenshots/lists.png)

## Features

- Single and multi-select modes
- Leading/trailing icons and avatars
- Supporting text
- Dividers between items

## Basic Usage

```rust
use bevy_material_ui::prelude::*;
use bevy_material_ui::list::{ListBuilder, ListItemBuilder};

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    ListBuilder::new()
        .add_item(ListItemBuilder::new("Item 1"))
        .add_item(ListItemBuilder::new("Item 2"))
        .add_item(ListItemBuilder::new("Item 3"))
        .spawn(&mut commands, &theme);
}
```

## With Supporting Text

```rust
ListBuilder::new()
    .add_item(
        ListItemBuilder::new("Primary Text")
            .supporting_text("Secondary supporting text")
    )
    .add_item(
        ListItemBuilder::new("Another Item")
            .supporting_text("More details here")
    )
    .spawn(&mut commands, &theme);
```

## With Icons

```rust
ListBuilder::new()
    .add_item(
        ListItemBuilder::new("Settings")
            .leading_icon(ICON_SETTINGS)
    )
    .add_item(
        ListItemBuilder::new("Account")
            .leading_icon(ICON_PERSON)
            .trailing_icon(ICON_ARROW_FORWARD)
    )
    .spawn(&mut commands, &theme);
```

## With Avatars

```rust
ListBuilder::new()
    .add_item(
        ListItemBuilder::new("John Doe")
            .avatar("assets/avatars/john.png")
            .supporting_text("john@example.com")
    )
    .spawn(&mut commands, &theme);
```

## With Dividers

```rust
ListBuilder::new()
    .with_dividers()
    .add_item(ListItemBuilder::new("Item 1"))
    .add_item(ListItemBuilder::new("Item 2"))
    .spawn(&mut commands, &theme);
```

## Scrollable List

```rust
ListBuilder::new()
    .scrollable()
    .max_height(300.0)
    .add_item(ListItemBuilder::new("Item 1"))
    // ... many items
    .spawn(&mut commands, &theme);
```

## Selection Modes

### Single Select

```rust
// Resource to track selection state
#[derive(Resource)]
struct ListSelectionState {
    mode: ListSelectionMode,
    selected: Vec<Entity>,
}

// Handle selection
fn handle_list_selection(
    items: Query<(Entity, &Interaction), (With<SelectableListItem>, Changed<Interaction>)>,
    mut selection: ResMut<ListSelectionState>,
) {
    for (entity, interaction) in items.iter() {
        if *interaction == Interaction::Pressed {
            selection.selected.clear();
            selection.selected.push(entity);
        }
    }
}
```

### Multi Select

```rust
fn handle_multi_selection(
    items: Query<(Entity, &Interaction), (With<SelectableListItem>, Changed<Interaction>)>,
    mut selection: ResMut<ListSelectionState>,
) {
    for (entity, interaction) in items.iter() {
        if *interaction == Interaction::Pressed {
            if let Some(pos) = selection.selected.iter().position(|e| *e == entity) {
                selection.selected.remove(pos); // Deselect
            } else {
                selection.selected.push(entity); // Select
            }
        }
    }
}
```

## Handling Item Clicks

```rust
use bevy_material_ui::list::ListItemClickEvent;

fn handle_list_item_clicks(
    mut reader: EventReader<ListItemClickEvent>,
) {
    for event in reader.read() {
        println!("List item clicked: {:?}", event.entity);
    }
}
```

## Properties

### ListBuilder

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `scrollable` | `bool` | `false` | Enable scrolling |
| `max_height` | `Option<f32>` | `None` | Max height for scrollable |
| `with_dividers` | `bool` | `false` | Show dividers |

### ListItemBuilder

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `headline` | `String` | Required | Primary text |
| `supporting_text` | `Option<String>` | `None` | Secondary text |
| `leading_icon` | `Option<String>` | `None` | Left icon |
| `trailing_icon` | `Option<String>` | `None` | Right icon |
| `avatar` | `Option<String>` | `None` | Avatar image path |

## State Layers

List items apply MD3 state layers:
- **Hover**: Surface container high color
- **Pressed**: Surface container highest color
- **Selected**: Primary container color
