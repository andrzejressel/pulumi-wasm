#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkbookTemplateGallery {
    /// Category for the gallery.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: Box<String>,
    /// Name of the workbook template in the gallery.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Order of the template within the gallery. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "order")]
    pub r#order: Box<Option<i32>>,
    /// Azure resource type supported by the gallery. Defaults to `Azure Monitor`.
    #[builder(into, default)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Box<Option<String>>,
    /// Type of workbook supported by the workbook template. Defaults to `workbook`.
    /// 
    /// > **Note:** See [documentation](https://docs.microsoft.com/en-us/azure/azure-monitor/visualize/workbooks-automate#galleries) for more information of `resource_type` and `type`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
