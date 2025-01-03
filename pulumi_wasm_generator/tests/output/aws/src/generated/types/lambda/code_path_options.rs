#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CodePathOptions {
    /// Packages to explicitly exclude from the Assets for a serialized closure. This can be used when clients want to trim down the size of a closure, and they know that some package won't ever actually be needed at runtime, but is still a dependency of some package that is being used at runtime.
    #[builder(into, default)]
    #[serde(rename = "extraExcludePackages")]
    pub r#extra_exclude_packages: Box<Option<Vec<String>>>,
    /// Extra packages to include when producing the Assets for a serialized closure. This can be useful if the packages are acquired in a way that the serialization code does not understand. For example, if there was some sort of module that was pulled in based off of a computed string.
    #[builder(into, default)]
    #[serde(rename = "extraIncludePackages")]
    pub r#extra_include_packages: Box<Option<Vec<String>>>,
    /// Local file/directory paths that should be included when producing the Assets for a serialized closure.
    #[builder(into, default)]
    #[serde(rename = "extraIncludePaths")]
    pub r#extra_include_paths: Box<Option<Vec<String>>>,
}
