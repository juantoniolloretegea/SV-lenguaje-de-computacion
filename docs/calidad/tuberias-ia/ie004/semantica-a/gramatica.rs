use std::convert::TryFrom;

#[derive(Clone, Copy)]
enum E { L(&'static str), R(usize), S(&'static [E]), A(&'static [E]), O(&'static E), M(&'static E), D }
macro_rules! l { ($v:literal) => { E::L($v) }; }
macro_rules! r { ($v:literal) => { E::R($v - 1) }; }
macro_rules! s { ($($v:expr),* $(,)?) => { E::S(&[$($v),*]) }; }
macro_rules! a { ($($v:expr),* $(,)?) => { E::A(&[$($v),*]) }; }
macro_rules! o { ($v:expr) => { E::O(&$v) }; }
macro_rules! m { ($v:expr) => { E::M(&$v) }; }
const G: [E; 25] = [
 s!(o!(l!("¿")),r!(2),o!(a!(l!("?"),l!(".")))),
 a!(r!(3),s!(r!(18),a!(l!(";"),l!(":")),r!(3)),s!(r!(3),a!(l!(","),l!(";")),r!(18)),s!(r!(3),l!(","),r!(18),l!(":"),r!(3)),s!(r!(19),l!(","),r!(3)),s!(r!(19),l!("no"),l!(";"),r!(19),l!(","),r!(3)),r!(18)),
 s!(r!(4),o!(r!(20)),o!(r!(21))),
 a!(s!(r!(5),r!(6)),s!(l!("escriba"),r!(23),l!("en"),r!(6)),r!(6),s!(a!(l!("cuál es"),l!("adónde estará"),l!("y")),r!(6)),s!(l!("qué"),r!(9),l!("tiene"),r!(8)),s!(l!("en qué unidad"),a!(l!("está expresada"),l!("figura")),r!(8))),
 s!(o!(a!(l!("sólo"),l!("solo"))),r!(24)),
 s!(r!(7),m!(r!(13))),
 a!(r!(9),r!(10),r!(12),s!(r!(22),l!("registro"))),
 s!(a!(r!(10),s!(r!(22),l!("registro"))),m!(r!(13))),
 s!(r!(22),a!(l!("valor"),l!("dato"),s!(l!("cifra"),o!(l!("registrada"))),l!("unidad"),s!(l!("estado"),o!(l!("registrado"))),l!("fuente"),l!("procedencia"),l!("límites"),l!("alcance"),l!("intervalo de referencia"))),
 a!(r!(11),s!(r!(11),l!("o"),r!(11))),
 s!(o!(a!(l!("el"),l!("la"),l!("esa"))),a!(l!("igg"),l!("iga"),l!("igm"),l!("inmunoglobulina"),l!("inmunoglobina"),l!("linmunoglobina"),s!(l!("hemoglobina"),o!(l!("glicosilada"))))),
 s!(r!(22),a!(l!("actual"),l!("ahora"),l!("anterior"),l!("previo"))),
 a!(r!(12),s!(a!(l!("de"),l!("para")),r!(10)),s!(l!("para"),r!(14)),r!(25),s!(a!(l!("de"),l!("del")),o!(l!("corte")),r!(17)),s!(l!("de la"),r!(17)),s!(l!("en el corte"),r!(17)),s!(l!("del"),a!(l!("dato"),l!("registro")))),
 a!(r!(15),r!(16)),
 a!(l!("caso-a"),l!("caso-b")),
 a!(s!(a!(l!("este"),l!("ese")),a!(l!("caso"),l!("paciente"))),l!("el caso que estamos viendo"),s!(l!("este"),l!("..."))),
 a!(l!("actual"),l!("ahora"),l!("anterior"),l!("previo")),
 a!(s!(l!("no"),r!(24),r!(6)),s!(l!("no escriba"),r!(23),l!("en"),r!(6)),s!(l!("no quiero"),r!(6)),s!(l!("no"),r!(6)),l!("no cambie nada")),
 r!(25),s!(l!("porque estoy revisando"),r!(14)),s!(o!(l!(",")),l!("por favor")),
 o!(a!(l!("el"),l!("la"))),E::D,
 a!(l!("consulte"),l!("lea"),l!("traiga"),l!("dígame"),l!("necesito"),l!("cambie"),l!("elimine")),
 a!(s!(a!(l!("de"),l!("del")),r!(15)),s!(l!("de"),a!(s!(a!(l!("este"),l!("ese")),a!(l!("caso"),l!("paciente"))),s!(l!("este"),l!("...")))),l!("del caso que estamos viendo"))
];

