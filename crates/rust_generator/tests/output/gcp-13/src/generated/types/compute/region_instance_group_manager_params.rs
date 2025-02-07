#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionInstanceGroupManagerParams {
    /// Resource manager tags to bind to the managed instance group. The tags are key-value pairs. Keys must be in the format tagKeys/123 and values in the format tagValues/456. For more information, see [Manage tags for resources](https://cloud.google.com/compute/docs/tag-resources)
    #[builder(into, default)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: Box<Option<std::collections::HashMap<String, String>>>,
}
