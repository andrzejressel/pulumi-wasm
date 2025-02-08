#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpaceSpaceSharingSettings {
    /// Specifies the sharing type of the space. Valid values are `Private` and `Shared`.
    #[builder(into)]
    #[serde(rename = "sharingType")]
    pub r#sharing_type: Box<String>,
}
