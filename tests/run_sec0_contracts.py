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
    GuaranteeDefinition,
    Presentation,
    ProtectedForm,
    SUTDefinition,
    TestClassRequirement,
    TestVerdict,
    TransitionKind,
    Verification,
    applicable_classes,
    authoritative_lookup,
    build_compensation_sufficient,
    public_evidence_admissible,
    rotate_root,
    signature_matches_presentation,
)

CORE = frozenset({"form_valid", "authority_valid", "verifier_admitted", "no_self_accreditation"})


def ok_verifications(*, fresh: bool = True):
    return [Verification(name, CheckStatus.D_A, fresh=fresh) for name in CORE]


def base_form(**kwargs):
    params = dict(
        name="contain",
        transition=TransitionKind.TE,
        effect="contain",
        obj="obj-1",
        authority=Authority("auth", frozenset({"contain"}), frozenset({"obj-1"})),
        requirements=CORE,
    )
    params.update(kwargs)
    return ProtectedForm(**params)


# ---- A/D -----------------------------------------------------------------

def test_a_authority_not_created_by_verification():
    f = base_form(authority=Authority("auth", frozenset(), frozenset({"obj-1"})))
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
    checks = [Verification(name, CheckStatus.D_A) for name in CORE]
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


def test_m_human_attention_must_be_budgeted():
    f = base_form(human_privileged_action=True, budget=Budget({"cpu": 1}))
    d = DecisionEngine.authorize(f, ok_verifications())
    assert d.status == CheckStatus.D_N and not d.permitted


def test_m_budget_exhaustion_is_not_success():
    f = base_form(repeatable=True, budget=Budget({"cpu": 10}))
    d = DecisionEngine.authorize(f, ok_verifications(), resource_request={"cpu": 11})
    assert d.status == CheckStatus.D_N and not d.permitted


def test_m_clone_local_witness_cannot_prove_unique_consumption():
    f = base_form(consumable=True)
    local = ContinuityWitness("local-counter", frozenset())
    d = DecisionEngine.authorize(f, ok_verifications(), continuity_witness=local)
    assert d.status == CheckStatus.D_N and not d.permitted


def test_m_independent_witness_can_support_consumption_contract():
    f = base_form(consumable=True)
    w = ContinuityWitness("external-witness", frozenset({"clone_or_rollback"}))
    d = DecisionEngine.authorize(f, ok_verifications(), continuity_witness=w)
    assert d.status == CheckStatus.D_A and d.permitted


# ---- X -------------------------------------------------------------------

def test_x_root_compromise_cannot_self_rotate():
    g = GuaranteeDefinition(
        "boot",
        root="R0",
        tcb=frozenset({"firmware"}),
        threat_model=frozenset({"root_compromise"}),
        failure_limit=frozenset(),
    )
    status, g2 = rotate_root(g, "R1", outgoing_root_signature=True, independent_recovery=False)
    assert status == CheckStatus.D_N and g2 is None


def test_x_independent_root_recovery_allowed():
    g = GuaranteeDefinition(
        "boot",
        root="R0",
        tcb=frozenset({"firmware"}),
        threat_model=frozenset({"root_compromise"}),
        failure_limit=frozenset(),
    )
    status, g2 = rotate_root(g, "R1", outgoing_root_signature=True, independent_recovery=True)
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


# ---- T -------------------------------------------------------------------

def test_t_nominal_case_without_exercised_mutation_does_not_cover():
    t = ContractTestCase(
        "nominal",
        frozenset({"A2-05"}),
        mutation_specified=True,
        mutation_exercised=False,
        reached_target=False,
        oracle_distinguishes=True,
        verdict=TestVerdict.PASS,
    )
    assert not t.falsifiable() and not t.covers()


def test_t_unreached_injection_does_not_cover():
    t = ContractTestCase(
        "unreached",
        frozenset({"D2-03"}),
        mutation_specified=True,
        mutation_exercised=True,
        reached_target=False,
        oracle_distinguishes=True,
        verdict=TestVerdict.PASS,
    )
    assert not t.covers()


def test_t_inconclusive_does_not_cover():
    t = ContractTestCase(
        "race",
        frozenset({"M2-28"}),
        True, True, True, True,
        TestVerdict.INCONCLUSO,
    )
    assert not t.covers()


def test_t_observer_that_removes_fault_not_transferable():
    t = ContractTestCase(
        "serialized-race",
        frozenset({"M2-28"}),
        True, True, True, True,
        TestVerdict.PASS,
        observer_changes_fault=True,
        sut_includes_observer=False,
    )
    assert not t.covers()


def test_t_same_observer_can_cover_only_explicit_augmented_sut():
    t = ContractTestCase(
        "serialized-race-augmented",
        frozenset({"M2-28"}),
        True, True, True, True,
        TestVerdict.PASS,
        observer_changes_fault=True,
        sut_includes_observer=True,
    )
    assert t.covers()


def test_t_public_evidence_needs_independence_for_same_fault():
    assert not public_evidence_admissible(
        sut_fault="sut_can_rewrite",
        evidence_independent_faults=frozenset(),
    )
    assert public_evidence_admissible(
        sut_fault="sut_can_rewrite",
        evidence_independent_faults=frozenset({"sut_can_rewrite"}),
    )


def test_x_build_compensation_must_resist_same_fault():
    assert not build_compensation_sufficient(
        excluded_tool_fault="build_chain_compromise",
        evidence_independent_faults=frozenset({"network_failure"}),
    )
    assert build_compensation_sufficient(
        excluded_tool_fault="build_chain_compromise",
        evidence_independent_faults=frozenset({"build_chain_compromise"}),
    )


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
        authority=Authority("none", frozenset(), frozenset({"obj-1"})),
        requirements=CORE | {"live_check"},
    )
    checks = ok_verifications() + [Verification("live_check", CheckStatus.D_N)]
    d = DecisionEngine.authorize(emergency, checks)
    assert d.status in (CheckStatus.D_N, CheckStatus.D_R) and not d.permitted


def test_integral_clone_plus_consumption_plus_local_counter_blocked():
    f = base_form(
        name="single-use",
        consumable=True,
        authority=Authority("token", frozenset({"contain"}), frozenset({"obj-1"}), consumable=True),
    )
    d = DecisionEngine.authorize(
        f,
        ok_verifications(),
        continuity_witness=ContinuityWitness("cloned-counter", frozenset()),
        fault="clone_or_rollback",
    )
    assert d.status == CheckStatus.D_N and not d.permitted


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
