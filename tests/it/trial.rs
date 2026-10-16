//! The display trial's vocabulary: the trial id.

use casement_protocols::vocab::TrialId;

#[test]
fn a_trial_id_is_its_wire_number_in_serde_too() {
    let id = TrialId::new(9);
    assert_eq!(id.get(), 9);
    assert_eq!(serde_json::to_string(&id).expect("serializes"), "9");
    assert_eq!(serde_json::from_str::<TrialId>("9").expect("parses"), id);
    assert!(TrialId::new(1) < TrialId::new(2), "ids order by number");
}
