"""SV-AUTH v0.2.2 — capa estática J6 de referencia.

No ejecuta autoridad ni sustituye al runtime. Valida declaraciones de autoridad,
la tabla sellada de efectos y el lowering canónico de la especificación AUTH.
"""
from __future__ import annotations

from dataclasses import dataclass, asdict
from enum import Enum
import json
from typing import FrozenSet, Iterable, Optional, Tuple


class J6Error(Exception):
    def __init__(self, code: str, message: str):
        self.code = code
        self.message = message
        super().__init__(f"{code}: {message}")


E601 = "E601"
E602 = "E602"
E603 = "E603"
E604 = "E604"
E605 = "E605"
E606 = "E606"
E607 = "E607"
E608 = "E608"
E609 = "E609"
E610 = "E610"
E611 = "E611"
E612 = "E612"
E613 = "E613"
E614 = "E614"
E615 = "E615"
E616 = "E616"
E617 = "E617"


class PrincipalClass(str, Enum):
    HUMAN = "Human"
    SERVICE = "Service"
    EXTERNAL = "External"
    GOVERNANCE = "Governance"


class AuthorityKind(str, Enum):
    DET = "Det"
    HUMAN = "Human"
    GOV = "Gov"


class EffectClass(str, Enum):
    INFORMATIONAL = "Informational"
    BOUNDARY = "Boundary"
    CAPABILITY = "Capability"
    COMMIT = "Commit"
    GOVERNANCE = "Governance"
    CONSTITUTIONAL = "Constitutional"
    INITIALIZATION = "Initialization"


COMPONENTS = frozenset({"C", "I", "A", "Tok", "Hist"})
SENSITIVE_TYPES = frozenset({
    "AuthorityRole", "PrincipalBinding", "Grant",
    "DetToken", "HumanToken", "GovernanceToken",
    "HumanAuthorizationAct", "GovernanceAuthorizationAct",
    "SovereignDecision",
})
TOKEN_TYPES = frozenset({"DetToken", "HumanToken", "GovernanceToken"})
SOVEREIGN_OPS = frozenset({"commit_sov_u", "resolve_sov_u"})


@dataclass(frozen=True)
class AuthorityRoleSpec:
    role_id: str
    kind: AuthorityKind


@dataclass(frozen=True)
class PrincipalSpec:
    principal_id: str
    principal_class: PrincipalClass


@dataclass(frozen=True)
class PrincipalBindingSpec:
    binding_id: str
    principal_id: str
    role_id: str


@dataclass(frozen=True)
class VerifierSpec:
    verifier_id: str
    resolver: str
    can_certify_nonclosure: bool = False


@dataclass(frozen=True)
class GrantSpec:
    grant_id: str
    principal_id: str
    kind: AuthorityKind
    operations: FrozenSet[str]
    scopes: FrozenSet[str]
    objects: FrozenSet[str]
    resolver: Optional[str] = None
    verifier_id: Optional[str] = None
    requires_verified_basis: bool = True


@dataclass(frozen=True)
class RuleEffectSpec:
    name: str
    effect_class: EffectClass
    reads: FrozenSet[str]
    writes: FrozenSet[str]
    introduces: FrozenSet[str]
    requires_subject_principal_match: bool = False


@dataclass(frozen=True)
class AuthorityStaticProgram:
    roles: Tuple[AuthorityRoleSpec, ...]
    principals: Tuple[PrincipalSpec, ...]
    bindings: Tuple[PrincipalBindingSpec, ...]
    verifiers: Tuple[VerifierSpec, ...]
    grants: Tuple[GrantSpec, ...]
    rules: Tuple[RuleEffectSpec, ...]
    version: int = 1


