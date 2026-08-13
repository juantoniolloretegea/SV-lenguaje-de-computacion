from dataclasses import replace
import json
import pytest

from svp_authority_static import *


def valid_program():
    roles = (
        AuthorityRoleSpec("role-human", AuthorityKind.HUMAN),
        AuthorityRoleSpec("role-service", AuthorityKind.DET),
        AuthorityRoleSpec("role-gov", AuthorityKind.GOV),
    )
    principals = (
        PrincipalSpec("alice", PrincipalClass.HUMAN),
        PrincipalSpec("svc", PrincipalClass.SERVICE),
        PrincipalSpec("gov", PrincipalClass.GOVERNANCE),
    )
    bindings = (
        PrincipalBindingSpec("b-alice", "alice", "role-human"),
        PrincipalBindingSpec("b-svc", "svc", "role-service"),
        PrincipalBindingSpec("b-gov", "gov", "role-gov"),
    )
    verifiers = (
        VerifierSpec("v-det", "resolver-main", False),
        VerifierSpec("v-u", "resolver-u", True),
    )
    grants = (
        GrantSpec("g-det", "svc", AuthorityKind.DET, frozenset({"commit_det"}), frozenset({"scope"}), frozenset({"obj"}), "resolver-main", "v-det"),
        GrantSpec("g-human", "alice", AuthorityKind.HUMAN, frozenset({"commit_sov_u", "resolve_sov_u"}), frozenset({"scope"}), frozenset({"obj"}), "resolver-u", "v-u"),
        GrantSpec("g-gov", "gov", AuthorityKind.GOV, frozenset({"bind_principal", "grant_add", "constitution_revision"}), frozenset({"authority"}), frozenset({"auth"}), None, None, False),
    )
    return AuthorityStaticProgram(roles, principals, bindings, verifiers, grants, sealed_rules_tuple())


def code(exc):
    return exc.value.code


def test_j6_valid_reference_program():
    validate_authority_static(valid_program())


def test_j6_01_illegal_authority_coercion_rejected():
    p = valid_program()
    bad = replace(p, bindings=(replace(p.bindings[0], principal_id="svc"), p.bindings[1], p.bindings[2]))
    with pytest.raises(J6Error) as e: validate_authority_static(bad)
    assert code(e) == E601


def test_j6_02_duplicate_grant_rejected():
    p = valid_program()
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, grants=p.grants + (p.grants[0],)))
    assert code(e) == E602


def test_j6_03_grant_kind_escalation_rejected():
    p = valid_program()
    bad = replace(p.grants[0], kind=AuthorityKind.HUMAN)
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, grants=(bad,) + p.grants[1:]))
    assert code(e) == E603


def test_j6_04_nonhuman_sovereign_commit_rejected():
    p = valid_program()
    bad = replace(p.grants[0], operations=frozenset({"commit_sov_u"}), resolver="resolver-u", verifier_id="v-u")
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, grants=(bad,) + p.grants[1:]))
    assert code(e) == E609


def test_j6_05_nonhuman_sovereign_resolution_rejected():
    p = valid_program()
    bad = replace(p.grants[0], operations=frozenset({"resolve_sov_u"}), resolver="resolver-u", verifier_id="v-u")
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, grants=(bad,) + p.grants[1:]))
    assert code(e) == E610


def test_j6_06_verified_basis_requires_declared_verifier():
    p = valid_program()
    bad = replace(p.grants[0], verifier_id="missing")
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, grants=(bad,) + p.grants[1:]))
    assert code(e) == E607


def test_j6_07_sovereign_u_requires_nonclosure_verifier():
    p = valid_program()
    bad = replace(p.grants[1], verifier_id="v-det", resolver="resolver-main")
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, grants=(p.grants[0], bad, p.grants[2])))
    assert code(e) == E607


def test_j6_08_missing_sealed_rule_rejected():
    p = valid_program()
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, rules=p.rules[:-1]))
    assert code(e) == E615


