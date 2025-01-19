/// Creates a job on Dataflow, which is an implementation of Apache Beam running on Google Compute Engine. For more information see
/// the official documentation for
/// [Beam](https://beam.apache.org) and [Dataflow](https://cloud.google.com/dataflow/).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   bigDataJob:
///     type: gcp:dataflow:Job
///     name: big_data_job
///     properties:
///       name: dataflow-job
///       templateGcsPath: gs://my-bucket/templates/template_file
///       tempGcsLocation: gs://my-bucket/tmp_dir
///       parameters:
///         foo: bar
///         baz: qux
/// ```
///
/// ### Streaming Job
///
/// ```yaml
/// resources:
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: dataflow-job1
///   bucket1:
///     type: gcp:storage:Bucket
///     properties:
///       name: tf-test-bucket1
///       location: US
///       forceDestroy: true
///   bucket2:
///     type: gcp:storage:Bucket
///     properties:
///       name: tf-test-bucket2
///       location: US
///       forceDestroy: true
///   pubsubStream:
///     type: gcp:dataflow:Job
///     name: pubsub_stream
///     properties:
///       name: tf-test-dataflow-job1
///       templateGcsPath: gs://my-bucket/templates/template_file
///       tempGcsLocation: gs://my-bucket/tmp_dir
///       enableStreamingEngine: true
///       parameters:
///         inputFilePattern: ${bucket1.url}/*.json
///         outputTopic: ${topic.id}
///       transformNameMapping:
///         name: test_job
///         env: test
///       onDelete: cancel
/// ```
///
/// ## Note on "destroy" / "apply"
///
/// There are many types of Dataflow jobs.  Some Dataflow jobs run constantly, getting new data from (e.g.) a GCS bucket, and outputting data continuously.  Some jobs process a set amount of data then terminate.  All jobs can fail while running due to programming errors or other issues.  In this way, Dataflow jobs are different from most other Google resources.
///
/// The Dataflow resource is considered 'existing' while it is in a nonterminal state.  If it reaches a terminal state (e.g. 'FAILED', 'COMPLETE', 'CANCELLED'), it will be recreated on the next 'apply'.  This is as expected for jobs which run continuously, but may surprise users who use this resource for other kinds of Dataflow jobs.
///
/// A Dataflow job which is 'destroyed' may be "cancelled" or "drained".  If "cancelled", the job terminates - any data written remains where it is, but no new data will be processed.  If "drained", no new data will enter the pipeline, but any data currently in the pipeline will finish being processed.  The default is "drain". When `on_delete` is set to `"drain"` in the configuration, you may experience a long wait for your `pulumi destroy` to complete.
///
/// You can potentially short-circuit the wait by setting `skip_wait_on_job_termination` to `true`, but beware that unless you take active steps to ensure that the job `name` parameter changes between instances, the name will conflict and the launch of the new job will fail. One way to do this is with a random_id resource, for example:
///
/// ```yaml
/// configuration:
///   bigDataJobSubscriptionId:
///     type: string
///     default: projects/myproject/subscriptions/messages
/// resources:
///   bigDataJobNameSuffix:
///     type: random:RandomId
///     name: big_data_job_name_suffix
///     properties:
///       byteLength: 4
///       keepers:
///         region: ${region}
///         subscription_id: ${bigDataJobSubscriptionId}
///   bigDataJob:
///     type: gcp:dataflow:FlexTemplateJob
///     name: big_data_job
///     properties:
///       name: dataflow-flextemplates-job-${bigDataJobNameSuffix.dec}
///       region: ${region}
///       containerSpecGcsPath: gs://my-bucket/templates/template.json
///       skipWaitOnJobTermination: true
///       parameters:
///         inputSubscription: ${bigDataJobSubscriptionId}
/// ```
///
/// ## Import
///
/// Dataflow jobs can be imported using the job `id` e.g.
///
/// * `{{id}}`
///
/// When using the `pulumi import` command, dataflow jobs can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataflow/job:Job default {{id}}
/// ```
///
pub mod job {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobArgs {
        /// List of experiments that should be used by the job. An example value is `["enable_stackdriver_agent_metrics"]`.
        #[builder(into, default)]
        pub additional_experiments: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Enable/disable the use of [Streaming Engine](https://cloud.google.com/dataflow/docs/guides/deploying-a-pipeline#streaming-engine) for the job. Note that Streaming Engine is enabled by default for pipelines developed against the Beam SDK for Python v2.21.0 or later when using Python 3.
        #[builder(into, default)]
        pub enable_streaming_engine: pulumi_wasm_rust::Output<Option<bool>>,
        /// The configuration for VM IPs.  Options are `"WORKER_IP_PUBLIC"` or `"WORKER_IP_PRIVATE"`.
        #[builder(into, default)]
        pub ip_configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// The name for the Cloud KMS key for the job. Key format is: `projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY`
        #[builder(into, default)]
        pub kms_key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// User labels to be specified for the job. Keys and values should follow the restrictions
        /// specified in the [labeling restrictions](https://cloud.google.com/compute/docs/labeling-resources#restrictions) page.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The machine type to use for the job.
        #[builder(into, default)]
        pub machine_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of workers permitted to work on the job.  More workers may improve processing speed at additional cost.
        #[builder(into, default)]
        pub max_workers: pulumi_wasm_rust::Output<Option<i32>>,
        /// A unique name for the resource, required by Dataflow.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The network to which VMs will be assigned. If it is not provided, "default" will be used.
        #[builder(into, default)]
        pub network: pulumi_wasm_rust::Output<Option<String>>,
        /// One of "drain" or "cancel".  Specifies behavior of deletion during `pulumi destroy`.  See above note.
        #[builder(into, default)]
        pub on_delete: pulumi_wasm_rust::Output<Option<String>>,
        /// **Template specific** Key/Value pairs to be forwarded to the pipeline's options; keys are
        /// case-sensitive based on the language on which the pipeline is coded, mostly Java.
        /// **Note**: do not configure Dataflow options here in parameters.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The project in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region in which the created job should run.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The Service Account email used to create the job. This should be just an email e.g. `myserviceaccount@myproject.iam.gserviceaccount.com`. Do not include any `serviceAccount:` or other prefix.
        #[builder(into, default)]
        pub service_account_email: pulumi_wasm_rust::Output<Option<String>>,
        /// If set to `true`, Pulumi will treat `DRAINING` and `CANCELLING` as terminal states when deleting the resource, and will remove the resource from Pulumi state and move on.  See above note.
        #[builder(into, default)]
        pub skip_wait_on_job_termination: pulumi_wasm_rust::Output<Option<bool>>,
        /// The subnetwork to which VMs will be assigned. Should be of the form "regions/REGION/subnetworks/SUBNETWORK". If the [subnetwork is located in a Shared VPC network](https://cloud.google.com/dataflow/docs/guides/specifying-networks#shared), you must use the complete URL. For example `"googleapis.com/compute/v1/projects/PROJECT_ID/regions/REGION/subnetworks/SUBNET_NAME"`
        #[builder(into, default)]
        pub subnetwork: pulumi_wasm_rust::Output<Option<String>>,
        /// A writeable location on GCS for the Dataflow job to dump its temporary data.
        ///
        /// - - -
        #[builder(into)]
        pub temp_gcs_location: pulumi_wasm_rust::Output<String>,
        /// The GCS path to the Dataflow job template.
        #[builder(into)]
        pub template_gcs_path: pulumi_wasm_rust::Output<String>,
        /// Only applicable when updating a pipeline. Map of transform name prefixes of the job to be replaced with the corresponding name prefixes of the new job. This field is not used outside of update.
        #[builder(into, default)]
        pub transform_name_mapping: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The zone in which the created job should run. If it is not provided, the provider zone is used.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct JobResult {
        /// List of experiments that should be used by the job. An example value is `["enable_stackdriver_agent_metrics"]`.
        pub additional_experiments: pulumi_wasm_rust::Output<Vec<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Enable/disable the use of [Streaming Engine](https://cloud.google.com/dataflow/docs/guides/deploying-a-pipeline#streaming-engine) for the job. Note that Streaming Engine is enabled by default for pipelines developed against the Beam SDK for Python v2.21.0 or later when using Python 3.
        pub enable_streaming_engine: pulumi_wasm_rust::Output<Option<bool>>,
        /// The configuration for VM IPs.  Options are `"WORKER_IP_PUBLIC"` or `"WORKER_IP_PRIVATE"`.
        pub ip_configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique ID of this job.
        pub job_id: pulumi_wasm_rust::Output<String>,
        /// The name for the Cloud KMS key for the job. Key format is: `projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY`
        pub kms_key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// User labels to be specified for the job. Keys and values should follow the restrictions
        /// specified in the [labeling restrictions](https://cloud.google.com/compute/docs/labeling-resources#restrictions) page.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration. Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The machine type to use for the job.
        pub machine_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of workers permitted to work on the job.  More workers may improve processing speed at additional cost.
        pub max_workers: pulumi_wasm_rust::Output<Option<i32>>,
        /// A unique name for the resource, required by Dataflow.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The network to which VMs will be assigned. If it is not provided, "default" will be used.
        pub network: pulumi_wasm_rust::Output<Option<String>>,
        /// One of "drain" or "cancel".  Specifies behavior of deletion during `pulumi destroy`.  See above note.
        pub on_delete: pulumi_wasm_rust::Output<Option<String>>,
        /// **Template specific** Key/Value pairs to be forwarded to the pipeline's options; keys are
        /// case-sensitive based on the language on which the pipeline is coded, mostly Java.
        /// **Note**: do not configure Dataflow options here in parameters.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The project in which the resource belongs. If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region in which the created job should run.
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The Service Account email used to create the job. This should be just an email e.g. `myserviceaccount@myproject.iam.gserviceaccount.com`. Do not include any `serviceAccount:` or other prefix.
        pub service_account_email: pulumi_wasm_rust::Output<Option<String>>,
        /// If set to `true`, Pulumi will treat `DRAINING` and `CANCELLING` as terminal states when deleting the resource, and will remove the resource from Pulumi state and move on.  See above note.
        pub skip_wait_on_job_termination: pulumi_wasm_rust::Output<Option<bool>>,
        /// The current state of the resource, selected from the [JobState enum](https://cloud.google.com/dataflow/docs/reference/rest/v1b3/projects.jobs#Job.JobState)
        pub state: pulumi_wasm_rust::Output<String>,
        /// The subnetwork to which VMs will be assigned. Should be of the form "regions/REGION/subnetworks/SUBNETWORK". If the [subnetwork is located in a Shared VPC network](https://cloud.google.com/dataflow/docs/guides/specifying-networks#shared), you must use the complete URL. For example `"googleapis.com/compute/v1/projects/PROJECT_ID/regions/REGION/subnetworks/SUBNET_NAME"`
        pub subnetwork: pulumi_wasm_rust::Output<Option<String>>,
        /// A writeable location on GCS for the Dataflow job to dump its temporary data.
        ///
        /// - - -
        pub temp_gcs_location: pulumi_wasm_rust::Output<String>,
        /// The GCS path to the Dataflow job template.
        pub template_gcs_path: pulumi_wasm_rust::Output<String>,
        /// Only applicable when updating a pipeline. Map of transform name prefixes of the job to be replaced with the corresponding name prefixes of the new job. This field is not used outside of update.
        pub transform_name_mapping: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of this job, selected from the [JobType enum](https://cloud.google.com/dataflow/docs/reference/rest/v1b3/projects.jobs#Job.JobType)
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The zone in which the created job should run. If it is not provided, the provider zone is used.
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: JobArgs) -> JobResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_experiments_binding = args.additional_experiments.get_inner();
        let enable_streaming_engine_binding = args.enable_streaming_engine.get_inner();
        let ip_configuration_binding = args.ip_configuration.get_inner();
        let kms_key_name_binding = args.kms_key_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let machine_type_binding = args.machine_type.get_inner();
        let max_workers_binding = args.max_workers.get_inner();
        let name_binding = args.name.get_inner();
        let network_binding = args.network.get_inner();
        let on_delete_binding = args.on_delete.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let service_account_email_binding = args.service_account_email.get_inner();
        let skip_wait_on_job_termination_binding = args
            .skip_wait_on_job_termination
            .get_inner();
        let subnetwork_binding = args.subnetwork.get_inner();
        let temp_gcs_location_binding = args.temp_gcs_location.get_inner();
        let template_gcs_path_binding = args.template_gcs_path.get_inner();
        let transform_name_mapping_binding = args.transform_name_mapping.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataflow/job:Job".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalExperiments".into(),
                    value: &additional_experiments_binding,
                },
                register_interface::ObjectField {
                    name: "enableStreamingEngine".into(),
                    value: &enable_streaming_engine_binding,
                },
                register_interface::ObjectField {
                    name: "ipConfiguration".into(),
                    value: &ip_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyName".into(),
                    value: &kms_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "machineType".into(),
                    value: &machine_type_binding,
                },
                register_interface::ObjectField {
                    name: "maxWorkers".into(),
                    value: &max_workers_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "onDelete".into(),
                    value: &on_delete_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccountEmail".into(),
                    value: &service_account_email_binding,
                },
                register_interface::ObjectField {
                    name: "skipWaitOnJobTermination".into(),
                    value: &skip_wait_on_job_termination_binding,
                },
                register_interface::ObjectField {
                    name: "subnetwork".into(),
                    value: &subnetwork_binding,
                },
                register_interface::ObjectField {
                    name: "tempGcsLocation".into(),
                    value: &temp_gcs_location_binding,
                },
                register_interface::ObjectField {
                    name: "templateGcsPath".into(),
                    value: &template_gcs_path_binding,
                },
                register_interface::ObjectField {
                    name: "transformNameMapping".into(),
                    value: &transform_name_mapping_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalExperiments".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "enableStreamingEngine".into(),
                },
                register_interface::ResultField {
                    name: "ipConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "jobId".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyName".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "machineType".into(),
                },
                register_interface::ResultField {
                    name: "maxWorkers".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "onDelete".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccountEmail".into(),
                },
                register_interface::ResultField {
                    name: "skipWaitOnJobTermination".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "subnetwork".into(),
                },
                register_interface::ResultField {
                    name: "tempGcsLocation".into(),
                },
                register_interface::ResultField {
                    name: "templateGcsPath".into(),
                },
                register_interface::ResultField {
                    name: "transformNameMapping".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        JobResult {
            additional_experiments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalExperiments").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            enable_streaming_engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableStreamingEngine").unwrap(),
            ),
            ip_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipConfiguration").unwrap(),
            ),
            job_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobId").unwrap(),
            ),
            kms_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyName").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            machine_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("machineType").unwrap(),
            ),
            max_workers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxWorkers").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            on_delete: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("onDelete").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            service_account_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccountEmail").unwrap(),
            ),
            skip_wait_on_job_termination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipWaitOnJobTermination").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            subnetwork: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetwork").unwrap(),
            ),
            temp_gcs_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tempGcsLocation").unwrap(),
            ),
            template_gcs_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateGcsPath").unwrap(),
            ),
            transform_name_mapping: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transformNameMapping").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
