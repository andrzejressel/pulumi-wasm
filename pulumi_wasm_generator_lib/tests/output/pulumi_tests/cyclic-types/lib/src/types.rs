
#[derive(serde::Serialize)]
pub struct AcyclicReferent {
    #[serde(rename = "bar")]
    pub r#bar: Box<crate::types::IndirectCycleS>,
    #[serde(rename = "baz")]
    pub r#baz: Box<crate::types::IndirectCycleT>,
    #[serde(rename = "foo4")]
    pub r#foo_4: Box<crate::types::DirectCycle>,
}


#[derive(serde::Serialize)]
pub struct AcyclicS {
    #[serde(rename = "foo5")]
    pub r#foo_5: Box<String>,
}


#[derive(serde::Serialize)]
pub struct AcyclicT {
    #[serde(rename = "foo6")]
    pub r#foo_6: Box<crate::types::AcyclicS>,
}


#[derive(serde::Serialize)]
pub struct DirectCycle {
    #[serde(rename = "foo")]
    pub r#foo: Box<crate::types::DirectCycle>,
}


#[derive(serde::Serialize)]
pub struct IndirectCycleS {
    #[serde(rename = "foo2")]
    pub r#foo_2: Box<crate::types::IndirectCycleT>,
}


#[derive(serde::Serialize)]
pub struct IndirectCycleT {
    #[serde(rename = "foo3")]
    pub r#foo_3: Box<crate::types::IndirectCycleS>,
}


