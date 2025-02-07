#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionSnapStart {
    /// Conditions where snap start is enabled. Valid values are `PublishedVersions`.
    #[builder(into)]
    #[serde(rename = "applyOn")]
    pub r#apply_on: Box<String>,
    /// Optimization status of the snap start configuration. Valid values are `On` and `Off`.
    #[builder(into, default)]
    #[serde(rename = "optimizationStatus")]
    pub r#optimization_status: Box<Option<String>>,
}
