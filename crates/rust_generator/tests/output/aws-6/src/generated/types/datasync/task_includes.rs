#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TaskIncludes {
    /// The type of filter rule to apply. Valid values: `SIMPLE_PATTERN`.
    #[builder(into, default)]
    #[serde(rename = "filterType")]
    pub r#filter_type: Box<Option<String>>,
    /// A single filter string that consists of the patterns to include. The patterns are delimited by "|" (that is, a pipe), for example: `/folder1|/folder2`
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
