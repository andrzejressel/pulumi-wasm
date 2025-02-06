/// ## Example Usage
///
/// ```yaml
/// resources:
///   bigDataJob:
///     type: gcp:dataflow:FlexTemplateJob
///     name: big_data_job
///     properties:
///       name: dataflow-flextemplates-job
///       containerSpecGcsPath: gs://my-bucket/templates/template.json
///       parameters:
///         inputSubscription: messages
/// ```
///
/// ## Note on "destroy" / "apply"
///
/// There are many types of Dataflow jobs.  Some Dataflow jobs run constantly,
/// getting new data from (e.g.) a GCS bucket, and outputting data continuously.
/// Some jobs process a set amount of data then terminate. All jobs can fail while
/// running due to programming errors or other issues. In this way, Dataflow jobs
/// are different from most other provider / Google resources.
///
/// The Dataflow resource is considered 'existing' while it is in a nonterminal
/// state.  If it reaches a terminal state (e.g. 'FAILED', 'COMPLETE',
/// 'CANCELLED'), it will be recreated on the next 'apply'.  This is as expected for
/// jobs which run continuously, but may surprise users who use this resource for
/// other kinds of Dataflow jobs.
///
/// A Dataflow job which is 'destroyed' may be "cancelled" or "drained".  If
/// "cancelled", the job terminates - any data written remains where it is, but no
/// new data will be processed.  If "drained", no new data will enter the pipeline,
/// but any data currently in the pipeline will finish being processed.  The default
/// is "cancelled", but if a user sets `on_delete` to `"drain"` in the
/// configuration, you may experience a long wait for your `pulumi destroy` to
/// complete.
///
/// You can potentially short-circuit the wait by setting `skip_wait_on_job_termination`
/// to `true`, but beware that unless you take active steps to ensure that the job
/// `name` parameter changes between instances, the name will conflict and the launch
/// of the new job will fail. One way to do this is with a
/// random_id
/// resource, for example:
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
/// This resource does not support import.
///
pub mod flex_template_job {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexTemplateJobArgs {
        /// List of experiments that should be used by the job. An example value is `["enable_stackdriver_agent_metrics"]`.
        #[builder(into, default)]
        pub additional_experiments: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The algorithm to use for autoscaling.
        #[builder(into, default)]
        pub autoscaling_algorithm: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The GCS path to the Dataflow job Flex
        /// Template.
        ///
        /// - - -
        #[builder(into)]
        pub container_spec_gcs_path: pulumi_wasm_rust::InputOrOutput<String>,
        /// Immutable. Indicates if the job should use the streaming engine feature.
        #[builder(into, default)]
        pub enable_streaming_engine: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The configuration for VM IPs.  Options are `"WORKER_IP_PUBLIC"` or `"WORKER_IP_PRIVATE"`.
        #[builder(into, default)]
        pub ip_configuration: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name for the Cloud KMS key for the job. Key format is: `projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY`
        #[builder(into, default)]
        pub kms_key_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// User labels to be specified for the job. Keys and values
        /// should follow the restrictions specified in the [labeling restrictions](https://cloud.google.com/compute/docs/labeling-resources#restrictions)
        /// page. **Note**: This field is marked as deprecated as the API does not currently
        /// support adding labels.
        /// **NOTE**: Google-provided Dataflow templates often provide default labels
        /// that begin with `goog-dataflow-provided`. Unless explicitly set in config, these
        /// labels will be ignored to prevent diffs on re-apply.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The machine type to use for launching the job. The default is n1-standard-1.
        #[builder(into, default)]
        pub launcher_machine_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The machine type to use for the job.
        #[builder(into, default)]
        pub machine_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Immutable. The maximum number of Google Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000.
        #[builder(into, default)]
        pub max_workers: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Immutable. A unique name for the resource, required by Dataflow.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The network to which VMs will be assigned. If it is not provided, "default" will be used.
        #[builder(into, default)]
        pub network: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Immutable. The initial number of Google Compute Engine instances for the job.
        #[builder(into, default)]
        pub num_workers: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// One of "drain" or "cancel". Specifies behavior of
        /// deletion during `pulumi destroy`.  See above note.
        #[builder(into, default)]
        pub on_delete: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// **Template specific** Key/Value pairs to be forwarded to the pipeline's options; keys are
        /// case-sensitive based on the language on which the pipeline is coded, mostly Java.
        /// **Note**: do not configure Dataflow options here in parameters.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The project in which the resource belongs. If it is not
        /// provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Immutable. The region in which the created job should run.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Docker registry location of container image to use for the 'worker harness. Default is the container for the version of the SDK. Note this field is only valid for portable pipelines.
        #[builder(into, default)]
        pub sdk_container_image: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Service account email to run the workers as. This should be just an email e.g. `myserviceaccount@myproject.iam.gserviceaccount.com`. Do not include any `serviceAccount:` or other prefix.
        #[builder(into, default)]
        pub service_account_email: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub skip_wait_on_job_termination: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The Cloud Storage path to use for staging files. Must be a valid Cloud Storage URL, beginning with gs://.
        #[builder(into, default)]
        pub staging_location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The subnetwork to which VMs will be assigned. Should be of the form "regions/REGION/subnetworks/SUBNETWORK".
        #[builder(into, default)]
        pub subnetwork: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with gs://.
        #[builder(into, default)]
        pub temp_location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Only applicable when updating a pipeline. Map of transform name prefixes of the job to be replaced with the corresponding name prefixes of the new job.Only applicable when updating a pipeline. Map of transform name prefixes of the job to be replaced with the corresponding name prefixes of the new job.
        #[builder(into, default)]
        pub transform_name_mapping: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FlexTemplateJobResult {
        /// List of experiments that should be used by the job. An example value is `["enable_stackdriver_agent_metrics"]`.
        pub additional_experiments: pulumi_wasm_rust::Output<Vec<String>>,
        /// The algorithm to use for autoscaling.
        pub autoscaling_algorithm: pulumi_wasm_rust::Output<String>,
        /// The GCS path to the Dataflow job Flex
        /// Template.
        ///
        /// - - -
        pub container_spec_gcs_path: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Immutable. Indicates if the job should use the streaming engine feature.
        pub enable_streaming_engine: pulumi_wasm_rust::Output<Option<bool>>,
        /// The configuration for VM IPs.  Options are `"WORKER_IP_PUBLIC"` or `"WORKER_IP_PRIVATE"`.
        pub ip_configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique ID of this job.
        pub job_id: pulumi_wasm_rust::Output<String>,
        /// The name for the Cloud KMS key for the job. Key format is: `projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY`
        pub kms_key_name: pulumi_wasm_rust::Output<String>,
        /// User labels to be specified for the job. Keys and values
        /// should follow the restrictions specified in the [labeling restrictions](https://cloud.google.com/compute/docs/labeling-resources#restrictions)
        /// page. **Note**: This field is marked as deprecated as the API does not currently
        /// support adding labels.
        /// **NOTE**: Google-provided Dataflow templates often provide default labels
        /// that begin with `goog-dataflow-provided`. Unless explicitly set in config, these
        /// labels will be ignored to prevent diffs on re-apply.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The machine type to use for launching the job. The default is n1-standard-1.
        pub launcher_machine_type: pulumi_wasm_rust::Output<String>,
        /// The machine type to use for the job.
        pub machine_type: pulumi_wasm_rust::Output<String>,
        /// Immutable. The maximum number of Google Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000.
        pub max_workers: pulumi_wasm_rust::Output<i32>,
        /// Immutable. A unique name for the resource, required by Dataflow.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The network to which VMs will be assigned. If it is not provided, "default" will be used.
        pub network: pulumi_wasm_rust::Output<String>,
        /// Immutable. The initial number of Google Compute Engine instances for the job.
        pub num_workers: pulumi_wasm_rust::Output<i32>,
        /// One of "drain" or "cancel". Specifies behavior of
        /// deletion during `pulumi destroy`.  See above note.
        pub on_delete: pulumi_wasm_rust::Output<Option<String>>,
        /// **Template specific** Key/Value pairs to be forwarded to the pipeline's options; keys are
        /// case-sensitive based on the language on which the pipeline is coded, mostly Java.
        /// **Note**: do not configure Dataflow options here in parameters.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The project in which the resource belongs. If it is not
        /// provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Immutable. The region in which the created job should run.
        pub region: pulumi_wasm_rust::Output<String>,
        /// Docker registry location of container image to use for the 'worker harness. Default is the container for the version of the SDK. Note this field is only valid for portable pipelines.
        pub sdk_container_image: pulumi_wasm_rust::Output<String>,
        /// Service account email to run the workers as. This should be just an email e.g. `myserviceaccount@myproject.iam.gserviceaccount.com`. Do not include any `serviceAccount:` or other prefix.
        pub service_account_email: pulumi_wasm_rust::Output<String>,
        pub skip_wait_on_job_termination: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Cloud Storage path to use for staging files. Must be a valid Cloud Storage URL, beginning with gs://.
        pub staging_location: pulumi_wasm_rust::Output<String>,
        /// The current state of the resource, selected from the [JobState enum](https://cloud.google.com/dataflow/docs/reference/rest/v1b3/projects.jobs#Job.JobState)
        pub state: pulumi_wasm_rust::Output<String>,
        /// The subnetwork to which VMs will be assigned. Should be of the form "regions/REGION/subnetworks/SUBNETWORK".
        pub subnetwork: pulumi_wasm_rust::Output<String>,
        /// The Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with gs://.
        pub temp_location: pulumi_wasm_rust::Output<String>,
        /// Only applicable when updating a pipeline. Map of transform name prefixes of the job to be replaced with the corresponding name prefixes of the new job.Only applicable when updating a pipeline. Map of transform name prefixes of the job to be replaced with the corresponding name prefixes of the new job.
        pub transform_name_mapping: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of this job, selected from the JobType enum.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FlexTemplateJobArgs,
    ) -> FlexTemplateJobResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_experiments_binding = args
            .additional_experiments
            .get_output(context)
            .get_inner();
        let autoscaling_algorithm_binding = args
            .autoscaling_algorithm
            .get_output(context)
            .get_inner();
        let container_spec_gcs_path_binding = args
            .container_spec_gcs_path
            .get_output(context)
            .get_inner();
        let enable_streaming_engine_binding = args
            .enable_streaming_engine
            .get_output(context)
            .get_inner();
        let ip_configuration_binding = args
            .ip_configuration
            .get_output(context)
            .get_inner();
        let kms_key_name_binding = args.kms_key_name.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let launcher_machine_type_binding = args
            .launcher_machine_type
            .get_output(context)
            .get_inner();
        let machine_type_binding = args.machine_type.get_output(context).get_inner();
        let max_workers_binding = args.max_workers.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let num_workers_binding = args.num_workers.get_output(context).get_inner();
        let on_delete_binding = args.on_delete.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let sdk_container_image_binding = args
            .sdk_container_image
            .get_output(context)
            .get_inner();
        let service_account_email_binding = args
            .service_account_email
            .get_output(context)
            .get_inner();
        let skip_wait_on_job_termination_binding = args
            .skip_wait_on_job_termination
            .get_output(context)
            .get_inner();
        let staging_location_binding = args
            .staging_location
            .get_output(context)
            .get_inner();
        let subnetwork_binding = args.subnetwork.get_output(context).get_inner();
        let temp_location_binding = args.temp_location.get_output(context).get_inner();
        let transform_name_mapping_binding = args
            .transform_name_mapping
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataflow/flexTemplateJob:FlexTemplateJob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalExperiments".into(),
                    value: &additional_experiments_binding,
                },
                register_interface::ObjectField {
                    name: "autoscalingAlgorithm".into(),
                    value: &autoscaling_algorithm_binding,
                },
                register_interface::ObjectField {
                    name: "containerSpecGcsPath".into(),
                    value: &container_spec_gcs_path_binding,
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
                    name: "launcherMachineType".into(),
                    value: &launcher_machine_type_binding,
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
                    name: "numWorkers".into(),
                    value: &num_workers_binding,
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
                    name: "sdkContainerImage".into(),
                    value: &sdk_container_image_binding,
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
                    name: "stagingLocation".into(),
                    value: &staging_location_binding,
                },
                register_interface::ObjectField {
                    name: "subnetwork".into(),
                    value: &subnetwork_binding,
                },
                register_interface::ObjectField {
                    name: "tempLocation".into(),
                    value: &temp_location_binding,
                },
                register_interface::ObjectField {
                    name: "transformNameMapping".into(),
                    value: &transform_name_mapping_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FlexTemplateJobResult {
            additional_experiments: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("additionalExperiments"),
            ),
            autoscaling_algorithm: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoscalingAlgorithm"),
            ),
            container_spec_gcs_path: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("containerSpecGcsPath"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            enable_streaming_engine: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableStreamingEngine"),
            ),
            ip_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipConfiguration"),
            ),
            job_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("jobId")),
            kms_key_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyName"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            launcher_machine_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("launcherMachineType"),
            ),
            machine_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("machineType"),
            ),
            max_workers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maxWorkers"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            num_workers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("numWorkers"),
            ),
            on_delete: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("onDelete"),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            sdk_container_image: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sdkContainerImage"),
            ),
            service_account_email: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceAccountEmail"),
            ),
            skip_wait_on_job_termination: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skipWaitOnJobTermination"),
            ),
            staging_location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("stagingLocation"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            subnetwork: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetwork"),
            ),
            temp_location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tempLocation"),
            ),
            transform_name_mapping: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("transformNameMapping"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
