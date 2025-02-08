#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FeatureMembershipPolicycontrollerPolicyControllerHubConfigPolicyContentBundle {
    /// The name of the bundle.
    #[builder(into)]
    #[serde(rename = "bundleName")]
    pub r#bundle_name: Box<String>,
    /// The set of namespaces to be exempted from the bundle.
    #[builder(into, default)]
    #[serde(rename = "exemptedNamespaces")]
    pub r#exempted_namespaces: Box<Option<Vec<String>>>,
}
