#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EventConnectionAuthParameters {
    /// Parameters used for API_KEY authorization. An API key to include in the header for each authentication request. A maximum of 1 are allowed. Conflicts with `basic` and `oauth`. Documented below.
    #[builder(into, default)]
    #[serde(rename = "apiKey")]
    pub r#api_key: Box<Option<super::super::types::cloudwatch::EventConnectionAuthParametersApiKey>>,
    /// Parameters used for BASIC authorization. A maximum of 1 are allowed. Conflicts with `api_key` and `oauth`. Documented below.
    #[builder(into, default)]
    #[serde(rename = "basic")]
    pub r#basic: Box<Option<super::super::types::cloudwatch::EventConnectionAuthParametersBasic>>,
    /// Invocation Http Parameters are additional credentials used to sign each Invocation of the ApiDestination created from this Connection. If the ApiDestination Rule Target has additional HttpParameters, the values will be merged together, with the Connection Invocation Http Parameters taking precedence. Secret values are stored and managed by AWS Secrets Manager. A maximum of 1 are allowed. Documented below.
    #[builder(into, default)]
    #[serde(rename = "invocationHttpParameters")]
    pub r#invocation_http_parameters: Box<Option<super::super::types::cloudwatch::EventConnectionAuthParametersInvocationHttpParameters>>,
    /// Parameters used for OAUTH_CLIENT_CREDENTIALS authorization. A maximum of 1 are allowed. Conflicts with `basic` and `api_key`. Documented below.
    #[builder(into, default)]
    #[serde(rename = "oauth")]
    pub r#oauth: Box<Option<super::super::types::cloudwatch::EventConnectionAuthParametersOauth>>,
}
