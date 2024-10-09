use serde::Serialize;
use crate::misc::script::Script;
use crate::full_text::intervals::interval_rule::IntervalRule;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all="snake_case")]
pub enum FilterRule {
    After(IntervalRule),
    Before(IntervalRule),
    ContainedBy(IntervalRule),
    Containing(IntervalRule),
    NotContainedBy(IntervalRule),
    NotContaining(IntervalRule),
    NotOverlapping(IntervalRule),
    Overlapping(IntervalRule),
    Script(Script)
}


