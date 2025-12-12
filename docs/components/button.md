# Button

Material Design 3 button component with multiple variants and state support.

![Button Example](./screenshots/buttons.png)

## Variants

| Variant | Description | Use Case |
|---------|-------------|----------|
| `Filled` | High emphasis, solid background | Primary actions |
| `FilledTonal` | Medium emphasis, tonal background | Secondary actions |
| `Outlined` | Medium emphasis, bordered | Alternative actions |
| `Text` | Low emphasis, text only | Tertiary actions |
| `Elevated` | Surface-level emphasis with shadow | Floating actions |

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    // Filled button (default)
    MaterialButton::new("Click Me")
        .spawn(&mut commands, &theme);

    // Other variants
    MaterialButton::new("Outlined")
        .with_variant(ButtonVariant::Outlined)
        .spawn(&mut commands, &theme);

    MaterialButton::new("Text")
        .with_variant(ButtonVariant::Text)
        .spawn(&mut commands, &theme);
}
```

## With Icons

```rust
// Leading icon
MaterialButton::new("Add Item")
    .with_icon(ICON_ADD)
    .spawn(&mut commands, &theme);

// Trailing icon
MaterialButton::new("Next")
    .with_trailing_icon(ICON_ARROW_FORWARD)
    .spawn(&mut commands, &theme);

// Icon gravity control
MaterialButton::new("Settings")
    .with_icon(ICON_SETTINGS)
    .icon_gravity(IconGravity::End)
    .spawn(&mut commands, &theme);
```

## Disabled State

```rust
MaterialButton::new("Disabled")
    .disabled(true)
    .spawn(&mut commands, &theme);
```

## Custom Styling

```rust
MaterialButton::new("Custom")
    .corner_radius(24.0)
    .min_width(200.0)
    .custom_background_color(Color::srgb(0.2, 0.6, 0.9))
    .custom_text_color(Color::WHITE)
    .spawn(&mut commands, &theme);
```

## Toggle Button (Checkable)

```rust
MaterialButton::new("Toggle")
    .checkable(true)
    .spawn(&mut commands, &theme);
```

## Handling Clicks

```rust
fn handle_button_clicks(
    mut reader: EventReader<ButtonClickEvent>,
) {
    for event in reader.read() {
        println!("Button clicked: {:?}", event.entity);
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `variant` | `ButtonVariant` | `Filled` | Button style variant |
| `disabled` | `bool` | `false` | Disables interaction |
| `label` | `String` | Required | Button text |
| `icon` | `Option<String>` | `None` | Leading icon |
| `trailing_icon` | `Option<String>` | `None` | Trailing icon |
| `icon_gravity` | `IconGravity` | `Start` | Icon position |
| `icon_padding` | `f32` | `8.0` | Space between icon and label |
| `icon_size` | `f32` | `18.0` | Icon dimensions |
| `corner_radius` | `Option<f32>` | `None` | Custom radius (uses variant default if None) |
| `min_width` | `Option<f32>` | `None` | Minimum button width |
| `min_height` | `Option<f32>` | `None` | Minimum button height |
| `stroke_width` | `f32` | `1.0` | Border width for outlined variant |
| `checkable` | `bool` | `false` | Enable toggle behavior |
| `checked` | `bool` | `false` | Toggle state |

## State Layers

Buttons automatically apply MD3 state layers:
- **Hover**: 8% opacity overlay of content color
- **Pressed**: 12% opacity overlay of content color
- **Focused**: Focus ring indicator
