"""Modelo ejecutable de referencia para los contratos SEC.0 del Lenguaje SV.

Este módulo NO es un entorno de ejecución de producción ni modifica la semántica del
Lenguaje SV. Materializa un subconjunto comprobable de obligaciones abstractas de
SEC.0-A/D/M/X/T para construir casos adversariales falsables.

Las propiedades que dependen de independencia física, autenticidad criptográfica,
persistencia real o atestación material se representan únicamente como premisas
externas explícitas; este modelo no pretende demostrarlas desde Python.
"""
from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass, field, replace
from enum import Enum
from typing import Any, Callable, FrozenSet, Mapping, Optional, Sequence, Tuple


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


@dataclass
class ExecutionLedger:
    """Estado mínimo acumulativo de consumo de recursos."""

    used: dict[str, int] = field(default_factory=dict)

    def can_reserve(self, budget: Budget, requested: Mapping[str, int]) -> bool:
        if not budget.valid() or not requested:
            return False
        for resource, amount in requested.items():
            if not isinstance(amount, int) or amount < 0:
                return False
            limit = budget.limits.get(resource)
            if limit is None:
                return False
            if self.used.get(resource, 0) + amount > limit:
                return False
        return True

    def reserve(self, requested: Mapping[str, int]) -> None:
        for resource, amount in requested.items():
            self.used[resource] = self.used.get(resource, 0) + amount


_AUTHORITY_SEAL = object()
_FORM_SEAL = object()
_INDEPENDENCE_SEAL = object()
_MUTATION_SEAL = object()


@dataclass(frozen=True)
class Authority:
    name: str
    effects: FrozenSet[str]
    domain: FrozenSet[str]
    active: bool = True
    consumable: bool = False
    _seal: object | None = field(default=None, repr=False, compare=False)

    @property
    def constituted(self) -> bool:
        return self._seal is _AUTHORITY_SEAL

    def permits(self, effect: str, obj: str) -> bool:
        return self.constituted and self.active and effect in self.effects and obj in self.domain


def genesis_authority(
    name: str,
    effects: FrozenSet[str],
    domain: FrozenSet[str],
    *,
    active: bool = True,
    consumable: bool = False,
) -> Authority:
    """Representa una autoridad ya constituida por una T-0 admitida fuera del modelo."""
    return Authority(name, effects, domain, active=active, consumable=consumable, _seal=_AUTHORITY_SEAL)


def governed_authority(
    source: Authority,
    name: str,
    effects: FrozenSet[str],
    domain: FrozenSet[str],
    *,
    consumable: bool = False,
) -> Authority:
    """Representa una autoridad nacida por una transición gobernada admitida.

    Esta función no concede poder por sí misma: exige una autoridad fuente ya constituida y
    sólo permite delegación no amplificadora dentro de su envolvente representada.
    """
    if not source.constituted or not source.active:
        raise ValueError("la autoridad fuente no está constituida")
    if not effects.issubset(source.effects) or not domain.issubset(source.domain):
        raise ValueError("la delegación ampliaría la autoridad fuente")
    return Authority(name, effects, domain, consumable=consumable, _seal=_AUTHORITY_SEAL)


@dataclass(frozen=True)
class Verification:
    """Resultado de verificación precomputado.

    El modelo evalúa su efecto de control; no ejecuta ni autentica el verificador material.
    """

    name: str
    status: CheckStatus
    admitted: bool = True
    applicable: bool = True
    fresh: bool = True
    self_accrediting: bool = False


CORE_REQUIREMENTS = frozenset({"form_valid", "authority_valid", "verifier_admitted", "no_self_accreditation"})


@dataclass(frozen=True)
class FormDescriptor:
    name: str
    transition: TransitionKind
    effect: str
    repeatable: bool = False
    recursive: bool = False
    externally_exposed: bool = False
    human_privileged_action: bool = False
    consumable: bool = False
    _seal: object | None = field(default=None, repr=False, compare=False)

    @property
    def constituted(self) -> bool:
        return self._seal is _FORM_SEAL

    @property
    def budget_required(self) -> bool:
        return self.repeatable or self.recursive or self.externally_exposed or self.human_privileged_action


