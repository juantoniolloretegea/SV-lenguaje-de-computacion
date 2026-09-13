//! ES: Montaje documental C02–C05; no constituye un dominio ni un host productivo.
//! EN: C02–C05 documentary assembly; not a domain constitution or production host.
pub mod json;
pub mod sha;
use json::{n, obj, s, J};
use std::io::Read;
use sv_core::{
    ir::{IrObjectKind, IrProgram},
    SourceProfile, Tri,
};
#[derive(Debug, Clone)]
pub struct Rejection {
    pub guard: &'static str,
    pub cause: String,
    pub passed: Vec<&'static str>,
}
fn err(guard: &'static str, cause: impl Into<String>, passed: &[&'static str]) -> Rejection {
    Rejection {
        guard,
        cause: cause.into(),
        passed: passed.to_vec(),
    }
}
// ES: Canales propios e inmutables, leídos dentro de cuota; no rutas/órdenes.
// EN: Owned immutable channels, read within quota; no paths/commands.
pub struct ReceivedBytes {
    metadata: Vec<u8>,
    source: Vec<u8>,
    state: Vec<u8>,
    support: Option<Vec<u8>>,
    geometry: Vec<u8>,
}
impl ReceivedBytes {
    pub fn read(
        meta: &mut impl Read,
        source: &mut impl Read,
        state: &mut impl Read,
        support: Option<&mut impl Read>,
        geometry: &mut impl Read,
    ) -> Result<Self, Rejection> {
        let mut total = 0usize;
        let mut channel = |r: &mut dyn Read, max: usize| -> Result<Vec<u8>, Rejection> {
            let mut r = r;
            let b = json::receive(&mut r, max).map_err(|e| err("R01", e, &[]))?;
            total = total
                .checked_add(b.len())
                .ok_or_else(|| err("R01", "sum overflow", &[]))?;
            if total > 16384 {
                return Err(err("R01", "aggregate input", &[]));
            }
            Ok(b)
        };
        Ok(Self {
            metadata: channel(meta, 4096)?,
            source: channel(source, 4096)?,
            state: channel(state, 1024)?,
            support: match support {
                Some(r) => Some(channel(r, 1024)?),
                None => None,
            },
            geometry: channel(geometry, 8192)?,
        })
    }
}
#[derive(Clone)]
pub struct CellIdentity {
    value: J,
}
impl CellIdentity {
    pub fn documentary(&self) -> &J {
        &self.value
    }
}
#[derive(Clone)]
pub struct DeliveryContext {
    value: J,
}
impl DeliveryContext {
    pub fn documentary(&self) -> &J {
        &self.value
    }
}
pub struct TrustedContext {
    value: J,
}
pub struct TrustedRegistry {
    value: J,
    constitution: J,
    convention: J,
    transforms: J,
}
fn texts(j: &J, keys: &[&str]) -> bool {
    keys.iter()
        .all(|k| j.field(k).text().is_some_and(|x| !x.is_empty()))
}
fn identity_shape(j: &J) -> bool {
    j.pairs().is_some_and(|p| p.len() == 11)
        && texts(
            j,
            &[
                "dominio",
                "arquitectura",
                "nodo",
                "cellspec",
                "instancia_celular",
                "revision",
                "estado_ir",
                "componente",
                "representacion",
            ],
        )
        && texts(
            j.field("constitucion"),
            &["identifier", "version", "sha256"],
        )
        && texts(
            j.field("programa_fuente"),
            &["source_file", "source_sha256"],
        )
}
fn delivery_shape(j: &J) -> bool {
    j.pairs().is_some_and(|p| p.len() == 9)
        && texts(
            j,
            &[
                "ambito",
                "invocacion",
                "operacion",
                "consumidor",
                "canal",
                "instancia_celular",
                "revision",
                "representacion",
                "transformacion",
            ],
        )
}
fn asset(reference: &J, b: &[u8]) -> bool {
    reference.field("bytes").uint() == Some(b.len())
        && reference.field("sha256").text() == Some(sha::hash(b).as_str())
}
impl TrustedContext {
    // ES: Sólo el conductor llama a este constructor con su registro fijado.
    // EN: Only the harness calls this constructor with its pinned custody record.
    pub fn from_custody(b: &[u8]) -> Result<Self, String> {
        let j = json::decode(b).map_err(str::to_owned)?;
        if !identity_shape(j.field("vinculo"))
            || !delivery_shape(j.field("entrega"))
            || !texts(&j, &["id", "perfil_fuente"])
        {
            return Err("custody context schema".into());
        }
        Ok(Self { value: j })
    }
}
impl TrustedRegistry {
    pub fn from_custody(
        b: &[u8],
        constitution: &[u8],
        convention: &[u8],
        transforms: &[u8],
    ) -> Result<Self, String> {
        let value = json::decode(b).map_err(str::to_owned)?;
        if value.field("version").text() != Some("BIS-I0205-REGISTRO/0.1")
            || !asset(value.field("constitucion"), constitution)
            || !asset(value.field("convenio"), convention)
            || !asset(value.field("transformaciones"), transforms)
        {
            return Err("custody registry integrity".into());
        }
        for x in value
            .field("soportes")
            .array()
            .ok_or("custody support list")?
        {
            if !texts(x.field("referencia"), &["id", "version", "sha256"])
                || x.field("tamanos").array().is_none()
            {
                return Err("custody support schema".into());
            }
        }
        Ok(Self {
            value,
            constitution: json::decode(constitution).map_err(str::to_owned)?,
            convention: json::decode(convention).map_err(str::to_owned)?,
            transforms: json::decode(transforms).map_err(str::to_owned)?,
        })
    }
}
enum FixedVector {
    N16([Tri; 16]),
    N25([Tri; 25]),
    N49([Tri; 49]),
}
impl FixedVector {
    fn from_supported(v: &[Tri], sizes: &[J]) -> Result<Self, &'static str> {
        if !sizes.contains(&n(v.len())) {
            return Err("dimension not supported");
        };
        match v.len() {
            16 => Ok(Self::N16(v.try_into().map_err(|_| "length")?)),
            25 => Ok(Self::N25(v.try_into().map_err(|_| "length")?)),
            49 => Ok(Self::N49(v.try_into().map_err(|_| "length")?)),
            _ => Err("fixed storage length"),
        }
    }
    fn as_slice(&self) -> &[Tri] {
        match self {
            Self::N16(v) => v,
            Self::N25(v) => v,
            Self::N49(v) => v,
        }
    }
}
pub struct AdmittedDelivery {
    vector: FixedVector,
    descriptor: Vec<u8>,
    identity: CellIdentity,
    context: DeliveryContext,
    receipt: J,
    passed: Vec<&'static str>,
}
impl AdmittedDelivery {
    pub fn vector(&self) -> &[Tri] {
        self.vector.as_slice()
    }
    pub fn descriptor(&self) -> &[u8] {
        &self.descriptor
    }
    pub fn identity(&self) -> &CellIdentity {
        &self.identity
    }
    pub fn context(&self) -> &DeliveryContext {
        &self.context
    }
    pub fn passed(&self) -> &[&'static str] {
        &self.passed
    }
    pub fn receipt_candidate(&self) -> &J {
        &self.receipt
    }
    pub fn storage_bytes(&self) -> usize {
        std::mem::size_of::<FixedVector>()
    }
}
fn ir_object<'a>(ir: &'a IrProgram, name: &str) -> Option<&'a IrObjectKind> {
    ir.objects()
        .iter()
        .find(|x| x.name() == name)
        .map(|x| x.kind())
}
pub fn admit(
    request: ReceivedBytes,
    context: &TrustedContext,
    registry: &TrustedRegistry,
) -> Result<AdmittedDelivery, Rejection> {
    let mut passed = vec!["R01"];
    macro_rules! check {
        ($g:literal,$b:expr,$cause:expr) => {
            if !$b {
                return Err(err($g, $cause, &passed));
            }
        };
    }
    macro_rules! parse {
        ($g:literal,$b:expr) => {
            json::decode($b).map_err(|e| err($g, e, &passed))?
        };
    }
    let meta = parse!("P01", &request.metadata);
    let ctx = &context.value;
    let reg = &registry.value;
    check!(
        "P01",
        meta.field("version").text() == Some("BIS-I0205-SOLICITUD/0.1")
            && meta.field("perfil_fuente") == ctx.field("perfil_fuente"),
        "explicit matching source profile"
    );
    let profile = meta
        .field("perfil_fuente")
        .text()
        .and_then(SourceProfile::from_tag)
        .ok_or_else(|| err("P01", "source profile", &passed))?;
    passed.push("P01");
    let source =
        std::str::from_utf8(&request.source).map_err(|e| err("P02", e.to_string(), &passed))?;
    let filename = ctx
        .field("vinculo")
        .field("programa_fuente")
        .field("source_file")
        .text()
        .ok_or_else(|| err("P02", "trusted filename", &passed))?;
    let ir = sv_core::compile_svp_profile(source, filename, profile)
        .map_err(|e| err("P02", format!("{e:?}"), &passed))?;
    passed.push("P02");
    let support = meta.field("soporte");
    let reference = support.field("referencia");
    let entry = reg
        .field("soportes")
        .array()
        .and_then(|a| a.iter().find(|x| x.field("referencia") == reference))
        .ok_or_else(|| err("S01", "support reference absent/unregistered", &passed))?;
    let body = request
        .support
        .as_ref()
        .ok_or_else(|| err("S01", "missing support body", &passed))?;
    check!(
        "S01",
        reference == ctx.field("soporte")
            && asset(support.field("contenido"), body)
            && asset(entry.field("contenido"), body),
        "support reference/body integrity"
    );
    let support_doc = parse!("S01", body);
    let sizes = entry
        .field("tamanos")
        .array()
        .ok_or_else(|| err("S01", "support sizes", &passed))?;
    check!(
        "S01",
        support_doc.field("tamanos") == entry.field("tamanos")
            && support_doc.field("id") == reference.field("id")
            && support_doc.field("version") == reference.field("version"),
        "support schema"
    );
    passed.push("S01");
    // ES: Nat se compara como decimal exacto antes de convertir a usize.
    // EN: Compare Nat as exact decimal before converting to usize.
    let mut dimensions = Vec::new();
    for o in ir.objects() {
        if let IrObjectKind::CellSpec { n: dim, .. } = o.kind() {
            check!(
                "S02",
                sizes.iter().any(|x| x.lexeme() == Some(dim.as_decimal())),
                format!(
                    "unsupported IR CellSpec {} n={}",
                    o.name(),
                    dim.as_decimal()
                )
            );
            let v = dim
                .as_decimal()
                .parse::<usize>()
                .map_err(|_| err("S02", "dimension representation", &passed))?;
            if !dimensions.contains(&v) {
                dimensions.push(v)
            }
        }
    }
    dimensions.sort_unstable();
    passed.push("S02");
    let binding = meta.field("vinculo");
    check!(
        "I01",
        identity_shape(binding),
        "required identity fields/types"
    );
    passed.push("I01");
    check!(
        "I02",
        binding == ctx.field("vinculo")
            && reg
                .field("vinculos")
                .pairs()
                .is_some_and(|a| a.iter().any(|(_, x)| x == binding)),
        "full identity/custody mismatch"
    );
    check!(
        "I02",
        asset(meta.field("fuente"), &request.source)
            && asset(ctx.field("fuente"), &request.source)
            && binding
                .field("programa_fuente")
                .field("source_sha256")
                .text()
                == Some(ir.source_sha256())
            && filename == ir.source_file(),
        "source identity"
    );
    let constitution = &registry.constitution;
    check!(
        "I02",
        binding.field("constitucion").field("sha256") == reg.field("constitucion").field("sha256")
            && binding.field("constitucion").field("identifier")
                == constitution.field("identifier")
            && binding.field("constitucion").field("version") == constitution.field("version")
            && binding.field("dominio") == constitution.field("dominio")
            && binding.field("cellspec") == constitution.field("cellspec")
            && constitution
                .field("nodos")
                .array()
                .is_some_and(|x| x.contains(binding.field("nodo"))),
        "constitution binding"
    );
    let name = |k| binding.field(k).text().unwrap_or("");
    check!(
        "I02",
        matches!(ir_object(&ir,name("arquitectura")),Some(IrObjectKind::CompositionGraph{nodes,..}) if nodes.iter().any(|x|x==name("nodo"))),
        "architecture/node"
    );
    check!(
        "I02",
        matches!(ir_object(&ir,name("nodo")),Some(IrObjectKind::CoupledSpec{cell,..}) if cell==name("cellspec")),
        "node/CellSpec"
    );
    let (b, ncell) = match ir_object(&ir, name("cellspec")) {
        Some(IrObjectKind::CellSpec { b, n, .. }) => (
            b.as_decimal().parse::<usize>().ok(),
            n.as_decimal().parse::<usize>().ok(),
        ),
        _ => (None, None),
    };
    let (b, ncell) = b
        .zip(ncell)
        .ok_or_else(|| err("I02", "cell dimensions", &passed))?;
    check!(
        "I02",
        b >= 3
            && b.checked_mul(b) == Some(ncell)
            && constitution.field("b").uint() == Some(b)
            && constitution.field("n").uint() == Some(ncell)
            && constitution.field("orden_de_coordenadas")
                == &J::Array((1..=ncell).map(n).collect()),
        "flat positional constitution"
    );
    let vector = match ir_object(&ir, name("estado_ir")) {
        Some(IrObjectKind::CoupledState {
            spec,
            updated_vector,
            ..
        }) if spec == name("nodo") && name("componente") == "updated_vector" => {
            updated_vector.as_slice()
        }
        _ => return Err(err("I02", "IR state/component", &passed)),
    };
    check!("I02", vector.len() == ncell, "IR vector length");
    passed.push("I02");
    let state = parse!("I03", &request.state);
    check!(
        "I03",
        asset(meta.field("estado_matematico"), &request.state)
            && asset(ctx.field("estado_matematico"), &request.state)
            && asset(reg.field("estado_matematico"), &request.state),
        "mathematical state identity"
    );
    check!(
        "I03",
        state.field("b").uint() == Some(b)
            && state.field("n").uint() == Some(ncell)
            && state.field("vector") == &J::Array(vector.iter().map(|x| s(x.ir_label())).collect()),
        "IR/state positional equality"
    );
    passed.push("I03");
    let delivery = meta.field("entrega");
    check!(
        "A01",
        delivery_shape(delivery)
            && delivery == ctx.field("entrega")
            && delivery.field("operacion").text() == Some("entrega_documental")
            && delivery.field("instancia_celular") == binding.field("instancia_celular")
            && delivery.field("revision") == binding.field("revision")
            && delivery.field("invocacion") == ctx.field("id"),
        "documentary delivery authority"
    );
    passed.push("A01");
    let geo = parse!("M01", &request.geometry);
    check!(
        "M01",
        geo.field("formato").text() == Some("BIS-C04-GEOMETRIA/0.1")
            && geo.field("n").uint() == Some(ncell)
            && geo.field("convenio").field("id") == registry.convention.field("id")
            && geo.field("convenio").field("sha256") == reg.field("convenio").field("sha256")
            && geo.field("transformacion") == delivery.field("transformacion")
            && geo.field("leyenda") == registry.convention.field("leyenda")
            && geo.field("codificacion").text() == Some("inyectiva")
            && geo.field("pretension_equivalencia").boolean() == Some(true),
        "geometry convention/schema/transform"
    );
    let transform = registry
        .transforms
        .field("transformaciones")
        .array()
        .and_then(|a| {
            a.iter()
                .find(|x| x.field("id") == geo.field("transformacion"))
        })
        .ok_or_else(|| err("M01", "unknown transform", &passed))?;
    check!(
        "M01",
        matches!(
            geo.field("transformacion").text(),
            Some("identidad" | "superior-horario")
        ),
        "transform outside first route"
    );
    passed.push("M01");
    let vertices = geo
        .field("vertices")
        .array()
        .ok_or_else(|| err("M02", "vertices", &passed))?;
    check!(
        "M02",
        vertices.len() == ncell && geo.field("cerrada").boolean() == Some(true),
        "vertex count/closure"
    );
    for (i, (v, t)) in vertices.iter().zip(vector).enumerate() {
        let radius = match t {
            Tri::Zero => 1,
            Tri::One => 2,
            Tri::U => 3,
        };
        check!(
            "M02",
            v.field("posicion").uint() == Some(i + 1)
                && v.field("parametro_sintetico").text() == Some(format!("P{}", i + 1).as_str())
                && v.field("simbolo").text() == Some(t.ir_label())
                && v.field("radio").uint() == Some(radius)
                && v.field("angulo_vueltas") == &J::Array(vec![n(i), n(ncell)]),
            format!("position {} symbol/radius/angle", i + 1)
        );
        for (axis, k) in ["x_coef_cos_sin_constante", "y_coef_cos_sin_constante"]
            .iter()
            .enumerate()
        {
            let row = transform
                .field("matriz")
                .array()
                .and_then(|a| a.get(axis))
                .and_then(J::array)
                .ok_or_else(|| err("M02", "transform matrix", &passed))?;
            let expected = J::Array(vec![
                n(row[0]
                    .uint()
                    .ok_or_else(|| err("M02", "transform coefficient", &passed))?
                    * radius),
                n(row[1]
                    .uint()
                    .ok_or_else(|| err("M02", "transform coefficient", &passed))?
                    * radius),
                n(0),
            ]);
            check!(
                "M02",
                v.field(k) == &expected,
                format!("position {} coefficients", i + 1)
            );
        }
    }
    check!(
        "M02",
        geo.field("aristas")
            == &J::Array(
                (1..=ncell)
                    .map(|i| J::Array(vec![n(i), n(if i == ncell { 1 } else { i + 1 })]))
                    .collect()
            ),
        "consecutive edges"
    );
    passed.push("M02");
    check!(
        "M03",
        asset(meta.field("geometria"), &request.geometry)
            && asset(ctx.field("geometria"), &request.geometry),
        "exact geometry identity"
    );
    passed.push("M03");
    let fixed = FixedVector::from_supported(vector, sizes).map_err(|e| err("I03", e, &passed))?;
    let receipt = obj(vec![
        ("version", s("BIS-I0205-RECIBO/0.1")),
        ("caso", ctx.field("id").clone()),
        ("resultado", s("ENTREGA_DOCUMENTAL_CONCORDANTE")),
        ("perfil_fuente", meta.field("perfil_fuente").clone()),
        ("fuente_sha256", s(ir.source_sha256())),
        ("soporte", reference.clone()),
        (
            "dimensiones_ir",
            J::Array(dimensions.into_iter().map(n).collect()),
        ),
        ("vinculo", binding.clone()),
        ("estado_sha256", s(&sha::hash(&request.state))),
        (
            "convenio_sha256",
            reg.field("convenio").field("sha256").clone(),
        ),
        ("geometria_sha256", s(&sha::hash(&request.geometry))),
        ("geometria_bytes", n(request.geometry.len())),
        ("contexto_entrega", delivery.clone()),
    ]);
    let encoded = json::encode(&receipt, 2048)
        .map_err(|e| err("R01", format!("output receipt {e}"), &passed))?;
    check!(
        "R01",
        encoded
            .len()
            .checked_add(request.geometry.len())
            .is_some_and(|x| x <= 10240),
        "aggregate output"
    );
    Ok(AdmittedDelivery {
        vector: fixed,
        descriptor: request.geometry,
        identity: CellIdentity {
            value: binding.clone(),
        },
        context: DeliveryContext {
            value: delivery.clone(),
        },
        receipt,
        passed,
    })
}
// ES: Datos de captura procedentes del receptor externo al admisor.
// EN: Capture data from the receiver outside the admission validator.
#[derive(Clone, Debug)]
pub struct Capture {
    pub captor: String,
    pub context: J,
    pub bytes: Vec<u8>,
}
pub fn certify(
    admitted: &AdmittedDelivery,
    capture: Option<&Capture>,
    post_dispatch_failure: bool,
    registry: &TrustedRegistry,
) -> Result<J, Rejection> {
    let mut passed = admitted.passed.clone();
    if post_dispatch_failure {
        return Err(err(
            "D07",
            "instrumented failure after dispatch; effect unknown",
            &passed,
        ));
    }
    let cap = capture.ok_or_else(|| err("D01", "no final capture; effect unknown", &passed))?;
    passed.push("D01");
    if !registry
        .value
        .field("captores_admitidos")
        .array()
        .is_some_and(|a| a.contains(&s(&cap.captor)))
    {
        return Err(err("D02", "unregistered captor", &passed));
    }
    passed.push("D02");
    for (g, keys) in [
        (
            "D03",
            &["ambito", "invocacion", "operacion", "consumidor", "canal"][..],
        ),
        ("D04", &["instancia_celular", "revision"][..]),
        ("D05", &["representacion", "transformacion"][..]),
    ] {
        if keys
            .iter()
            .any(|k| cap.context.field(k) != admitted.context.value.field(k))
        {
            return Err(err(g, "capture context mismatch", &passed));
        }
        passed.push(g)
    }
    if cap.bytes != admitted.descriptor {
        return Err(err(
            "D06",
            "captured bytes differ from admitted buffer",
            &passed,
        ));
    }
    Ok(admitted.receipt.clone())
}
#[cfg(test)]
mod construction_tests {
    use super::*;
    // ES: Ensayos de almacenamiento posteriores a la puerta de soporte.
    // EN: Storage tests after the support gate.
    #[test]
    fn lengths_and_support() {
        let v1 = vec![n(16), n(25)];
        let v2 = vec![n(16), n(25), n(49)];
        assert!(FixedVector::from_supported(&[Tri::Zero; 15], &v2).is_err());
        assert!(FixedVector::from_supported(&[Tri::Zero; 49], &v1).is_err());
        assert_eq!(
            FixedVector::from_supported(&[Tri::U; 16], &v1)
                .unwrap()
                .as_slice(),
            &[Tri::U; 16]
        );
        assert_eq!(
            FixedVector::from_supported(&[Tri::One; 25], &v1)
                .unwrap()
                .as_slice(),
            &[Tri::One; 25]
        );
        assert_eq!(
            FixedVector::from_supported(&[Tri::Zero; 49], &v2)
                .unwrap()
                .as_slice(),
            &[Tri::Zero; 49]
        );
    }
}
