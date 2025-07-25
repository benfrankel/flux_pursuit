use crate::animation::shake::NodeShake;
use crate::deck::PlayerDeck;
use crate::hud::HudConfig;
use crate::phase::Phase;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<FluxLabel>();
}

pub(super) fn flux_display(hud_config: &HudConfig) -> impl Bundle {
    (
        Name::new("FluxDisplay"),
        Node {
            width: Vw(22.5),
            height: Vw(5.0),
            border: UiRect::all(Vw(0.2083)),
            ..Node::ROW.center()
        },
        ThemeColor::Monitor.set::<BackgroundColor>(),
        ThemeColor::MonitorDimText.set::<BorderColor>(),
        Tooltip::fixed(
            Anchor::CenterRight,
            parse_rich(
                "[b]Flux multiplier[r]\n\nChain \"reactor modules\" together to multiply their output.",
            ),
        ),
        children![(
            FluxLabel,
            widget::colored_label(default(), ""),
            hud_config.flux_label_shake,
        )],
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct FluxLabel;

impl Configure for FluxLabel {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            sync_flux_label
                .in_set(UpdateSystems::SyncLate)
                .run_if(resource_changed::<PlayerDeck>.or(any_match_filter::<Added<FluxLabel>>)),
        );
        app.add_systems(StateFlush, Phase::ANY.on_enter(sync_flux_display_to_phase));
    }
}

fn sync_flux_display_to_phase(
    phase: NextRef<Phase>,
    mut label_query: Query<&ChildOf, With<FluxLabel>>,
    mut border_query: Query<&mut ThemeColorFor<BorderColor>>,
) {
    for child_of in &mut label_query {
        let mut border_color = cq!(border_query.get_mut(child_of.parent()));
        border_color.0 = if phase.will_be_in(&state!(Phase::Reactor | Phase::Player)) {
            ThemeColor::MonitorText
        } else {
            ThemeColor::MonitorDimText
        };
    }
}

fn sync_flux_label(
    hud_config: ConfigRef<HudConfig>,
    player_deck: Res<PlayerDeck>,
    mut label_query: Query<
        (&mut RichText, &mut ThemeColorForText, &mut NodeShake),
        With<FluxLabel>,
    >,
) {
    let hud_config = r!(hud_config.get());
    for (mut text, mut text_color, mut shake) in &mut label_query {
        text_color.0 = if player_deck.flux > f32::EPSILON {
            vec![ThemeColor::MonitorText]
        } else {
            vec![ThemeColor::MonitorDimText]
        };

        let new_text = RichText::from_sections(parse_rich(format!("flux {}x", player_deck.flux)));
        if !text.sections.is_empty() && text.sections[0].value != new_text.sections[0].value {
            shake.trauma += hud_config
                .flux_label_flux_trauma
                .sample_clamped(player_deck.flux);
        }
        *text = new_text;
    }
}
