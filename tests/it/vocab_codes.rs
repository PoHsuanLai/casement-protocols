//! Every vocab enum against the enum wayland-scanner generates from the XML, on both sides:
//! the same codes, the same variant names.

use std::fmt::Debug;

use casement_protocols::vocab::{self, MacMods, WireCode};
use casement_protocols::{client, server};

/// Codes past every enum's last, so a code only one side knows shows up.
const PROBE: std::ops::Range<u32> = 0..64;

/// `V` and `G` accept the same codes and give each the same variant name.
fn same_codes<V, G>()
where
    V: WireCode + Debug,
    G: TryFrom<u32> + Into<u32> + Copy + Debug,
{
    let what = (std::any::type_name::<V>(), std::any::type_name::<G>());
    for code in PROBE.chain([u32::MAX]) {
        let ours = V::from_code(code).ok().map(|value| format!("{value:?}"));
        let generated = G::try_from(code).ok();
        assert_eq!(
            ours,
            generated.map(|value| format!("{value:?}")),
            "{what:?} at code {code}"
        );
        if let Some(value) = generated {
            assert_eq!(value.into(), code, "{what:?} round trip at {code}");
        }
    }
    assert!(!V::ALL.is_empty(), "{what:?} has values");
}

/// One table for both sides: `$side` is `client` or `server`.
macro_rules! check_side {
    ($side:ident) => {{
        use $side::quire_agent_v1::{
            quire_agent_lease_v1 as lease, quire_agent_manager_v1 as agent,
        };
        use $side::quire_desktop_v1::{
            quire_desktop_app_v1 as app, quire_desktop_manager_v1 as desktop,
            quire_desktop_space_v1 as space, quire_desktop_window_v1 as window,
        };
        use $side::quire_sensitive_v1::quire_sensitive_surface_v1 as sensitive;
        use $side::quire_shell_v1::quire_shell_v1 as shell;
        use $side::quire_trusted_surface_v1::{
            quire_trusted_manager_v1 as trusted, quire_trusted_surface_v1 as trusted_surface,
        };
        use $side::quire_window_v1::quire_window_v1 as quire_window;

        same_codes::<vocab::ActorKind, desktop::ActorKind>();
        same_codes::<vocab::ProofKind, app::ProofKind>();
        same_codes::<vocab::AppLifeKind, app::AppLifeKind>();
        same_codes::<vocab::AppVisibility, app::AppVisibility>();
        same_codes::<vocab::MenuSourceKind, app::MenuSourceKind>();
        same_codes::<vocab::Attention, app::Attention>();
        same_codes::<vocab::Modality, window::Modality>();
        same_codes::<vocab::WindowStateKind, window::WindowStateKind>();
        same_codes::<vocab::PlaceKind, window::PlaceKind>();
        same_codes::<vocab::Responsiveness, window::Responsiveness>();
        same_codes::<vocab::LeaseState, window::LeaseState>();
        same_codes::<vocab::Toggle, window::Toggle>();
        same_codes::<vocab::Assign, window::Assign>();
        same_codes::<vocab::SpaceKind, space::SpaceKind>();
        same_codes::<vocab::SpaceActivity, space::SpaceActivity>();
        same_codes::<vocab::ShellRole, shell::ShellRole>();
        same_codes::<vocab::KeyGesture, shell::KeyGesture>();
        same_codes::<vocab::OverviewMode, shell::OverviewMode>();
        same_codes::<vocab::OverviewState, shell::OverviewState>();
        same_codes::<vocab::Corner, shell::Corner>();
        same_codes::<vocab::ScreenEdge, shell::ScreenEdge>();
        same_codes::<vocab::Shown, shell::Shown>();
        same_codes::<vocab::WindowEntry, shell::WindowEntry>();
        same_codes::<vocab::SplitSide, shell::SplitSide>();
        same_codes::<vocab::DragOutcome, shell::DragOutcome>();
        same_codes::<vocab::Power, shell::Power>();
        same_codes::<vocab::SubjectKind, shell::SubjectKind>();
        same_codes::<vocab::Memory, shell::Memory>();
        same_codes::<vocab::Answer, shell::Answer>();
        same_codes::<vocab::TrialAnswer, shell::TrialAnswer>();
        same_codes::<vocab::WindowRole, quire_window::WindowRole>();
        same_codes::<vocab::TileZone, quire_window::TileZone>();
        same_codes::<vocab::PlaceTarget, quire_window::PlaceTarget>();
        same_codes::<vocab::PlacementMark, quire_window::PlacementMark>();
        same_codes::<vocab::Emphasis, quire_window::Emphasis>();
        same_codes::<vocab::FrameMode, quire_window::FrameMode>();
        same_codes::<vocab::Edited, quire_window::Edited>();
        same_codes::<vocab::SensitivityLevel, sensitive::SensitivityLevel>();
        same_codes::<vocab::RunMode, agent::LeaseMode>();
        same_codes::<vocab::RevokeReason, lease::RevokeReason>();
        same_codes::<vocab::Suspend, lease::SuspendReason>();
        same_codes::<vocab::GlowCode, lease::GlowState>();
        same_codes::<vocab::MaskReason, lease::MaskReason>();
        same_codes::<vocab::TrustedKind, trusted::TrustedKind>();
        same_codes::<vocab::TrustedAnchor, trusted::TrustedAnchor>();
        same_codes::<vocab::Withdrawn, trusted_surface::Withdrawn>();

        let mods = [
            (MacMods::NONE, shell::MacMods::None),
            (MacMods::SHIFT, shell::MacMods::Shift),
            (MacMods::CONTROL, shell::MacMods::Control),
            (MacMods::OPTION, shell::MacMods::Option),
            (MacMods::COMMAND, shell::MacMods::Command),
            (MacMods::CAPS_LOCK, shell::MacMods::CapsLock),
            (MacMods::FUNCTION, shell::MacMods::Function),
        ];
        for (ours, generated) in mods {
            assert_eq!(ours.0, generated.bits(), "{generated:?}");
        }
        let every = mods
            .iter()
            .fold(MacMods::NONE, |set, (ours, _)| set.with(*ours));
        assert_eq!(every.0, shell::MacMods::all().bits());
    }};
}

#[test]
fn client_enums_have_the_vocab_codes_and_names() {
    check_side!(client);
}

#[test]
fn server_enums_have_the_vocab_codes_and_names() {
    check_side!(server);
}

#[test]
fn cross_protocol_enum_arguments_use_the_owning_protocols_type() {
    use client::quire_desktop_v1::quire_desktop_window_v1::Event;
    use wayland_client::WEnum;
    // The desktop window's lease glow is the agent lease's GlowState, not a copy of it.
    let glow: WEnum<client::quire_agent_v1::quire_agent_lease_v1::GlowState> =
        WEnum::Value(client::quire_agent_v1::quire_agent_lease_v1::GlowState::Acting);
    let event = Event::Lease {
        state: WEnum::Value(client::quire_desktop_v1::quire_desktop_window_v1::LeaseState::Granted),
        glow,
    };
    let Event::Lease { glow, .. } = event else {
        unreachable!("built as Lease")
    };
    assert_eq!(
        glow.into_result().ok().map(u32::from),
        Some(vocab::GlowCode::Acting.code())
    );
}
