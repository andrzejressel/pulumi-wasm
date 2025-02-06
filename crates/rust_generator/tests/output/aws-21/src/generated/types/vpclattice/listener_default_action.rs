#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListenerDefaultAction {
    #[builder(into, default)]
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Box<Option<super::super::types::vpclattice::ListenerDefaultActionFixedResponse>>,
    /// Route requests to one or more target groups. See Forward blocks below.
    /// 
    /// > **NOTE:** You must specify exactly one of the following argument blocks: `fixed_response` or `forward`.
    #[builder(into, default)]
    #[serde(rename = "forwards")]
    pub r#forwards: Box<Option<Vec<super::super::types::vpclattice::ListenerDefaultActionForward>>>,
}
