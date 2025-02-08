#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetTriggerBuildStep {
    /// Allow this build step to fail without failing the entire build if and
    /// only if the exit code is one of the specified codes.
    /// 
    /// If 'allowFailure' is also specified, this field will take precedence.
    #[builder(into)]
    #[serde(rename = "allowExitCodes")]
    pub r#allow_exit_codes: Box<Vec<i32>>,
    /// Allow this build step to fail without failing the entire build.
    /// If false, the entire build will fail if this step fails. Otherwise, the
    /// build will succeed, but this step will still have a failure status.
    /// Error information will be reported in the 'failureDetail' field.
    /// 
    /// 'allowExitCodes' takes precedence over this field.
    #[builder(into)]
    #[serde(rename = "allowFailure")]
    pub r#allow_failure: Box<bool>,
    /// A list of arguments that will be presented to the step when it is started.
    /// 
    /// If the image used to run the step's container has an entrypoint, the args
    /// are used as arguments to that entrypoint. If the image does not define an
    /// entrypoint, the first element in args is used as the entrypoint, and the
    /// remainder will be used as arguments.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Box<Vec<String>>,
    /// Working directory to use when running this step's container.
    /// 
    /// If this value is a relative path, it is relative to the build's working
    /// directory. If this value is absolute, it may be outside the build's working
    /// directory, in which case the contents of the path may not be persisted
    /// across build step executions, unless a 'volume' for that path is specified.
    /// 
    /// If the build specifies a 'RepoSource' with 'dir' and a step with a
    /// 'dir',
    /// which specifies an absolute path, the 'RepoSource' 'dir' is ignored
    /// for the step's execution.
    #[builder(into)]
    #[serde(rename = "dir")]
    pub r#dir: Box<String>,
    /// Entrypoint to be used instead of the build step image's
    /// default entrypoint.
    /// If unset, the image's default entrypoint is used
    #[builder(into)]
    #[serde(rename = "entrypoint")]
    pub r#entrypoint: Box<String>,
    /// A list of environment variable definitions to be used when
    /// running a step.
    /// 
    /// The elements are of the form "KEY=VALUE" for the environment variable
    /// "KEY" being given the value "VALUE".
    #[builder(into)]
    #[serde(rename = "envs")]
    pub r#envs: Box<Vec<String>>,
    /// Unique identifier for this build step, used in 'wait_for' to
    /// reference this build step as a dependency.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of the container image that will run this particular build step.
    /// 
    /// If the image is available in the host's Docker daemon's cache, it will be
    /// run directly. If not, the host will attempt to pull the image first, using
    /// the builder service account's credentials if necessary.
    /// 
    /// The Docker daemon's cache will already have the latest versions of all of
    /// the officially supported build steps (see https://github.com/GoogleCloudPlatform/cloud-builders
    /// for images and examples).
    /// The Docker daemon will also have cached many of the layers for some popular
    /// images, like "ubuntu", "debian", but they will be refreshed at the time
    /// you attempt to use them.
    /// 
    /// If you built an image in a previous build step, it will be stored in the
    /// host's Docker daemon's cache and is available to use as the name for a
    /// later build step.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A shell script to be executed in the step.
    /// When script is provided, the user cannot specify the entrypoint or args.
    #[builder(into)]
    #[serde(rename = "script")]
    pub r#script: Box<String>,
    /// A list of environment variables which are encrypted using
    /// a Cloud Key
    /// Management Service crypto key. These values must be specified in
    /// the build's 'Secret'.
    #[builder(into)]
    #[serde(rename = "secretEnvs")]
    pub r#secret_envs: Box<Vec<String>>,
    /// Time limit for executing this build step. If not defined,
    /// the step has no
    /// time limit and will be allowed to continue to run until either it
    /// completes or the build itself times out.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<String>,
    /// Output only. Stores timing information for executing this
    /// build step.
    #[builder(into)]
    #[serde(rename = "timing")]
    pub r#timing: Box<String>,
    /// List of volumes to mount into the build step.
    /// 
    /// Each volume is created as an empty volume prior to execution of the
    /// build step. Upon completion of the build, volumes and their contents
    /// are discarded.
    /// 
    /// Using a named volume in only one step is not valid as it is
    /// indicative of a build request with an incorrect configuration.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Box<Vec<super::super::types::cloudbuild::GetTriggerBuildStepVolume>>,
    /// The ID(s) of the step(s) that this build step depends on.
    /// 
    /// This build step will not start until all the build steps in 'wait_for'
    /// have completed successfully. If 'wait_for' is empty, this build step
    /// will start when all previous build steps in the 'Build.Steps' list
    /// have completed successfully.
    #[builder(into)]
    #[serde(rename = "waitFors")]
    pub r#wait_fors: Box<Vec<String>>,
}
