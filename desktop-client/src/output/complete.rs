//! Completing a configuration for the wire.

use super::config::{HeadChangeRequest, HeadConfig, HeadSettings, ModeChoice, OutputConfig};
use super::model::Head;
use super::set::Outputs;
use super::units::Enabled;

/// The parts to send for `config`: each mentioned head with every setting it leaves open filled
/// from the head's current state, then every enabled head the configuration does not mention,
/// kept as it is (the protocol wants every enabled head in a configuration).
///
/// The mode of a head with no current mode falls back to its preferred mode, then its first;
/// a head with no modes has no mode set. Adaptive sync is set only for a head that reports it.
/// Heads `config` names but `outputs` lacks are passed through untouched (validate first).
pub fn complete(config: &OutputConfig, outputs: &Outputs) -> Vec<HeadConfig> {
    let mut parts: Vec<HeadConfig> = config
        .heads
        .iter()
        .map(|part| match (&part.request, outputs.get(part.head)) {
            (HeadChangeRequest::Enable(settings), Some(head)) => {
                HeadConfig::new(part.head, HeadChangeRequest::Enable(fill(settings, head)))
            }
            _ => part.clone(),
        })
        .collect();
    for head in outputs.heads() {
        let mentioned = config.heads.iter().any(|p| p.head == head.key);
        if !mentioned && head.enabled == Enabled::On {
            parts.push(HeadConfig::new(
                head.key,
                HeadChangeRequest::Enable(fill(&HeadSettings::keep(), head)),
            ));
        }
    }
    parts
}

fn fill(settings: &HeadSettings, head: &Head) -> HeadSettings {
    let mode = settings.mode.clone().or_else(|| current_mode(head));
    HeadSettings {
        mode,
        position: settings.position.or(Some(head.position)),
        transform: settings.transform.or(Some(head.transform)),
        scale: settings.scale.or(Some(head.scale)),
        adaptive_sync: settings.adaptive_sync.or(head.adaptive_sync),
    }
}

fn current_mode(head: &Head) -> Option<ModeChoice> {
    let key = head
        .current_mode
        .or_else(|| {
            head.modes
                .iter()
                .find(|m| m.preferred == super::units::Preferred::Yes)
                .map(|m| m.key)
        })
        .or_else(|| head.modes.first().map(|m| m.key))?;
    Some(ModeChoice::Existing(key))
}
