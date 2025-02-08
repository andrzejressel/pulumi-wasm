#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureMembershipPolicycontrollerPolicyControllerHubConfigPolicyContentTemplateLibrary {
    /// Configures the manner in which the template library is installed on the cluster. Must be one of `ALL`, `NOT_INSTALLED` or `INSTALLATION_UNSPECIFIED`. Defaults to `ALL`.
    #[builder(into, default)]
    #[serde(rename = "installation")]
    pub r#installation: Box<Option<String>>,
}
