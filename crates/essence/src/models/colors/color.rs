use super::value::Value;

#[derive(Debug, PartialEq)]
pub struct Color {
    r: Value,
    g: Value,
    b: Value,
    a: Value,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            r: Value::from(r),
            g: Value::from(g),
            b: Value::from(b),
            a: Value::zero(),
        }
    }

    #[allow(dead_code)]
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            r: Value::from(r),
            g: Value::from(g),
            b: Value::from(b),
            a: Value::from(a),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb() {
        let r = Color::rgb(
            Value::full().into(),
            Value::zero().into(),
            Value::zero().into(),
        );
        assert_eq!(r.r, Value::full());
        assert_eq!(r.g, Value::zero());
        assert_eq!(r.b, Value::zero());
    }

    #[test]
    fn rgba() {
        let r = Color::rgba(
            Value::full().into(),
            Value::zero().into(),
            Value::zero().into(),
            Value::full().into(),
        );
        assert_eq!(r.r, Value::full());
        assert_eq!(r.g, Value::zero());
        assert_eq!(r.b, Value::zero());
        assert_eq!(r.a, Value::full());
    }
}
