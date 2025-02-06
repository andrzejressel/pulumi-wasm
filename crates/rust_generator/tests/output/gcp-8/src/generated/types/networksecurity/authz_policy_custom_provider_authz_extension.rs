#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthzPolicyCustomProviderAuthzExtension {
    /// A list of references to authorization extensions that will be invoked for requests matching this policy. Limited to 1 custom provider.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Vec<String>>,
}
