#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureFleetDefaultMemberConfigPolicycontrollerPolicyControllerHubConfigPolicyContentTemplateLibrary {
    /// Configures the manner in which the template library is installed on the cluster.
    /// Possible values are: `INSTALLATION_UNSPECIFIED`, `NOT_INSTALLED`, `ALL`.
    #[builder(into, default)]
    #[serde(rename = "installation")]
    pub r#installation: Box<Option<String>>,
}
