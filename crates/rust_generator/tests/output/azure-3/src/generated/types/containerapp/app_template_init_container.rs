#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppTemplateInitContainer {
    /// A list of extra arguments to pass to the container.
    #[builder(into, default)]
    #[serde(rename = "args")]
    pub r#args: Box<Option<Vec<String>>>,
    /// A command to pass to the container to override the default. This is provided as a list of command line elements without spaces.
    #[builder(into, default)]
    #[serde(rename = "commands")]
    pub r#commands: Box<Option<Vec<String>>>,
    /// The amount of vCPU to allocate to the container. Possible values include `0.25`, `0.5`, `0.75`, `1.0`, `1.25`, `1.5`, `1.75`, and `2.0`. When there's a workload profile specified, there's no such constraint.
    /// 
    /// > **NOTE:** `cpu` and `memory` must be specified in `0.25'/'0.5Gi` combination increments. e.g. `1.0` / `2.0` or `0.5` / `1.0`
    #[builder(into, default)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<Option<f64>>,
    /// One or more `env` blocks as detailed below.
    #[builder(into, default)]
    #[serde(rename = "envs")]
    pub r#envs: Box<Option<Vec<super::super::types::containerapp::AppTemplateInitContainerEnv>>>,
    /// The amount of ephemeral storage available to the Container App.
    /// 
    /// > **NOTE:** `ephemeral_storage` is currently in preview and not configurable at this time.
    #[builder(into, default)]
    #[serde(rename = "ephemeralStorage")]
    pub r#ephemeral_storage: Box<Option<String>>,
    /// The image to use to create the container.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Box<String>,
    /// The amount of memory to allocate to the container. Possible values are `0.5Gi`, `1Gi`, `1.5Gi`, `2Gi`, `2.5Gi`, `3Gi`, `3.5Gi` and `4Gi`. When there's a workload profile specified, there's no such constraint.
    /// 
    /// > **NOTE:** `cpu` and `memory` must be specified in `0.25'/'0.5Gi` combination increments. e.g. `1.25` / `2.5Gi` or `0.75` / `1.5Gi`
    #[builder(into, default)]
    #[serde(rename = "memory")]
    pub r#memory: Box<Option<String>>,
    /// The name of the container
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A `volume_mounts` block as detailed below.
    #[builder(into, default)]
    #[serde(rename = "volumeMounts")]
    pub r#volume_mounts: Box<Option<Vec<super::super::types::containerapp::AppTemplateInitContainerVolumeMount>>>,
}
