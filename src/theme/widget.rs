use bevy::ecs::system::IntoObserverSystem;

use crate::animation::backup::Backup;
use crate::animation::offset::NodeOffset;
use crate::prelude::*;
use crate::theme::color::Rainbow;

pub fn nonblocking_overlay(z: i32) -> impl Bundle {
    (
        Name::new("NonblockingOverlay"),
        Node::DEFAULT.full_size().abs(),
        Pickable::IGNORE,
        GlobalZIndex(z),
    )
}

pub fn blocking_overlay(z: i32) -> impl Bundle {
    (
        Name::new("BlockingOverlay"),
        Node::DEFAULT.full_size().abs(),
        FocusPolicy::Block,
        GlobalZIndex(z),
    )
}

pub fn dimming_overlay() -> impl Bundle {
    (
        Name::new("DimmingOverlay"),
        Node::ROW_CENTER.full_size(),
        ThemeColor::Overlay.set::<BackgroundColor>(),
        FocusPolicy::Block,
    )
}

pub fn rainbow_overlay() -> impl Bundle {
    (
        Name::new("RainbowOverlay"),
        Node::DEFAULT.full_size().abs(),
        Pickable::IGNORE,
        ThemeColor::RainbowOverlay.set::<BackgroundColor>(),
        Rainbow { rate: 0.2 },
    )
}

pub fn popup(children: impl Bundle) -> impl Bundle {
    (
        Name::new("PopupContainer"),
        Node {
            padding: UiRect::all(Vw(5.0)),
            ..Node::COLUMN_CENTER.full_size()
        },
        children![(
            Name::new("Popup"),
            Node {
                display: Display::Block,
                padding: UiRect::all(Vw(3.5)),
                border: UiRect::all(Px(1.0)),
                ..Node::DEFAULT.full_size()
            },
            ThemeColor::Popup.set::<BackgroundColor>(),
            BorderRadius::all(Vw(3.0)),
            ThemeColor::BorderColor.set::<BorderColor>(),
            BoxShadow::from(ShadowStyle {
                x_offset: Val::ZERO,
                y_offset: Val::ZERO,
                ..default()
            }),
            FocusPolicy::Block,
            children,
        )],
    )
}

pub fn body(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Body"),
        Node {
            display: Display::Block,
            padding: UiRect::all(Vw(3.5)),
            ..Node::DEFAULT.full_size()
        },
        children,
    )
}

pub fn center(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Center"),
        Node::COLUMN_CENTER.full_size(),
        children,
    )
}

pub fn column_of_buttons(children: impl Bundle) -> impl Bundle {
    (
        Name::new("ColumnOfButtons"),
        Node {
            margin: UiRect::vertical(Vw(2.5)),
            row_gap: Vw(2.5),
            ..Node::COLUMN_CENTER
        },
        children,
    )
}

pub fn row_of_buttons(children: impl Bundle) -> impl Bundle {
    (
        Name::new("RowOfButtons"),
        Node {
            margin: UiRect::vertical(Vw(2.5)),
            column_gap: Vw(2.5),
            ..Node::ROW_CENTER
        },
        children,
    )
}

pub fn stretch(children: impl Bundle) -> impl Bundle {
    (Name::new("Stretch"), Node::ROW_CENTER.grow(), children)
}

pub fn header(text: impl AsRef<str>) -> impl Bundle {
    (
        label_base(Vw(5.0), ThemeColor::BodyText, text),
        Node {
            margin: UiRect::bottom(Vw(4.2)),
            ..default()
        },
    )
}

pub fn big_label(text: impl AsRef<str>) -> impl Bundle {
    label_base(Vw(5.0), ThemeColor::BodyText, text)
}

pub fn label(text: impl AsRef<str>) -> impl Bundle {
    label_base(Vw(3.5), ThemeColor::BodyText, text)
}

pub fn small_label(text: impl AsRef<str>) -> impl Bundle {
    label_base(Vw(2.0), ThemeColor::BodyText, text)
}

pub fn small_colored_label(text: impl AsRef<str>, color: ThemeColor) -> impl Bundle {
    label_base(Vw(2.0), color, text)
}

pub fn colored_label(text: impl AsRef<str>, color: ThemeColor) -> impl Bundle {
    label_base(Vw(3.5), color, text)
}

pub fn paragraph(text: &'static str) -> impl Bundle {
    (
        Name::new("Paragraph"),
        Node {
            margin: UiRect::top(Vw(4.0)).with_bottom(Vw(5.2)),
            row_gap: Vw(1.4),
            ..Node::COLUMN_MID
        },
        Children::spawn(SpawnIter(text.lines().map(label))),
    )
}

