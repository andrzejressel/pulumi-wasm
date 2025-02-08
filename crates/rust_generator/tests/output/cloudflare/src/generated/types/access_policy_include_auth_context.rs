#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessPolicyIncludeAuthContext {
    /// The ACID of the Authentication Context.
    #[builder(into)]
    #[serde(rename = "acId")]
    pub r#ac_id: Box<String>,
    /// The ID of the Authentication Context.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The ID of the Azure identity provider.
    #[builder(into)]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Box<String>,
}
