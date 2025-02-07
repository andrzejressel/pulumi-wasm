#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTemplateSpecContainer {
    /// Arguments to the entrypoint.
    /// The docker image's CMD is used if this is not provided.
    #[builder(into, default)]
    #[serde(rename = "args")]
    pub r#args: Box<Option<Vec<String>>>,
    /// Entrypoint array. Not executed within a shell.
    /// The docker image's ENTRYPOINT is used if this is not provided.
    #[builder(into, default)]
    #[serde(rename = "commands")]
    pub r#commands: Box<Option<Vec<String>>>,
    /// (Optional, Deprecated)
    /// List of sources to populate environment variables in the container.
    /// All invalid keys will be reported as an event when the container is starting.
    /// When a key exists in multiple sources, the value associated with the last source will
    /// take precedence. Values defined by an Env with a duplicate key will take
    /// precedence.
    /// Structure is documented below.
    /// 
    /// > **Warning:** `env_from` is deprecated and will be removed in a future major release. This field is not supported by the Cloud Run API.
    #[builder(into, default)]
    #[serde(rename = "envFroms")]
    pub r#env_froms: Box<Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecContainerEnvFrom>>>,
    /// List of environment variables to set in the container.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "envs")]
    pub r#envs: Box<Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecContainerEnv>>>,
    /// Docker image name. This is most often a reference to a container located
    /// in the container registry, such as gcr.io/cloudrun/hello
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Box<String>,
    /// Periodic probe of container liveness. Container will be restarted if the probe fails. More info:
    /// https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "livenessProbe")]
    pub r#liveness_probe: Box<Option<super::super::types::cloudrun::ServiceTemplateSpecContainerLivenessProbe>>,
    /// Name of the container
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// List of open ports in the container.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecContainerPort>>>,
    /// Compute Resources required by this container. Used to set values such as max memory
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Option<super::super::types::cloudrun::ServiceTemplateSpecContainerResources>>,
    /// Startup probe of application within the container.
    /// All other probes are disabled if a startup probe is provided, until it
    /// succeeds. Container will not be added to service endpoints if the probe fails.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "startupProbe")]
    pub r#startup_probe: Box<Option<super::super::types::cloudrun::ServiceTemplateSpecContainerStartupProbe>>,
    /// Volume to mount into the container's filesystem.
    /// Only supports SecretVolumeSources.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "volumeMounts")]
    pub r#volume_mounts: Box<Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecContainerVolumeMount>>>,
    /// (Optional, Deprecated)
    /// Container's working directory.
    /// If not specified, the container runtime's default will be used, which
    /// might be configured in the container image.
    /// 
    /// > **Warning:** `working_dir` is deprecated and will be removed in a future major release. This field is not supported by the Cloud Run API.
    #[builder(into, default)]
    #[serde(rename = "workingDir")]
    pub r#working_dir: Box<Option<String>>,
}
