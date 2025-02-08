#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FlexibleAppVersionDeploymentContainer {
    /// URI to the hosted container image in Google Container Registry. The URI must be fully qualified and include a tag or digest.
    /// Examples: "gcr.io/my-project/image:tag" or "gcr.io/my-project/image@digest"
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Box<String>,
}
