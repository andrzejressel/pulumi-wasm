/// A Cloud Run Job resource that references a container image which is run to completion.
///
///
/// To get more information about Job, see:
///
/// * [API documentation](https://cloud.google.com/run/docs/reference/rest/v2/projects.locations.jobs)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/run/docs/)
///
/// ## Example Usage
///
/// ### Cloudrunv2 Job Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = job::create(
///         "default",
///         JobArgs::builder()
///             .deletion_protection(false)
///             .location("us-central1")
///             .name("cloudrun-job")
///             .template(
///                 JobTemplate::builder()
///                     .template(
///                         JobTemplateTemplate::builder()
///                             .containers(
///                                 vec![
///                                     JobTemplateTemplateContainer::builder()
///                                     .image("us-docker.pkg.dev/cloudrun/container/job")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloudrunv2 Job Limits
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:cloudrunv2:Job
///     properties:
///       name: cloudrun-job
///       location: us-central1
///       deletionProtection: false
///       template:
///         template:
///           containers:
///             - image: us-docker.pkg.dev/cloudrun/container/job
///               resources:
///                 limits:
///                   cpu: '2'
///                   memory: 1024Mi
/// ```
/// ### Cloudrunv2 Job Sql
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:cloudrunv2:Job
///     properties:
///       name: cloudrun-job
///       location: us-central1
///       deletionProtection: false
///       template:
///         template:
///           volumes:
///             - name: cloudsql
///               cloudSqlInstance:
///                 instances:
///                   - ${instance.connectionName}
///           containers:
///             - image: us-docker.pkg.dev/cloudrun/container/job
///               envs:
///                 - name: FOO
///                   value: bar
///                 - name: latestdclsecret
///                   valueSource:
///                     secretKeyRef:
///                       secret: ${secret.secretId}
///                       version: '1'
///               volumeMounts:
///                 - name: cloudsql
///                   mountPath: /cloudsql
///   secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret
///       replication:
///         auto: {}
///   secret-version-data:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${secret.name}
///       secretData: secret-data
///   secret-access:
///     type: gcp:secretmanager:SecretIamMember
///     properties:
///       secretId: ${secret.id}
///       role: roles/secretmanager.secretAccessor
///       member: serviceAccount:${project.number}-compute@developer.gserviceaccount.com
///     options:
///       dependsOn:
///         - ${secret}
///   instance:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: cloudrun-sql
///       region: us-central1
///       databaseVersion: MYSQL_5_7
///       settings:
///         tier: db-f1-micro
///       deletionProtection: true
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Cloudrunv2 Job Vpcaccess
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let connector = connector::create(
///         "connector",
///         ConnectorArgs::builder()
///             .machine_type("e2-standard-4")
///             .max_instances(3)
///             .min_instances(2)
///             .name("run-vpc")
///             .region("us-central1")
///             .subnet(ConnectorSubnet::builder().name("${customTest.name}").build_struct())
///             .build_struct(),
///     );
///     let customTest = subnetwork::create(
///         "customTest",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.2.0.0/28")
///             .name("run-subnetwork")
///             .network("${customTestNetwork.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let customTestNetwork = network::create(
///         "customTestNetwork",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("run-network")
///             .build_struct(),
///     );
///     let default = job::create(
///         "default",
///         JobArgs::builder()
///             .deletion_protection(false)
///             .location("us-central1")
///             .name("cloudrun-job")
///             .template(
///                 JobTemplate::builder()
///                     .template(
///                         JobTemplateTemplate::builder()
///                             .containers(
///                                 vec![
///                                     JobTemplateTemplateContainer::builder()
///                                     .image("us-docker.pkg.dev/cloudrun/container/job")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .vpcAccess(
///                                 JobTemplateTemplateVpcAccess::builder()
///                                     .connector("${connector.id}")
///                                     .egress("ALL_TRAFFIC")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloudrunv2 Job Directvpc
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = job::create(
///         "default",
///         JobArgs::builder()
///             .deletion_protection(false)
///             .launch_stage("GA")
///             .location("us-central1")
///             .name("cloudrun-job")
///             .template(
///                 JobTemplate::builder()
///                     .template(
///                         JobTemplateTemplate::builder()
///                             .containers(
///                                 vec![
///                                     JobTemplateTemplateContainer::builder()
///                                     .image("us-docker.pkg.dev/cloudrun/container/job")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .vpcAccess(
///                                 JobTemplateTemplateVpcAccess::builder()
///                                     .networkInterfaces(
///                                         vec![
///                                             JobTemplateTemplateVpcAccessNetworkInterface::builder()
///                                             .network("default").subnetwork("default").tags(vec!["tag1",
///                                             "tag2", "tag3",]).build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloudrunv2 Job Secret
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:cloudrunv2:Job
///     properties:
///       name: cloudrun-job
///       location: us-central1
///       deletionProtection: false
///       template:
///         template:
///           volumes:
///             - name: a-volume
///               secret:
///                 secret: ${secret.secretId}
///                 defaultMode: 292
///                 items:
///                   - version: '1'
///                     path: my-secret
///                     mode: 256
///           containers:
///             - image: us-docker.pkg.dev/cloudrun/container/job
///               volumeMounts:
///                 - name: a-volume
///                   mountPath: /secrets
///     options:
///       dependsOn:
///         - ${["secret-version-data"]}
///         - ${["secret-access"]}
///   secret:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret
///       replication:
///         auto: {}
///   secret-version-data:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${secret.name}
///       secretData: secret-data
///   secret-access:
///     type: gcp:secretmanager:SecretIamMember
///     properties:
///       secretId: ${secret.id}
///       role: roles/secretmanager.secretAccessor
///       member: serviceAccount:${project.number}-compute@developer.gserviceaccount.com
///     options:
///       dependsOn:
///         - ${secret}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Cloudrunv2 Job Emptydir
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = job::create(
///         "default",
///         JobArgs::builder()
///             .deletion_protection(false)
///             .location("us-central1")
///             .name("cloudrun-job")
///             .template(
///                 JobTemplate::builder()
///                     .template(
///                         JobTemplateTemplate::builder()
///                             .containers(
///                                 vec![
///                                     JobTemplateTemplateContainer::builder()
///                                     .image("us-docker.pkg.dev/cloudrun/container/job")
///                                     .volumeMounts(vec![JobTemplateTemplateContainerVolumeMount::builder()
///                                     .mountPath("/mnt").name("empty-dir-volume")
///                                     .build_struct(),]).build_struct(),
///                                 ],
///                             )
///                             .volumes(
///                                 vec![
///                                     JobTemplateTemplateVolume::builder()
///                                     .emptyDir(JobTemplateTemplateVolumeEmptyDir::builder()
///                                     .medium("MEMORY").sizeLimit("128Mi").build_struct())
///                                     .name("empty-dir-volume").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloudrunv2 Job Run Job
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = job::create(
///         "default",
///         JobArgs::builder()
///             .deletion_protection(false)
///             .location("us-central1")
///             .name("cloudrun-job")
///             .start_execution_token("start-once-created")
///             .template(
///                 JobTemplate::builder()
///                     .template(
///                         JobTemplateTemplate::builder()
///                             .containers(
///                                 vec![
///                                     JobTemplateTemplateContainer::builder()
///                                     .image("us-docker.pkg.dev/cloudrun/container/job")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Job can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/jobs/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Job can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudrunv2/job:Job default projects/{{project}}/locations/{{location}}/jobs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudrunv2/job:Job default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudrunv2/job:Job default {{location}}/{{name}}
/// ```
///
pub mod job {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobArgs {
        /// Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and
        /// should be preserved when modifying objects. Cloud Run API v2 does not support annotations with 'run.googleapis.com',
        /// 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected on new
        /// resources. All system annotations in v1 now have a corresponding field in v2 Job. This field follows Kubernetes
        /// annotations' namespacing, limits, and rules. **Note**: This field is non-authoritative, and will only manage the
        /// annotations present in your configuration. Please refer to the field 'effective_annotations' for all of the annotations
        /// present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Settings for the Binary Authorization feature.
        #[builder(into, default)]
        pub binary_authorization: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudrunv2::JobBinaryAuthorization>,
        >,
        /// Arbitrary identifier for the API client.
        #[builder(into, default)]
        pub client: pulumi_wasm_rust::Output<Option<String>>,
        /// Arbitrary version identifier for the API client.
        #[builder(into, default)]
        pub client_version: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with
        /// Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment,
        /// state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or
        /// https://cloud.google.com/run/docs/configuring/labels. Cloud Run API v2 does not support labels with
        /// 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they
        /// will be rejected. All system labels in v1 now have a corresponding field in v2 Job. **Note**: This field is
        /// non-authoritative, and will only manage the labels present in your configuration. Please refer to the field
        /// 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The launch stage as defined by [Google Cloud Platform Launch
        /// Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA. If no value is
        /// specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that
        /// stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as
        /// input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values:
        /// ["UNIMPLEMENTED", "PRELAUNCH", "EARLY_ACCESS", "ALPHA", "BETA", "GA", "DEPRECATED"]
        #[builder(into, default)]
        pub launch_stage: pulumi_wasm_rust::Output<Option<String>>,
        /// The location of the cloud run job
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the Job.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique string used as a suffix creating a new execution upon job create or update. The Job will become ready when the
        /// execution is successfully completed. The sum of job name and token length must be fewer than 63 characters.
        #[builder(into, default)]
        pub run_execution_token: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique string used as a suffix creating a new execution upon job create or update. The Job will become ready when the
        /// execution is successfully started. The sum of job name and token length must be fewer than 63 characters.
        #[builder(into, default)]
        pub start_execution_token: pulumi_wasm_rust::Output<Option<String>>,
        /// The template used to create executions for this Job.
        /// Structure is documented below.
        #[builder(into)]
        pub template: pulumi_wasm_rust::Output<
            super::super::types::cloudrunv2::JobTemplate,
        >,
    }
    #[allow(dead_code)]
    pub struct JobResult {
        /// Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and
        /// should be preserved when modifying objects. Cloud Run API v2 does not support annotations with 'run.googleapis.com',
        /// 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they will be rejected on new
        /// resources. All system annotations in v1 now have a corresponding field in v2 Job. This field follows Kubernetes
        /// annotations' namespacing, limits, and rules. **Note**: This field is non-authoritative, and will only manage the
        /// annotations present in your configuration. Please refer to the field 'effective_annotations' for all of the annotations
        /// present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Settings for the Binary Authorization feature.
        pub binary_authorization: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudrunv2::JobBinaryAuthorization>,
        >,
        /// Arbitrary identifier for the API client.
        pub client: pulumi_wasm_rust::Output<Option<String>>,
        /// Arbitrary version identifier for the API client.
        pub client_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Job does not reach its desired state. See comments in reconciling for additional information on `reconciliation` process in Cloud Run.
        /// Structure is documented below.
        pub conditions: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudrunv2::JobCondition>,
        >,
        /// (Output)
        /// Creation timestamp of the execution.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Email address of the authenticated creator.
        pub creator: pulumi_wasm_rust::Output<String>,
        /// The deletion time.
        pub delete_time: pulumi_wasm_rust::Output<String>,
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Number of executions created for this job.
        pub execution_count: pulumi_wasm_rust::Output<i32>,
        /// For a deleted resource, the time after which it will be permanently deleted.
        pub expire_time: pulumi_wasm_rust::Output<String>,
        /// A number that monotonically increases every time the user modifies the desired state.
        pub generation: pulumi_wasm_rust::Output<String>,
        /// Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with
        /// Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment,
        /// state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or
        /// https://cloud.google.com/run/docs/configuring/labels. Cloud Run API v2 does not support labels with
        /// 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they
        /// will be rejected. All system labels in v1 now have a corresponding field in v2 Job. **Note**: This field is
        /// non-authoritative, and will only manage the labels present in your configuration. Please refer to the field
        /// 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Email address of the last authenticated modifier.
        pub last_modifier: pulumi_wasm_rust::Output<String>,
        /// Name of the last created execution.
        /// Structure is documented below.
        pub latest_created_executions: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudrunv2::JobLatestCreatedExecution>,
        >,
        /// The launch stage as defined by [Google Cloud Platform Launch
        /// Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA. If no value is
        /// specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that
        /// stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as
        /// input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values:
        /// ["UNIMPLEMENTED", "PRELAUNCH", "EARLY_ACCESS", "ALPHA", "BETA", "GA", "DEPRECATED"]
        pub launch_stage: pulumi_wasm_rust::Output<String>,
        /// The location of the cloud run job
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the Job.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The generation of this Job. See comments in reconciling for additional information on reconciliation process in Cloud Run.
        pub observed_generation: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Returns true if the Job is currently being acted upon by the system to bring it into the desired state.
        /// When a new Job is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Job to the desired state. This process is called reconciliation. While reconciliation is in process, observedGeneration and latest_succeeded_execution, will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the state matches the Job, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.
        /// If reconciliation succeeded, the following fields will match: observedGeneration and generation, latest_succeeded_execution and latestCreatedExecution.
        /// If reconciliation failed, observedGeneration and latest_succeeded_execution will have the state of the last succeeded execution or empty for newly created Job. Additional information on the failure can be found in terminalCondition and conditions
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// A unique string used as a suffix creating a new execution upon job create or update. The Job will become ready when the
        /// execution is successfully completed. The sum of job name and token length must be fewer than 63 characters.
        pub run_execution_token: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique string used as a suffix creating a new execution upon job create or update. The Job will become ready when the
        /// execution is successfully started. The sum of job name and token length must be fewer than 63 characters.
        pub start_execution_token: pulumi_wasm_rust::Output<Option<String>>,
        /// The template used to create executions for this Job.
        /// Structure is documented below.
        pub template: pulumi_wasm_rust::Output<
            super::super::types::cloudrunv2::JobTemplate,
        >,
        /// The Condition of this Job, containing its readiness status, and detailed error information in case it did not reach the desired state
        /// Structure is documented below.
        pub terminal_conditions: pulumi_wasm_rust::Output<
            Vec<super::super::types::cloudrunv2::JobTerminalCondition>,
        >,
        /// Server assigned unique identifier for the Execution. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// The last-modified time.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: JobArgs) -> JobResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_inner();
        let binary_authorization_binding = args.binary_authorization.get_inner();
        let client_binding = args.client.get_inner();
        let client_version_binding = args.client_version.get_inner();
        let deletion_protection_binding = args.deletion_protection.get_inner();
        let labels_binding = args.labels.get_inner();
        let launch_stage_binding = args.launch_stage.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let run_execution_token_binding = args.run_execution_token.get_inner();
        let start_execution_token_binding = args.start_execution_token.get_inner();
        let template_binding = args.template.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:cloudrunv2/job:Job".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "binaryAuthorization".into(),
                    value: &binary_authorization_binding,
                },
                register_interface::ObjectField {
                    name: "client".into(),
                    value: &client_binding,
                },
                register_interface::ObjectField {
                    name: "clientVersion".into(),
                    value: &client_version_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "launchStage".into(),
                    value: &launch_stage_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "runExecutionToken".into(),
                    value: &run_execution_token_binding,
                },
                register_interface::ObjectField {
                    name: "startExecutionToken".into(),
                    value: &start_execution_token_binding,
                },
                register_interface::ObjectField {
                    name: "template".into(),
                    value: &template_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "binaryAuthorization".into(),
                },
                register_interface::ResultField {
                    name: "client".into(),
                },
                register_interface::ResultField {
                    name: "clientVersion".into(),
                },
                register_interface::ResultField {
                    name: "conditions".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "creator".into(),
                },
                register_interface::ResultField {
                    name: "deleteTime".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "executionCount".into(),
                },
                register_interface::ResultField {
                    name: "expireTime".into(),
                },
                register_interface::ResultField {
                    name: "generation".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "lastModifier".into(),
                },
                register_interface::ResultField {
                    name: "latestCreatedExecutions".into(),
                },
                register_interface::ResultField {
                    name: "launchStage".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "observedGeneration".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "reconciling".into(),
                },
                register_interface::ResultField {
                    name: "runExecutionToken".into(),
                },
                register_interface::ResultField {
                    name: "startExecutionToken".into(),
                },
                register_interface::ResultField {
                    name: "template".into(),
                },
                register_interface::ResultField {
                    name: "terminalConditions".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
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
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            binary_authorization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("binaryAuthorization").unwrap(),
            ),
            client: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("client").unwrap(),
            ),
            client_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientVersion").unwrap(),
            ),
            conditions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("conditions").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            creator: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creator").unwrap(),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteTime").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            execution_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionCount").unwrap(),
            ),
            expire_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expireTime").unwrap(),
            ),
            generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("generation").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            last_modifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifier").unwrap(),
            ),
            latest_created_executions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestCreatedExecutions").unwrap(),
            ),
            launch_stage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launchStage").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            observed_generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("observedGeneration").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconciling").unwrap(),
            ),
            run_execution_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runExecutionToken").unwrap(),
            ),
            start_execution_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startExecutionToken").unwrap(),
            ),
            template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("template").unwrap(),
            ),
            terminal_conditions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terminalConditions").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
