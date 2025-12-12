# Text Field

Material Design 3 text input component.

## Variants

| Variant | Description |
|---------|-------------|
| `Filled` | Filled background style |
| `Outlined` | Border outline style |

## Basic Usage

```rust
use bevy_material_ui::prelude::*;
use bevy_material_ui::text_field::TextFieldVariant;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    // Filled text field (default)
    MaterialTextField::new("Username")
        .spawn(&mut commands, &theme);

    // Outlined text field
    MaterialTextField::new("Email")
        .with_variant(TextFieldVariant::Outlined)
        .spawn(&mut commands, &theme);
}
```

## With Icons

```rust
// Leading icon
MaterialTextField::new("Search")
    .leading_icon(ICON_SEARCH)
    .spawn(&mut commands, &theme);

// Trailing icon
MaterialTextField::new("Password")
    .trailing_icon(ICON_VISIBILITY)
    .spawn(&mut commands, &theme);
```

## With Helper Text

```rust
MaterialTextField::new("Email")
    .helper_text("We'll never share your email")
    .spawn(&mut commands, &theme);
```

## With Character Counter

```rust
MaterialTextField::new("Bio")
    .max_length(200)
    .show_counter(true)
    .spawn(&mut commands, &theme);
```

## Error State

```rust
MaterialTextField::new("Email")
    .error(true)
    .error_text("Please enter a valid email")
    .spawn(&mut commands, &theme);
```

## Disabled State

```rust
MaterialTextField::new("Disabled Field")
    .disabled(true)
    .value("Cannot edit this")
    .spawn(&mut commands, &theme);
```

## Password Field

```rust
MaterialTextField::new("Password")
    .password(true)
    .spawn(&mut commands, &theme);
```

## Multiline

```rust
MaterialTextField::new("Description")
    .multiline(true)
    .min_lines(3)
    .max_lines(10)
    .spawn(&mut commands, &theme);
```

## Handling Input

```rust
use bevy_material_ui::text_field::TextFieldChangeEvent;

fn handle_text_changes(
    mut reader: EventReader<TextFieldChangeEvent>,
) {
    for event in reader.read() {
        println!("Text changed to: {}", event.value);
    }
}
```

## Reading Values

```rust
fn read_text_fields(
    fields: Query<&MaterialTextField>,
) {
    for field in fields.iter() {
        println!("Field '{}' value: {}", field.label, field.value);
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `label` | `String` | Required | Field label |
| `variant` | `TextFieldVariant` | `Filled` | Visual style |
| `value` | `String` | `""` | Current text value |
| `placeholder` | `Option<String>` | `None` | Placeholder text |
| `leading_icon` | `Option<String>` | `None` | Left icon |
| `trailing_icon` | `Option<String>` | `None` | Right icon |
| `helper_text` | `Option<String>` | `None` | Helper text below |
| `error` | `bool` | `false` | Error state |
| `error_text` | `Option<String>` | `None` | Error message |
| `disabled` | `bool` | `false` | Disabled state |
| `password` | `bool` | `false` | Hide text input |
| `max_length` | `Option<usize>` | `None` | Maximum characters |
| `multiline` | `bool` | `false` | Enable multiline |

## TextFieldChangeEvent

| Field | Type | Description |
|-------|------|-------------|
| `entity` | `Entity` | The text field entity |
| `value` | `String` | New text value |