def constitute_form_descriptor(
    name: str,
    transition: TransitionKind,
    effect: str,
    *,
    repeatable: bool = False,
    recursive: bool = False,
    externally_exposed: bool = False,
    human_privileged_action: bool = False,
    consumable: bool = False,
) -> FormDescriptor:
    return FormDescriptor(
        name,
        transition,
        effect,
        repeatable=repeatable,
        recursive=recursive,
        externally_exposed=externally_exposed,
        human_privileged_action=human_privileged_action,
        consumable=consumable,
        _seal=_FORM_SEAL,
    )


@dataclass(frozen=True)
class ProtectedForm:
    descriptor: FormDescriptor
    obj: str
    authority: Authority
    requirements: FrozenSet[str]
    budget: Optional[Budget] = None
    needs_live_platform: bool = False

    @property
    def name(self) -> str:
        return self.descriptor.name

    @property
    def transition(self) -> TransitionKind:
        return self.descriptor.transition

    @property
    def effect(self) -> str:
        return self.descriptor.effect

    @property
    def consumable(self) -> bool:
        return self.descriptor.consumable

    def controlled(self) -> bool:
        return self.transition != TransitionKind.T0

    def structurally_valid(self) -> bool:
        if not self.descriptor.constituted:
            return False
        if not self.controlled():
            return True
        if not self.requirements or not CORE_REQUIREMENTS.issubset(self.requirements):
            return False
        if self.descriptor.budget_required and (self.budget is None or not self.budget.valid()):
            return False
        if self.descriptor.human_privileged_action:
            if self.budget is None or "atencion_humana" not in self.budget.limits:
                return False
        return True


@dataclass(frozen=True)
class IndependencePremise:
    """Premisa externa admitida; no prueba independencia física dentro del modelo."""

    fault: str
    basis: FrozenSet[str]
    _seal: object | None = field(default=None, repr=False, compare=False)

    @property
    def admitted(self) -> bool:
        return self._seal is _INDEPENDENCE_SEAL and bool(self.basis)


def admit_independence_premise(fault: str, basis: FrozenSet[str]) -> IndependencePremise:
    if not fault or not basis:
        raise ValueError("la premisa de independencia exige fallo y fundamento externos")
    return IndependencePremise(fault, basis, _seal=_INDEPENDENCE_SEAL)


@dataclass
class ContinuityWitness:
    """Testigo lógico de continuidad condicionado a una premisa externa de independencia."""

    name: str
    independence: IndependencePremise
    consumed_authorities: set[str] = field(default_factory=set)

    def independent_for(self, fault: str) -> bool:
        return self.independence.admitted and self.independence.fault == fault

    def consume_authority(self, name: str) -> bool:
        if name in self.consumed_authorities:
            return False
        self.consumed_authorities.add(name)
        return True


@dataclass(frozen=True)
class Decision:
    status: CheckStatus
    permitted: bool
    reason: str


