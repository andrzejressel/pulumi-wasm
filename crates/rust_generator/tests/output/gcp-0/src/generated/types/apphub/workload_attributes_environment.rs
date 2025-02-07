#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkloadAttributesEnvironment {
    /// Environment type.
    /// Possible values are: `PRODUCTION`, `STAGING`, `TEST`, `DEVELOPMENT`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
