# Progress

Material Design 3 progress indicator components.

![Progress Example](./screenshots/progress.png)

## Types

| Type | Description |
|------|-------------|
| Linear | Horizontal progress bar |
| Circular | Spinning circle indicator |

## Linear Progress

### Determinate

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    // 50% progress
    MaterialProgress::linear()
        .progress(0.5)
        .spawn(&mut commands, &theme);
}
```

### Indeterminate

```rust
// Animated indeterminate progress
MaterialProgress::linear()
    .indeterminate()
    .spawn(&mut commands, &theme);
```

## Circular Progress

### Determinate

```rust
// 75% circular progress
MaterialProgress::circular()
    .progress(0.75)
    .spawn(&mut commands, &theme);
```

### Indeterminate

```rust
// Spinning indicator
MaterialProgress::circular()
    .indeterminate()
    .spawn(&mut commands, &theme);
```

## Custom Size

```rust
// Custom width for linear
MaterialProgress::linear()
    .progress(0.5)
    .width(200.0)
    .spawn(&mut commands, &theme);

// Custom size for circular
MaterialProgress::circular()
    .progress(0.5)
    .size(64.0)
    .spawn(&mut commands, &theme);
```

## Custom Track Color

```rust
MaterialProgress::linear()
    .progress(0.5)
    .track_color(Color::srgba(0.5, 0.5, 0.5, 0.3))
    .spawn(&mut commands, &theme);
```

## Updating Progress

```rust
fn update_progress(
    mut progress_query: Query<&mut MaterialProgress>,
    time: Res<Time>,
) {
    for mut progress in progress_query.iter_mut() {
        progress.value = (progress.value + time.delta_secs() * 0.1) % 1.0;
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `progress_type` | `ProgressType` | `Linear` | Progress style |
| `value` | `f32` | `0.0` | Progress value (0.0-1.0) |
| `indeterminate` | `bool` | `false` | Animated indeterminate |
| `width` | `f32` | `240.0` | Linear progress width |
| `size` | `f32` | `48.0` | Circular progress size |
| `track_color` | `Option<Color>` | `None` | Custom track color |

## Animation

Indeterminate progress uses MD3 motion tokens for smooth animation.
