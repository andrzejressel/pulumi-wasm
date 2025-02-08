#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RepositoryImageScanningConfiguration {
    /// Indicates whether images are scanned after being pushed to the repository (true) or not scanned (false).
    #[builder(into)]
    #[serde(rename = "scanOnPush")]
    pub r#scan_on_push: Box<bool>,
}
