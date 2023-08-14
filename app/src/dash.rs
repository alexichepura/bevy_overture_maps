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
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[0].value = format!("{:.0}fps", average);
            }
        }
    }
}

pub fn dash_start_system(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    let height = Val::Px(32.);
    let width = Val::Px(80.);
    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(120.),
            height: height.clone(),
            justify_content: JustifyContent::End,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        let background_color: BackgroundColor = Color::rgba(0.15, 0.15, 0.15, 0.5).into();
        parent
            .spawn(NodeBundle {
                background_color,
                style: Style {
                    width,
                    height: height.clone(),
                    padding: UiRect::all(Val::Px(4.0)),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(TextBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(4.),
                            right: Val::Px(4.),
                            ..default()
                        },
                        text: Text {
                            sections: vec![TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: medium.clone(),
                                    font_size: 24.0,
                                    color: Color::YELLOW_GREEN,
                                },
                            }],
                            ..default()
                        },
                        ..default()
                    })
                    .insert(FpsText);
            });
    });
}
