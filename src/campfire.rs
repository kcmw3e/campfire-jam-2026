use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_easy_gif::*;

use crate::GameState;
use crate::background::on_add_background;
use crate::y_sort::{DEFAULT_POS, DEFAULT_Z, YSort, z_indices};

pub struct CampfirePlugin;

impl Plugin for CampfirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (CampfireMeter::update, CampfireFuelBurn::update),
        )
        .add_systems(OnEnter(GameState::Campsite), Campfire::setup)
        .add_systems(OnExit(GameState::Campsite), Campfire::teardown)
        .add_observer(CampfireMeter::setup);
    }
}

/// The campfire is the central part of the game that the player must interact
/// with. It will burn through fuel over time.
///
/// Campfire fuel represents the number of seconds (in-game time) that the fire
/// has left before it dies and the game is over. Different fuels will have
/// different effects on the campfire. Some will simply prolong the campfire's
/// life by increasing its fuel (up to the capacity), others may increase the
/// capacity itself.
#[derive(Component)]
#[component(on_add = on_add_background)]
pub struct Campfire {
    /// The amount of fuel the campfire has left to burn.
    fuel: f32,

    /// The maximum amount of fuel the campfire can hold.
    capacity: f32,
}

/// This component causes the campfire to burn through its fuel based on a burn
/// timer.
#[derive(Component)]
pub struct CampfireFuelBurn {
    /// The timer that controls how often fuel is burned. Every time the timer
    /// is exhausted, some fuel will be removed from the campfire.
    burn_timer: Timer,
}

/// The visual aspect of the campfire, showing how much fuel is left.
#[derive(Component)]
pub struct CampfireMeter {
    percent: f32,
}

impl Campfire {
    /// The default amount of starting fuel for the campfire.
    const STARTING_FUEL: f32 = 60.; // 1 minute

    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            (
                Gif {
                    handle: asset_server.load("campfire/campfire_preview.gif"),
                },
                Anchor::BOTTOM_CENTER,
                Transform::from_translation(DEFAULT_POS.extend(DEFAULT_Z)),
            ),
            Campfire::default(),
            YSort { z: z_indices::MIDGROUND },
        ));
    }

    fn teardown(mut commands: Commands, query: Query<Entity, With<Campfire>>) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

impl CampfireFuelBurn {
    /// The default burn rate of the campfire, in seconds.
    const BURN_RATE: f32 = 1.;

    fn update(
        query: Query<(&mut Campfire, &mut CampfireFuelBurn)>,
        time: Res<Time>,
    ) {
        for (mut campfire, mut fuel_burn) in query {
            fuel_burn.burn_timer.tick(time.delta());

            if fuel_burn.burn_timer.just_finished() {
                campfire.fuel -= 1.;
            }
        }
    }
}

impl CampfireMeter {
    const WIDTH: Val = Val::Px(250.);
    const HEIGHT: Val = Val::Px(40.);
    const BORDER_WIDTH: UiRect = UiRect::all(Val::Px(3.));

    const FILL_COLOR: Color = Color::linear_rgb(0.3, 0.1, 0.1);
    const BORDER_COLOR: Color = Color::linear_rgb(0.7, 0.7, 0.7);

    /// Whenever a campfire is spawned, add a campfire meter to the it to
    /// display the amount of fuel left.
    fn setup(add: On<Add, Campfire>, mut commands: Commands) {
        commands
            .entity(add.entity)
            .insert((CampfireMeter::default(), CampfireFuelBurn::default()))
            .insert((
                Node {
                    width: Self::WIDTH,
                    height: Self::HEIGHT,
                    border: Self::BORDER_WIDTH,
                    ..default()
                },
                BackgroundColor(Color::BLACK),
                BorderColor::all(Self::BORDER_COLOR),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Self::FILL_COLOR),
                ));
            });
    }

    /// Update the campfire meter's fill percent with the campfire's current
    /// state.
    fn update(
        query: Query<(&mut CampfireMeter, &Campfire, &Children), With<Node>>,
        mut fill_query: Query<&mut Node>,
    ) {
        for (mut meter, campfire, children) in query {
            meter.percent = campfire.fuel / campfire.capacity * 100.;
            if let Some(&fill) = children.first() {
                if let Ok(mut fill) = fill_query.get_mut(fill) {
                    fill.width = Val::Percent(meter.percent);
                }
            }
        }
    }
}

impl Default for Campfire {
    fn default() -> Self {
        Self { fuel: Self::STARTING_FUEL, capacity: Self::STARTING_FUEL }
    }
}

impl Default for CampfireFuelBurn {
    fn default() -> Self {
        Self {
            burn_timer: Timer::from_seconds(
                Self::BURN_RATE,
                TimerMode::Repeating,
            ),
        }
    }
}

impl Default for CampfireMeter {
    fn default() -> Self {
        Self { percent: 100. }
    }
}
