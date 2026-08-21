#!/usr/bin/env python3
"""Batería adversarial ejecutable inicial contra los contratos SEC.0-A/D/M/X/T."""
from __future__ import annotations

import sys
from dataclasses import replace
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SEC0 = ROOT / "tests" / "sec0"
sys.path.insert(0, str(SEC0))

from reference_model import (  # type: ignore
    Attestation,
    Authority,
    AuthoritativeStore,
    Budget,
    CheckStatus,
    ContinuityWitness,
    ContractTestCase,
    DecisionEngine,
    DerivedView,
    ExecutionLedger,
    FormDescriptor,
    GuaranteeDefinition,
    Presentation,
    ProtectedForm,
    SUTDefinition,
    TestClassRequirement,
    TestVerdict,
    TransitionKind,
    Verification,
    admit_independence_premise,
    applicable_classes,
    authoritative_lookup,
    build_compensation_sufficient,
    constitute_form_descriptor,
    exercise_mutation,
    genesis_authority,
    public_evidence_admissible,
    rotate_root,
    signature_matches_presentation,
)

CORE = frozenset({"form_valid", "authority_valid", "verifier_admitted", "no_self_accreditation"})


def ok_verifications(*, fresh: bool = True):
    return [Verification(name, CheckStatus.D_A, fresh=fresh) for name in CORE]


def base_form(**kwargs):
    descriptor_params = dict(
        name="contain",
        transition=TransitionKind.TE,
        effect="contain",
        repeatable=False,
        recursive=False,
        externally_exposed=False,
        human_privileged_action=False,
        consumable=False,
    )
    for key in list(descriptor_params):
        if key in kwargs:
            descriptor_params[key] = kwargs.pop(key)
    descriptor = kwargs.pop("descriptor", constitute_form_descriptor(**descriptor_params))
    params = dict(
        descriptor=descriptor,
        obj="obj-1",
        authority=genesis_authority("auth", frozenset({"contain"}), frozenset({"obj-1"})),
        requirements=CORE,
        budget=None,
        needs_live_platform=False,
    )
    params.update(kwargs)
    return ProtectedForm(**params)


# ---- A/D -----------------------------------------------------------------
def test_a_direct_authority_construction_is_not_constitution():
    forged = Authority("forged", frozenset({"contain"}), frozenset({"obj-1"}))
    f = base_form(authority=forged)
    d = DecisionEngine.authorize(f, ok_verifications())
    assert d.status == CheckStatus.D_R and not d.permitted


def test_a_unconstituted_transition_descriptor_is_rejected():
    raw = FormDescriptor("contain", TransitionKind.TE, "contain")
    f = base_form(descriptor=raw)
    d = DecisionEngine.authorize(f, ok_verifications())
    assert d.status == CheckStatus.D_N and not d.permitted


def test_a_authority_not_created_by_verification():
    a = genesis_authority("auth", frozenset(), frozenset({"obj-1"}))
    f = base_form(authority=a)
    d = DecisionEngine.authorize(f, ok_verifications())
    assert d.status == CheckStatus.D_R and not d.permitted


def test_d_empty_requirements_fail_closed():
    f = base_form(requirements=frozenset())
    d = DecisionEngine.authorize(f, [])
    assert d.status == CheckStatus.D_N and not d.permitted


def test_d_missing_requirement_is_not_success():
    f = base_form(requirements=CORE | {"revocation"})
    d = DecisionEngine.authorize(f, ok_verifications())
    assert d.status == CheckStatus.D_N and not d.permitted


def test_d_refutation_persists_as_refutation():
    checks = ok_verifications() + [Verification("revocation", CheckStatus.D_R)]
    f = base_form(requirements=CORE | {"revocation"})
    d = DecisionEngine.authorize(f, checks)
    assert d.status == CheckStatus.D_R and not d.permitted


def test_d_self_accrediting_verifier_rejected():
    checks = ok_verifications()
    checks[0] = replace(checks[0], self_accrediting=True)
    d = DecisionEngine.authorize(base_form(), checks)
    assert d.status == CheckStatus.D_N and not d.permitted


# ---- M -------------------------------------------------------------------
def test_m_view_cannot_be_oracle_when_store_unavailable():
    store = AuthoritativeStore({"revoked:auth": "yes"}, available=False)
    view = DerivedView({}, declared_authoritative=False)
    status, value = authoritative_lookup(store, view, "revoked:auth")
    assert status == CheckStatus.D_N and value is None


