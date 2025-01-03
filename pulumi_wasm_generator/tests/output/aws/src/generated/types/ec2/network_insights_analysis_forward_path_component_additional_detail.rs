#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkInsightsAnalysisForwardPathComponentAdditionalDetail {
    #[builder(into, default)]
    #[serde(rename = "additionalDetailType")]
    pub r#additional_detail_type: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "components")]
    pub r#components: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisForwardPathComponentAdditionalDetailComponent>>>,
}
