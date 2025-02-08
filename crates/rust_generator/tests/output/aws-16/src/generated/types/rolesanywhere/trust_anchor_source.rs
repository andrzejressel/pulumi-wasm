#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TrustAnchorSource {
    /// The data denoting the source of trust, documented below
    #[builder(into)]
    #[serde(rename = "sourceData")]
    pub r#source_data: Box<super::super::types::rolesanywhere::TrustAnchorSourceSourceData>,
    /// The type of the source of trust. Must be either `AWS_ACM_PCA` or `CERTIFICATE_BUNDLE`.
    #[builder(into)]
    #[serde(rename = "sourceType")]
    pub r#source_type: Box<String>,
}
