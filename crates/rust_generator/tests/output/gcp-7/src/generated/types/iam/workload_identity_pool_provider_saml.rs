#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkloadIdentityPoolProviderSaml {
    /// SAML Identity provider configuration metadata xml doc.
    /// 
    /// <a name="nested_x509"></a>The `x509` block supports:
    #[builder(into)]
    #[serde(rename = "idpMetadataXml")]
    pub r#idp_metadata_xml: Box<String>,
}