def test_j6_09_extra_sensitive_rule_rejected():
    p = valid_program()
    extra = RuleEffectSpec("MINT_GRANT_ORDINARY", EffectClass.CAPABILITY, frozenset({"I"}), frozenset({"A"}), frozenset({"Grant"}))
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, rules=p.rules + (extra,)))
    assert code(e) == E615


def test_j6_10_informational_authority_write_rejected():
    p = valid_program()
    rules = list(p.rules)
    i = next(i for i,r in enumerate(rules) if r.name == "INFO")
    rules[i] = replace(rules[i], writes=frozenset({"I","A"}))
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, rules=tuple(rules)))
    assert code(e) in {E614, E615}


def test_j6_11_commit_requires_subject_principal_match():
    p = valid_program()
    rules = list(p.rules)
    i = next(i for i,r in enumerate(rules) if r.name == "COMMIT_DET")
    rules[i] = replace(rules[i], requires_subject_principal_match=False)
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, rules=tuple(rules)))
    assert code(e) in {E604, E615}


def test_j6_12_grant_only_governance_rule_can_introduce_grant():
    p = valid_program()
    rules = list(p.rules)
    i = next(i for i,r in enumerate(rules) if r.name == "MINT_DET")
    rules[i] = replace(rules[i], introduces=frozenset({"DetToken","Grant"}))
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, rules=tuple(rules)))
    assert code(e) in {E602, E615}


def test_j6_13_token_only_capability_rules_introduce_tokens():
    p = valid_program()
    rules = list(p.rules)
    i = next(i for i,r in enumerate(rules) if r.name == "INFO")
    rules[i] = replace(rules[i], introduces=frozenset({"DetToken"}))
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, rules=tuple(rules)))
    assert code(e) in {E603, E614, E615}


def test_j6_14_unknown_component_rejected():
    p = valid_program()
    rules = list(p.rules)
    i = next(i for i,r in enumerate(rules) if r.name == "VERIFY")
    rules[i] = replace(rules[i], reads=frozenset({"C","I","ROOT"}))
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, rules=tuple(rules)))
    assert code(e) == E615


def test_j6_15_canonical_lower_is_deterministic():
    p = valid_program()
    a = canonical_lower(p)
    b = canonical_lower(p)
    assert a == b
    decoded = json.loads(a)
    assert decoded["authority_version"] == 1
    assert len(decoded["sealed_rules"]) == 15


def test_j6_16_lowering_contains_no_runtime_tokens_or_acts_as_values():
    payload = json.loads(canonical_lower(valid_program()))
    s = json.dumps(payload, sort_keys=True)
    assert '"token_id"' not in s
    assert '"act_id"' not in s


def test_j6_17_rule_set_exactly_matches_a2_seal():
    p = valid_program()
    assert {r.name for r in p.rules} == set(SEALED_RULES)
    for name in {"COMMIT_DET","COMMIT_SOV_U","RESOLVE_SOV_U"}:
        assert SEALED_RULES[name].requires_subject_principal_match is True


def test_j6_18_external_det_binding_is_legal_but_not_sovereign():
    p = valid_program()
    ext = PrincipalSpec("external-ai", PrincipalClass.EXTERNAL)
    b = PrincipalBindingSpec("b-ext", "external-ai", "role-service")
    g = GrantSpec("g-ext", "external-ai", AuthorityKind.DET, frozenset({"commit_det"}), frozenset({"scope"}), frozenset({"obj"}), "resolver-main", "v-det")
    q = replace(p, principals=p.principals+(ext,), bindings=p.bindings+(b,), grants=p.grants+(g,))
    validate_authority_static(q)


def test_j6_19_external_human_binding_is_rejected():
    p = valid_program()
    ext = PrincipalSpec("external-ai", PrincipalClass.EXTERNAL)
    b = PrincipalBindingSpec("b-ext", "external-ai", "role-human")
    q = replace(p, principals=p.principals+(ext,), bindings=p.bindings+(b,))
    with pytest.raises(J6Error) as e: validate_authority_static(q)
    assert code(e) == E601


def test_j6_20_stale_or_zero_version_rejected():
    p = valid_program()
    with pytest.raises(J6Error) as e: validate_authority_static(replace(p, version=0))
    assert code(e) == E605
