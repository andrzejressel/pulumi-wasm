#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionBackendServiceSubsetting {
    /// The algorithm used for subsetting.
    /// Possible values are: `CONSISTENT_HASH_SUBSETTING`.
    #[builder(into)]
    #[serde(rename = "policy")]
    pub r#policy: Box<String>,
}
