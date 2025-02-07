#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetListenerDefaultAction {
    #[builder(into)]
    #[serde(rename = "authenticateCognitos")]
    pub r#authenticate_cognitos: Box<Vec<super::super::types::lb::GetListenerDefaultActionAuthenticateCognito>>,
    #[builder(into)]
    #[serde(rename = "authenticateOidcs")]
    pub r#authenticate_oidcs: Box<Vec<super::super::types::lb::GetListenerDefaultActionAuthenticateOidc>>,
    #[builder(into)]
    #[serde(rename = "fixedResponses")]
    pub r#fixed_responses: Box<Vec<super::super::types::lb::GetListenerDefaultActionFixedResponse>>,
    #[builder(into)]
    #[serde(rename = "forwards")]
    pub r#forwards: Box<Vec<super::super::types::lb::GetListenerDefaultActionForward>>,
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Box<i32>,
    #[builder(into)]
    #[serde(rename = "redirects")]
    pub r#redirects: Box<Vec<super::super::types::lb::GetListenerDefaultActionRedirect>>,
    #[builder(into)]
    #[serde(rename = "targetGroupArn")]
    pub r#target_group_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
