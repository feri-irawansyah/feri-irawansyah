use crate::markdown::{CIRCUIT_FAILURE_THRESHOLD, GithubCircuit};

#[test]
fn closed_initially_allows_requests() {
    let circuit = GithubCircuit::new();
    assert!(circuit.allow_request());
}

#[test]
fn stays_closed_below_failure_threshold() {
    let circuit = GithubCircuit::new();
    for _ in 0..(CIRCUIT_FAILURE_THRESHOLD - 1) {
        circuit.record_transient_failure();
    }
    assert!(circuit.allow_request());
}

#[test]
fn opens_after_reaching_failure_threshold() {
    let circuit = GithubCircuit::new();
    for _ in 0..CIRCUIT_FAILURE_THRESHOLD {
        circuit.record_transient_failure();
    }
    assert!(!circuit.allow_request());
}

#[test]
fn success_resets_failure_count_and_closes_the_circuit() {
    let circuit = GithubCircuit::new();
    for _ in 0..CIRCUIT_FAILURE_THRESHOLD {
        circuit.record_transient_failure();
    }
    assert!(!circuit.allow_request());

    circuit.record_success();
    assert!(circuit.allow_request());
}
