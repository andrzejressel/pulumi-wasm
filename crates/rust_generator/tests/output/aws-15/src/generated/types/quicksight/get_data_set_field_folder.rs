#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetFieldFolder {
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    #[builder(into)]
    #[serde(rename = "fieldFoldersId")]
    pub r#field_folders_id: Box<String>,
}
