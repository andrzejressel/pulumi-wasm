#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAppTemplateInitContainer {
    /// A list of extra arguments to pass to the container.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Box<Vec<String>>,
    /// A command to pass to the container to override the default. This is provided as a list of command line elements without spaces.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Box<Vec<String>>,
    /// The amount of vCPU to allocate to the container. Possible values include `0.25`, `0.5`, `0.75`, `1.0`, `1.25`, `1.5`, `1.75`, and `2.0`.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<f64>,
    /// One or more `env` blocks as detailed below.
    #[builder(into)]
    #[serde(rename = "envs")]
    pub r#envs: Box<Vec<super::super::types::containerapp::GetAppTemplateInitContainerEnv>>,
    /// The amount of ephemeral storage available to the Container App.
    #[builder(into)]
    #[serde(rename = "ephemeralStorage")]
    pub r#ephemeral_storage: Box<String>,
    /// The image to use to create the container.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Box<String>,
    /// The amount of memory to allocate to the container. Possible values include `0.5Gi`, `1Gi`, `1.5Gi`, `2Gi`, `2.5Gi`, `3Gi`, `3.5Gi`, and `4Gi`.
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: Box<String>,
    /// The name of the Container App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A `volume_mounts` block as detailed below.
    #[builder(into)]
    #[serde(rename = "volumeMounts")]
    pub r#volume_mounts: Box<Vec<super::super::types::containerapp::GetAppTemplateInitContainerVolumeMount>>,
}
