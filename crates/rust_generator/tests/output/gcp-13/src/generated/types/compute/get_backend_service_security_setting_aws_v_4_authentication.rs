#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetBackendServiceSecuritySettingAwsV4Authentication {
    /// The access key used for s3 bucket authentication.
    /// Required for updating or creating a backend that uses AWS v4 signature authentication, but will not be returned as part of the configuration when queried with a REST API GET request.
    #[builder(into)]
    #[serde(rename = "accessKey")]
    pub r#access_key: Box<String>,
    /// The identifier of an access key used for s3 bucket authentication.
    #[builder(into)]
    #[serde(rename = "accessKeyId")]
    pub r#access_key_id: Box<String>,
    /// The optional version identifier for the access key. You can use this to keep track of different iterations of your access key.
    #[builder(into)]
    #[serde(rename = "accessKeyVersion")]
    pub r#access_key_version: Box<String>,
    /// The name of the cloud region of your origin. This is a free-form field with the name of the region your cloud uses to host your origin.
    /// For example, "us-east-1" for AWS or "us-ashburn-1" for OCI.
    #[builder(into)]
    #[serde(rename = "originRegion")]
    pub r#origin_region: Box<String>,
}
