"""Modelo ejecutable de referencia para los contratos SEC.0 del Lenguaje SV.

Este módulo NO es un entorno de ejecución de producción ni modifica la semántica del
Lenguaje SV. Materializa un subconjunto comprobable de las obligaciones abstractas de
SEC.0-A/D/M/X/T para construir casos adversariales falsables.
"""
from __future__ import annotations

from dataclasses import dataclass, field, replace
from enum import Enum
from typing import FrozenSet, Mapping, Optional, Sequence, Tuple


class CheckStatus(str, Enum):
    D_A = "D-A"
    D_R = "D-R"
    D_N = "D-N"


class TransitionKind(str, Enum):
    T0 = "T-0"
    TI = "T-I"
    TV = "T-V"
    TH = "T-H"
    TE = "T-E"
    TG = "T-G"
    TC = "T-C"
    TR = "T-R"


class TestVerdict(str, Enum):
    PASS = "PASS"
    FAIL = "FAIL"
    NO_EJECUTADO = "NO_EJECUTADO"
    NO_PROBADO = "NO_PROBADO"
    INCONCLUSO = "INCONCLUSO"


@dataclass(frozen=True)
class Budget:
    limits: Mapping[str, int]

    def valid(self) -> bool:
        return bool(self.limits) and all(isinstance(v, int) and v > 0 for v in self.limits.values())

    def admits(self, requested: Mapping[str, int]) -> bool:
        if not self.valid():
            return False
        for resource, amount in requested.items():
            limit = self.limits.get(resource)
            if limit is None or amount < 0 or amount > limit:
                return False
        return True


@dataclass(frozen=True)
class Authority:
    name: str
    effects: FrozenSet[str]
    domain: FrozenSet[str]
    active: bool = True
    consumable: bool = False

    def permits(self, effect: str, obj: str) -> bool:
        return self.active and effect in self.effects and obj in self.domain


@dataclass(frozen=True)
class Verification:
    name: str
    status: CheckStatus
    admitted: bool = True
    applicable: bool = True
    fresh: bool = True
    self_accrediting: bool = False


CORE_REQUIREMENTS = frozenset({"form_valid", "authority_valid", "verifier_admitted", "no_self_accreditation"})


@dataclass(frozen=True)
class ProtectedForm:
    name: str
    transition: TransitionKind
    effect: str
    obj: str
    authority: Authority
    requirements: FrozenSet[str]
    repeatable: bool = False
    recursive: bool = False
    externally_exposed: bool = False
    human_privileged_action: bool = False
    consumable: bool = False
    budget: Optional[Budget] = None
    needs_live_platform: bool = False

    def controlled(self) -> bool:
        return self.transition != TransitionKind.T0

    def structurally_valid(self) -> bool:
        if not self.controlled():
            return True
        if not self.requirements:
            return False
        if not CORE_REQUIREMENTS.issubset(self.requirements):
            return False
        budget_required = self.repeatable or self.recursive or self.externally_exposed or self.human_privileged_action
        if budget_required and (self.budget is None or not self.budget.valid()):
            return False
        if self.human_privileged_action and "atencion_humana" not in (self.budget.limits if self.budget else {}):
            return False
        return True


@dataclass(frozen=True)
class ContinuityWitness:
    name: str
    independent_faults: FrozenSet[str]

    def independent_for(self, fault: str) -> bool:
        return fault in self.independent_faults


@dataclass(frozen=True)
class Decision:
    status: CheckStatus
    permitted: bool
    reason: str


class DecisionEngine:
    """Aplicación mínima de fallo cerrado y autoridad preconstituida."""

    @staticmethod
    def authorize(
        form: ProtectedForm,
        verifications: Sequence[Verification],
        *,
        resource_request: Optional[Mapping[str, int]] = None,
        continuity_witness: Optional[ContinuityWitness] = None,
        fault: str = "clone_or_rollback",
    ) -> Decision:
        if not form.structurally_valid():
            return Decision(CheckStatus.D_N, False, "forma o presupuesto no acreditable")
        if not form.authority.permits(form.effect, form.obj):
            return Decision(CheckStatus.D_R, False, "autoridad inexistente, inactiva o fuera de alcance")

        by_name = {v.name: v for v in verifications}
        for required in form.requirements:
            v = by_name.get(required)
            if v is None:
                return Decision(CheckStatus.D_N, False, f"falta obligación: {required}")
            if not v.admitted or not v.applicable:
                return Decision(CheckStatus.D_N, False, f"verificador no admisible: {required}")
            if v.self_accrediting:
                return Decision(CheckStatus.D_N, False, f"acreditación propia: {required}")
            if form.needs_live_platform and not v.fresh:
                return Decision(CheckStatus.D_N, False, f"evidencia no fresca: {required}")
            if v.status == CheckStatus.D_R:
                return Decision(CheckStatus.D_R, False, f"obligación refutada: {required}")
            if v.status == CheckStatus.D_N:
                return Decision(CheckStatus.D_N, False, f"obligación no verificable: {required}")

        if resource_request:
            if form.budget is None or not form.budget.admits(resource_request):
                return Decision(CheckStatus.D_N, False, "presupuesto de recursos excedido o ausente")

        if form.consumable or form.authority.consumable:
            if continuity_witness is None or not continuity_witness.independent_for(fault):
                return Decision(CheckStatus.D_N, False, "unicidad de consumo no acreditable")

        return Decision(CheckStatus.D_A, True, "todas las obligaciones aplicables acreditadas")


