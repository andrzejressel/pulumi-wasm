#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetJobDefinitionEksPropertyPodPropertyVolumeSecret {
    /// Specifies whether the secret or the secret's keys must be defined.
    #[builder(into)]
    #[serde(rename = "optional")]
    pub r#optional: Box<bool>,
    /// The name of the secret. The name must be allowed as a DNS subdomain name
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<String>,
}