class DecisionEngine:
    """Aplicación mínima de fallo cerrado, autoridad constituida y consumo acumulativo."""

    @staticmethod
    def authorize(
        form: ProtectedForm,
        verifications: Sequence[Verification],
        *,
        resource_request: Optional[Mapping[str, int]] = None,
        ledger: Optional[ExecutionLedger] = None,
        continuity_witness: Optional[ContinuityWitness] = None,
        fault: str = "clone_or_rollback",
    ) -> Decision:
        if not form.structurally_valid():
            return Decision(CheckStatus.D_N, False, "forma, descriptor o presupuesto no acreditable")
        if not form.authority.constituted:
            return Decision(CheckStatus.D_R, False, "autoridad no constituida")
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

        if form.descriptor.budget_required:
            if form.budget is None or resource_request is None or ledger is None:
                return Decision(CheckStatus.D_N, False, "presupuesto acumulativo no acreditable")
            if form.descriptor.human_privileged_action and resource_request.get("atencion_humana", 0) <= 0:
                return Decision(CheckStatus.D_N, False, "consumo de atención humana no declarado")
            if not ledger.can_reserve(form.budget, resource_request):
                return Decision(CheckStatus.D_N, False, "presupuesto acumulativo excedido")
        elif resource_request:
            if form.budget is None or ledger is None or not ledger.can_reserve(form.budget, resource_request):
                return Decision(CheckStatus.D_N, False, "presupuesto de recursos excedido o ausente")

        if form.consumable or form.authority.consumable:
            if continuity_witness is None or not continuity_witness.independent_for(fault):
                return Decision(CheckStatus.D_N, False, "unicidad de consumo no acreditable")
            if form.authority.name in continuity_witness.consumed_authorities:
                return Decision(CheckStatus.D_R, False, "autoridad ya consumida")

        if resource_request:
            assert ledger is not None
            ledger.reserve(resource_request)
        if form.consumable or form.authority.consumable:
            assert continuity_witness is not None
            if not continuity_witness.consume_authority(form.authority.name):
                return Decision(CheckStatus.D_R, False, "autoridad ya consumida")

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
    recovery_independence: Optional[IndependencePremise] = None,
) -> Tuple[CheckStatus, Optional[GuaranteeDefinition]]:
    compromise_in_scope = "root_compromise" in guarantee.threat_model
    suspected = "root_suspected" in guarantee.threat_model
    if compromise_in_scope or suspected:
        if recovery_independence is None or not recovery_independence.admitted:
            return CheckStatus.D_N, None
        if recovery_independence.fault not in {"root_compromise", "root_suspected"}:
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
    """Comprueba únicamente la ligadura material de revisión; no autentica una firma."""
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


def build_compensation_sufficient(*, excluded_tool_fault: str, independence: Optional[IndependencePremise]) -> bool:
    """Conclusión condicional a una premisa externa de independencia frente al mismo fallo."""
    return independence is not None and independence.admitted and independence.fault == excluded_tool_fault


@dataclass(frozen=True)
class MutationExecution:
    target: str
    before_target: Any
    after_target: Any
    baseline_observed: Any
    mutant_observed: Any
    _seal: object | None = field(default=None, repr=False, compare=False)

    @property
    def exercised(self) -> bool:
        return self._seal is _MUTATION_SEAL

    @property
    def reached_target(self) -> bool:
        return self.exercised and self.before_target != self.after_target

    @property
    def oracle_distinguishes(self) -> bool:
        return self.exercised and self.baseline_observed != self.mutant_observed


def exercise_mutation(
    baseline: Any,
    *,
    target: str,
    mutate: Callable[[Any], Any],
    target_probe: Callable[[Any], Any],
    oracle: Callable[[Any], Any],
) -> MutationExecution:
    """Ejecuta realmente una mutación sobre una copia y observa si alcanzó el objetivo."""
    baseline_obj = deepcopy(baseline)
    mutant_obj = deepcopy(baseline)
    before_target = target_probe(baseline_obj)
    baseline_observed = oracle(baseline_obj)
    mutated = mutate(mutant_obj)
    if mutated is not None:
        mutant_obj = mutated
    after_target = target_probe(mutant_obj)
    mutant_observed = oracle(mutant_obj)
    return MutationExecution(
        target,
        before_target,
        after_target,
        baseline_observed,
        mutant_observed,
        _seal=_MUTATION_SEAL,
    )


@dataclass(frozen=True)
class ContractTestCase:
    name: str
    targets: FrozenSet[str]
    execution: Optional[MutationExecution]
    verdict: TestVerdict
    observer_changes_fault: bool = False
    sut_includes_observer: bool = False

    def falsifiable(self) -> bool:
        return (
            bool(self.targets)
            and self.execution is not None
            and self.execution.exercised
            and self.execution.reached_target
            and self.execution.oracle_distinguishes
        )

    def covers(self) -> bool:
        if not self.falsifiable():
            return False
        if self.verdict not in (TestVerdict.PASS, TestVerdict.FAIL):
            return False
        if self.observer_changes_fault and not self.sut_includes_observer:
            return False
        return True


def public_evidence_admissible(*, sut_fault: str, independence: Optional[IndependencePremise]) -> bool:
    """Conclusión condicional; no demuestra materialmente la independencia de la evidencia."""
    return independence is not None and independence.admitted and independence.fault == sut_fault
