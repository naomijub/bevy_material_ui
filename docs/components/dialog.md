# Dialog

Material Design 3 dialog component for modal interactions.

![Dialog Example](./screenshots/dialogs.png)

## Types

| Type | Description |
|------|-------------|
| `Basic` | Standard modal dialog |
| `FullScreen` | Full-screen dialog for complex content |

## Basic Usage

```rust
use bevy_material_ui::prelude::*;

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    // Create a dialog (starts closed by default)
    MaterialDialog::new()
        .title("Confirm Action")
        .spawn_with_children(&mut commands, &theme, |parent| {
            parent.spawn(Text::new("Are you sure you want to proceed?"));
        });
}
```

## Opening Dialogs

```rust
// Use the DialogOpenEvent to open a dialog
fn open_dialog(
    mut writer: EventWriter<DialogOpenEvent>,
    dialog_query: Query<Entity, With<MaterialDialog>>,
) {
    if let Ok(dialog_entity) = dialog_query.get_single() {
        writer.send(DialogOpenEvent { entity: dialog_entity });
    }
}
```

## With Icon

```rust
MaterialDialog::new()
    .title("Warning")
    .icon(ICON_WARNING)
    .spawn_with_children(&mut commands, &theme, |parent| {
        parent.spawn(Text::new("This action cannot be undone."));
    });
```

## Full-Screen Dialog

```rust
MaterialDialog::new()
    .with_type(DialogType::FullScreen)
    .title("Edit Profile")
    .spawn_with_children(&mut commands, &theme, |parent| {
        // Full-screen content
    });
```

## Dismiss Behavior

```rust
// Prevent scrim click dismissal
MaterialDialog::new()
    .title("Important")
    .no_scrim_dismiss()
    .spawn_with_children(&mut commands, &theme, |parent| {
        // Content
    });

// Prevent escape key dismissal
MaterialDialog::new()
    .title("Confirmation Required")
    .no_escape_dismiss()
    .spawn_with_children(&mut commands, &theme, |parent| {
        // Content
    });
```

## Handling Events

```rust
use bevy_material_ui::dialog::{DialogOpenEvent, DialogCloseEvent, DialogConfirmEvent};

fn handle_dialog_events(
    mut open_reader: EventReader<DialogOpenEvent>,
    mut close_reader: EventReader<DialogCloseEvent>,
    mut confirm_reader: EventReader<DialogConfirmEvent>,
) {
    for event in open_reader.read() {
        println!("Dialog opened: {:?}", event.entity);
    }
    
    for event in close_reader.read() {
        println!("Dialog closed: {:?}", event.entity);
    }
    
    for event in confirm_reader.read() {
        println!("Dialog confirmed: {:?}", event.entity);
    }
}
```

## Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `dialog_type` | `DialogType` | `Basic` | Dialog style |
| `open` | `bool` | `false` | Visibility state |
| `title` | `Option<String>` | `None` | Dialog title |
| `icon` | `Option<String>` | `None` | Header icon |
| `dismiss_on_scrim_click` | `bool` | `true` | Close when clicking outside |
| `dismiss_on_escape` | `bool` | `true` | Close on Escape key |

## Elevation

Dialogs use Level 3 elevation with appropriate shadow.
