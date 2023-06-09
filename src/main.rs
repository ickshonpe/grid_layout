use std::iter;

use bevy::prelude::*;

const ALIGN_ITEMS_COLOR: Color = Color::rgb(1., 0.066, 0.349);
const JUSTIFY_CONTENT_COLOR: Color = Color::rgb(0.102, 0.522, 1.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_layout)
        .run();
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let justifications = [
        JustifyContent::Default,
        JustifyContent::Start,
        JustifyContent::FlexStart,
        JustifyContent::Center,
        JustifyContent::FlexEnd,
        JustifyContent::End,
        JustifyContent::SpaceEvenly,
        JustifyContent::SpaceAround,
        JustifyContent::SpaceBetween,
    ];
    let alignments = [
        AlignItems::Default,
        AlignItems::Baseline,
        AlignItems::Start,
        AlignItems::FlexStart,
        AlignItems::Center,
        AlignItems::FlexEnd,
        AlignItems::End,
        AlignItems::Stretch,
    ];
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::px(alignments.len(), 100.),
                        grid_template_rows: vec![
                            GridTrack::auto(),
                            RepeatedGridTrack::px(justifications.len(), 80.),
                        ],
                        gap: Size::all(Val::Px(5.)),
                        padding: UiRect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    background_color: Color::BLACK.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    let row_len = alignments.len() as u16;

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                justify_items: JustifyItems::Center,
                                padding: UiRect {
                                    top: Val::Px(1.),
                                    bottom: Val::Px(1.),
                                    ..Default::default()
                                },
                                grid_column: GridPlacement::start(row_len / 2),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(ALIGN_ITEMS_COLOR),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "AlignItems",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 14.,
                                    color: Color::BLACK,
                                },
                            ));
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                justify_items: JustifyItems::Center,
                                padding: UiRect {
                                    top: Val::Px(1.),
                                    bottom: Val::Px(1.),
                                    ..Default::default()
                                },
                                grid_column: GridPlacement::start(1 + row_len / 2),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(JUSTIFY_CONTENT_COLOR),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "JustifyContent",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 14.,
                                    color: Color::BLACK,
                                },
                            ));
                        });

                    parent.spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            grid_column: GridPlacement::span(row_len - (1 + row_len / 2)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    for justify_content in justifications {
                        for align_items in alignments {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        gap: Size::all(Val::Px(3.)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content,
                                        align_items,
                                        ..Default::default()
                                    },
                                    background_color: BackgroundColor(Color::DARK_GRAY),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                padding: UiRect {
                                                    top: Val::Px(1.),
                                                    left: Val::Px(5.),
                                                    right: Val::Px(5.),
                                                    bottom: Val::Px(1.),
                                                },
                                                justify_content: JustifyContent::Center,
                                                ..Default::default()
                                            },
                                            background_color: BackgroundColor(ALIGN_ITEMS_COLOR),

                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn(
                                                TextBundle::from_section(
                                                    format!("{align_items:?}"),
                                                    TextStyle {
                                                        font: font.clone(),
                                                        font_size: 14.,
                                                        color: Color::BLACK,
                                                    },
                                                )
                                                .with_background_color(ALIGN_ITEMS_COLOR),
                                            );
                                        });

                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                padding: UiRect {
                                                    top: Val::Px(1.),
                                                    left: Val::Px(5.),
                                                    right: Val::Px(5.),
                                                    bottom: Val::Px(1.),
                                                },
                                                justify_content: JustifyContent::Center,
                                                ..Default::default()
                                            },
                                            background_color: BackgroundColor(
                                                JUSTIFY_CONTENT_COLOR,
                                            ),
                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                format!("{justify_content:?}"),
                                                TextStyle {
                                                    font: font.clone(),
                                                    font_size: 12.,
                                                    color: Color::BLACK,
                                                },
                                            ));
                                        });
                                });
                        }
                    }
                });
        });
}
