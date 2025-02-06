#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionResizeRequestStatusLastAttemptErrorErrorErrorDetailHelp {
    /// (Output)
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "links")]
    pub r#links: Box<Option<Vec<super::super::types::compute::RegionResizeRequestStatusLastAttemptErrorErrorErrorDetailHelpLink>>>,
}
