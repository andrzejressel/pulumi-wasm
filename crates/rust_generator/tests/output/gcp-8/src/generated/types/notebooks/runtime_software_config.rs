#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuntimeSoftwareConfig {
    /// Specify a custom Cloud Storage path where the GPU driver is stored.
    /// If not specified, we'll automatically choose from official GPU drivers.
    #[builder(into, default)]
    #[serde(rename = "customGpuDriverPath")]
    pub r#custom_gpu_driver_path: Box<Option<String>>,
    /// Verifies core internal services are running. Default: True.
    #[builder(into, default)]
    #[serde(rename = "enableHealthMonitoring")]
    pub r#enable_health_monitoring: Box<Option<bool>>,
    /// Runtime will automatically shutdown after idle_shutdown_time.
    /// Default: True
    #[builder(into, default)]
    #[serde(rename = "idleShutdown")]
    pub r#idle_shutdown: Box<Option<bool>>,
    /// Time in minutes to wait before shuting down runtime.
    /// Default: 180 minutes
    #[builder(into, default)]
    #[serde(rename = "idleShutdownTimeout")]
    pub r#idle_shutdown_timeout: Box<Option<i32>>,
    /// Install Nvidia Driver automatically.
    #[builder(into, default)]
    #[serde(rename = "installGpuDriver")]
    pub r#install_gpu_driver: Box<Option<bool>>,
    /// Use a list of container images to use as Kernels in the notebook instance.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "kernels")]
    pub r#kernels: Box<Option<Vec<super::super::types::notebooks::RuntimeSoftwareConfigKernel>>>,
    /// Cron expression in UTC timezone for schedule instance auto upgrade.
    /// Please follow the [cron format](https://en.wikipedia.org/wiki/Cron).
    #[builder(into, default)]
    #[serde(rename = "notebookUpgradeSchedule")]
    pub r#notebook_upgrade_schedule: Box<Option<String>>,
    /// Path to a Bash script that automatically runs after a notebook instance
    /// fully boots up. The path must be a URL or
    /// Cloud Storage path (gs://path-to-file/file-name).
    #[builder(into, default)]
    #[serde(rename = "postStartupScript")]
    pub r#post_startup_script: Box<Option<String>>,
    /// Behavior for the post startup script.
    /// Possible values are: `POST_STARTUP_SCRIPT_BEHAVIOR_UNSPECIFIED`, `RUN_EVERY_START`, `DOWNLOAD_AND_RUN_EVERY_START`.
    #[builder(into, default)]
    #[serde(rename = "postStartupScriptBehavior")]
    pub r#post_startup_script_behavior: Box<Option<String>>,
    /// (Output)
    /// Bool indicating whether an newer image is available in an image family.
    #[builder(into, default)]
    #[serde(rename = "upgradeable")]
    pub r#upgradeable: Box<Option<bool>>,
}
