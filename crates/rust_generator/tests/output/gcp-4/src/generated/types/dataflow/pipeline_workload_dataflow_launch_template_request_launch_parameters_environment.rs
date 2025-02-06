#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelineWorkloadDataflowLaunchTemplateRequestLaunchParametersEnvironment {
    /// Additional experiment flags for the job.
    #[builder(into, default)]
    #[serde(rename = "additionalExperiments")]
    pub r#additional_experiments: Box<Option<Vec<String>>>,
    /// Additional user labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. An object containing a list of key/value pairs.
    /// 'Example: { "name": "wrench", "mass": "1kg", "count": "3" }.'
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into, default)]
    #[serde(rename = "additionalUserLabels")]
    pub r#additional_user_labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Whether to bypass the safety checks for the job's temporary directory. Use with caution.
    #[builder(into, default)]
    #[serde(rename = "bypassTempDirValidation")]
    pub r#bypass_temp_dir_validation: Box<Option<bool>>,
    /// Whether to enable Streaming Engine for the job.
    #[builder(into, default)]
    #[serde(rename = "enableStreamingEngine")]
    pub r#enable_streaming_engine: Box<Option<bool>>,
    /// Configuration for VM IPs.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#WorkerIPAddressConfiguration
    /// Possible values are: `WORKER_IP_UNSPECIFIED`, `WORKER_IP_PUBLIC`, `WORKER_IP_PRIVATE`.
    #[builder(into, default)]
    #[serde(rename = "ipConfiguration")]
    pub r#ip_configuration: Box<Option<String>>,
    /// 'Name for the Cloud KMS key for the job. The key format is: projects//locations//keyRings//cryptoKeys/'
    #[builder(into, default)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Box<Option<String>>,
    /// The machine type to use for the job. Defaults to the value from the template if not specified.
    #[builder(into, default)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<Option<String>>,
    /// The maximum number of Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000.
    #[builder(into, default)]
    #[serde(rename = "maxWorkers")]
    pub r#max_workers: Box<Option<i32>>,
    /// Network to which VMs will be assigned. If empty or unspecified, the service will use the network "default".
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// The initial number of Compute Engine instances for the job.
    #[builder(into, default)]
    #[serde(rename = "numWorkers")]
    pub r#num_workers: Box<Option<i32>>,
    /// The email address of the service account to run the job as.
    #[builder(into, default)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<Option<String>>,
    /// Subnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form "https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK" or "regions/REGION/subnetworks/SUBNETWORK". If the subnetwork is located in a Shared VPC network, you must use the complete URL.
    #[builder(into, default)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Box<Option<String>>,
    /// The Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with gs://.
    #[builder(into, default)]
    #[serde(rename = "tempLocation")]
    pub r#temp_location: Box<Option<String>>,
    /// The Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. "us-west1". Mutually exclusive with workerZone. If neither workerRegion nor workerZone is specified, default to the control plane's region.
    #[builder(into, default)]
    #[serde(rename = "workerRegion")]
    pub r#worker_region: Box<Option<String>>,
    /// The Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. "us-west1-a". Mutually exclusive with workerRegion. If neither workerRegion nor workerZone is specified, a zone in the control plane's region is chosen based on available capacity. If both workerZone and zone are set, workerZone takes precedence.
    #[builder(into, default)]
    #[serde(rename = "workerZone")]
    pub r#worker_zone: Box<Option<String>>,
    /// The Compute Engine availability zone for launching worker instances to run your pipeline. In the future, workerZone will take precedence.
    #[builder(into, default)]
    #[serde(rename = "zone")]
    pub r#zone: Box<Option<String>>,
}
