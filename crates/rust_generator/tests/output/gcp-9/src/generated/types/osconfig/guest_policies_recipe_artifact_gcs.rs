#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GuestPoliciesRecipeArtifactGcs {
    /// Bucket of the Google Cloud Storage object. Given an example URL: https://storage.googleapis.com/my-bucket/foo/bar#1234567
    /// this value would be my-bucket.
    #[builder(into, default)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<Option<String>>,
    /// Must be provided if allowInsecure is false. Generation number of the Google Cloud Storage object.
    /// https://storage.googleapis.com/my-bucket/foo/bar#1234567 this value would be 1234567.
    #[builder(into, default)]
    #[serde(rename = "generation")]
    pub r#generation: Box<Option<i32>>,
    /// Name of the Google Cloud Storage object. Given an example URL: https://storage.googleapis.com/my-bucket/foo/bar#1234567
    /// this value would be foo/bar.
    #[builder(into, default)]
    #[serde(rename = "object")]
    pub r#object: Box<Option<String>>,
}
