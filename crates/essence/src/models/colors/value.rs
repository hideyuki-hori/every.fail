

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value(u8);

impl From<Value> for u8 {
    fn from(value: Value) -> Self {
        value.0
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value(value)
    }
}

impl Value {
    pub const fn zero() -> Self {
        Value(0)
    }

    pub const fn full() -> Self {
        Value(255)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let val = Value::from(128);
        assert_eq!(val.0, 128);
    }

    #[test]
    fn zero() {
        let zero = Value::zero();
        assert_eq!(zero.0, 0);
        assert_eq!(zero, Value::from(0));
    }

    #[test]
    fn full() {
        let full = Value::full();
        assert_eq!(full.0, 255);
        assert_eq!(full, Value::from(255));
    }

    #[test]
    fn equality() {
        let val1 = Value::from(100);
        let val2 = Value::from(100);
        let val3 = Value::from(200);

        assert_eq!(val1, val2);
        assert_ne!(val1, val3);
    }

    #[test]
    fn clone() {
        let original = Value::from(42);
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }

    #[test]
    fn copy() {
        let original = Value::from(42);
        let copied = original;

        assert_eq!(original, copied);
        assert_eq!(original.0, 42);
    }

    #[test]
    fn into() {
        let value = Value::from(128);
        let raw: u8 = value.into();
        assert_eq!(raw, 128);
    }
}
