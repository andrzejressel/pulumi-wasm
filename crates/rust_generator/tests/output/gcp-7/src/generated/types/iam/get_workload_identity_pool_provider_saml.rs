#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWorkloadIdentityPoolProviderSaml {
    /// SAML Identity provider configuration metadata xml doc.
    #[builder(into)]
    #[serde(rename = "idpMetadataXml")]
    pub r#idp_metadata_xml: Box<String>,
}
