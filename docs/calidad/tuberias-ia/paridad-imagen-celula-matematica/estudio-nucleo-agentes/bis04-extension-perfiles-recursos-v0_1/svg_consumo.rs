//! ES: Representación SVG experimental y consumo de sus coordenadas materiales.
//! EN: Experimental SVG representation and consumption of its material coordinates.
#![forbid(unsafe_code)]
use std::io::Read;
use sv_bis_i0205::{
    json::{self, n, obj, s, J},
    sha, AdmittedDelivery,
};
pub const MAX_SVG: usize = 4096;
pub const PREFIX: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="320" height="360" viewBox="-4000000 -4000000 8000000 9000000">
<title>SV: poligonal de laboratorio de 16 posiciones</title>
<desc>Proyeccion aproximada; escala 1000000. P1 en +X; orden P1 a P16 antihorario en el plano canonico. Radios: 0=1, 1=2, U=3.</desc>
<polygon id="celula" data-orden="P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14 P15 P16" points=""##;
pub const SUFFIX: &str = r##"" fill="none" stroke="#16324f" stroke-width="40000"/>
<text x="-3500000" y="4400000" font-size="320000">0: radio 1 | 1: radio 2 | U: radio 3</text>
</svg>
"##;
const X: [i64; 16] = [
    1000000, 923880, 707107, 382683, 0, -382683, -707107, -923880, -1000000, -923880, -707107,
    -382683, 0, 382683, 707107, 923880,
];
const Y: [i64; 16] = [
    0, 382683, 707107, 923880, 1000000, 923880, 707107, 382683, 0, -382683, -707107, -923880,
    -1000000, -923880, -707107, -382683,
];
pub struct Rendered {
    bytes: Vec<u8>,
    record: J,
}
impl Rendered {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn record(&self) -> &J {
        &self.record
    }
}
#[derive(Debug)]
pub struct Consumed {
    pub vertices: [[i64; 2]; 16],
    pub differences: [[i64; 2]; 16],
    pub reported_sha256: String,
}
pub fn render(a: &AdmittedDelivery) -> Result<Rendered, &'static str> {
    if a.vector().len() != 16 {
        return Err("V01_DIMENSION");
    }
    let g = json::decode(a.descriptor()).map_err(|_| "V01_DESCRIPTOR")?;
    if g.field("transformacion").text() != Some("identidad") {
        return Err("V01_TRANSFORMACION");
    }
    let v = g.field("vertices").array().ok_or("V01_DESCRIPTOR")?;
    if v.len() != 16 {
        return Err("V01_DESCRIPTOR");
    }
    let mut points = [[0i64; 2]; 16];
    for (i, item) in v.iter().enumerate() {
        let radius = i64::try_from(item.field("radio").uint().ok_or("V01_RADIO")?)
            .map_err(|_| "V01_RADIO")?;
        if !(1..=3).contains(&radius) {
            return Err("V01_RADIO");
        }
        points[i] = [
            X[i].checked_mul(radius).ok_or("V01_ARITMETICA")?,
            Y[i].checked_mul(radius)
                .and_then(i64::checked_neg)
                .ok_or("V01_ARITMETICA")?,
        ];
    }
    // ES: Mutante de cualificación; altera posiciones sin falsear su propio hash.
    // EN: Qualification mutant; changes positions without falsifying its own hash.
    if cfg!(svg_permutado) {
        points.swap(0, 1);
    }
    let list = points
        .iter()
        .map(|p| format!("{},{}", p[0], p[1]))
        .collect::<Vec<_>>()
        .join(" ");
    let bytes = format!("{PREFIX}{list}{SUFFIX}").into_bytes();
    if bytes.len() > MAX_SVG {
        return Err("V02_CUOTA");
    }
    let record = obj(vec![
        ("formato", s("SV-SVG16-MATERIAL/1")),
        ("identidad", a.identity().documentary().clone()),
        ("entrega_origen", a.context().documentary().clone()),
        ("descriptor_sha256", s(&sha::hash(a.descriptor()))),
        ("proyeccion", s("micro16-pantalla/1")),
        ("operacion", s("diferencias-ciclicas-svg/1")),
        ("consumidor", s("consumidor-svg16-local/1")),
        ("svg_sha256", s(&sha::hash(&bytes))),
        ("svg_bytes", n(bytes.len())),
    ]);
    Ok(Rendered { bytes, record })
}
// ES: Lector de un único perfil SVG; no es un intérprete XML general.
// EN: Reader for one SVG profile; not a general XML interpreter.
pub fn inspect_svg(b: &[u8]) -> Result<[[i64; 2]; 16], &'static str> {
    if b.len() > MAX_SVG {
        return Err("C03_CUOTA");
    }
    let text = std::str::from_utf8(b).map_err(|_| "C05_PERFIL")?;
    let body = text
        .strip_prefix(PREFIX)
        .and_then(|x| x.strip_suffix(SUFFIX))
        .ok_or("C05_PERFIL")?;
    let mut result = [[0i64; 2]; 16];
    let mut count = 0usize;
    for point in body.split(' ') {
        if count >= 16 {
            return Err("C05_PERFIL");
        }
        let (x, y) = point.split_once(',').ok_or("C05_PERFIL")?;
        for (axis, s) in [x, y].iter().enumerate() {
            let value = s.parse::<i64>().map_err(|_| "C05_PERFIL")?;
            if value.to_string() != *s || !(-3000000..=3000000).contains(&value) {
                return Err("C05_PERFIL");
            }
            result[count][axis] = value;
        }
        count += 1;
    }
    if count != 16 {
        return Err("C05_PERFIL");
    }
    Ok(result)
}
pub fn consume(
    authorized: &Rendered,
    offered: &J,
    reader: Option<&mut dyn Read>,
    alternate_dependency: &[u8],
) -> Result<Consumed, &'static str> {
    let reader = reader.ok_or("C01_AUSENCIA")?;
    if offered != authorized.record() {
        return Err("C02_CONTEXTO");
    }
    let mut storage = [0u8; MAX_SVG + 1];
    let mut len = 0;
    loop {
        let n = reader.read(&mut storage[len..]).map_err(|_| "C03_IO")?;
        if n == 0 {
            break;
        }
        len = len.checked_add(n).ok_or("C03_CUOTA")?;
        if len > MAX_SVG {
            return Err("C03_CUOTA");
        }
    }
    let captured = &storage[..len];
    if captured != authorized.bytes() {
        return Err("C04_BYTES");
    }
    // ES: Mutante compilable de consumo falso; conserva la captura y el metadato.
    // EN: Compilable false-consumption mutant; preserves capture and metadata.
    let used = if cfg!(consumo_falso_svg) {
        alternate_dependency
    } else {
        captured
    };
    let vertices = inspect_svg(used)?;
    let mut differences = [[0i64; 2]; 16];
    for i in 0..16 {
        for axis in 0..2 {
            differences[i][axis] = vertices[(i + 1) % 16][axis]
                .checked_sub(vertices[i][axis])
                .ok_or("C06_ARITMETICA")?;
        }
    }
    Ok(Consumed {
        vertices,
        differences,
        reported_sha256: sha::hash(captured),
    })
}
