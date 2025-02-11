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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
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
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Settings for the Binary Authorization feature.
        #[builder(into, default)]
        pub binary_authorization: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudrunv2::JobBinaryAuthorization>,
        >,
        /// Arbitrary identifier for the API client.
        #[builder(into, default)]
        pub client: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Arbitrary version identifier for the API client.
        #[builder(into, default)]
        pub client_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with
        /// Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment,
        /// state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or
        /// https://cloud.google.com/run/docs/configuring/labels. Cloud Run API v2 does not support labels with
        /// 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they
        /// will be rejected. All system labels in v1 now have a corresponding field in v2 Job. **Note**: This field is
        /// non-authoritative, and will only manage the labels present in your configuration. Please refer to the field
        /// 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The launch stage as defined by [Google Cloud Platform Launch
        /// Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA. If no value is
        /// specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that
        /// stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as
        /// input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values:
        /// ["UNIMPLEMENTED", "PRELAUNCH", "EARLY_ACCESS", "ALPHA", "BETA", "GA", "DEPRECATED"]
        #[builder(into, default)]
        pub launch_stage: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location of the cloud run job
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the Job.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A unique string used as a suffix creating a new execution upon job create or update. The Job will become ready when the
        /// execution is successfully completed. The sum of job name and token length must be fewer than 63 characters.
        #[builder(into, default)]
        pub run_execution_token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A unique string used as a suffix creating a new execution upon job create or update. The Job will become ready when the
        /// execution is successfully started. The sum of job name and token length must be fewer than 63 characters.
        #[builder(into, default)]
        pub start_execution_token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The template used to create executions for this Job.
        /// Structure is documented below.
        #[builder(into)]
        pub template: pulumi_gestalt_rust::InputOrOutput<
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
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Settings for the Binary Authorization feature.
        pub binary_authorization: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudrunv2::JobBinaryAuthorization>,
        >,
        /// Arbitrary identifier for the API client.
        pub client: pulumi_gestalt_rust::Output<Option<String>>,
        /// Arbitrary version identifier for the API client.
        pub client_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Job does not reach its desired state. See comments in reconciling for additional information on `reconciliation` process in Cloud Run.
        /// Structure is documented below.
        pub conditions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cloudrunv2::JobCondition>,
        >,
        /// (Output)
        /// Creation timestamp of the execution.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Email address of the authenticated creator.
        pub creator: pulumi_gestalt_rust::Output<String>,
        /// The deletion time.
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Number of executions created for this job.
        pub execution_count: pulumi_gestalt_rust::Output<i32>,
        /// For a deleted resource, the time after which it will be permanently deleted.
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        /// A number that monotonically increases every time the user modifies the desired state.
        pub generation: pulumi_gestalt_rust::Output<String>,
        /// Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with
        /// Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment,
        /// state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or
        /// https://cloud.google.com/run/docs/configuring/labels. Cloud Run API v2 does not support labels with
        /// 'run.googleapis.com', 'cloud.googleapis.com', 'serving.knative.dev', or 'autoscaling.knative.dev' namespaces, and they
        /// will be rejected. All system labels in v1 now have a corresponding field in v2 Job. **Note**: This field is
        /// non-authoritative, and will only manage the labels present in your configuration. Please refer to the field
        /// 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Email address of the last authenticated modifier.
        pub last_modifier: pulumi_gestalt_rust::Output<String>,
        /// Name of the last created execution.
        /// Structure is documented below.
        pub latest_created_executions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cloudrunv2::JobLatestCreatedExecution>,
        >,
        /// The launch stage as defined by [Google Cloud Platform Launch
        /// Stages](https://cloud.google.com/products#product-launch-stages). Cloud Run supports ALPHA, BETA, and GA. If no value is
        /// specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that
        /// stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as
        /// input, but only BETA and GA-level features are used, this field will be BETA on output. Possible values:
        /// ["UNIMPLEMENTED", "PRELAUNCH", "EARLY_ACCESS", "ALPHA", "BETA", "GA", "DEPRECATED"]
        pub launch_stage: pulumi_gestalt_rust::Output<String>,
        /// The location of the cloud run job
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the Job.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The generation of this Job. See comments in reconciling for additional information on reconciliation process in Cloud Run.
        pub observed_generation: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Returns true if the Job is currently being acted upon by the system to bring it into the desired state.
        /// When a new Job is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Job to the desired state. This process is called reconciliation. While reconciliation is in process, observedGeneration and latest_succeeded_execution, will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the state matches the Job, or there was an error, and reconciliation failed. This state can be found in terminalCondition.state.
        /// If reconciliation succeeded, the following fields will match: observedGeneration and generation, latest_succeeded_execution and latestCreatedExecution.
        /// If reconciliation failed, observedGeneration and latest_succeeded_execution will have the state of the last succeeded execution or empty for newly created Job. Additional information on the failure can be found in terminalCondition and conditions
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// A unique string used as a suffix creating a new execution upon job create or update. The Job will become ready when the
        /// execution is successfully completed. The sum of job name and token length must be fewer than 63 characters.
        pub run_execution_token: pulumi_gestalt_rust::Output<Option<String>>,
        /// A unique string used as a suffix creating a new execution upon job create or update. The Job will become ready when the
        /// execution is successfully started. The sum of job name and token length must be fewer than 63 characters.
        pub start_execution_token: pulumi_gestalt_rust::Output<Option<String>>,
        /// The template used to create executions for this Job.
        /// Structure is documented below.
        pub template: pulumi_gestalt_rust::Output<
            super::super::types::cloudrunv2::JobTemplate,
        >,
        /// The Condition of this Job, containing its readiness status, and detailed error information in case it did not reach the desired state
        /// Structure is documented below.
        pub terminal_conditions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cloudrunv2::JobTerminalCondition>,
        >,
        /// Server assigned unique identifier for the Execution. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The last-modified time.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: JobArgs,
    ) -> JobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let binary_authorization_binding = args.binary_authorization.get_output(context);
        let client_binding = args.client.get_output(context);
        let client_version_binding = args.client_version.get_output(context);
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let launch_stage_binding = args.launch_stage.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let run_execution_token_binding = args.run_execution_token.get_output(context);
        let start_execution_token_binding = args
            .start_execution_token
            .get_output(context);
        let template_binding = args.template.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:cloudrunv2/job:Job".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "binaryAuthorization".into(),
                    value: &binary_authorization_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "client".into(),
                    value: &client_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientVersion".into(),
                    value: &client_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "launchStage".into(),
                    value: &launch_stage_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runExecutionToken".into(),
                    value: &run_execution_token_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startExecutionToken".into(),
                    value: &start_execution_token_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "template".into(),
                    value: &template_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        JobResult {
            annotations: o.get_field("annotations"),
            binary_authorization: o.get_field("binaryAuthorization"),
            client: o.get_field("client"),
            client_version: o.get_field("clientVersion"),
            conditions: o.get_field("conditions"),
            create_time: o.get_field("createTime"),
            creator: o.get_field("creator"),
            delete_time: o.get_field("deleteTime"),
            deletion_protection: o.get_field("deletionProtection"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            execution_count: o.get_field("executionCount"),
            expire_time: o.get_field("expireTime"),
            generation: o.get_field("generation"),
            labels: o.get_field("labels"),
            last_modifier: o.get_field("lastModifier"),
            latest_created_executions: o.get_field("latestCreatedExecutions"),
            launch_stage: o.get_field("launchStage"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            observed_generation: o.get_field("observedGeneration"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            run_execution_token: o.get_field("runExecutionToken"),
            start_execution_token: o.get_field("startExecutionToken"),
            template: o.get_field("template"),
            terminal_conditions: o.get_field("terminalConditions"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
