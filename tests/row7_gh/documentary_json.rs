//! Subconjunto documental de RETP-101. No importa ni amplía el lenguaje SV.
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

pub fn parse(bytes: &[u8]) -> Result<Value, &'static str> {
    let text = std::str::from_utf8(bytes).map_err(|_| "JSON_UTF8")?;
    let mut reader = Reader { text, at: 0 };
    let value = reader.value()?;
    reader.space();
    if reader.at != text.len() { return Err("JSON_SINTAXIS"); }
    Ok(value)
}

struct Reader<'a> { text: &'a str, at: usize }
impl Reader<'_> {
    fn byte(&self) -> Option<u8> { self.text.as_bytes().get(self.at).copied() }
    fn space(&mut self) {
        while matches!(self.byte(), Some(b' ' | b'\r' | b'\n' | b'\t')) { self.at += 1; }
    }
    fn take(&mut self, byte: u8) -> Result<(), &'static str> {
        if self.byte() != Some(byte) { return Err("JSON_SINTAXIS"); }
        self.at += 1;
        Ok(())
    }
    fn keyword(&mut self, text: &str, value: Value) -> Result<Value, &'static str> {
        if !self.text[self.at..].starts_with(text) { return Err("JSON_SINTAXIS"); }
        self.at += text.len();
        Ok(value)
    }
    fn value(&mut self) -> Result<Value, &'static str> {
        self.space();
        match self.byte() {
            Some(b'n') => self.keyword("null", Value::Null),
            Some(b't') => self.keyword("true", Value::Bool(true)),
            Some(b'f') => self.keyword("false", Value::Bool(false)),
            Some(b'"') => self.string().map(Value::String),
            Some(b'[') => self.array(),
            Some(b'{') => self.object(),
            Some(b'-' | b'0'..=b'9') => self.number(),
            _ => Err("JSON_SINTAXIS"),
        }
    }
    fn number(&mut self) -> Result<Value, &'static str> {
        let start = self.at;
        while matches!(self.byte(), Some(b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')) {
            self.at += 1;
        }
        let token = &self.text[start..self.at];
        if !canonical_integer(token) { return Err("JSON_NUMERO_FORMA"); }
        // Se compara el token; no se convierte primero a un número de máquina.
        if !decimal_at_most(token, "9007199254740991") { return Err("JSON_NUMERO_RANGO"); }
        Ok(Value::Number(token.to_owned()))
    }
    fn string(&mut self) -> Result<String, &'static str> {
        self.take(b'"')?;
        let mut output = String::new();
        loop {
            match self.byte().ok_or("JSON_SINTAXIS")? {
                b'"' => { self.at += 1; return Ok(output); }
                b'\\' => {
                    self.at += 1;
                    let escaped = self.byte().ok_or("JSON_SINTAXIS")?;
                    self.at += 1;
                    match escaped {
                        b'"' => output.push('"'), b'\\' => output.push('\\'), b'/' => output.push('/'),
                        b'b' => output.push('\u{08}'), b'f' => output.push('\u{0c}'),
                        b'n' => output.push('\n'), b'r' => output.push('\r'), b't' => output.push('\t'),
                        b'u' => {
                            let high = self.hex4()?;
                            let scalar = if (0xd800..=0xdbff).contains(&high) {
                                if !self.text[self.at..].starts_with("\\u") { return Err("JSON_UNICODE"); }
                                self.at += 2;
                                let low = self.hex4()?;
                                if !(0xdc00..=0xdfff).contains(&low) { return Err("JSON_UNICODE"); }
                                0x10000 + ((high - 0xd800) << 10) + low - 0xdc00
                            } else { high };
                            output.push(char::from_u32(scalar).ok_or("JSON_UNICODE")?);
                        }
                        _ => return Err("JSON_SINTAXIS"),
                    }
                }
                0..=31 => return Err("JSON_SINTAXIS"),
                _ => {
                    let c = self.text[self.at..].chars().next().ok_or("JSON_SINTAXIS")?;
                    output.push(c);
                    self.at += c.len_utf8();
                }
            }
        }
    }
    fn hex4(&mut self) -> Result<u32, &'static str> {
        let mut result = 0;
        for _ in 0..4 {
            let byte = self.byte().ok_or("JSON_SINTAXIS")?;
            let digit = (byte as char).to_digit(16).ok_or("JSON_SINTAXIS")?;
            result = result * 16 + digit;
            self.at += 1;
        }
        Ok(result)
    }
    fn array(&mut self) -> Result<Value, &'static str> {
        self.take(b'[')?;
        self.space();
        let mut items = Vec::new();
        if self.byte() == Some(b']') { self.at += 1; return Ok(Value::Array(items)); }
        loop {
            items.push(self.value()?);
            self.space();
            if self.byte() == Some(b']') { self.at += 1; return Ok(Value::Array(items)); }
            self.take(b',')?;
        }
    }
    fn object(&mut self) -> Result<Value, &'static str> {
        self.take(b'{')?;
        self.space();
        let mut items = Vec::new();
        let mut keys = HashSet::new();
        if self.byte() == Some(b'}') { self.at += 1; return Ok(Value::Object(items)); }
        loop {
            self.space();
            let key = self.string()?;
            if !keys.insert(key.clone()) { return Err("JSON_CLAVE_REPETIDA"); }
            if canonical_integer(&key) && decimal_at_most(&key, "4294967294") {
                return Err("JSON_CLAVE_INDICE");
            }
            self.space();
            self.take(b':')?;
            items.push((key, self.value()?));
            self.space();
            if self.byte() == Some(b'}') { self.at += 1; return Ok(Value::Object(items)); }
            self.take(b',')?;
        }
    }
}

