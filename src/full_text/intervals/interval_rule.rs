use serde::Serialize;
use crate::full_text::intervals::rules::{
    r#match::MatchRule,
    prefix::PrefixRule,
    wildcard::WildcardRule,
    fuzzy::FuzzyRule,
    all_of::AllOfRule,
    any_of::AnyOfRule
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all="snake_case")]
pub enum IntervalRule {
    Match(MatchRule),
    Prefix(PrefixRule),
    Wildcard(WildcardRule),
    Fuzzy(FuzzyRule),
    AllOf(AllOfRule),
    AnyOf(AnyOfRule)
}

macro_rules! from_types_enum {

    ($(($rule:ty, $enum_value:ident)),*)=> {
        $(
            impl From<$rule> for IntervalRule {
                fn from(val: $rule) -> Self {
                    Self::$enum_value(val.into())
                }
            }
        )*
    };

}


from_types_enum! {
    (MatchRule, Match),
    (PrefixRule, Prefix),
    (WildcardRule, Wildcard),
    (FuzzyRule, Fuzzy),
    (AllOfRule, AllOf),
    (AnyOfRule, AnyOf)
}
