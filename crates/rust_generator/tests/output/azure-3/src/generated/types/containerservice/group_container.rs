#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupContainer {
    /// A list of commands which should be run on the container. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "commands")]
    pub r#commands: Box<Option<Vec<String>>>,
    /// The required number of CPU cores of the containers. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<f64>,
    /// The upper limit of the number of CPU cores of the containers.
    #[builder(into, default)]
    #[serde(rename = "cpuLimit")]
    pub r#cpu_limit: Box<Option<f64>>,
    /// A list of environment variables to be set on the container. Specified as a map of name/value pairs. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    /// The container image name. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Box<String>,
    /// The definition of a readiness probe for this container as documented in the `liveness_probe` block below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "livenessProbe")]
    pub r#liveness_probe: Box<Option<super::super::types::containerservice::GroupContainerLivenessProbe>>,
    /// The required memory of the containers in GB. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: Box<f64>,
    /// The upper limit of the memory of the containers in GB.
    #[builder(into, default)]
    #[serde(rename = "memoryLimit")]
    pub r#memory_limit: Box<Option<f64>>,
    /// Specifies the name of the Container. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A set of public ports for the container. Changing this forces a new resource to be created. Set as documented in the `ports` block below.
    #[builder(into, default)]
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<super::super::types::containerservice::GroupContainerPort>>>,
    /// The definition of a readiness probe for this container as documented in the `readiness_probe` block below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "readinessProbe")]
    pub r#readiness_probe: Box<Option<super::super::types::containerservice::GroupContainerReadinessProbe>>,
    /// A list of sensitive environment variables to be set on the container. Specified as a map of name/value pairs. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "secureEnvironmentVariables")]
    pub r#secure_environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    /// The definition of the security context for this container as documented in the `security` block below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "securities")]
    pub r#securities: Box<Option<Vec<super::super::types::containerservice::GroupContainerSecurity>>>,
    /// The definition of a volume mount for this container as documented in the `volume` block below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "volumes")]
    pub r#volumes: Box<Option<Vec<super::super::types::containerservice::GroupContainerVolume>>>,
}
