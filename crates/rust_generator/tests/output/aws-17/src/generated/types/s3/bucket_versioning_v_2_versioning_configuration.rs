#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketVersioningV2VersioningConfiguration {
    /// Specifies whether MFA delete is enabled in the bucket versioning configuration. Valid values: `Enabled` or `Disabled`.
    #[builder(into, default)]
    #[serde(rename = "mfaDelete")]
    pub r#mfa_delete: Box<Option<String>>,
    /// Versioning state of the bucket. Valid values: `Enabled`, `Suspended`, or `Disabled`. `Disabled` should only be used when creating or importing resources that correspond to unversioned S3 buckets.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
