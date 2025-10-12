//! Heavily inspired by the json crate, but with all static str.
//! The parser part is bacisally a 1-1 copy of the orignial one, expect it builds our static types.
//! Oh, and I switched out the nice optimized binary tree for a O(n) linear storage.

mod object;
mod parser;

pub use parser::parse;

static NULL: StaticJsonValue = StaticJsonValue::Null;

/// Our own json value, very similar to the json crate (even reusing some types)
/// except the String variant stores a &'static str.
/// Also the types provide utility functions usefull for own usecase.
#[derive(Debug, Clone)]
pub enum StaticJsonValue {
    Null,
    String(&'static str),
    Number(json::number::Number),
    Boolean(bool),
    Object(object::StaticJsonObject),
    Array(Vec<StaticJsonValue>),
}

impl StaticJsonValue {
    pub fn kind(&self) -> &str {
        match self {
            StaticJsonValue::Null => "null",
            StaticJsonValue::String(_) => "string",
            StaticJsonValue::Number(_) => "number",
            StaticJsonValue::Boolean(_) => "boolean",
            StaticJsonValue::Array(_) => "array",
            StaticJsonValue::Object(_) => "object",
        }
    }
}

impl<'a> std::ops::Index<&'a str> for StaticJsonValue {
    type Output = StaticJsonValue;
    fn index(&self, index: &'a str) -> &Self::Output {
        match self {
            StaticJsonValue::Object(obj) => obj.index(index),
            _ => &NULL,
        }
    }
}

pub trait FromJsonValue: Sized {
    fn from_json_value(json: &StaticJsonValue) -> Result<Self, String>;
}

impl<'a> FromJsonValue for &'a str {
    fn from_json_value(json: &StaticJsonValue) -> Result<Self, String> {
        match json {
            StaticJsonValue::String(s) => Ok(s),
            other => Err(format!("Expected string, found {}", other.kind())),
        }
    }
}

impl FromJsonValue for bool {
    fn from_json_value(json: &StaticJsonValue) -> Result<Self, String> {
        match json {
            StaticJsonValue::Boolean(b) => Ok(*b),
            other => Err(format!("Expected boolean, found {}", other.kind())),
        }
    }
}

impl FromJsonValue for u64 {
    fn from_json_value(json: &StaticJsonValue) -> Result<Self, String> {
        match json {
            StaticJsonValue::Number(num) => {
                Ok(u64::try_from(*num)
                    .map_err(|_| format!("Invalid json number for u64: {num}"))?)
            }
            other => Err(format!("Expected string, found {}", other.kind())),
        }
    }
}

impl FromJsonValue for f64 {
    fn from_json_value(json: &StaticJsonValue) -> Result<Self, String> {
        match json {
            StaticJsonValue::Number(num) => Ok(f64::from(*num)),
            other => Err(format!("Expected string, found {}", other.kind())),
        }
    }
}

impl<T: FromJsonValue> FromJsonValue for Option<T> {
    fn from_json_value(json: &StaticJsonValue) -> Result<Self, String> {
        match json {
            StaticJsonValue::Null => Ok(None),
            other => T::from_json_value(other).map(Some),
        }
    }
}

impl<T: FromJsonValue, const N: usize> FromJsonValue for arrayvec::ArrayVec<T, N> {
    fn from_json_value(json: &StaticJsonValue) -> Result<Self, String> {
        match json {
            StaticJsonValue::Array(array) => {
                if array.len() <= N {
                    let mut result = arrayvec::ArrayVec::new();
                    for elem in array.iter() {
                        result.push(
                            T::from_json_value(elem)
                                .map_err(|e| format!("When building array: {e}"))?,
                        );
                    }
                    Ok(result)
                } else {
                    Err(format!(
                        "Array of length {} is too big for arrayvec of max cap {}",
                        array.len(),
                        N
                    ))
                }
            }
            other => Err(format!("Expected string, found {}", other.kind())),
        }
    }
}
