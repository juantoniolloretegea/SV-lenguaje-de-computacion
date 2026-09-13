//! ES: Observador del banco; no llama a admit ni a certify.
//! EN: Bank observer; never calls admit or certify.
use sv_bis_i0205::{
    json::{n, obj, s, J},
    Capture,
};
#[derive(Clone, Debug)]
pub struct Evidence {
    pub result: String,
    pub guard: Option<String>,
    pub cause: String,
    pub passed: Vec<String>,
    pub receipt: Option<J>,
    pub capture: Option<Capture>,
    pub dispatches: Option<usize>,
    pub attempts: usize,
    pub before: Vec<u8>,
    pub after: Vec<u8>,
    pub admitted_vector: Option<J>,
    pub preservation_claim: bool,
}
impl Evidence {
    pub fn documentary(&self) -> J {
        obj(vec![
            ("resultado", s(&self.result)),
            (
                "primera_guarda",
                self.guard.as_ref().map(|x| s(x)).unwrap_or(J::Null),
            ),
            ("causa", s(&self.cause)),
            (
                "guardas_superadas",
                J::Array(self.passed.iter().map(|x| s(x)).collect()),
            ),
            ("recibo", self.receipt.clone().unwrap_or(J::Null)),
            (
                "captura",
                self.capture
                    .as_ref()
                    .map(|c| {
                        obj(vec![
                            ("captor", s(&c.captor)),
                            ("contexto", c.context.clone()),
                            (
                                "bytes",
                                J::Array(c.bytes.iter().map(|x| n(*x as usize)).collect()),
                            ),
                        ])
                    })
                    .unwrap_or(J::Null),
            ),
            (
                "despachos_observados",
                self.dispatches.map(n).unwrap_or(J::Null),
            ),
            ("intentos_despacho", n(self.attempts)),
            (
                "estado_antes",
                J::Array(self.before.iter().map(|x| n(*x as usize)).collect()),
            ),
            (
                "estado_despues",
                J::Array(self.after.iter().map(|x| n(*x as usize)).collect()),
            ),
            (
                "vector_admitido",
                self.admitted_vector.clone().unwrap_or(J::Null),
            ),
            ("afirmacion_preservacion", J::Bool(self.preservation_claim)),
        ])
    }
}
pub fn inspect(
    e: &Evidence,
    oracle: &J,
    canonical: &[u8],
    expected_geometry: Option<&[u8]>,
) -> Vec<String> {
    let mut faults = Vec::new();
    macro_rules! demand {
        ($b:expr,$s:literal) => {
            if !$b {
                faults.push($s.into())
            }
        };
    }
    demand!(
        Some(e.result.as_str()) == oracle.field("resultado_esperado").text(),
        "resultado"
    );
    demand!(
        e.guard.as_deref() == oracle.field("primera_guarda_esperada").text(),
        "primera guarda"
    );
    demand!(
        e.before == canonical && e.after == canonical,
        "preservacion real del estado"
    );
    demand!(
        Some(e.receipt.is_some()) == oracle.field("recibo_favorable").boolean(),
        "existencia recibo"
    );
    demand!(
        e.dispatches == oracle.field("despachos_locales_esperados").uint(),
        "despachos observados"
    );
    // ES: Precedencia independiente; exige todas las guardas anteriores.
    // EN: Independent precedence; requires every preceding guard.
    let stages = [
        "R01", "P01", "P02", "S01", "S02", "I01", "I02", "I03", "A01", "M01", "M02", "M03", "D01",
        "D02", "D03", "D04", "D05", "D06",
    ];
    let expected_passes = match oracle.field("primera_guarda_esperada").text() {
        None => stages.len(),
        Some("D07") => 12,
        Some(g) => stages.iter().position(|x| *x == g).unwrap_or(usize::MAX),
    };
    demand!(
        expected_passes <= stages.len()
            && e.passed
                .iter()
                .map(String::as_str)
                .eq(stages[..expected_passes.min(stages.len())].iter().copied()),
        "precedencia/traza"
    );
    if let Some(receipt) = &e.receipt {
        demand!(
            receipt == oracle.field("recibo_esperado"),
            "campos completos recibo"
        );
        demand!(e.capture.is_some(), "captura independiente ausente");
        if let Some(cap) = &e.capture {
            demand!(
                Some(cap.bytes.as_slice()) == expected_geometry,
                "bytes entregados"
            );
            demand!(
                &cap.context == oracle.field("contexto_entrega_esperado"),
                "contexto capturado"
            );
            demand!(cap.captor == "captor-local-i0205/1", "captor");
        }
        let vector = sv_bis_i0205::json::decode(canonical)
            .ok()
            .map(|j| j.field("vector").clone());
        demand!(
            e.admitted_vector == vector,
            "vector admitido frente a testigo independiente"
        );
    }
    if matches!(e.guard.as_deref(), Some("D01" | "D07")) {
        demand!(
            e.attempts == 1 && e.capture.is_none() && e.dispatches.is_none(),
            "efecto desconocido tras despacho"
        );
    }
    faults
}
