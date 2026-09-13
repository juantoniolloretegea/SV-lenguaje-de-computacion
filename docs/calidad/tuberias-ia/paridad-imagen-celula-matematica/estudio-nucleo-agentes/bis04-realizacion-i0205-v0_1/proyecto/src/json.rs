//! ES: AST documental acotada; adaptación de recepción/escapes IE004.
//! EN: Bounded documentary AST; IE004 reception/escape adaptation.
use std::io::Read;
#[derive(Clone, Debug)]
pub enum J {
    Null,
    Bool(bool),
    Number(String),
    Text(String),
    Array(Vec<J>),
    Object(Vec<(String, J)>),
}
impl PartialEq for J {
    fn eq(&self, b: &Self) -> bool {
        match (self, b) {
            (Self::Null, Self::Null) => true,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Number(a), Self::Number(b)) | (Self::Text(a), Self::Text(b)) => a == b,
            (Self::Array(a), Self::Array(b)) => a == b,
            (Self::Object(a), Self::Object(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .all(|(k, v)| b.iter().any(|(q, w)| k == q && v == w))
            }
            _ => false,
        }
    }
}
impl J {
    pub fn get(&self, k: &str) -> Option<&J> {
        if let Self::Object(v) = self {
            v.iter().find(|(s, _)| s == k).map(|(_, j)| j)
        } else {
            None
        }
    }
    pub fn field(&self, k: &str) -> &J {
        self.get(k).unwrap_or(&J::Null)
    }
    pub fn text(&self) -> Option<&str> {
        if let Self::Text(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn lexeme(&self) -> Option<&str> {
        if let Self::Number(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn uint(&self) -> Option<usize> {
        self.lexeme()?.parse().ok()
    }
    pub fn array(&self) -> Option<&[J]> {
        if let Self::Array(a) = self {
            Some(a)
        } else {
            None
        }
    }
    pub fn boolean(&self) -> Option<bool> {
        if let Self::Bool(x) = self {
            Some(*x)
        } else {
            None
        }
    }
    pub fn pairs(&self) -> Option<&[(String, J)]> {
        if let Self::Object(a) = self {
            Some(a)
        } else {
            None
        }
    }
    pub fn set(&mut self, k: &str, v: J) {
        if let Self::Object(a) = self {
            if let Some((_, x)) = a.iter_mut().find(|(s, _)| s == k) {
                *x = v
            } else {
                a.push((k.into(), v))
            }
        }
    }
}
pub fn s(x: &str) -> J {
    J::Text(x.into())
}
pub fn n(x: usize) -> J {
    J::Number(x.to_string())
}
pub fn obj(v: Vec<(&str, J)>) -> J {
    J::Object(v.into_iter().map(|(k, v)| (k.into(), v)).collect())
}
pub type E = &'static str;
// ES: Se lee como máximo cuota+1, sin abrir rutas procedentes del sujeto.
// EN: Read at most quota+1, without opening paths supplied by the subject.
pub fn receive(r: &mut impl Read, max: usize) -> Result<Vec<u8>, E> {
    let mut out = Vec::new();
    out.try_reserve_exact(max).map_err(|_| "RESERVA")?;
    let mut buf = [0u8; 4096];
    loop {
        let remaining = max.checked_sub(out.len()).ok_or("BYTES")?;
        let cap = 4096.min(remaining.checked_add(1).ok_or("BYTES")?);
        let len = r.read(&mut buf[..cap]).map_err(|_| "IO")?;
        if len == 0 {
            return Ok(out);
        }
        if len > remaining {
            return Err("BYTES");
        }
        out.extend_from_slice(&buf[..len]);
    }
}
fn push<T>(v: &mut Vec<T>, x: T, max: usize) -> Result<(), E> {
    if v.len() >= max {
        return Err("RESERVA");
    }
    v.try_reserve_exact(1).map_err(|_| "RESERVA")?;
    v.push(x);
    Ok(())
}
struct Decoder<'a> {
    b: &'a [u8],
    i: usize,
    nodes: usize,
}
impl Decoder<'_> {
    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }
    fn take(&mut self) -> Result<u8, E> {
        let b = self.peek().ok_or("JSON_SINTAXIS")?;
        self.i += 1;
        Ok(b)
    }
    fn ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.i += 1
        }
    }
    fn expect(&mut self, b: u8) -> Result<(), E> {
        if self.take()? == b {
            Ok(())
        } else {
            Err("JSON_SINTAXIS")
        }
    }
    fn hex4(&mut self) -> Result<u32, E> {
        let mut x = 0;
        for _ in 0..4 {
            let d = match self.take()? {
                b @ b'0'..=b'9' => (b - b'0') as u32,
                b @ b'a'..=b'f' => (b - b'a' + 10) as u32,
                b @ b'A'..=b'F' => (b - b'A' + 10) as u32,
                _ => return Err("JSON_SINTAXIS"),
            };
            x = x * 16 + d;
        }
        Ok(x)
    }
    fn string(&mut self) -> Result<String, E> {
        self.expect(b'"')?;
        let mut v = Vec::new();
        loop {
            match self.take()? {
                b'"' => break,
                0..=31 => return Err("JSON_SINTAXIS"),
                b'\\' => {
                    let e = self.take()?;
                    let decoded = match e {
                        b'"' => Some(b'"'),
                        b'\\' => Some(b'\\'),
                        b'/' => Some(b'/'),
                        b'b' => Some(8),
                        b'f' => Some(12),
                        b'n' => Some(b'\n'),
                        b'r' => Some(b'\r'),
                        b't' => Some(b'\t'),
                        b'u' => None,
                        _ => return Err("JSON_SINTAXIS"),
                    };
                    if let Some(x) = decoded {
                        push(&mut v, x, 8192)?
                    } else {
                        let hi = self.hex4()?;
                        let cp = if (0xd800..=0xdbff).contains(&hi) {
                            self.expect(b'\\')?;
                            self.expect(b'u')?;
                            let lo = self.hex4()?;
                            if !(0xdc00..=0xdfff).contains(&lo) {
                                return Err("JSON_SINTAXIS");
                            }
                            0x10000 + ((hi - 0xd800) << 10) + (lo - 0xdc00)
                        } else {
                            hi
                        };
                        let ch = char::from_u32(cp).ok_or("JSON_SINTAXIS")?;
                        let mut bytes = [0u8; 4];
                        for x in ch.encode_utf8(&mut bytes).bytes() {
                            push(&mut v, x, 8192)?
                        }
                    }
                }
                x => push(&mut v, x, 8192)?,
            }
        }
        String::from_utf8(v).map_err(|_| "UTF8")
    }
    fn value(&mut self, depth: usize) -> Result<J, E> {
        self.ws();
        if matches!(self.peek(), Some(b'[' | b'{')) && depth >= 32 {
            return Err("PROFUNDIDAD");
        }
        self.nodes += 1;
        if self.nodes > 512 {
            return Err("NODOS");
        }
        match self.peek().ok_or("JSON_SINTAXIS")? {
            b'"' => Ok(J::Text(self.string()?)),
            b'n' => {
                for b in b"null" {
                    self.expect(*b)?
                }
                Ok(J::Null)
            }
            b't' => {
                for b in b"true" {
                    self.expect(*b)?
                }
                Ok(J::Bool(true))
            }
            b'f' => {
                for b in b"false" {
                    self.expect(*b)?
                }
                Ok(J::Bool(false))
            }
            b'-' | b'0'..=b'9' => {
                let start = self.i;
                if self.peek() == Some(b'-') {
                    self.i += 1;
                }
                let first = self.take()?;
                if !first.is_ascii_digit() {
                    return Err("NUMERO_FORMA");
                }
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.i += 1;
                }
                let digits = &self.b[start..self.i];
                if matches!(self.peek(), Some(b'.' | b'e' | b'E'))
                    || digits == b"-0"
                    || (first == b'0' && self.i - start > if digits[0] == b'-' { 2 } else { 1 })
                {
                    return Err("NUMERO_FORMA");
                }
                Ok(J::Number(
                    std::str::from_utf8(digits).map_err(|_| "UTF8")?.into(),
                ))
            }
            b'[' => {
                self.i += 1;
                self.ws();
                let mut a = Vec::new();
                if self.peek() == Some(b']') {
                    self.i += 1;
                    return Ok(J::Array(a));
                }
                loop {
                    let x = self.value(depth + 1)?;
                    push(&mut a, x, 512)?;
                    self.ws();
                    match self.take()? {
                        b']' => break,
                        b',' => (),
                        _ => return Err("JSON_SINTAXIS"),
                    }
                }
                Ok(J::Array(a))
            }
            b'{' => {
                self.i += 1;
                self.ws();
                let mut a: Vec<(String, J)> = Vec::new();
                if self.peek() == Some(b'}') {
                    self.i += 1;
                    return Ok(J::Object(a));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    if a.iter().any(|(old, _)| old == &k) {
                        return Err("CLAVE_DUPLICADA");
                    }
                    if a.len() >= 32 {
                        return Err("MIEMBROS");
                    }
                    self.ws();
                    self.expect(b':')?;
                    let x = self.value(depth + 1)?;
                    push(&mut a, (k, x), 32)?;
                    self.ws();
                    match self.take()? {
                        b'}' => break,
                        b',' => (),
                        _ => return Err("JSON_SINTAXIS"),
                    }
                }
                Ok(J::Object(a))
            }
            _ => Err("JSON_SINTAXIS"),
        }
    }
}
pub fn decode(b: &[u8]) -> Result<J, E> {
    if b.len() > 8192 {
        return Err("BYTES");
    }
    std::str::from_utf8(b).map_err(|_| "UTF8")?;
    let mut d = Decoder { b, i: 0, nodes: 0 };
    let j = d.value(0)?;
    d.ws();
    if d.i != b.len() {
        return Err("JSON_SINTAXIS");
    }
    Ok(j)
}
// ES: Serialización de recibos limitada antes de cada escritura.
// EN: Receipt serialization bounded before every write.
pub fn encode(j: &J, max: usize) -> Result<Vec<u8>, E> {
    struct Out {
        b: Vec<u8>,
        max: usize,
    }
    impl Out {
        fn raw(&mut self, b: &[u8]) -> Result<(), E> {
            if self.b.len().checked_add(b.len()).ok_or("BYTES")? > self.max {
                return Err("BYTES");
            }
            self.b.extend_from_slice(b);
            Ok(())
        }
        fn string(&mut self, s: &str) -> Result<(), E> {
            self.raw(b"\"")?;
            for c in s.chars() {
                match c {
                    '"' => self.raw(b"\\\"")?,
                    '\\' => self.raw(b"\\\\")?,
                    c if (c as u32) < 32 => self.raw(format!("\\u{:04x}", c as u32).as_bytes())?,
                    c => {
                        let mut b = [0; 4];
                        self.raw(c.encode_utf8(&mut b).as_bytes())?
                    }
                }
            }
            self.raw(b"\"")
        }
        fn value(&mut self, j: &J) -> Result<(), E> {
            match j {
                J::Null => self.raw(b"null"),
                J::Bool(b) => self.raw(if *b { b"true" } else { b"false" }),
                J::Number(s) => self.raw(s.as_bytes()),
                J::Text(s) => self.string(s),
                J::Array(a) => {
                    self.raw(b"[")?;
                    for (i, j) in a.iter().enumerate() {
                        if i > 0 {
                            self.raw(b",")?
                        }
                        self.value(j)?
                    }
                    self.raw(b"]")
                }
                J::Object(a) => {
                    self.raw(b"{")?;
                    for (i, (k, j)) in a.iter().enumerate() {
                        if i > 0 {
                            self.raw(b",")?
                        }
                        self.string(k)?;
                        self.raw(b":")?;
                        self.value(j)?
                    }
                    self.raw(b"}")
                }
            }
        }
    }
    let mut b = Vec::new();
    b.try_reserve_exact(max).map_err(|_| "RESERVA")?;
    let mut o = Out { b, max };
    o.value(j)?;
    Ok(o.b)
}
