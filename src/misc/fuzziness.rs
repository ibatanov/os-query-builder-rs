use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum Fuzziness {
    Auto,
    Uint(u64),
}

impl<T: Into<u64>> From<T> for Fuzziness {
    fn from(val: T) -> Self {
        Self::Uint(val.into())
    }
}

impl Serialize for Fuzziness {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            Fuzziness::Auto => serializer.serialize_str("AUTO"),
            Fuzziness::Uint(u) => serializer.serialize_u64(*u),
        }
    }
}