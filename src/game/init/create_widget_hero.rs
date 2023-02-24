use bevy::prelude::*;
use bevy::text::Text2dBounds;

use crate::config::data_layout::LayoutData;
use crate::game::combat::Hero;
use crate::game::{AssetStorage, CleanupOnGameplayEnd, FontId, HealthBar};
use crate::positioning::Depth;

#[derive(Component)]
pub struct HeroNameDisplay;

#[derive(Component)]
pub struct HeroStatsDisplay;

#[derive(Component)]
pub struct HeroMaxHealthDisplay;

#[derive(Component)]
pub struct HeroProficiencyDisplay;

#[derive(Component)]
pub struct HeroDamageResDisplay;

#[derive(Component)]
pub struct HeroDamageBonusDisplay;

#[derive(Component)]
pub struct HeroCurrentHealthDisplay;

#[derive(Component)]
pub struct HeroCurrentArmourDisplay;

#[derive(Component)]
pub struct HeroCurrentShieldDisplay;

#[derive(Component)]
pub struct HeroCurrentWeaponDisplay;

// Ridiculous amount of magic number for the hero name and stats..
pub fn create_layout_hero(
    mut commands: Commands,
    layout: Res<LayoutData>,
    assets: Res<AssetStorage>,
    hero: Res<Hero>,
) {
    let x = layout.right_x();
    let width = layout.right_width();
    let y = layout.c_right.hero_y();
    let height = layout.c_right.hero_height(&layout);
    let text_style = TextStyle {
        font: assets.font(&FontId::FiraSansMedium),
        font_size: 60.0,
        color: Color::ANTIQUE_WHITE,
    };
    let text_style_sm = TextStyle {
        font: assets.font(&FontId::FiraSansMedium),
        font_size: 60.0,
        color: Color::ANTIQUE_WHITE,
    };

    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Left,
    };
    // let pos_text = Vec2::new(5., 2.);
    let dimens_text = Vec2::new(width - 2., 0.6667);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(x + width * 0.5, y + height * 0.5, Depth::Grid.z()),
            ..default()
        })
        .insert(Name::new("Hero"))
        .insert(CleanupOnGameplayEnd)
        .with_children(|parent| {
            let health_bar_margin = 0.25;
            let health_bar_size = Vec2::new(width - health_bar_margin * 2., 0.25);
            parent
                .spawn_empty()
                .insert(HeroNameDisplay)
                .insert(Text2dBundle {
                    text: Text::from_section("Sir Hoardalot", text_style.clone())
                        .with_alignment(text_alignment),
                    // The max size that it should fit in:
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(
                            dimens_text.x * layout.text_factor,
                            dimens_text.y * layout.text_factor,
                        ),
                    },
                    transform: Transform::from_xyz(
                        -1., // Centered on parent.
                        height * 0.5 - health_bar_size.y * 0.5 - health_bar_margin + 0.6,
                        11., // Relative to parent
                    )
                    .with_scale(Vec3::new(
                        1. / layout.text_factor,
                        1. / layout.text_factor,
                        1.,
                    )),
                    ..default()
                });
            parent
                .spawn_empty()
                .insert(HeroCurrentHealthDisplay)
                .insert(Text2dBundle {
                    text: Text::from_section(
                        format!(
                            "{}/{}",
                            hero.combat_stats.health, hero.combat_stats.max_health
                        ),
                        text_style.clone(),
                    )
                    .with_alignment(text_alignment),
                    // The max size that it should fit in:
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(
                            dimens_text.x * layout.text_factor,
                            dimens_text.y * layout.text_factor,
                        ),
                    },
                    transform: Transform::from_xyz(
                        -0.5, // Centered on parent.
                        height * 0.5 - health_bar_size.y * 0.5 - health_bar_margin,
                        12., // Relative to parent
                    )
                    .with_scale(Vec3::new(
                        1. / layout.text_factor,
                        1. / layout.text_factor,
                        1.,
                    )),
                    ..default()
                });

            parent
                .spawn_empty()
                .insert(HeroStatsDisplay)
                .insert(Text2dBundle {
                    text: Text::from_section("Combat Stats", text_style.clone())
                        .with_alignment(text_alignment),
                    // The max size that it should fit in:
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(
                            dimens_text.x * layout.text_factor,
                            dimens_text.y * layout.text_factor,
                        ),
                    },
                    transform: Transform::from_xyz(
                        -1., // Centered on parent.
                        height * 0.5 - health_bar_size.y * 0.5 - health_bar_margin - 0.5,
                        11., // Relative to parent
                    )
                    .with_scale(Vec3::new(
                        1. / layout.text_factor,
                        1. / layout.text_factor,
                        1.,
                    )),
                    ..default()
                });

            parent
                .spawn_empty()
                .insert(HeroStatsDisplay)
                .insert(HeroProficiencyDisplay)
                .insert(Text2dBundle {
                    text: Text::from_section(
                        format!("Combat Proficiency: {}", hero.combat_stats.proficiency),
                        text_style_sm.clone(),
                    )
                    .with_alignment(text_alignment),
                    // The max size that it should fit in:
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(
                            dimens_text.x * layout.text_factor,
                            dimens_text.y * layout.text_factor,
                        ),
                    },
                    transform: Transform::from_xyz(
                        -2., // Centered on parent.
                        height * 0.5 - health_bar_size.y * 0.5 - health_bar_margin - 1.,
                        11., // Relative to parent
                    )
                    .with_scale(Vec3::new(
                        1. / layout.text_factor,
                        1. / layout.text_factor,
                        1.,
                    )),
                    ..default()
                });
            parent
                .spawn_empty()
                .insert(HeroStatsDisplay)
                .insert(HeroDamageResDisplay)
                .insert(Text2dBundle {
                    text: Text::from_section(
                        format!("Damage Resistance: {}", hero.combat_stats.damage_res),
                        text_style_sm.clone(),
                    )
                    .with_alignment(text_alignment),
                    // The max size that it should fit in:
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(
                            dimens_text.x * layout.text_factor,
                            dimens_text.y * layout.text_factor,
                        ),
                    },
                    transform: Transform::from_xyz(
                        -2., // Centered on parent.
                        height * 0.5 - health_bar_size.y * 0.5 - health_bar_margin - 1.5,
                        11., // Relative to parent
                    )
                    .with_scale(Vec3::new(
                        1. / layout.text_factor,
                        1. / layout.text_factor,
                        1.,
                    )),
                    ..default()
                });
            parent
                .spawn_empty()
                .insert(HeroStatsDisplay)
                .insert(HeroDamageBonusDisplay)
                .insert(Text2dBundle {
                    text: Text::from_section(
                        format!("Damage Bonus: {}", hero.combat_stats.damage_bonus),
                        text_style_sm.clone(),
                    )
                    .with_alignment(text_alignment),
                    // The max size that it should fit in:
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(
                            dimens_text.x * layout.text_factor,
                            dimens_text.y * layout.text_factor,
                        ),
                    },
                    transform: Transform::from_xyz(
                        -2., // Centered on parent.
                        height * 0.5 - health_bar_size.y * 0.5 - health_bar_margin - 2.,
                        11., // Relative to parent
                    )
                    .with_scale(Vec3::new(
                        1. / layout.text_factor,
                        1. / layout.text_factor,
                        1.,
                    )),
                    ..default()
                });
            // Equipped Items
            parent
                .spawn_empty()
                .insert(HeroStatsDisplay)
                .insert(HeroCurrentArmourDisplay)
                .insert(Text2dBundle {
                    text: Text::from_section(format!("Armour: None"), text_style_sm.clone())
                        .with_alignment(text_alignment),
                    // The max size that it should fit in:
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(
                            dimens_text.x * layout.text_factor,
                            dimens_text.y * layout.text_factor,
                        ),
                    },
                    transform: Transform::from_xyz(
                        -2., // Centered on parent.
                        height * 0.5 - health_bar_size.y * 0.5 - health_bar_margin - 2.5,
                        11., // Relative to parent
                    )
                    .with_scale(Vec3::new(
                        1. / layout.text_factor,
                        1. / layout.text_factor,
                        1.,
                    )),
                    ..default()
                });
            parent
                .spawn_empty()
                .insert(HeroStatsDisplay)
                .insert(HeroCurrentShieldDisplay)
                .insert(Text2dBundle {
                    text: Text::from_section(format!("Shield: None"), text_style_sm.clone())
                        .with_alignment(text_alignment),
                    // The max size that it should fit in:
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(
                            dimens_text.x * layout.text_factor,
                            dimens_text.y * layout.text_factor,
                        ),
                    },
                    transform: Transform::from_xyz(
                        -2., // Centered on parent.
                        height * 0.5 - health_bar_size.y * 0.5 - health_bar_margin - 3.0,
                        11., // Relative to parent
                    )
                    .with_scale(Vec3::new(
                        1. / layout.text_factor,
                        1. / layout.text_factor,
                        1.,
                    )),
                    ..default()
                });
            parent
                .spawn_empty()
                .insert(HeroStatsDisplay)
                .insert(HeroCurrentWeaponDisplay)
                .insert(Text2dBundle {
                    text: Text::from_section(format!("Weapon: None"), text_style_sm.clone())
                        .with_alignment(text_alignment),
                    // The max size that it should fit in:
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(
                            dimens_text.x * layout.text_factor,
                            dimens_text.y * layout.text_factor,
                        ),
                    },
                    transform: Transform::from_xyz(
                        -2., // Centered on parent.
                        height * 0.5 - health_bar_size.y * 0.5 - health_bar_margin - 3.5,
                        11., // Relative to parent
                    )
                    .with_scale(Vec3::new(
                        1. / layout.text_factor,
                        1. / layout.text_factor,
                        1.,
                    )),
                    ..default()
                });
            // Health Bar
            parent
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(255., 0.2, 0.2, 0.8),
                        custom_size: Some(health_bar_size),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        0., // Centered on parent.
                        height * 0.5 - health_bar_size.y * 0.5 - health_bar_margin,
                        11., // Relative to parent
                    ),
                    ..default()
                })
                .insert(Name::new("HealthBar"))
                .insert(HealthBar);

            parent
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.1, 0.1, 0.1, 1.),
                        custom_size: Some(health_bar_size),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        0.,
                        height * 0.5 - health_bar_size.y * 0.5 - health_bar_margin,
                        10., // Relative to parent
                    ),
                    ..default()
                })
                .insert(Name::new("HealthBarBackground"));
        });
}
