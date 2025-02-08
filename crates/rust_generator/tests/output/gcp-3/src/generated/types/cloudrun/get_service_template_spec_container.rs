#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetServiceTemplateSpecContainer {
    /// Arguments to the entrypoint.
    /// The docker image's CMD is used if this is not provided.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Box<Vec<String>>,
    /// Entrypoint array. Not executed within a shell.
    /// The docker image's ENTRYPOINT is used if this is not provided.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Box<Vec<String>>,
    /// List of sources to populate environment variables in the container.
    /// All invalid keys will be reported as an event when the container is starting.
    /// When a key exists in multiple sources, the value associated with the last source will
    /// take precedence. Values defined by an Env with a duplicate key will take
    /// precedence.
    #[builder(into)]
    #[serde(rename = "envFroms")]
    pub r#env_froms: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerEnvFrom>>,
    /// List of environment variables to set in the container.
    #[builder(into)]
    #[serde(rename = "envs")]
    pub r#envs: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerEnv>>,
    /// Docker image name. This is most often a reference to a container located
    /// in the container registry, such as gcr.io/cloudrun/hello
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Box<String>,
    /// Periodic probe of container liveness. Container will be restarted if the probe fails.
    #[builder(into)]
    #[serde(rename = "livenessProbes")]
    pub r#liveness_probes: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerLivenessProbe>>,
    /// The name of the Cloud Run Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// List of open ports in the container.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerPort>>,
    /// Compute Resources required by this container. Used to set values such as max memory
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerResource>>,
    /// Startup probe of application within the container.
    /// All other probes are disabled if a startup probe is provided, until it
    /// succeeds. Container will not be added to service endpoints if the probe fails.
    #[builder(into)]
    #[serde(rename = "startupProbes")]
    pub r#startup_probes: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerStartupProbe>>,
    /// Volume to mount into the container's filesystem.
    /// Only supports SecretVolumeSources.
    #[builder(into)]
    #[serde(rename = "volumeMounts")]
    pub r#volume_mounts: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerVolumeMount>>,
    /// Container's working directory.
    /// If not specified, the container runtime's default will be used, which
    /// might be configured in the container image.
    #[builder(into)]
    #[serde(rename = "workingDir")]
    pub r#working_dir: Box<String>,
}
