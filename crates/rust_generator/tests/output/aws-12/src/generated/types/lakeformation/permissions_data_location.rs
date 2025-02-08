#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PermissionsDataLocation {
    /// Amazon Resource Name (ARN) that uniquely identifies the data location resource.
    /// 
    /// The following argument is optional:
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// Identifier for the Data Catalog where the location is registered with Lake Formation. By default, it is the account ID of the caller.
    #[builder(into, default)]
    #[serde(rename = "catalogId")]
    pub r#catalog_id: Box<Option<String>>,
}