SEALED_RULES = {
    "INFO": RuleEffectSpec("INFO", EffectClass.INFORMATIONAL, frozenset({"I"}), frozenset({"I"}), frozenset()),
    "VERIFY": RuleEffectSpec("VERIFY", EffectClass.INFORMATIONAL, frozenset({"C", "I"}), frozenset({"I"}), frozenset({"VerifiedCertificate"})),
    "REQUEST_HUMAN": RuleEffectSpec("REQUEST_HUMAN", EffectClass.INFORMATIONAL, frozenset({"I"}), frozenset({"I"}), frozenset({"HumanReviewRequest"})),
    "ADMIT_HUMAN_ACT": RuleEffectSpec("ADMIT_HUMAN_ACT", EffectClass.BOUNDARY, frozenset({"C", "A", "I", "Hist"}), frozenset({"Hist"}), frozenset({"HumanAuthorizationAct"})),
    "MINT_DET": RuleEffectSpec("MINT_DET", EffectClass.CAPABILITY, frozenset({"C", "A", "I", "Tok", "Hist"}), frozenset({"Tok", "Hist"}), frozenset({"DetToken"})),
    "MINT_HUMAN": RuleEffectSpec("MINT_HUMAN", EffectClass.CAPABILITY, frozenset({"C", "A", "I", "Tok", "Hist"}), frozenset({"Tok", "Hist"}), frozenset({"HumanToken"})),
    "ADMIT_GOV_ACT": RuleEffectSpec("ADMIT_GOV_ACT", EffectClass.BOUNDARY, frozenset({"C", "A", "Hist"}), frozenset({"Hist"}), frozenset({"GovernanceAuthorizationAct"})),
    "MINT_GOV": RuleEffectSpec("MINT_GOV", EffectClass.CAPABILITY, frozenset({"C", "A", "Tok", "Hist"}), frozenset({"Tok", "Hist"}), frozenset({"GovernanceToken"})),
    "COMMIT_DET": RuleEffectSpec("COMMIT_DET", EffectClass.COMMIT, frozenset({"C", "A", "I", "Tok", "Hist"}), frozenset({"Tok", "Hist"}), frozenset({"CommittedDecision"}), True),
    "COMMIT_SOV_U": RuleEffectSpec("COMMIT_SOV_U", EffectClass.COMMIT, frozenset({"C", "A", "I", "Tok", "Hist"}), frozenset({"Tok", "Hist"}), frozenset({"SovereignDecision"}), True),
    "RESOLVE_SOV_U": RuleEffectSpec("RESOLVE_SOV_U", EffectClass.COMMIT, frozenset({"C", "A", "I", "Tok", "Hist"}), frozenset({"Tok", "Hist"}), frozenset({"SovereignDecision"}), True),
    "GOV_BIND": RuleEffectSpec("GOV_BIND", EffectClass.GOVERNANCE, frozenset({"C", "A", "Tok", "Hist"}), frozenset({"A", "Tok", "Hist"}), frozenset({"PrincipalBinding"})),
    "GOV_GRANT": RuleEffectSpec("GOV_GRANT", EffectClass.GOVERNANCE, frozenset({"C", "A", "Tok", "Hist"}), frozenset({"A", "Tok", "Hist"}), frozenset({"Grant"})),
    "CONSTITUTION_REVISION": RuleEffectSpec("CONSTITUTION_REVISION", EffectClass.CONSTITUTIONAL, frozenset({"C", "A", "Tok", "Hist"}), frozenset({"C", "Tok", "Hist"}), frozenset()),
    "RESTORE": RuleEffectSpec("RESTORE", EffectClass.INITIALIZATION, frozenset({"C", "I", "A", "Hist"}), frozenset({"C", "I", "A", "Hist", "Tok"}), frozenset()),
}


def _unique(items: Iterable[str], label: str) -> None:
    seen = set()
    for item in items:
        if item in seen:
            raise J6Error(E602, f"duplicate {label}: {item}")
        seen.add(item)


def _role_compatible(principal_class: PrincipalClass, kind: AuthorityKind) -> bool:
    if kind == AuthorityKind.HUMAN:
        return principal_class == PrincipalClass.HUMAN
    if kind == AuthorityKind.GOV:
        return principal_class == PrincipalClass.GOVERNANCE
    return principal_class in {PrincipalClass.SERVICE, PrincipalClass.EXTERNAL}


