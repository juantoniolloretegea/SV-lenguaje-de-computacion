/// Natural canónico del Lenguaje SV.
///
/// La gramática define `nat` como una secuencia decimal sin cota semántica.
/// Esta representación conserva ese dominio sin depender del tamaño de palabra
/// de la plataforma. En R0-2 sólo se necesita identidad y conservación exacta;
/// la aritmética y el orden sobre naturales quedan fuera de este frente.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Nat {
    decimal: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidNat;

impl Nat {
    /// Construye un natural desde una representación decimal.
    ///
    /// Se aceptan únicamente dígitos ASCII. Los ceros iniciales se eliminan
    /// para que valores léxicamente distintos con el mismo valor natural tengan
    /// una única representación interna; cero se conserva como `"0"`.
    pub fn from_decimal(value: &str) -> Result<Self, InvalidNat> {
        if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(InvalidNat);
        }

        let canonical = value.trim_start_matches('0');
        let decimal = if canonical.is_empty() { "0" } else { canonical };

        Ok(Self {
            decimal: decimal.to_owned(),
        })
    }

    pub fn from_u64(value: u64) -> Self {
        Self {
            decimal: value.to_string(),
        }
    }

    /// Representación decimal canónica del valor natural.
    pub fn as_decimal(&self) -> &str {
        &self.decimal
    }
}

impl TryFrom<&str> for Nat {
    type Error = InvalidNat;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_decimal(value)
    }
}

impl TryFrom<String> for Nat {
    type Error = InvalidNat;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_decimal(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nat_accepts_values_beyond_u64_without_narrowing() {
        let value = "184467440737095516160000000000000000000";
        let nat = Nat::from_decimal(value).expect("natural válido");
        assert_eq!(nat.as_decimal(), value);
    }

    #[test]
    fn nat_has_one_internal_decimal_representation_per_value() {
        assert_eq!(
            Nat::from_decimal("000123").expect("natural válido").as_decimal(),
            "123"
        );
        assert_eq!(Nat::from_decimal("000").expect("natural válido").as_decimal(), "0");
    }

    #[test]
    fn nat_rejects_non_decimal_material() {
        for invalid in ["", "-1", "+1", "1.0", " 1", "1 ", "１２"] {
            assert_eq!(Nat::from_decimal(invalid), Err(InvalidNat));
        }
    }
}
