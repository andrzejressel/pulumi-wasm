#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationGatewayHttpListenerCustomErrorConfiguration {
    /// Error page URL of the application gateway customer error.
    #[builder(into)]
    #[serde(rename = "customErrorPageUrl")]
    pub r#custom_error_page_url: Box<String>,
    /// The ID of the Rewrite Rule Set
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Status code of the application gateway customer error. Possible values are `HttpStatus400`, `HttpStatus403`, `HttpStatus404`, `HttpStatus405`, `HttpStatus408`, `HttpStatus500`, `HttpStatus502`, `HttpStatus503` and `HttpStatus504`
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<String>,
}
