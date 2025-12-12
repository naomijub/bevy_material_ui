# Divider

Material Design 3 divider component for visual separation.

## Types

| Type | Description |
|------|-------------|
| Horizontal | Full-width horizontal line |
| Vertical | Full-height vertical line |
| Inset | Horizontal with left margin |

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    // Horizontal divider
    MaterialDivider::horizontal()
        .spawn(&mut commands, &theme);

    // Vertical divider
    MaterialDivider::vertical()
        .spawn(&mut commands, &theme);

    // Inset divider (with left padding)
    MaterialDivider::horizontal()
        .inset(16.0)
        .spawn(&mut commands, &theme);
}
```

## Custom Thickness

```rust
MaterialDivider::horizontal()
    .thickness(2.0)
    .spawn(&mut commands, &theme);
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `horizontal` | `bool` | `true` | Orientation |
| `thickness` | `f32` | `1.0` | Line thickness |
| `inset_start` | `f32` | `0.0` | Left/top padding |
| `inset_end` | `f32` | `0.0` | Right/bottom padding |

## Colors

Dividers use `outline_variant` color from the theme by default.