pub fn label_base(font_size: Val, text_color: ThemeColor, text: impl AsRef<str>) -> impl Bundle {
    let text = text.as_ref();
    (
        Name::new(format!("Label(\"{text}\")")),
        RichText::from_sections(parse_rich(text)).with_justify(JustifyText::Center),
        DynamicFontSize::new(font_size).with_step(8.0),
        ThemeColorForText(vec![text_color]),
    )
}

pub fn small_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    button_base(Vw(3.0), Vw(4.0), Vw(3.0), text, action)
}

pub fn button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    button_base(Vw(30.0), Vw(7.0), Vw(3.0), text, action)
}

pub fn wide_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    button_base(Vw(38.0), Vw(7.0), Vw(3.0), text, action)
}

pub fn big_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    button_base(Vw(38.0), Vw(10.0), Vw(4.0), text, action)
}

fn button_base<E, B, M, I>(
    width: Val,
    height: Val,
    font_size: Val,
    text: impl Into<String>,
    action: I,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    (
        Name::new(format!("Button(\"{text}\")")),
        Button,
        Node {
            width,
            height,
            ..Node::ROW_CENTER
        },
        BorderRadius::all(Vw(1.0)),
        ThemeColor::default().set::<BackgroundColor>(),
        InteractionTheme {
            none: ThemeColor::Primary.set::<BackgroundColor>(),
            hovered: ThemeColor::PrimaryHovered.set::<BackgroundColor>(),
            pressed: ThemeColor::PrimaryPressed.set::<BackgroundColor>(),
            disabled: ThemeColor::PrimaryDisabled.set::<BackgroundColor>(),
        },
        NodeOffset::default(),
        InteractionTheme {
            hovered: NodeOffset::new(Val::ZERO, Vw(-0.5)),
            pressed: NodeOffset::new(Val::ZERO, Vw(0.5)),
            ..default()
        },
        BoxShadow::from(ShadowStyle {
            color: Color::BLACK.with_alpha(0.5),
            x_offset: Val::ZERO,
            y_offset: Vw(0.7),
            spread_radius: Vw(0.5),
            blur_radius: Vw(0.5),
        }),
        Backup::<BoxShadow>::default(),
        InteractionSfx,
        children![(
            label_base(font_size, ThemeColor::PrimaryText, text),
            Pickable::IGNORE,
        )],
        InteractionDisabled(false),
        Patch(|entity| {
            entity.observe(action);
        }),
    )
}

pub fn selector<E1, B1, M1, I1, C, E2, B2, M2, I2>(
    marker: C,
    left_action: I1,
    right_action: I2,
) -> impl Bundle
where
    C: Component,
    E1: Event,
    B1: Bundle,
    I1: Sync + IntoObserverSystem<E1, B1, M1>,
    E2: Event,
    B2: Bundle,
    I2: Sync + IntoObserverSystem<E2, B2, M2>,
{
    (
        Name::new("Selector"),
        Node {
            width: Vw(35.0),
            ..Node::ROW
        },
        marker,
        children![
            small_button("<", left_action),
            stretch(children![label("")]),
            small_button(">", right_action),
        ],
    )
}

pub fn loading_bar<S: State + Clone + PartialEq + Eq + Hash + Debug>() -> impl Bundle {
    (
        Name::new("LoadingBar"),
        Node {
            width: Percent(60.0),
            height: Vw(4.0),
            margin: UiRect::all(Vw(1.0)).with_top(Vw(1.1)),
            padding: UiRect::all(Vw(0.5)),
            border: UiRect::all(Vw(0.5)),
            ..default()
        },
        ThemeColor::BodyText.set::<BorderColor>(),
        children![(
            Name::new("LoadingBarFill"),
            Node::DEFAULT.full_height(),
            ThemeColor::Primary.set::<BackgroundColor>(),
            IsLoadingBarFill::<S>(PhantomData),
        )],
    )
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct IsLoadingBarFill<S: State + Clone + PartialEq + Eq + Hash + Debug>(
    #[reflect(ignore)] PhantomData<S>,
);

impl<S: State + Clone + PartialEq + Eq + Hash + Debug + TypePath> Configure
    for IsLoadingBarFill<S>
{
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            update_loading_bar_fill::<S>.in_set(UpdateSystems::SyncLate),
        );
    }
}

fn update_loading_bar_fill<S: State + Clone + PartialEq + Eq + Hash + Debug>(
    progress: Res<ProgressTracker<BevyState<S>>>,
    mut fill_query: Query<&mut Node, With<IsLoadingBarFill<S>>>,
) {
    let Progress { done, total } = progress.get_global_combined_progress();
    for mut node in &mut fill_query {
        node.width = Percent(100.0 * done as f32 / total as f32);
    }
}
