#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceTemplateSpecContainerEnvValueFromSecretKeyRef {
    /// A Cloud Secret Manager secret version. Must be 'latest' for the latest
    /// version or an integer for a specific version.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The name of the secret in Cloud Secret Manager. By default, the secret is assumed to be in the same project.
    /// If the secret is in another project, you must define an alias.
    /// An alias definition has the form: :projects/{project-id|project-number}/secrets/.
    /// If multiple alias definitions are needed, they must be separated by commas.
    /// The alias definitions must be set on the run.googleapis.com/secrets annotation.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
