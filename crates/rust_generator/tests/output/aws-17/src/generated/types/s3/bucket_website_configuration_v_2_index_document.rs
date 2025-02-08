#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketWebsiteConfigurationV2IndexDocument {
    /// Suffix that is appended to a request that is for a directory on the website endpoint.
    /// For example, if the suffix is `index.html` and you make a request to `samplebucket/images/`, the data that is returned will be for the object with the key name `images/index.html`.
    /// The suffix must not be empty and must not include a slash character.
    #[builder(into)]
    #[serde(rename = "suffix")]
    pub r#suffix: Box<String>,
}
