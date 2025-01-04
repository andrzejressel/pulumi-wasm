#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkforcePoolProviderExtraAttributesOauth2Client {
    /// Represents the IdP and type of claims that should be fetched.
    /// * AZURE_AD_GROUPS_MAIL: Used to get the user's group claims from the Azure AD identity provider using configuration provided
    /// in ExtraAttributesOAuth2Client and 'mail' property of the 'microsoft.graph.group' object is used for claim mapping.
    /// See https://learn.microsoft.com/en-us/graph/api/resources/group?view=graph-rest-1.0#properties for more details on
    /// 'microsoft.graph.group' properties. The attributes obtained from idntity provider are mapped to 'assertion.groups'. Possible values: ["AZURE_AD_GROUPS_MAIL"]
    #[builder(into)]
    #[serde(rename = "attributesType")]
    pub r#attributes_type: Box<String>,
    /// The OAuth 2.0 client ID for retrieving extra attributes from the identity provider. Required to get the Access Token using client credentials grant flow.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The OAuth 2.0 client secret for retrieving extra attributes from the identity provider. Required to get the Access Token using client credentials grant flow.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<super::super::types::iam::WorkforcePoolProviderExtraAttributesOauth2ClientClientSecret>,
    /// The OIDC identity provider's issuer URI. Must be a valid URI using the 'https' scheme. Required to get the OIDC discovery document.
    #[builder(into)]
    #[serde(rename = "issuerUri")]
    pub r#issuer_uri: Box<String>,
    /// Represents the parameters to control which claims are fetched from an IdP.
    #[builder(into, default)]
    #[serde(rename = "queryParameters")]
    pub r#query_parameters: Box<Option<super::super::types::iam::WorkforcePoolProviderExtraAttributesOauth2ClientQueryParameters>>,
}
