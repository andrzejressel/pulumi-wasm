#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetPhysicalTableMapS3SourceUploadSetting {
    #[builder(into)]
    #[serde(rename = "containsHeader")]
    pub r#contains_header: Box<bool>,
    #[builder(into)]
    #[serde(rename = "delimiter")]
    pub r#delimiter: Box<String>,
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Box<String>,
    #[builder(into)]
    #[serde(rename = "startFromRow")]
    pub r#start_from_row: Box<i32>,
    #[builder(into)]
    #[serde(rename = "textQualifier")]
    pub r#text_qualifier: Box<String>,
}
