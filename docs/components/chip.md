# Chip

Material Design 3 chips for selections, filters, and actions.

## Variants

| Variant | Description | Use Case |
|---------|-------------|----------|
| `Assist` | Help complete tasks | Workflow assistance |
| `Filter` | Filter content | Category selection |
| `Input` | Represent user input | Tags, contacts |
| `Suggestion` | Dynamic suggestions | Search suggestions |

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    // Assist chip
    MaterialChip::new("Get directions")
        .with_variant(ChipVariant::Assist)
        .with_icon(ICON_DIRECTIONS)
        .spawn(&mut commands, &theme);

    // Filter chip
    MaterialChip::new("Featured")
        .with_variant(ChipVariant::Filter)
        .spawn(&mut commands, &theme);

    // Input chip (with delete button)
    MaterialChip::new("john@example.com")
        .with_variant(ChipVariant::Input)
        .deletable()
        .spawn(&mut commands, &theme);

    // Suggestion chip
    MaterialChip::new("Try this")
        .with_variant(ChipVariant::Suggestion)
        .spawn(&mut commands, &theme);
}
```

## Selected State

```rust
// Pre-selected filter chip
MaterialChip::new("Active")
    .with_variant(ChipVariant::Filter)
    .selected(true)
    .spawn(&mut commands, &theme);
```

## Elevated Chips

```rust
MaterialChip::new("Elevated")
    .with_elevation(ChipElevation::Elevated)
    .spawn(&mut commands, &theme);
```

## With Icons

```rust
// Leading icon
MaterialChip::new("Add filter")
    .with_icon(ICON_ADD)
    .spawn(&mut commands, &theme);

// Avatar (for input chips)
MaterialChip::new("Jane Doe")
    .with_variant(ChipVariant::Input)
    .with_avatar("path/to/avatar.png")
    .spawn(&mut commands, &theme);
```

## Deletable Chips

```rust
MaterialChip::new("Tag")
    .with_variant(ChipVariant::Input)
    .deletable()
    .spawn(&mut commands, &theme);
```

## Handling Events

```rust
use bevy_material_ui::chip::{ChipClickEvent, ChipDeleteEvent};

fn handle_chip_events(
    mut click_reader: EventReader<ChipClickEvent>,
    mut delete_reader: EventReader<ChipDeleteEvent>,
) {
    for event in click_reader.read() {
        println!("Chip clicked: {:?}", event.value);
    }
    
    for event in delete_reader.read() {
        println!("Chip deleted: {:?}", event.value);
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `variant` | `ChipVariant` | `Assist` | Chip style variant |
| `label` | `String` | Required | Chip text |
| `icon` | `Option<String>` | `None` | Leading icon |
| `selected` | `bool` | `false` | Selected state |
| `disabled` | `bool` | `false` | Disabled state |
| `deletable` | `bool` | `false` | Shows delete button |
| `elevation` | `ChipElevation` | `Flat` | Elevation level |

## State Layers

Chips apply MD3 state layers:
- **Hover**: 8% opacity overlay
- **Pressed**: 12% opacity overlay
- **Selected**: Uses secondary container colors