fn canonical_integer(text: &str) -> bool {
    text == "0" || (text.as_bytes().first().is_some_and(|b| (b'1'..=b'9').contains(b))
        && text.bytes().all(|b| b.is_ascii_digit()))
}
fn decimal_at_most(token: &str, maximum: &str) -> bool {
    token.len() < maximum.len() || (token.len() == maximum.len() && token <= maximum)
}

fn quoted(text: &str) -> String {
    let mut output = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => output.push_str("\\\""), '\\' => output.push_str("\\\\"),
            '\u{08}' => output.push_str("\\b"), '\u{0c}' => output.push_str("\\f"),
            '\n' => output.push_str("\\n"), '\r' => output.push_str("\\r"), '\t' => output.push_str("\\t"),
            '\0'..='\u{1f}' => output.push_str(&format!("\\u{:04x}", c as u32)),
            _ => output.push(c),
        }
    }
    output.push('"');
    output
}
impl Value {
    pub fn compact(&self) -> String {
        match self {
            Self::Null => "null".into(), Self::Bool(b) => b.to_string(),
            Self::Number(n) => n.clone(), Self::String(s) => quoted(s),
            Self::Array(xs) => format!("[{}]", xs.iter().map(Value::compact).collect::<Vec<_>>().join(",")),
            Self::Object(xs) => Self::compact_members(xs.iter().map(|(k, v)| (k.as_str(), v))),
        }
    }
    pub fn compact_members<'a>(members: impl Iterator<Item = (&'a str, &'a Value)>) -> String {
        format!("{{{}}}", members.map(|(k,v)| format!("{}:{}", quoted(k), v.compact())).collect::<Vec<_>>().join(","))
    }
    pub fn members(&self) -> Result<&[(String, Value)], &'static str> {
        if let Self::Object(xs) = self { Ok(xs) } else { Err("GH_OBJETO") }
    }
    pub fn get(&self, name: &str) -> Result<&Value, &'static str> {
        self.members()?.iter().find(|(key, _)| key == name).map(|(_, v)| v).ok_or("GH_CAMPO_AUSENTE")
    }
    pub fn array(&self) -> Result<&[Value], &'static str> {
        if let Self::Array(xs) = self { Ok(xs) } else { Err("GH_LISTA") }
    }
    pub fn text(&self) -> Result<&str, &'static str> {
        if let Self::String(s) = self { Ok(s) } else { Err("GH_TEXTO") }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_fixed_values_order_and_unicode() {
        let input = br#" {"z":[0,9007199254740991,true,false,null],"a":"\u00f1\ud83d\ude00"} "#;
        assert_eq!(parse(input).unwrap().compact(), "{\"z\":[0,9007199254740991,true,false,null],\"a\":\"ñ😀\"}");
        assert_eq!(parse(br#""\b\f\n\r\t\u0000\/\\\"""#).unwrap().compact(), r#""\b\f\n\r\t\u0000/\\\"""#);
    }
    #[test]
    fn rejects_numeric_conversion_before_it_loses_information() {
        for token in ["1.0", "1e0", "-0", "9007199254740990.1", "01"] {
            assert_eq!(parse(token.as_bytes()), Err("JSON_NUMERO_FORMA"), "{token}");
        }
        assert_eq!(parse(b"9007199254740992"), Err("JSON_NUMERO_RANGO"));
    }
    #[test]
    fn rejects_duplicate_and_index_keys() {
        for token in [r#"{"a":0,"a":1}"#, r#"{"a":0,"\u0061":1}"#] {
            assert_eq!(parse(token.as_bytes()), Err("JSON_CLAVE_REPETIDA"));
        }
        for key in ["0", "1", "4294967294"] {
            assert_eq!(parse(format!("{{\"{key}\":0}}").as_bytes()), Err("JSON_CLAVE_INDICE"));
        }
        let allowed = br#"{"4294967295":0,"01":1,"__proto__":null}"#;
        assert_eq!(parse(allowed).unwrap().compact().as_bytes(), allowed);
    }
    #[test]
    fn rejects_invalid_unicode_and_utf8() {
        for token in [r#""\ud800""#, r#""\udc00""#, r#""\ud800\u0041""#] {
            assert_eq!(parse(token.as_bytes()), Err("JSON_UNICODE"));
        }
        for bytes in [&b"\"\xff\""[..], &b"\"\xed\xa0\x80\""[..], &b"\"\xc3"[..]] {
            assert_eq!(parse(bytes), Err("JSON_UTF8"));
        }
    }
    #[test]
    fn rejects_json_syntax_instead_of_repairing_it() {
        for token in ["", "[1,]", "{\"a\":0,}", "null null", "[1 2]", "{\"a\" 0}", "\"\\x\"", "\"\n\""] {
            assert_eq!(parse(token.as_bytes()), Err("JSON_SINTAXIS"), "{token}");
        }
    }
}