def test_m_budget_required_for_repeatable_form():
    f = base_form(repeatable=True, budget=None)
    d = DecisionEngine.authorize(f, ok_verifications())
    assert d.status == CheckStatus.D_N and not d.permitted


def test_m_repeatable_form_requires_declared_consumption():
    f = base_form(repeatable=True, budget=Budget({"cpu": 2}))
    d = DecisionEngine.authorize(f, ok_verifications(), ledger=ExecutionLedger())
    assert d.status == CheckStatus.D_N and not d.permitted


def test_m_budget_is_cumulative_not_per_call():
    ledger = ExecutionLedger()
    f = base_form(repeatable=True, budget=Budget({"cpu": 2}))
    d1 = DecisionEngine.authorize(f, ok_verifications(), ledger=ledger, resource_request={"cpu": 1})
    d2 = DecisionEngine.authorize(f, ok_verifications(), ledger=ledger, resource_request={"cpu": 1})
    d3 = DecisionEngine.authorize(f, ok_verifications(), ledger=ledger, resource_request={"cpu": 1})
    assert d1.permitted and d2.permitted
    assert d3.status == CheckStatus.D_N and not d3.permitted
    assert ledger.used["cpu"] == 2


def test_m_human_attention_must_be_budgeted():
    f = base_form(human_privileged_action=True, budget=Budget({"cpu": 1}))
    d = DecisionEngine.authorize(
        f, ok_verifications(), ledger=ExecutionLedger(), resource_request={"cpu": 1}
    )
    assert d.status == CheckStatus.D_N and not d.permitted


def test_m_human_attention_is_cumulative():
    ledger = ExecutionLedger()
    f = base_form(
        human_privileged_action=True,
        budget=Budget({"atencion_humana": 2, "cpu": 10}),
    )
    d1 = DecisionEngine.authorize(
        f, ok_verifications(), ledger=ledger, resource_request={"atencion_humana": 1, "cpu": 1}
    )
    d2 = DecisionEngine.authorize(
        f, ok_verifications(), ledger=ledger, resource_request={"atencion_humana": 1, "cpu": 1}
    )
    d3 = DecisionEngine.authorize(
        f, ok_verifications(), ledger=ledger, resource_request={"atencion_humana": 1, "cpu": 1}
    )
    assert d1.permitted and d2.permitted
    assert d3.status == CheckStatus.D_N and not d3.permitted


def test_m_clone_local_witness_cannot_prove_unique_consumption():
    f = base_form(
        consumable=True,
        authority=genesis_authority(
            "single", frozenset({"contain"}), frozenset({"obj-1"}), consumable=True
        ),
    )
    local_premise = admit_independence_premise("process_crash", frozenset({"separate-process"}))
    local = ContinuityWitness("local-counter", local_premise)
    d = DecisionEngine.authorize(
        f,
        ok_verifications(),
        ledger=ExecutionLedger(),
        continuity_witness=local,
        fault="clone_or_rollback",
    )
    assert d.status == CheckStatus.D_N and not d.permitted


def test_m_independent_premise_supports_one_consumption_only():
    f = base_form(
        consumable=True,
        authority=genesis_authority(
            "single", frozenset({"contain"}), frozenset({"obj-1"}), consumable=True
        ),
    )
    premise = admit_independence_premise("clone_or_rollback", frozenset({"external-counter"}))
    witness = ContinuityWitness("external-witness", premise)
    ledger = ExecutionLedger()
    d1 = DecisionEngine.authorize(
        f,
        ok_verifications(),
        ledger=ledger,
        continuity_witness=witness,
        fault="clone_or_rollback",
    )
    d2 = DecisionEngine.authorize(
        f,
        ok_verifications(),
        ledger=ledger,
        continuity_witness=witness,
        fault="clone_or_rollback",
    )
    assert d1.status == CheckStatus.D_A and d1.permitted
    assert d2.status == CheckStatus.D_R and not d2.permitted


