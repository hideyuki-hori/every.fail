
use super::value::Value;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
struct Color {
    r: Value,
    g: Value,
    b: Value,
    a: Value,
}

impl Color {
    #[allow(dead_code)]
    pub fn rgb(r: Value, g: Value, b: Value) -> Self {
        Color {
            r,
            g,
            b,
            a: Value::zero(),
        }
    }

    #[allow(dead_code)]
    pub fn rgba(r: Value, g: Value, b: Value, a: Value) -> Self {
        Color {
            r,
            g,
            b,
            a,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn rgb() {
        let r = Color::rgb(Value::full(), Value::zero(), Value::zero());
        assert_eq!(r.r, Value::full());
        assert_eq!(r.g, Value::zero());
        assert_eq!(r.b, Value::zero());
    }

    #[test]
    fn rgba() {
        let r = Color::rgba(Value::full(), Value::zero(), Value::zero(), Value::full());
        assert_eq!(r.r, Value::full());
        assert_eq!(r.g, Value::zero());
        assert_eq!(r.b, Value::zero());
        assert_eq!(r.a, Value::full());
    }
}