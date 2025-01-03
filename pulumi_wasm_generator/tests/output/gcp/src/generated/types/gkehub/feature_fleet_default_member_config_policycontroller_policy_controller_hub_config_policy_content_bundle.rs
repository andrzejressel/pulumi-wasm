#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContentBundle {
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "bundle")]
    pub r#bundle: Box<String>,
    /// The set of namespaces to be exempted from the bundle.
    #[builder(into, default)]
    #[serde(rename = "exemptedNamespaces")]
    pub r#exempted_namespaces: Box<Option<Vec<String>>>,
}
