#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiDiagnosticFrontendResponseDataMasking {
    /// A `headers` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::apimanagement::ApiDiagnosticFrontendResponseDataMaskingHeader>>>,
    /// A `query_params` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "queryParams")]
    pub r#query_params: Box<Option<Vec<super::super::types::apimanagement::ApiDiagnosticFrontendResponseDataMaskingQueryParam>>>,
}
