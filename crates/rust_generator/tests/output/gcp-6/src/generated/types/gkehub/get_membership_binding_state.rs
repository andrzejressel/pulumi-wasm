#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetMembershipBindingState {
    /// Code describes the state of a MembershipBinding resource.
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: Box<String>,
}
