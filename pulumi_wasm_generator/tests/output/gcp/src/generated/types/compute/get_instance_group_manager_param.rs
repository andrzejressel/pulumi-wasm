#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceGroupManagerParam {
    /// Resource manager tags to bind to the managed instance group. The tags are key-value pairs. Keys must be in the format tagKeys/123 and values in the format tagValues/456.
    #[builder(into)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: Box<std::collections::HashMap<String, String>>,
}
