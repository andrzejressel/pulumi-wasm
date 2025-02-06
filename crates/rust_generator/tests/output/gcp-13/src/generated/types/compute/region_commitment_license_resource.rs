#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionCommitmentLicenseResource {
    /// The number of licenses purchased.
    #[builder(into, default)]
    #[serde(rename = "amount")]
    pub r#amount: Box<Option<String>>,
    /// Specifies the core range of the instance for which this license applies.
    #[builder(into, default)]
    #[serde(rename = "coresPerLicense")]
    pub r#cores_per_license: Box<Option<String>>,
    /// Any applicable license URI.
    #[builder(into)]
    #[serde(rename = "license")]
    pub r#license: Box<String>,
}
