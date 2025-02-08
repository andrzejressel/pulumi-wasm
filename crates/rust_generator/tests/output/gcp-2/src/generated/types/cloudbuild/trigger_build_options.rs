#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TriggerBuildOptions {
    /// Requested disk size for the VM that runs the build. Note that this is NOT "disk free";
    /// some of the space will be used by the operating system and build utilities.
    /// Also note that this is the minimum disk size that will be allocated for the build --
    /// the build may run with a larger disk than requested. At present, the maximum disk size
    /// is 1000GB; builds that request more than the maximum are rejected with an error.
    #[builder(into, default)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<Option<i32>>,
    /// Option to specify whether or not to apply bash style string operations to the substitutions.
    /// NOTE this is always enabled for triggered builds and cannot be overridden in the build configuration file.
    #[builder(into, default)]
    #[serde(rename = "dynamicSubstitutions")]
    pub r#dynamic_substitutions: Box<Option<bool>>,
    /// A list of global environment variable definitions that will exist for all build steps
    /// in this build. If a variable is defined in both globally and in a build step,
    /// the variable will use the build step value.
    /// The elements are of the form "KEY=VALUE" for the environment variable "KEY" being given the value "VALUE".
    #[builder(into, default)]
    #[serde(rename = "envs")]
    pub r#envs: Box<Option<Vec<String>>>,
    /// Option to define build log streaming behavior to Google Cloud Storage.
    /// Possible values are: `STREAM_DEFAULT`, `STREAM_ON`, `STREAM_OFF`.
    #[builder(into, default)]
    #[serde(rename = "logStreamingOption")]
    pub r#log_streaming_option: Box<Option<String>>,
    /// Option to specify the logging mode, which determines if and where build logs are stored.
    /// Possible values are: `LOGGING_UNSPECIFIED`, `LEGACY`, `GCS_ONLY`, `STACKDRIVER_ONLY`, `CLOUD_LOGGING_ONLY`, `NONE`.
    #[builder(into, default)]
    #[serde(rename = "logging")]
    pub r#logging: Box<Option<String>>,
    /// Compute Engine machine type on which to run the build.
    #[builder(into, default)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<Option<String>>,
    /// Requested verifiability options.
    /// Possible values are: `NOT_VERIFIED`, `VERIFIED`.
    #[builder(into, default)]
    #[serde(rename = "requestedVerifyOption")]
    pub r#requested_verify_option: Box<Option<String>>,
    /// A list of global environment variables, which are encrypted using a Cloud Key Management
    /// Service crypto key. These values must be specified in the build's Secret. These variables
    /// will be available to all build steps in this build.
    #[builder(into, default)]
    #[serde(rename = "secretEnvs")]
    pub r#secret_envs: Box<Option<Vec<String>>>,
    /// Requested hash for SourceProvenance.
    /// Each value may be one of: `NONE`, `SHA256`, `MD5`.
    #[builder(into, default)]
    #[serde(rename = "sourceProvenanceHashes")]
    pub r#source_provenance_hashes: Box<Option<Vec<String>>>,
    /// Option to specify behavior when there is an error in the substitution checks.
    /// NOTE this is always set to ALLOW_LOOSE for triggered builds and cannot be overridden
    /// in the build configuration file.
    /// Possible values are: `MUST_MATCH`, `ALLOW_LOOSE`.
    #[builder(into, default)]
    #[serde(rename = "substitutionOption")]
    pub r#substitution_option: Box<Option<String>>,
    /// Global list of volumes to mount for ALL build steps
    /// Each volume is created as an empty volume prior to starting the build process.
    /// Upon completion of the build, volumes and their contents are discarded. Global
    /// volume names and paths cannot conflict with the volumes defined a build step.
    /// Using a global volume in a build with only one step is not valid as it is indicative
    /// of a build request with an incorrect configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "volumes")]
    pub r#volumes: Box<Option<Vec<super::super::types::cloudbuild::TriggerBuildOptionsVolume>>>,
    /// Option to specify a WorkerPool for the build. Format projects/{project}/workerPools/{workerPool}
    /// This field is experimental.
    #[builder(into, default)]
    #[serde(rename = "workerPool")]
    pub r#worker_pool: Box<Option<String>>,
}
