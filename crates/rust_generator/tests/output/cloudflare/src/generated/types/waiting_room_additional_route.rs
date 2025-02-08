#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WaitingRoomAdditionalRoute {
    /// The additional host name for which the waiting room to be applied on (no wildcards).
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The path within the additional host to enable the waiting room on. Defaults to `/`.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}
