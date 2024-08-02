use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TermType {
    String(String),
    Int(i128),
    UInt(u128),
    Float(f64)
}

macro_rules! from_types {
    ($(($from:ty, $to:ident)),*) => {
        $(
            impl From<$from> for TermType {
                fn from(val: $from) -> Self {
                    Self::$to(val.try_into().expect("try convert to TermType"))
                }
            }
        )*
    }
}

from_types! {
    (String, String),
    (char, String),
    (&str, String),
    
    (i8, Int),
    (i16, Int),
    (i32, Int),
    (i64, Int),
    (i128, Int),
    (isize, Int),
    
    (u8, UInt),
    (u16, UInt),
    (u32, UInt),
    (u64, UInt),
    (u128, UInt), 
    (usize, UInt),
    
    (f32, Float),
    (f64, Float)
}
