#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RestorePlanRestoreConfigExcludedNamespaces {
    /// A list of Kubernetes Namespaces.
    #[builder(into)]
    #[serde(rename = "namespaces")]
    pub r#namespaces: Box<Vec<String>>,
}
