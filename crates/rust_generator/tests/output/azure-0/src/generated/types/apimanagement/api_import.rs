#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApiImport {
    /// The format of the content from which the API Definition should be imported. Possible values are: `openapi`, `openapi+json`, `openapi+json-link`, `openapi-link`, `swagger-json`, `swagger-link-json`, `wadl-link-json`, `wadl-xml`, `wsdl` and `wsdl-link`.
    #[builder(into)]
    #[serde(rename = "contentFormat")]
    pub r#content_format: Box<String>,
    /// The Content from which the API Definition should be imported. When a `content_format` of `*-link-*` is specified this must be a URL, otherwise this must be defined inline. The URL must be accessible and return a valid document; otherwise, deployment may fail.
    #[builder(into)]
    #[serde(rename = "contentValue")]
    pub r#content_value: Box<String>,
    /// A `wsdl_selector` block as defined below, which allows you to limit the import of a WSDL to only a subset of the document. This can only be specified when `content_format` is `wsdl` or `wsdl-link`.
    #[builder(into, default)]
    #[serde(rename = "wsdlSelector")]
    pub r#wsdl_selector: Box<Option<super::super::types::apimanagement::ApiImportWsdlSelector>>,
}
