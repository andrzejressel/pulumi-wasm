#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContinuousDeploymentPolicyStagingDistributionDnsNames {
    /// A list of CloudFront domain names for the staging distribution.
    #[builder(into, default)]
    #[serde(rename = "items")]
    pub r#items: Box<Option<Vec<String>>>,
    /// Number of CloudFront domain names in the staging distribution.
    #[builder(into)]
    #[serde(rename = "quantity")]
    pub r#quantity: Box<i32>,
}