# Tooltip

Material Design 3 tooltip component for contextual hints.

![Tooltip Example](./screenshots/tooltips.png)

## Types

| Type | Description |
|------|-------------|
| Plain | Simple text tooltip |
| Rich | Tooltip with title and actions |

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    // Add tooltip to a button
    MaterialButton::new("Hover me")
        .with_tooltip("This is a helpful tooltip")
        .spawn(&mut commands, &theme);
}
```

## Using TooltipHost

```rust
// Any element can have a tooltip
commands.spawn((
    Node { /* ... */ },
    TooltipHost::new("Tooltip text"),
));
```

## Rich Tooltips

```rust
TooltipHost::rich("Title", "This is detailed tooltip content with more information.")
    .with_action("Learn more", || {
        // Action callback
    });
```

## Tooltip Placement

```rust
use bevy_material_ui::tooltip::TooltipPlacement;

TooltipHost::new("Tooltip")
    .placement(TooltipPlacement::Top);

TooltipHost::new("Tooltip")
    .placement(TooltipPlacement::Bottom);

TooltipHost::new("Tooltip")
    .placement(TooltipPlacement::Left);

TooltipHost::new("Tooltip")
    .placement(TooltipPlacement::Right);
```

## Custom Delay

```rust
// Show tooltip after 1 second
TooltipHost::new("Delayed tooltip")
    .delay(1000);  // milliseconds
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `text` | `String` | Required | Tooltip message |
| `title` | `Option<String>` | `None` | Rich tooltip title |
| `placement` | `TooltipPlacement` | `Top` | Position preference |
| `delay` | `u32` | `500` | Show delay (ms) |

## TooltipPlacement

| Value | Description |
|-------|-------------|
| `Top` | Above the element |
| `Bottom` | Below the element |
| `Left` | To the left |
| `Right` | To the right |

## Positioning

Tooltips automatically adjust position to stay within screen bounds. The placement is a preference that may be overridden if there isn't enough space.

## Styling

Plain tooltips use:
- Background: `inverse_surface`
- Text: `inverse_on_surface`
- Corner radius: 4dp

Rich tooltips use:
- Background: `surface_container`
- Text: `on_surface_variant`
- Corner radius: 12dp
