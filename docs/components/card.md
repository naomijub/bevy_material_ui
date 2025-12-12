# Card

Material Design 3 card component for containing content and actions.

## Variants

| Variant | Description | Use Case |
|---------|-------------|----------|
| `Elevated` | Raised with shadow | Default emphasis |
| `Filled` | Solid background | High emphasis content |
| `Outlined` | Bordered | Lower emphasis, grouped content |

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    // Elevated card (default)
    MaterialCard::new()
        .spawn_with_children(&mut commands, &theme, |parent| {
            parent.spawn(Text::new("Card Content"));
        });

    // Filled card
    MaterialCard::new()
        .with_variant(CardVariant::Filled)
        .spawn_with_children(&mut commands, &theme, |parent| {
            parent.spawn(Text::new("Filled Card"));
        });

    // Outlined card
    MaterialCard::new()
        .with_variant(CardVariant::Outlined)
        .spawn_with_children(&mut commands, &theme, |parent| {
            parent.spawn(Text::new("Outlined Card"));
        });
}
```

## Interactive Cards

```rust
// Clickable card
MaterialCard::new()
    .clickable()
    .spawn_with_children(&mut commands, &theme, |parent| {
        parent.spawn(Text::new("Click me!"));
    });

// Draggable card
MaterialCard::new()
    .draggable()
    .spawn_with_children(&mut commands, &theme, |parent| {
        parent.spawn(Text::new("Drag me!"));
    });
```

## Handling Clicks

```rust
fn handle_card_clicks(
    mut reader: EventReader<CardClickEvent>,
) {
    for event in reader.read() {
        println!("Card clicked: {:?}", event.entity);
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `variant` | `CardVariant` | `Elevated` | Card style variant |
| `clickable` | `bool` | `false` | Makes card interactive |
| `draggable` | `bool` | `false` | Enables drag behavior |

## State Layers

Interactive cards apply MD3 state layers:
- **Hover**: 8% opacity overlay
- **Pressed**: 12% opacity overlay

## Elevation

| Variant | Resting Level | Hovered Level |
|---------|---------------|---------------|
| Elevated | Level 1 | Level 2 |
| Filled | Level 0 | Level 1 |
| Outlined | Level 0 | Level 0 |
