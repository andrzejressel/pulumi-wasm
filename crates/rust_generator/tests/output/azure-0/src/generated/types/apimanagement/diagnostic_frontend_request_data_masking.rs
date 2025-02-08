#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DiagnosticFrontendRequestDataMasking {
    /// A `headers` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::apimanagement::DiagnosticFrontendRequestDataMaskingHeader>>>,
    /// A `query_params` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "queryParams")]
    pub r#query_params: Box<Option<Vec<super::super::types::apimanagement::DiagnosticFrontendRequestDataMaskingQueryParam>>>,
}
