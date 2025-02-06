#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetDataSetUsageConfiguration {
    #[builder(into)]
    #[serde(rename = "disableUseAsDirectQuerySource")]
    pub r#disable_use_as_direct_query_source: Box<bool>,
    #[builder(into)]
    #[serde(rename = "disableUseAsImportedSource")]
    pub r#disable_use_as_imported_source: Box<bool>,
}