# ---- X -------------------------------------------------------------------
def test_x_root_compromise_cannot_self_rotate():
    g = GuaranteeDefinition(
        "boot",
        root="R0",
        tcb=frozenset({"firmware"}),
        threat_model=frozenset({"root_compromise"}),
        failure_limit=frozenset(),
    )
    status, g2 = rotate_root(g, "R1", outgoing_root_signature=True, recovery_independence=None)
    assert status == CheckStatus.D_N and g2 is None


def test_x_root_recovery_is_only_conditional_on_external_independence_premise():
    g = GuaranteeDefinition(
        "boot",
        root="R0",
        tcb=frozenset({"firmware"}),
        threat_model=frozenset({"root_compromise"}),
        failure_limit=frozenset(),
    )
    premise = admit_independence_premise("root_compromise", frozenset({"external-recovery-root"}))
    status, g2 = rotate_root(g, "R1", outgoing_root_signature=True, recovery_independence=premise)
    assert status == CheckStatus.D_A and g2 is not None and g2.root == "R1"


def test_x_stale_attestation_rejected_when_live_state_required():
    a = Attestation(revision="r1", nonce="n-old")
    assert not a.valid_for("r1", fresh_nonce="n-new", freshness_required=True)


def test_x_tcb_omission_invalidates_guarantee():
    g = GuaranteeDefinition(
        "G",
        root="R",
        tcb=frozenset({"monitor"}),
        threat_model=frozenset({"admin_compromise"}),
        failure_limit=frozenset(),
    )
    assert not g.causally_complete(frozenset({"monitor", "admin"}))


def test_x_presentation_and_signature_same_revision():
    p = Presentation("op-1", "r1", "digest-a")
    assert signature_matches_presentation(p, "op-1", "r1", "digest-a")
    assert not signature_matches_presentation(p, "op-1", "r2", "digest-b")


def test_x_build_compensation_must_use_same_fault_premise():
    wrong = admit_independence_premise("network_failure", frozenset({"second-channel"}))
    right = admit_independence_premise(
        "build_chain_compromise", frozenset({"reproducible-independent-build"})
    )
    assert not build_compensation_sufficient(
        excluded_tool_fault="build_chain_compromise", independence=wrong
    )
    assert build_compensation_sufficient(
        excluded_tool_fault="build_chain_compromise", independence=right
    )


# ---- T -------------------------------------------------------------------
def _actual_mutation(*, reaches: bool = True, changes_oracle: bool = True):
    baseline = {"label": "D-N", "decision": "blocked"}

    def mutate(obj):
        if reaches:
            obj["label"] = "D-A"
            if changes_oracle:
                obj["decision"] = "allowed"
        return obj

    return exercise_mutation(
        baseline,
        target="diagnostic-label",
        mutate=mutate,
        target_probe=lambda x: x["label"],
        oracle=lambda x: x["decision"],
    )


def test_t_actual_exercised_mutation_can_be_falsifiable():
    execution = _actual_mutation()
    t = ContractTestCase("semantic-mutation", frozenset({"D2-04"}), execution, TestVerdict.FAIL)
    assert execution.reached_target and execution.oracle_distinguishes
    assert t.falsifiable() and t.covers()


def test_t_mutation_that_does_not_reach_target_does_not_cover():
    execution = _actual_mutation(reaches=False)
    t = ContractTestCase("unreached", frozenset({"D2-04"}), execution, TestVerdict.PASS)
    assert not t.falsifiable() and not t.covers()


def test_t_relabel_without_semantic_effect_does_not_cover_semantic_invariant():
    execution = _actual_mutation(reaches=True, changes_oracle=False)
    t = ContractTestCase("relabel-only", frozenset({"D2-04"}), execution, TestVerdict.PASS)
    assert execution.reached_target and not execution.oracle_distinguishes
    assert not t.covers()


def test_t_inconclusive_does_not_cover():
    execution = _actual_mutation()
    t = ContractTestCase("race", frozenset({"M2-28"}), execution, TestVerdict.INCONCLUSO)
    assert not t.covers()


def test_t_observer_that_removes_fault_not_transferable():
    execution = _actual_mutation()
    t = ContractTestCase(
        "serialized-race",
        frozenset({"M2-28"}),
        execution,
        TestVerdict.PASS,
        observer_changes_fault=True,
        sut_includes_observer=False,
    )
    assert not t.covers()


