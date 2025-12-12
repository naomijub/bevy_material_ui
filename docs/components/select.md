# Select

Material Design 3 dropdown select component.

## Variants

| Variant | Description |
|---------|-------------|
| `Filled` | Filled text field style |
| `Outlined` | Outlined text field style |

## Basic Usage

```rust
use bevy_material_ui::prelude::*;
use bevy_material_ui::select::SelectVariant;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    MaterialSelect::new("Choose an option")
        .add_option("option1", "Option 1")
        .add_option("option2", "Option 2")
        .add_option("option3", "Option 3")
        .spawn(&mut commands, &theme);
}
```

## With Default Value

```rust
MaterialSelect::new("Country")
    .add_option("us", "United States")
    .add_option("uk", "United Kingdom")
    .add_option("ca", "Canada")
    .selected("us")
    .spawn(&mut commands, &theme);
```

## Outlined Variant

```rust
MaterialSelect::new("Category")
    .with_variant(SelectVariant::Outlined)
    .add_option("cat1", "Category 1")
    .add_option("cat2", "Category 2")
    .spawn(&mut commands, &theme);
```

## With Icons

```rust
MaterialSelect::new("Priority")
    .add_option_with_icon("high", "High", ICON_PRIORITY_HIGH)
    .add_option_with_icon("medium", "Medium", ICON_REMOVE)
    .add_option_with_icon("low", "Low", ICON_PRIORITY_LOW)
    .spawn(&mut commands, &theme);
```

## Disabled State

```rust
MaterialSelect::new("Disabled Select")
    .add_option("a", "A")
    .disabled(true)
    .spawn(&mut commands, &theme);
```

## Handling Selection

```rust
use bevy_material_ui::select::SelectChangeEvent;

fn handle_select_changes(
    mut reader: EventReader<SelectChangeEvent>,
) {
    for event in reader.read() {
        println!("Selected: {} ({})", event.label, event.value);
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `label` | `String` | Required | Field label |
| `variant` | `SelectVariant` | `Filled` | Visual style |
| `options` | `Vec<SelectOption>` | `[]` | Available options |
| `selected` | `Option<String>` | `None` | Selected value |
| `disabled` | `bool` | `false` | Disabled state |
| `error` | `bool` | `false` | Error state |
| `helper_text` | `Option<String>` | `None` | Helper text below |

## SelectChangeEvent

| Field | Type | Description |
|-------|------|-------------|
| `entity` | `Entity` | The select entity |
| `value` | `String` | Selected option value |
| `label` | `String` | Selected option label |