def validate_authority_static(program: AuthorityStaticProgram) -> None:
    if program.version < 1:
        raise J6Error(E605, "authority version must be >= 1")

    _unique((r.role_id for r in program.roles), "role")
    _unique((p.principal_id for p in program.principals), "principal")
    _unique((b.binding_id for b in program.bindings), "binding")
    _unique((v.verifier_id for v in program.verifiers), "verifier")
    _unique((g.grant_id for g in program.grants), "grant")
    _unique((r.name for r in program.rules), "rule")

    role_by_id = {r.role_id: r for r in program.roles}
    principal_by_id = {p.principal_id: p for p in program.principals}
    binding_by_principal = {b.principal_id: b for b in program.bindings}
    verifier_by_id = {v.verifier_id: v for v in program.verifiers}

    for b in program.bindings:
        if b.principal_id not in principal_by_id or b.role_id not in role_by_id:
            raise J6Error(E601, f"binding {b.binding_id} references undeclared authority identity")
        p = principal_by_id[b.principal_id]
        role = role_by_id[b.role_id]
        if not _role_compatible(p.principal_class, role.kind):
            raise J6Error(E601, f"binding {b.binding_id} coerces {p.principal_class.value} into {role.kind.value}")

    for g in program.grants:
        p = principal_by_id.get(g.principal_id)
        b = binding_by_principal.get(g.principal_id)
        if p is None or b is None:
            raise J6Error(E601, f"grant {g.grant_id} lacks declared principal/binding")
        role = role_by_id[b.role_id]
        if role.kind != g.kind:
            raise J6Error(E603, f"grant {g.grant_id} kind exceeds or contradicts bound role")
        if g.kind != AuthorityKind.HUMAN and "commit_sov_u" in g.operations:
            raise J6Error(E609, f"non-human grant {g.grant_id} authorizes commit_sov_u")
        if g.kind != AuthorityKind.HUMAN and "resolve_sov_u" in g.operations:
            raise J6Error(E610, f"non-human grant {g.grant_id} authorizes resolve_sov_u")
        if g.requires_verified_basis:
            if not g.verifier_id or g.verifier_id not in verifier_by_id:
                raise J6Error(E607, f"grant {g.grant_id} requires undeclared verified basis")
            verifier = verifier_by_id[g.verifier_id]
            if g.resolver is None or verifier.resolver != g.resolver:
                raise J6Error(E607, f"grant {g.grant_id} verifier/resolver mismatch")
        if "commit_sov_u" in g.operations:
            verifier = verifier_by_id.get(g.verifier_id or "")
            if verifier is None or not verifier.can_certify_nonclosure:
                raise J6Error(E607, f"sovereign-U grant {g.grant_id} lacks nonclosure verifier")

    declared = {r.name: r for r in program.rules}
    if set(declared) != set(SEALED_RULES):
        missing = sorted(set(SEALED_RULES) - set(declared))
        extra = sorted(set(declared) - set(SEALED_RULES))
        raise J6Error(E615, f"sealed rule set mismatch; missing={missing}, extra={extra}")

    for name, rule in declared.items():
        if not rule.reads <= COMPONENTS or not rule.writes <= COMPONENTS:
            raise J6Error(E615, f"rule {name} references unknown state component")
        expected = SEALED_RULES[name]
        if rule != expected:
            raise J6Error(E615, f"rule {name} diverges from sealed A.2 effect signature")
        if rule.effect_class == EffectClass.INFORMATIONAL:
            if not rule.writes <= {"I", "Hist"}:
                raise J6Error(E614, f"informational rule {name} writes authority/constitution")
            if rule.introduces & SENSITIVE_TYPES:
                raise J6Error(E614, f"informational rule {name} introduces authority-sensitive type")
        if "Grant" in rule.introduces and rule.effect_class != EffectClass.GOVERNANCE:
            raise J6Error(E602, f"rule {name} introduces Grant outside governance")
        if rule.introduces & TOKEN_TYPES and rule.effect_class != EffectClass.CAPABILITY:
            raise J6Error(E603, f"rule {name} introduces token outside capability mint")
        if name in {"COMMIT_DET", "COMMIT_SOV_U", "RESOLVE_SOV_U"} and not rule.requires_subject_principal_match:
            raise J6Error(E604, f"rule {name} lacks subject=principal(token) precondition")


def canonical_lower(program: AuthorityStaticProgram) -> str:
    validate_authority_static(program)
    payload = {
        "authority_version": program.version,
        "roles": [asdict(x) for x in sorted(program.roles, key=lambda x: x.role_id)],
        "principals": [asdict(x) for x in sorted(program.principals, key=lambda x: x.principal_id)],
        "bindings": [asdict(x) for x in sorted(program.bindings, key=lambda x: x.binding_id)],
        "verifiers": [asdict(x) for x in sorted(program.verifiers, key=lambda x: x.verifier_id)],
        "grants": [
            {
                "grant_id": x.grant_id,
                "principal_id": x.principal_id,
                "kind": x.kind.value,
                "operations": sorted(x.operations),
                "scopes": sorted(x.scopes),
                "objects": sorted(x.objects),
                "resolver": x.resolver,
                "verifier_id": x.verifier_id,
                "requires_verified_basis": x.requires_verified_basis,
            }
            for x in sorted(program.grants, key=lambda x: x.grant_id)
        ],
        "sealed_rules": [
            {
                "name": x.name,
                "effect_class": x.effect_class.value,
                "reads": sorted(x.reads),
                "writes": sorted(x.writes),
                "introduces": sorted(x.introduces),
                "requires_subject_principal_match": x.requires_subject_principal_match,
            }
            for x in sorted(program.rules, key=lambda x: x.name)
        ],
    }
    return json.dumps(payload, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def sealed_rules_tuple() -> Tuple[RuleEffectSpec, ...]:
    return tuple(SEALED_RULES[name] for name in sorted(SEALED_RULES))
