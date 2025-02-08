#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetOsProfileWindow {
    /// A `patch` block as defined above.
    #[builder(into)]
    #[serde(rename = "patches")]
    pub r#patches: Box<Vec<super::super::types::arcmachine::GetOsProfileWindowPatch>>,
}
