//! The capture reducer.

use std::time::Duration;

use desktop_client::capture::{CaptureEvent, CaptureFailure, CapturePhase, Captures, ShmFormat};
use desktop_client::output::Transform;
use desktop_client::{DesktopState, Rect, Size};

use crate::support::{cap, cid, run};

fn negotiation(id: u64) -> Vec<desktop_client::DesktopEvent> {
    vec![
        cap(cid(id), CaptureEvent::BufferSize(Size::new(2, 1))),
        cap(
            cid(id),
            CaptureEvent::Format(ShmFormat::from_code(0x3231_564e)),
        ), // NV12: unreadable
        cap(cid(id), CaptureEvent::Format(ShmFormat::XRGB8888)),
        cap(cid(id), CaptureEvent::Format(ShmFormat::ARGB8888)),
        cap(cid(id), CaptureEvent::ConstraintsDone),
    ]
}

fn phase(state: &DesktopState, id: u64) -> CapturePhase {
    state.captures.get(cid(id)).expect("session").phase.clone()
}

#[test]
fn constraints_move_to_capturing() {
    let state = run(negotiation(1));
    assert_eq!(phase(&state, 1), CapturePhase::Capturing);
    let session = state.captures.get(cid(1)).unwrap();
    assert_eq!(session.size, Some(Size::new(2, 1)));
    assert_eq!(session.formats.len(), 3);
}

#[test]
fn the_phase_stays_negotiating_until_the_constraints_are_done() {
    let state = run([cap(cid(1), CaptureEvent::BufferSize(Size::new(2, 1)))]);
    assert_eq!(phase(&state, 1), CapturePhase::Negotiating);
}

#[test]
fn no_readable_format_fails_the_capture() {
    let state = run([
        cap(cid(1), CaptureEvent::BufferSize(Size::new(2, 1))),
        cap(cid(1), CaptureEvent::ConstraintsDone),
    ]);
    assert_eq!(
        phase(&state, 1),
        CapturePhase::Failed(CaptureFailure::NoShmFormat)
    );

    let state = run([
        cap(cid(1), CaptureEvent::BufferSize(Size::new(2, 1))),
        cap(
            cid(1),
            CaptureEvent::Format(ShmFormat::from_code(0x3231_564e)),
        ),
        cap(cid(1), CaptureEvent::ConstraintsDone),
    ]);
    assert_eq!(
        phase(&state, 1),
        CapturePhase::Failed(CaptureFailure::NoShmFormat)
    );
}

#[test]
fn no_size_or_an_empty_size_fails_the_capture() {
    for size in [None, Some(Size::new(0, 5))] {
        let mut events = Vec::new();
        if let Some(size) = size {
            events.push(cap(cid(1), CaptureEvent::BufferSize(size)));
        }
        events.push(cap(cid(1), CaptureEvent::Format(ShmFormat::ARGB8888)));
        events.push(cap(cid(1), CaptureEvent::ConstraintsDone));
        assert_eq!(
            phase(&run(events), 1),
            CapturePhase::Failed(CaptureFailure::BufferConstraints),
            "{size:?}"
        );
    }
}

#[test]
fn ready_builds_the_frame() {
    let mut events = negotiation(1);
    events.extend([
        cap(cid(1), CaptureEvent::Transform(Transform::Rotate90)),
        cap(cid(1), CaptureEvent::Damage(Rect::new(0, 0, 2, 1))),
        cap(cid(1), CaptureEvent::PresentationTime(Duration::new(5, 6))),
        cap(
            cid(1),
            CaptureEvent::Ready {
                stride: 8,
                pixels: vec![1; 8],
            },
        ),
    ]);
    let state = run(events);
    let CapturePhase::Ready(frame) = phase(&state, 1) else {
        panic!("expected ready");
    };
    assert_eq!(frame.size, Size::new(2, 1));
    assert_eq!(frame.stride, 8);
    // The first readable format offered wins: xrgb8888 came before argb8888.
    assert_eq!(frame.format, ShmFormat::XRGB8888);
    assert_eq!(frame.transform, Transform::Rotate90);
    assert_eq!(frame.damage, vec![Rect::new(0, 0, 2, 1)]);
    assert_eq!(frame.presented, Some(Duration::new(5, 6)));
    assert_eq!(frame.pixels, vec![1; 8]);
}

#[test]
fn ready_before_the_constraints_fails() {
    let state = run([cap(
        cid(1),
        CaptureEvent::Ready {
            stride: 4,
            pixels: vec![0; 4],
        },
    )]);
    assert_eq!(
        phase(&state, 1),
        CapturePhase::Failed(CaptureFailure::BufferConstraints)
    );
}

#[test]
fn failure_at_any_stage_is_kept() {
    let mut events = negotiation(1);
    events.push(cap(cid(1), CaptureEvent::Failed(CaptureFailure::Stopped)));
    assert_eq!(
        phase(&run(events), 1),
        CapturePhase::Failed(CaptureFailure::Stopped)
    );

    let state = run([cap(cid(2), CaptureEvent::Failed(CaptureFailure::Unknown))]);
    assert_eq!(
        phase(&state, 2),
        CapturePhase::Failed(CaptureFailure::Unknown)
    );
}

#[test]
fn a_finished_session_ignores_later_events() {
    let mut events = negotiation(1);
    events.push(cap(
        cid(1),
        CaptureEvent::Ready {
            stride: 8,
            pixels: vec![0; 8],
        },
    ));
    let ready = run(events.clone());
    events.push(cap(cid(1), CaptureEvent::Failed(CaptureFailure::Stopped)));
    events.push(cap(cid(1), CaptureEvent::Transform(Transform::Flipped)));
    assert_eq!(run(events), ready);
}

#[test]
fn take_ready_hands_out_a_finished_capture_once() {
    let mut events = negotiation(1);
    events.push(cap(
        cid(1),
        CaptureEvent::Ready {
            stride: 8,
            pixels: vec![0; 8],
        },
    ));
    events.extend(negotiation(2));
    events.push(cap(cid(3), CaptureEvent::Failed(CaptureFailure::Stopped)));
    let captures: Captures = run(events).captures;

    // Still going: not taken, not removed.
    let (captures, going) = captures.take_ready(cid(2));
    assert!(going.is_none());
    assert!(captures.get(cid(2)).is_some());

    let (captures, frame) = captures.take_ready(cid(1));
    assert!(matches!(frame, Some(Ok(_))));
    assert!(captures.get(cid(1)).is_none());
    let (captures, again) = captures.take_ready(cid(1));
    assert!(again.is_none());

    let (captures, failed) = captures.take_ready(cid(3));
    assert_eq!(failed, Some(Err(CaptureFailure::Stopped)));
    assert!(captures.get(cid(3)).is_none());

    let (_, unknown) = captures.take_ready(cid(99));
    assert!(unknown.is_none());
}
