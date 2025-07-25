//! Custom theming and UI tools.

#![allow(dead_code)]

pub mod color;
pub mod grid;
pub mod interaction;
pub mod text;
pub mod toast;
pub mod tooltip;
pub mod widget;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::color::ThemeColor;
    pub use super::color::ThemeColorFor;
    pub use super::color::ThemeColorForText;
    pub use super::grid::GridAlignment;
    pub use super::interaction::InteractionDisabled;
    pub use super::interaction::InteractionGlassSfx;
    pub use super::interaction::InteractionSfx;
    pub use super::interaction::InteractionTheme;
    pub use super::interaction::ParentInteractionTheme;
    pub use super::interaction::TargetInteractionTheme;
    pub use super::text::BOLD_FONT_HANDLE;
    pub use super::text::DynamicFontSize;
    pub use super::text::FONT_HANDLE;
    pub use super::text::parse_rich;
    pub use super::text::parse_rich_custom;
    pub use super::widget;
}

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ThemeAssets>();

    app.add_plugins((
        color::plugin,
        grid::plugin,
        interaction::plugin,
        text::plugin,
        toast::plugin,
        tooltip::plugin,
    ));
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct ThemeAssets {
    #[asset(path = "audio/sfx/251390__deadsillyrabbit__button_hover-mp3.ogg")]
    pub sfx_hover: Handle<AudioSource>,
    #[asset(path = "audio/sfx/253168__suntemple__sfx-ui-button-click.ogg")]
    pub sfx_click: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Movement SFX 3.ogg")]
    pub sfx_hover_glass: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Click SFX 1.ogg")]
    pub sfx_click_glass: Handle<AudioSource>,
}

impl Configure for ThemeAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}
