#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AcyclicReferent {
    #[builder(into)]
    #[serde(rename = "bar")]
    pub r#bar: Box<super::types::IndirectCycleS>,
    #[builder(into)]
    #[serde(rename = "baz")]
    pub r#baz: Box<super::types::IndirectCycleT>,
    #[builder(into)]
    #[serde(rename = "foo4")]
    pub r#foo_4: Box<super::types::DirectCycle>,
}
