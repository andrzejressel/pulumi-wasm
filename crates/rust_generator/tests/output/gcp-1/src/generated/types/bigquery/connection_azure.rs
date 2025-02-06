#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionAzure {
    /// (Output)
    /// The name of the Azure Active Directory Application.
    #[builder(into, default)]
    #[serde(rename = "application")]
    pub r#application: Box<Option<String>>,
    /// (Output)
    /// The client id of the Azure Active Directory Application.
    #[builder(into, default)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    /// The id of customer's directory that host the data.
    #[builder(into)]
    #[serde(rename = "customerTenantId")]
    pub r#customer_tenant_id: Box<String>,
    /// The Azure Application (client) ID where the federated credentials will be hosted.
    #[builder(into, default)]
    #[serde(rename = "federatedApplicationClientId")]
    pub r#federated_application_client_id: Box<Option<String>>,
    /// (Output)
    /// A unique Google-owned and Google-generated identity for the Connection. This identity will be used to access the user's Azure Active Directory Application.
    #[builder(into, default)]
    #[serde(rename = "identity")]
    pub r#identity: Box<Option<String>>,
    /// (Output)
    /// The object id of the Azure Active Directory Application.
    #[builder(into, default)]
    #[serde(rename = "objectId")]
    pub r#object_id: Box<Option<String>>,
    /// (Output)
    /// The URL user will be redirected to after granting consent during connection setup.
    #[builder(into, default)]
    #[serde(rename = "redirectUri")]
    pub r#redirect_uri: Box<Option<String>>,
}
