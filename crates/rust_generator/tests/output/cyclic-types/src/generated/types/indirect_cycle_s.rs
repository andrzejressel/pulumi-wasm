#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IndirectCycleS {
    #[builder(into)]
    #[serde(rename = "foo2")]
    pub r#foo_2: Box<super::types::IndirectCycleT>,
}
