#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetListenerDefaultAction {
    #[builder(into)]
    #[serde(rename = "fixedResponses")]
    pub r#fixed_responses: Box<Vec<super::super::types::vpclattice::GetListenerDefaultActionFixedResponse>>,
    #[builder(into)]
    #[serde(rename = "forwards")]
    pub r#forwards: Box<Vec<super::super::types::vpclattice::GetListenerDefaultActionForward>>,
}
