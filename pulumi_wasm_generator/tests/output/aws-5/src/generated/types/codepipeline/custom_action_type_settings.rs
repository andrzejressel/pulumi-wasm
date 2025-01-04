#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomActionTypeSettings {
    /// The URL returned to the AWS CodePipeline console that provides a deep link to the resources of the external system.
    #[builder(into, default)]
    #[serde(rename = "entityUrlTemplate")]
    pub r#entity_url_template: Box<Option<String>>,
    /// The URL returned to the AWS CodePipeline console that contains a link to the top-level landing page for the external system.
    #[builder(into, default)]
    #[serde(rename = "executionUrlTemplate")]
    pub r#execution_url_template: Box<Option<String>>,
    /// The URL returned to the AWS CodePipeline console that contains a link to the page where customers can update or change the configuration of the external action.
    #[builder(into, default)]
    #[serde(rename = "revisionUrlTemplate")]
    pub r#revision_url_template: Box<Option<String>>,
    /// The URL of a sign-up page where users can sign up for an external service and perform initial configuration of the action provided by that service.
    #[builder(into, default)]
    #[serde(rename = "thirdPartyConfigurationUrl")]
    pub r#third_party_configuration_url: Box<Option<String>>,
}
