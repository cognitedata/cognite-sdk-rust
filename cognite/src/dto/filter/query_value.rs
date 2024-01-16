use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
/// Raw filter value.
pub enum RawValue {
    /// String value.
    String(String),
    /// Numerical value.
    Number(serde_json::Number),
    /// Boolean value.
    Boolean(bool),
    /// String array value.
    StringArray(Vec<String>),
    /// Number array value.
    NumberArray(Vec<serde_json::Number>),
    /// Boolean array value.
    BooleanArray(Vec<bool>),
    /// JSON object value.
    Object(serde_json::Value),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Parameterized value.
pub struct ParameterizedPropertyValue {
    /// Parameter reference.
    pub parameter: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Referenced property value, use the value of a different property.
pub struct ReferencedPropertyValue {
    /// Property to reference.
    pub property: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", untagged)]
/// Value used in complex queries.
pub enum QueryValue {
    /// Parameterized value.
    Parameter(ParameterizedPropertyValue),
    /// Reference to a different property.
    Reference(ReferencedPropertyValue),
    /// Raw value.
    Raw(RawValue),
}

impl<T> From<T> for QueryValue
where
    T: Into<RawValue>,
{
    fn from(value: T) -> Self {
        QueryValue::Raw(value.into())
    }
}

impl From<ParameterizedPropertyValue> for QueryValue {
    fn from(value: ParameterizedPropertyValue) -> Self {
        QueryValue::Parameter(value)
    }
}

impl From<ReferencedPropertyValue> for QueryValue {
    fn from(value: ReferencedPropertyValue) -> Self {
        QueryValue::Reference(value)
    }
}

mod from_impls {
    use super::RawValue;

    macro_rules! from_num_impl {
        ($typ:ty) => {
            impl From<$typ> for RawValue {
                fn from(value: $typ) -> Self {
                    RawValue::Number(value.into())
                }
            }

            impl From<Vec<$typ>> for RawValue {
                fn from(value: Vec<$typ>) -> Self {
                    RawValue::NumberArray(value.into_iter().map(serde_json::Number::from).collect())
                }
            }

            impl From<&[$typ]> for RawValue {
                fn from(value: &[$typ]) -> Self {
                    RawValue::NumberArray(
                        value
                            .iter()
                            .copied()
                            .map(serde_json::Number::from)
                            .collect(),
                    )
                }
            }

            impl<const N: usize> From<&[$typ; N]> for RawValue {
                fn from(value: &[$typ; N]) -> Self {
                    RawValue::NumberArray(
                        value
                            .iter()
                            .copied()
                            .map(serde_json::Number::from)
                            .collect(),
                    )
                }
            }
        };
    }

    impl From<String> for RawValue {
        fn from(value: String) -> Self {
            RawValue::String(value)
        }
    }

    impl From<&str> for RawValue {
        fn from(value: &str) -> Self {
            RawValue::String(value.to_string())
        }
    }

    impl From<f64> for RawValue {
        fn from(value: f64) -> Self {
            RawValue::Number(
                serde_json::Number::from_f64(value)
                    .unwrap_or_else(|| serde_json::Number::from_f64(0.0).unwrap()),
            )
        }
    }

    impl From<f32> for RawValue {
        fn from(value: f32) -> Self {
            RawValue::Number(
                serde_json::Number::from_f64(value.into())
                    .unwrap_or_else(|| serde_json::Number::from_f64(0.0).unwrap()),
            )
        }
    }

    from_num_impl!(i64);
    from_num_impl!(i32);
    from_num_impl!(i16);
    from_num_impl!(u64);
    from_num_impl!(u32);
    from_num_impl!(u16);

    impl From<bool> for RawValue {
        fn from(value: bool) -> Self {
            RawValue::Boolean(value)
        }
    }

    impl From<Vec<String>> for RawValue {
        fn from(value: Vec<String>) -> Self {
            RawValue::StringArray(value)
        }
    }

    impl From<&[&str]> for RawValue {
        fn from(value: &[&str]) -> Self {
            RawValue::StringArray(value.iter().map(|&v| v.to_owned()).collect())
        }
    }

    impl<const N: usize> From<&[&str; N]> for RawValue {
        fn from(value: &[&str; N]) -> Self {
            RawValue::StringArray(value.iter().map(|&v| v.to_owned()).collect())
        }
    }

    impl From<Vec<bool>> for RawValue {
        fn from(value: Vec<bool>) -> Self {
            RawValue::BooleanArray(value)
        }
    }

    impl From<&[bool]> for RawValue {
        fn from(value: &[bool]) -> Self {
            RawValue::BooleanArray(value.to_owned())
        }
    }

    impl<const N: usize> From<&[bool; N]> for RawValue {
        fn from(value: &[bool; N]) -> Self {
            RawValue::BooleanArray(value.to_vec())
        }
    }

    impl From<&[f32]> for RawValue {
        fn from(value: &[f32]) -> Self {
            RawValue::NumberArray(
                value
                    .iter()
                    .copied()
                    .map(|v| {
                        serde_json::Number::from_f64(v.into())
                            .unwrap_or_else(|| serde_json::Number::from_f64(0.0).unwrap())
                    })
                    .collect(),
            )
        }
    }

    impl<const N: usize> From<&[f32; N]> for RawValue {
        fn from(value: &[f32; N]) -> Self {
            RawValue::NumberArray(
                value
                    .iter()
                    .copied()
                    .map(|v| {
                        serde_json::Number::from_f64(v.into())
                            .unwrap_or_else(|| serde_json::Number::from_f64(0.0).unwrap())
                    })
                    .collect(),
            )
        }
    }

    impl From<Vec<f32>> for RawValue {
        fn from(value: Vec<f32>) -> Self {
            RawValue::NumberArray(
                value
                    .into_iter()
                    .map(|v| {
                        serde_json::Number::from_f64(v.into())
                            .unwrap_or_else(|| serde_json::Number::from_f64(0.0).unwrap())
                    })
                    .collect(),
            )
        }
    }

    impl From<&[f64]> for RawValue {
        fn from(value: &[f64]) -> Self {
            RawValue::NumberArray(
                value
                    .iter()
                    .copied()
                    .map(|v| {
                        serde_json::Number::from_f64(v)
                            .unwrap_or_else(|| serde_json::Number::from_f64(0.0).unwrap())
                    })
                    .collect(),
            )
        }
    }

    impl<const N: usize> From<&[f64; N]> for RawValue {
        fn from(value: &[f64; N]) -> Self {
            RawValue::NumberArray(
                value
                    .iter()
                    .copied()
                    .map(|v| {
                        serde_json::Number::from_f64(v)
                            .unwrap_or_else(|| serde_json::Number::from_f64(0.0).unwrap())
                    })
                    .collect(),
            )
        }
    }

    impl From<Vec<f64>> for RawValue {
        fn from(value: Vec<f64>) -> Self {
            RawValue::NumberArray(
                value
                    .into_iter()
                    .map(|v| {
                        serde_json::Number::from_f64(v)
                            .unwrap_or_else(|| serde_json::Number::from_f64(0.0).unwrap())
                    })
                    .collect(),
            )
        }
    }

    impl From<serde_json::Value> for RawValue {
        fn from(value: serde_json::Value) -> Self {
            RawValue::Object(value)
        }
    }
}