@dataclass(frozen=True)
class AuthoritativeStore:
    values: Mapping[str, str]
    available: bool = True

    def read(self, key: str) -> Tuple[CheckStatus, Optional[str]]:
        if not self.available:
            return CheckStatus.D_N, None
        return CheckStatus.D_A, self.values.get(key)


@dataclass(frozen=True)
class DerivedView:
    values: Mapping[str, str]
    declared_authoritative: bool = False


def authoritative_lookup(store: AuthoritativeStore, view: DerivedView, key: str) -> Tuple[CheckStatus, Optional[str]]:
    """Una vista no autoritativa puede localizar, nunca decidir por sí sola."""
    if view.declared_authoritative:
        return CheckStatus.D_N, None
    return store.read(key)


@dataclass(frozen=True)
class GuaranteeDefinition:
    name: str
    root: str
    tcb: FrozenSet[str]
    threat_model: FrozenSet[str]
    failure_limit: FrozenSet[str]
    evidence: FrozenSet[str] = field(default_factory=frozenset)

    def causally_complete(self, actual_falsifiers: FrozenSet[str]) -> bool:
        return actual_falsifiers.issubset(self.tcb)


def rotate_root(
    guarantee: GuaranteeDefinition,
    new_root: str,
    *,
    outgoing_root_signature: bool,
    independent_recovery: bool,
) -> Tuple[CheckStatus, Optional[GuaranteeDefinition]]:
    compromise_in_scope = "root_compromise" in guarantee.threat_model
    suspected = "root_suspected" in guarantee.threat_model
    if compromise_in_scope or suspected:
        if not independent_recovery:
            return CheckStatus.D_N, None
        return CheckStatus.D_A, replace(guarantee, root=new_root)
    if outgoing_root_signature:
        return CheckStatus.D_A, replace(guarantee, root=new_root)
    return CheckStatus.D_N, None


@dataclass(frozen=True)
class Attestation:
    revision: str
    nonce: Optional[str]
    admitted: bool = True

    def valid_for(self, revision: str, *, fresh_nonce: Optional[str], freshness_required: bool) -> bool:
        if not self.admitted or self.revision != revision:
            return False
        if freshness_required:
            return self.nonce is not None and fresh_nonce is not None and self.nonce == fresh_nonce
        return True


@dataclass(frozen=True)
class Presentation:
    object_id: str
    revision: str
    rendered_digest: str


def signature_matches_presentation(presentation: Presentation, object_id: str, revision: str, rendered_digest: str) -> bool:
    return (
        presentation.object_id == object_id
        and presentation.revision == revision
        and presentation.rendered_digest == rendered_digest
    )


@dataclass(frozen=True)
class SUTDefinition:
    name: str
    capabilities: FrozenSet[str]

    def has(self, capability: str) -> bool:
        return capability in self.capabilities


@dataclass(frozen=True)
class TestClassRequirement:
    name: str
    capability: str

    def applicable_to(self, sut: SUTDefinition) -> bool:
        return sut.has(self.capability)


def applicable_classes(sut: SUTDefinition, requirements: Sequence[TestClassRequirement]) -> FrozenSet[str]:
    return frozenset(r.name for r in requirements if r.applicable_to(sut))


def build_compensation_sufficient(*, excluded_tool_fault: str, evidence_independent_faults: FrozenSet[str]) -> bool:
    """Una comprobación sólo excluye una herramienta si resiste el mismo fallo."""
    return excluded_tool_fault in evidence_independent_faults


@dataclass(frozen=True)
class ContractTestCase:
    name: str
    targets: FrozenSet[str]
    mutation_specified: bool
    mutation_exercised: bool
    reached_target: bool
    oracle_distinguishes: bool
    verdict: TestVerdict
    observer_changes_fault: bool = False
    sut_includes_observer: bool = False

    def falsifiable(self) -> bool:
        return (
            bool(self.targets)
            and self.mutation_specified
            and self.mutation_exercised
            and self.reached_target
            and self.oracle_distinguishes
        )

    def covers(self) -> bool:
        if not self.falsifiable():
            return False
        if self.verdict not in (TestVerdict.PASS, TestVerdict.FAIL):
            return False
        if self.observer_changes_fault and not self.sut_includes_observer:
            return False
        return True


def public_evidence_admissible(*, sut_fault: str, evidence_independent_faults: FrozenSet[str]) -> bool:
    return sut_fault in evidence_independent_faults
