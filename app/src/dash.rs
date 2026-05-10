use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct DashPlugin;

impl Plugin for DashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (dash_start_system,))
            .add_systems(Update, (dash_fps_system,))
            .add_plugins((FrameTimeDiagnosticsPlugin::default(),));
    }
}

#[derive(Component)]
pub struct FpsText;

pub fn dash_fps_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.0 = format!("{:.0}fps", average);
            }
        }
    }
}

pub fn dash_start_system(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");

    cmd.spawn((
        Node {
            width: percent(100.),
            height: px(32.),
            justify_content: JustifyContent::End,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.5)),
    ))
    .with_children(|parent| {
        parent
            .spawn((
                Node {
                    width: percent(100.),
                    height: px(32.),
                    padding: UiRect::all(Val::Px(4.0)),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.5)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(""),
                    TextFont {
                        font: medium.clone(),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 1.0, 0.0)),
                    FpsText,
                ));
            });
    });
}

fn percent(value: f32) -> Val {
    Val::Percent(value)
}

fn px(value: f32) -> Val {
    Val::Px(value)
}