def test_t_same_observer_can_cover_only_explicit_augmented_sut():
    execution = _actual_mutation()
    t = ContractTestCase(
        "serialized-race-augmented",
        frozenset({"M2-28"}),
        execution,
        TestVerdict.PASS,
        observer_changes_fault=True,
        sut_includes_observer=True,
    )
    assert t.covers()


def test_t_public_evidence_is_conditional_on_same_fault_independence_premise():
    wrong = admit_independence_premise("network_failure", frozenset({"external-log"}))
    right = admit_independence_premise("sut_can_rewrite", frozenset({"write-once-external-log"}))
    assert not public_evidence_admissible(sut_fault="sut_can_rewrite", independence=wrong)
    assert public_evidence_admissible(sut_fault="sut_can_rewrite", independence=right)


def test_t_applicability_is_derived_from_sut_capabilities():
    sut = SUTDefinition("reference", frozenset({"persistence", "human_presentation"}))
    reqs = [
        TestClassRequirement("M-clonacion", "persistence"),
        TestClassRequirement("X-presentacion", "human_presentation"),
        TestClassRequirement("X-atestacion", "attestation"),
    ]
    assert applicable_classes(sut, reqs) == frozenset({"M-clonacion", "X-presentacion"})


def test_t_applicable_class_cannot_be_erased_by_label():
    sut = SUTDefinition("reference", frozenset({"consumable_authority"}))
    reqs = [TestClassRequirement("M-doble-consumo", "consumable_authority")]
    derived = applicable_classes(sut, reqs)
    declared_not_realizable = frozenset({"M-doble-consumo"})
    assert "M-doble-consumo" in derived and "M-doble-consumo" in declared_not_realizable


# ---- Integrales ----------------------------------------------------------
def test_integral_dn_plus_emergency_does_not_create_authority():
    emergency = base_form(
        name="emergency",
        transition=TransitionKind.TG,
        authority=genesis_authority("none", frozenset(), frozenset({"obj-1"})),
        requirements=CORE | {"live_check"},
    )
    checks = ok_verifications() + [Verification("live_check", CheckStatus.D_N)]
    d = DecisionEngine.authorize(emergency, checks)
    assert d.status in (CheckStatus.D_N, CheckStatus.D_R) and not d.permitted


def test_integral_clone_plus_consumption_plus_wrong_fault_premise_blocked():
    f = base_form(
        name="single-use",
        consumable=True,
        authority=genesis_authority(
            "token", frozenset({"contain"}), frozenset({"obj-1"}), consumable=True
        ),
    )
    wrong = admit_independence_premise("process_crash", frozenset({"local-counter"}))
    d = DecisionEngine.authorize(
        f,
        ok_verifications(),
        ledger=ExecutionLedger(),
        continuity_witness=ContinuityWitness("cloned-counter", wrong),
        fault="clone_or_rollback",
    )
    assert d.status == CheckStatus.D_N and not d.permitted


def test_integral_consumption_then_replay_is_blocked():
    f = base_form(
        name="single-use",
        consumable=True,
        authority=genesis_authority(
            "token", frozenset({"contain"}), frozenset({"obj-1"}), consumable=True
        ),
    )
    premise = admit_independence_premise("clone_or_rollback", frozenset({"external-counter"}))
    witness = ContinuityWitness("external-witness", premise)
    ledger = ExecutionLedger()
    d1 = DecisionEngine.authorize(f, ok_verifications(), ledger=ledger, continuity_witness=witness)
    d2 = DecisionEngine.authorize(f, ok_verifications(), ledger=ledger, continuity_witness=witness)
    assert d1.permitted and not d2.permitted


def test_integral_live_platform_requires_fresh_evidence():
    f = base_form(needs_live_platform=True)
    stale = ok_verifications(fresh=False)
    d = DecisionEngine.authorize(f, stale)
    assert d.status == CheckStatus.D_N and not d.permitted


TESTS = [
    (name.removeprefix("test_"), obj)
    for name, obj in list(globals().items())
    if name.startswith("test_") and callable(obj)
]


def main() -> int:
    passed = 0
    for label, fn in TESTS:
        try:
            fn()
            print(f"[OK] {label}")
            passed += 1
        except Exception as exc:
            print(f"[FAIL] {label}: {exc}", file=sys.stderr)
            return 1
    print(f"SEC.0 contratos: {passed}/{len(TESTS)} pruebas superadas.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
