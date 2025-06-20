mod credits;
mod defeat;
mod help;
mod intro;
mod loading;
mod main;
mod pause;
mod settings;
mod upgrade;
mod victory;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(MenuRoot, Menu, MenuAction, MenuTime)>();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct MenuRoot {
    pub ui: Entity,
}

impl Configure for MenuRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for MenuRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            ui: world
                .spawn((
                    Name::new("MenuUi"),
                    Node::DEFAULT.full_size(),
                    GlobalZIndex(2),
                    Pickable::IGNORE,
                    DespawnOnExitState::<Menu>::Descendants,
                ))
                .id(),
        }
    }
}

#[derive(State, Reflect, Copy, Clone, Eq, PartialEq, Debug)]
#[state(before(Pause), next(NextStateStack<Self>), react, log_flush)]
#[reflect(Resource)]
pub enum Menu {
    Main,
    Intro,
    Settings,
    Credits,
    Loading,
    Help,
    Pause,
    Upgrade,
    Defeat,
    Victory,
}

impl Configure for Menu {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_systems(
            StateFlush,
            (
                Menu::ANY.on_enable(Pause::enable_default),
                Menu::ANY.on_disable(Pause::disable),
            ),
        );
        app.add_plugins((
            main::plugin,
            intro::plugin,
            settings::plugin,
            credits::plugin,
            loading::plugin,
            help::plugin,
            pause::plugin,
            upgrade::plugin,
            defeat::plugin,
            victory::plugin,
        ));
    }
}

#[derive(Actionlike, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MenuAction {
    Back,
}

impl Configure for MenuAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .with(Self::Back, GamepadButton::South)
                .with(Self::Back, KeyCode::Escape),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            Menu::pop
                .in_set(UpdateSystems::RecordInput)
                .run_if(Menu::is_enabled.and(action_just_pressed(Self::Back))),
        );
    }
}

/// The total time elapsed in the current menu.
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct MenuTime(pub Duration);

impl Configure for MenuTime {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(StateFlush, Menu::ANY.on_exit(reset_menu_time));
        app.add_systems(
            Update,
            tick_menu_time
                .in_set(UpdateSystems::TickTimers)
                .run_if(Menu::is_enabled),
        );
    }
}

fn reset_menu_time(mut menu_time: ResMut<MenuTime>) {
    *menu_time = default();
}

fn tick_menu_time(time: Res<Time>, mut menu_time: ResMut<MenuTime>) {
    menu_time.0 += time.delta();
}
