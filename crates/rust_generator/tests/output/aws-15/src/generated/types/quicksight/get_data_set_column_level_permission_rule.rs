#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetColumnLevelPermissionRule {
    #[builder(into)]
    #[serde(rename = "columnNames")]
    pub r#column_names: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "principals")]
    pub r#principals: Box<Vec<String>>,
}
