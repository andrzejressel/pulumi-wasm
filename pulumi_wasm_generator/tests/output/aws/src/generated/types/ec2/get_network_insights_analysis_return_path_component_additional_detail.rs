#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkInsightsAnalysisReturnPathComponentAdditionalDetail {
    #[builder(into)]
    #[serde(rename = "additionalDetailType")]
    pub r#additional_detail_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentAdditionalDetailComponent>>,
}
