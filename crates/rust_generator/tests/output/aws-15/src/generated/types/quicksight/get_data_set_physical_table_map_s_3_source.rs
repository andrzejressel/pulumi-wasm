#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetPhysicalTableMapS3Source {
    #[builder(into)]
    #[serde(rename = "dataSourceArn")]
    pub r#data_source_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "inputColumns")]
    pub r#input_columns: Box<Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapS3SourceInputColumn>>,
    #[builder(into)]
    #[serde(rename = "uploadSettings")]
    pub r#upload_settings: Box<Vec<super::super::types::quicksight::GetDataSetPhysicalTableMapS3SourceUploadSetting>>,
}
