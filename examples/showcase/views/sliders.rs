//! Sliders view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::slider::spawn_slider_control;

use crate::showcase::common::*;

/// Spawn the sliders section content
pub fn spawn_sliders_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section,
                theme,
                "Sliders",
                "Select values from a range - Continuous and Discrete with optional ticks",
            );

            // Slider demos using the library's spawn traits
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(24.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|col| {
                    // Continuous slider
                    col.spawn_slider(theme, 0.0, 100.0, 40.0, Some("Continuous"));

                    // Discrete slider with ticks
                    col.spawn_discrete_slider(theme, 0.0, 100.0, 60.0, 20.0, Some("Discrete"));

                    // Vertical slider
                    col.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(16.0),
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            Text::new("Vertical"),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(theme.on_surface_variant),
                            Node {
                                width: Val::Px(64.0),
                                ..default()
                            },
                        ));

                        row.spawn(Node {
                            width: Val::Px(48.0),
                            height: Val::Px(220.0),
                            ..default()
                        })
                        .with_children(|slot| {
                            let slider = MaterialSlider::new(0.0, 1.0).with_value(0.5).vertical();
                            spawn_slider_control(slot, theme, slider);
                        });
                    });
                });

            spawn_code_block(
                section,
                theme,
                r#"// Create a continuous slider
commands.spawn_slider(theme, 0.0, 100.0, 50.0, Some("Volume"));

// Create a discrete slider with ticks
commands.spawn_discrete_slider(theme, 0.0, 100.0, 60.0, 20.0, Some("Steps"));

// Use builder for more control
let slider = MaterialSlider::new(0.0, 100.0)
    .with_value(50.0)
    .with_step(10.0);
commands.spawn_slider_with(theme, slider, Some("Custom"));"#,
            );
        });
}
