#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DockerBuild {
    /// Custom host-to-IP mappings to use while building (format: "host:ip")
    #[builder(into, default)]
    #[serde(rename = "addHosts")]
    pub r#add_hosts: Box<Option<Vec<String>>>,
    /// An optional map of named build-time argument variables to set during the Docker build. This flag allows you to pass build-time variables that can be accessed like environment variables inside the RUN instruction.
    #[builder(into, default)]
    #[serde(rename = "args")]
    pub r#args: Box<Option<std::collections::HashMap<String, String>>>,
    /// The version of the Docker builder.
    #[builder(into, default)]
    #[serde(rename = "builderVersion")]
    pub r#builder_version: Box<Option<super::types::BuilderVersion>>,
    /// A list of image names to use as build cache. Images provided must have a cache manifest. Must provide authentication to cache registry.
    #[builder(into, default)]
    #[serde(rename = "cacheFrom")]
    pub r#cache_from: Box<Option<super::types::CacheFrom>>,
    /// The path to the build context to use.
    #[builder(into, default)]
    #[serde(rename = "context")]
    pub r#context: Box<Option<String>>,
    /// The path to the Dockerfile to use.
    #[builder(into, default)]
    #[serde(rename = "dockerfile")]
    pub r#dockerfile: Box<Option<String>>,
    /// Set the networking mode for RUN instructions
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// The architecture of the platform you want to build this image for, e.g. `linux/arm64`.
    #[builder(into, default)]
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
    /// The target of the Dockerfile to build
    #[builder(into, default)]
    #[serde(rename = "target")]
    pub r#target: Box<Option<String>>,
}
