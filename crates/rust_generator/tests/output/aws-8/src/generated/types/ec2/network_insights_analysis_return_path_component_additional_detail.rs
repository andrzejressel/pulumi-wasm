#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct NetworkInsightsAnalysisReturnPathComponentAdditionalDetail {
    #[builder(into, default)]
    #[serde(rename = "additionalDetailType")]
    pub r#additional_detail_type: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "components")]
    pub r#components: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentAdditionalDetailComponent>>>,
}
