#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DiagnosticFrontendResponseDataMasking {
    /// A `headers` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::apimanagement::DiagnosticFrontendResponseDataMaskingHeader>>>,
    /// A `query_params` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "queryParams")]
    pub r#query_params: Box<Option<Vec<super::super::types::apimanagement::DiagnosticFrontendResponseDataMaskingQueryParam>>>,
}
