#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DevEnvironmentIdes {
    /// The name of the IDE. Valid values include Cloud9, IntelliJ, PyCharm, GoLand, and VSCode.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// A link to the IDE runtime image. This parameter is not required if the name is VSCode. Values of the runtime can be for example public.ecr.aws/jetbrains/py,public.ecr.aws/jetbrains/go
    #[builder(into, default)]
    #[serde(rename = "runtime")]
    pub r#runtime: Box<Option<String>>,
}