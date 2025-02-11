/// Creates a new Cloud Function. For more information see:
///
/// * [API documentation](https://cloud.google.com/functions/docs/reference/rest/v1/projects.locations.functions)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/functions/docs)
///
///
/// > **Warning:** As of November 1, 2019, newly created Functions are
/// private-by-default and will require [appropriate IAM permissions](https://cloud.google.com/functions/docs/reference/iam/roles)
/// to be invoked. See below examples for how to set up the appropriate permissions,
/// or view the [Cloud Functions IAM resources](https://www.terraform.io/docs/providers/google/r/cloudfunctions_cloud_function_iam.html)
/// for Cloud Functions.
///
/// ## Example Usage
///
/// ### Public Function
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: test-bucket
///       location: US
///   archive:
///     type: gcp:storage:BucketObject
///     properties:
///       name: index.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: ./path/to/zip/file/which/contains/code
///   function:
///     type: gcp:cloudfunctions:Function
///     properties:
///       name: function-test
///       description: My function
///       runtime: nodejs16
///       availableMemoryMb: 128
///       sourceArchiveBucket: ${bucket.name}
///       sourceArchiveObject: ${archive.name}
///       triggerHttp: true
///       entryPoint: helloGET
///   # IAM entry for all users to invoke the function
///   invoker:
///     type: gcp:cloudfunctions:FunctionIamMember
///     properties:
///       project: ${function.project}
///       region: ${function.region}
///       cloudFunction: ${function.name}
///       role: roles/cloudfunctions.invoker
///       member: allUsers
/// ```
///
/// ### Single User
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: test-bucket
///       location: US
///   archive:
///     type: gcp:storage:BucketObject
///     properties:
///       name: index.zip
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: ./path/to/zip/file/which/contains/code
///   function:
///     type: gcp:cloudfunctions:Function
///     properties:
///       name: function-test
///       description: My function
///       runtime: nodejs16
///       availableMemoryMb: 128
///       sourceArchiveBucket: ${bucket.name}
///       sourceArchiveObject: ${archive.name}
///       triggerHttp: true
///       httpsTriggerSecurityLevel: SECURE_ALWAYS
///       timeout: 60
///       entryPoint: helloGET
///       labels:
///         my-label: my-label-value
///       environmentVariables:
///         MY_ENV_VAR: my-env-var-value
///   # IAM entry for a single user to invoke the function
///   invoker:
///     type: gcp:cloudfunctions:FunctionIamMember
///     properties:
///       project: ${function.project}
///       region: ${function.region}
///       cloudFunction: ${function.name}
///       role: roles/cloudfunctions.invoker
///       member: user:myFunctionInvoker@example.com
/// ```
///
/// ## Import
///
/// Functions can be imported using the `name` or `{{project}}/{{region}}/name`, e.g.
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Functions can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudfunctions/function:Function default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudfunctions/function:Function default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod function {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionArgs {
        /// Memory (in MB), available to the function. Default value is `256`. Possible values include `128`, `256`, `512`, `1024`, etc.
        #[builder(into, default)]
        pub available_memory_mb: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A set of key/value environment variable pairs available during build time.
        #[builder(into, default)]
        pub build_environment_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If provided, the self-provided service account to use to build the function. The format of this field is `projects/{project}/serviceAccounts/{serviceAccountEmail}`
        #[builder(into, default)]
        pub build_service_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the Cloud Build Custom Worker Pool that should be used to build the function.
        #[builder(into, default)]
        pub build_worker_pool: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of the function.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Docker Registry to use for storing the function's Docker images. Allowed values are ARTIFACT_REGISTRY (default) and CONTAINER_REGISTRY.
        #[builder(into, default)]
        pub docker_registry: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-managed repository created in Artifact Registry to which the function's Docker image will be pushed after it is built by Cloud Build. May optionally be encrypted with a customer-managed encryption key (CMEK). If unspecified and `docker_registry` is not explicitly set to `CONTAINER_REGISTRY`, GCF will create and use a default Artifact Registry repository named 'gcf-artifacts' in the region.
        #[builder(into, default)]
        pub docker_repository: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the function that will be executed when the Google Cloud Function is triggered.
        #[builder(into, default)]
        pub entry_point: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A set of key/value environment variable pairs to assign to the function.
        #[builder(into, default)]
        pub environment_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A source that fires events in response to a condition in another service. Structure is documented below. Cannot be used with `trigger_http`.
        #[builder(into, default)]
        pub event_trigger: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudfunctions::FunctionEventTrigger>,
        >,
        /// The security level for the function. The following options are available:
        ///
        /// * `SECURE_ALWAYS` Requests for a URL that match this handler that do not use HTTPS are automatically redirected to the HTTPS URL with the same path. Query parameters are reserved for the redirect.
        /// * `SECURE_OPTIONAL` Both HTTP and HTTPS requests with URLs that match the handler succeed without redirects. The application can examine the request to determine which protocol was used and respond accordingly.
        #[builder(into, default)]
        pub https_trigger_security_level: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// URL which triggers function execution. Returned only if `trigger_http` is used.
        #[builder(into, default)]
        pub https_trigger_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// String value that controls what traffic can reach the function. Allowed values are `ALLOW_ALL`, `ALLOW_INTERNAL_AND_GCLB` and `ALLOW_INTERNAL_ONLY`. Check [ingress documentation](https://cloud.google.com/functions/docs/networking/network-settings#ingress_settings) to see the impact of each settings value. Changes to this field will recreate the cloud function.
        #[builder(into, default)]
        pub ingress_settings: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources. It must match the pattern `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`.
        /// If specified, you must also provide an artifact registry repository using the `docker_repository` field that was created with the same KMS crypto key. Before deploying, please complete all pre-requisites described in https://cloud.google.com/functions/docs/securing/cmek#granting_service_accounts_access_to_the_key
        #[builder(into, default)]
        pub kms_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A set of key/value label pairs to assign to the function. Label keys must follow the requirements at https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The limit on the maximum number of function instances that may coexist at a given time.
        #[builder(into, default)]
        pub max_instances: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The limit on the minimum number of function instances that may coexist at a given time.
        #[builder(into, default)]
        pub min_instances: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A user-defined name of the function. Function names must be unique globally.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Project of the function. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region of function. If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The runtime in which the function is going to run.
        /// Eg. `"nodejs16"`, `"python39"`, `"dotnet3"`, `"go116"`, `"java11"`, `"ruby30"`, `"php74"`, etc. Check the [official doc](https://cloud.google.com/functions/docs/concepts/exec#runtimes) for the up-to-date list.
        ///
        /// - - -
        #[builder(into)]
        pub runtime: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Secret environment variables configuration. Structure is documented below.
        #[builder(into, default)]
        pub secret_environment_variables: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::cloudfunctions::FunctionSecretEnvironmentVariable,
                >,
            >,
        >,
        /// Secret volumes configuration. Structure is documented below.
        #[builder(into, default)]
        pub secret_volumes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cloudfunctions::FunctionSecretVolume>>,
        >,
        /// If provided, the self-provided service account to run the function with.
        #[builder(into, default)]
        pub service_account_email: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The GCS bucket containing the zip archive which contains the function.
        #[builder(into, default)]
        pub source_archive_bucket: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The source archive object (file) in archive bucket.
        #[builder(into, default)]
        pub source_archive_object: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Represents parameters related to source repository where a function is hosted.
        /// Cannot be set alongside `source_archive_bucket` or `source_archive_object`. Structure is documented below. It must match the pattern `projects/{project}/locations/{location}/repositories/{repository}`.*
        #[builder(into, default)]
        pub source_repository: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudfunctions::FunctionSourceRepository>,
        >,
        /// Timeout (in seconds) for the function. Default value is 60 seconds. Cannot be more than 540 seconds.
        #[builder(into, default)]
        pub timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Boolean variable. Any HTTP request (of a supported type) to the endpoint will trigger function execution. Supported HTTP request types are: POST, PUT, GET, DELETE, and OPTIONS. Endpoint is returned as `https_trigger_url`. Cannot be used with `event_trigger`.
        #[builder(into, default)]
        pub trigger_http: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The VPC Network Connector that this cloud function can connect to. It should be set up as fully-qualified URI. The format of this field is `projects/*/locations/*/connectors/*`.
        #[builder(into, default)]
        pub vpc_connector: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The egress settings for the connector, controlling what traffic is diverted through it. Allowed values are `ALL_TRAFFIC` and `PRIVATE_RANGES_ONLY`. Defaults to `PRIVATE_RANGES_ONLY`. If unset, this field preserves the previously set value.
        #[builder(into, default)]
        pub vpc_connector_egress_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct FunctionResult {
        /// Memory (in MB), available to the function. Default value is `256`. Possible values include `128`, `256`, `512`, `1024`, etc.
        pub available_memory_mb: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A set of key/value environment variable pairs available during build time.
        pub build_environment_variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If provided, the self-provided service account to use to build the function. The format of this field is `projects/{project}/serviceAccounts/{serviceAccountEmail}`
        pub build_service_account: pulumi_gestalt_rust::Output<String>,
        /// Name of the Cloud Build Custom Worker Pool that should be used to build the function.
        pub build_worker_pool: pulumi_gestalt_rust::Output<Option<String>>,
        /// Description of the function.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Docker Registry to use for storing the function's Docker images. Allowed values are ARTIFACT_REGISTRY (default) and CONTAINER_REGISTRY.
        pub docker_registry: pulumi_gestalt_rust::Output<String>,
        /// User-managed repository created in Artifact Registry to which the function's Docker image will be pushed after it is built by Cloud Build. May optionally be encrypted with a customer-managed encryption key (CMEK). If unspecified and `docker_registry` is not explicitly set to `CONTAINER_REGISTRY`, GCF will create and use a default Artifact Registry repository named 'gcf-artifacts' in the region.
        pub docker_repository: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Name of the function that will be executed when the Google Cloud Function is triggered.
        pub entry_point: pulumi_gestalt_rust::Output<Option<String>>,
        /// A set of key/value environment variable pairs to assign to the function.
        pub environment_variables: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A source that fires events in response to a condition in another service. Structure is documented below. Cannot be used with `trigger_http`.
        pub event_trigger: pulumi_gestalt_rust::Output<
            super::super::types::cloudfunctions::FunctionEventTrigger,
        >,
        /// The security level for the function. The following options are available:
        ///
        /// * `SECURE_ALWAYS` Requests for a URL that match this handler that do not use HTTPS are automatically redirected to the HTTPS URL with the same path. Query parameters are reserved for the redirect.
        /// * `SECURE_OPTIONAL` Both HTTP and HTTPS requests with URLs that match the handler succeed without redirects. The application can examine the request to determine which protocol was used and respond accordingly.
        pub https_trigger_security_level: pulumi_gestalt_rust::Output<String>,
        /// URL which triggers function execution. Returned only if `trigger_http` is used.
        pub https_trigger_url: pulumi_gestalt_rust::Output<String>,
        /// String value that controls what traffic can reach the function. Allowed values are `ALLOW_ALL`, `ALLOW_INTERNAL_AND_GCLB` and `ALLOW_INTERNAL_ONLY`. Check [ingress documentation](https://cloud.google.com/functions/docs/networking/network-settings#ingress_settings) to see the impact of each settings value. Changes to this field will recreate the cloud function.
        pub ingress_settings: pulumi_gestalt_rust::Output<Option<String>>,
        /// Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources. It must match the pattern `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`.
        /// If specified, you must also provide an artifact registry repository using the `docker_repository` field that was created with the same KMS crypto key. Before deploying, please complete all pre-requisites described in https://cloud.google.com/functions/docs/securing/cmek#granting_service_accounts_access_to_the_key
        pub kms_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A set of key/value label pairs to assign to the function. Label keys must follow the requirements at https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The limit on the maximum number of function instances that may coexist at a given time.
        pub max_instances: pulumi_gestalt_rust::Output<i32>,
        /// The limit on the minimum number of function instances that may coexist at a given time.
        pub min_instances: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A user-defined name of the function. Function names must be unique globally.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Project of the function. If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Region of function. If it is not provided, the provider region is used.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The runtime in which the function is going to run.
        /// Eg. `"nodejs16"`, `"python39"`, `"dotnet3"`, `"go116"`, `"java11"`, `"ruby30"`, `"php74"`, etc. Check the [official doc](https://cloud.google.com/functions/docs/concepts/exec#runtimes) for the up-to-date list.
        ///
        /// - - -
        pub runtime: pulumi_gestalt_rust::Output<String>,
        /// Secret environment variables configuration. Structure is documented below.
        pub secret_environment_variables: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::cloudfunctions::FunctionSecretEnvironmentVariable,
                >,
            >,
        >,
        /// Secret volumes configuration. Structure is documented below.
        pub secret_volumes: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cloudfunctions::FunctionSecretVolume>>,
        >,
        /// If provided, the self-provided service account to run the function with.
        pub service_account_email: pulumi_gestalt_rust::Output<String>,
        /// The GCS bucket containing the zip archive which contains the function.
        pub source_archive_bucket: pulumi_gestalt_rust::Output<Option<String>>,
        /// The source archive object (file) in archive bucket.
        pub source_archive_object: pulumi_gestalt_rust::Output<Option<String>>,
        /// Represents parameters related to source repository where a function is hosted.
        /// Cannot be set alongside `source_archive_bucket` or `source_archive_object`. Structure is documented below. It must match the pattern `projects/{project}/locations/{location}/repositories/{repository}`.*
        pub source_repository: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudfunctions::FunctionSourceRepository>,
        >,
        /// Describes the current stage of a deployment.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Timeout (in seconds) for the function. Default value is 60 seconds. Cannot be more than 540 seconds.
        pub timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Boolean variable. Any HTTP request (of a supported type) to the endpoint will trigger function execution. Supported HTTP request types are: POST, PUT, GET, DELETE, and OPTIONS. Endpoint is returned as `https_trigger_url`. Cannot be used with `event_trigger`.
        pub trigger_http: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The version identifier of the Cloud Function. Each deployment attempt results in a new version of a function being
        /// created.
        pub version_id: pulumi_gestalt_rust::Output<String>,
        /// The VPC Network Connector that this cloud function can connect to. It should be set up as fully-qualified URI. The format of this field is `projects/*/locations/*/connectors/*`.
        pub vpc_connector: pulumi_gestalt_rust::Output<Option<String>>,
        /// The egress settings for the connector, controlling what traffic is diverted through it. Allowed values are `ALL_TRAFFIC` and `PRIVATE_RANGES_ONLY`. Defaults to `PRIVATE_RANGES_ONLY`. If unset, this field preserves the previously set value.
        pub vpc_connector_egress_settings: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FunctionArgs,
    ) -> FunctionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let available_memory_mb_binding = args.available_memory_mb.get_output(context);
        let build_environment_variables_binding = args
            .build_environment_variables
            .get_output(context);
        let build_service_account_binding = args
            .build_service_account
            .get_output(context);
        let build_worker_pool_binding = args.build_worker_pool.get_output(context);
        let description_binding = args.description.get_output(context);
        let docker_registry_binding = args.docker_registry.get_output(context);
        let docker_repository_binding = args.docker_repository.get_output(context);
        let entry_point_binding = args.entry_point.get_output(context);
        let environment_variables_binding = args
            .environment_variables
            .get_output(context);
        let event_trigger_binding = args.event_trigger.get_output(context);
        let https_trigger_security_level_binding = args
            .https_trigger_security_level
            .get_output(context);
        let https_trigger_url_binding = args.https_trigger_url.get_output(context);
        let ingress_settings_binding = args.ingress_settings.get_output(context);
        let kms_key_name_binding = args.kms_key_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let max_instances_binding = args.max_instances.get_output(context);
        let min_instances_binding = args.min_instances.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let runtime_binding = args.runtime.get_output(context);
        let secret_environment_variables_binding = args
            .secret_environment_variables
            .get_output(context);
        let secret_volumes_binding = args.secret_volumes.get_output(context);
        let service_account_email_binding = args
            .service_account_email
            .get_output(context);
        let source_archive_bucket_binding = args
            .source_archive_bucket
            .get_output(context);
        let source_archive_object_binding = args
            .source_archive_object
            .get_output(context);
        let source_repository_binding = args.source_repository.get_output(context);
        let timeout_binding = args.timeout.get_output(context);
        let trigger_http_binding = args.trigger_http.get_output(context);
        let vpc_connector_binding = args.vpc_connector.get_output(context);
        let vpc_connector_egress_settings_binding = args
            .vpc_connector_egress_settings
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:cloudfunctions/function:Function".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availableMemoryMb".into(),
                    value: &available_memory_mb_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "buildEnvironmentVariables".into(),
                    value: &build_environment_variables_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "buildServiceAccount".into(),
                    value: &build_service_account_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "buildWorkerPool".into(),
                    value: &build_worker_pool_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dockerRegistry".into(),
                    value: &docker_registry_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dockerRepository".into(),
                    value: &docker_repository_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entryPoint".into(),
                    value: &entry_point_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentVariables".into(),
                    value: &environment_variables_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventTrigger".into(),
                    value: &event_trigger_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpsTriggerSecurityLevel".into(),
                    value: &https_trigger_security_level_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpsTriggerUrl".into(),
                    value: &https_trigger_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ingressSettings".into(),
                    value: &ingress_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyName".into(),
                    value: &kms_key_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxInstances".into(),
                    value: &max_instances_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minInstances".into(),
                    value: &min_instances_binding.drop_type(),
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
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtime".into(),
                    value: &runtime_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretEnvironmentVariables".into(),
                    value: &secret_environment_variables_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretVolumes".into(),
                    value: &secret_volumes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccountEmail".into(),
                    value: &service_account_email_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceArchiveBucket".into(),
                    value: &source_archive_bucket_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceArchiveObject".into(),
                    value: &source_archive_object_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceRepository".into(),
                    value: &source_repository_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeout".into(),
                    value: &timeout_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggerHttp".into(),
                    value: &trigger_http_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConnector".into(),
                    value: &vpc_connector_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConnectorEgressSettings".into(),
                    value: &vpc_connector_egress_settings_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FunctionResult {
            available_memory_mb: o.get_field("availableMemoryMb"),
            build_environment_variables: o.get_field("buildEnvironmentVariables"),
            build_service_account: o.get_field("buildServiceAccount"),
            build_worker_pool: o.get_field("buildWorkerPool"),
            description: o.get_field("description"),
            docker_registry: o.get_field("dockerRegistry"),
            docker_repository: o.get_field("dockerRepository"),
            effective_labels: o.get_field("effectiveLabels"),
            entry_point: o.get_field("entryPoint"),
            environment_variables: o.get_field("environmentVariables"),
            event_trigger: o.get_field("eventTrigger"),
            https_trigger_security_level: o.get_field("httpsTriggerSecurityLevel"),
            https_trigger_url: o.get_field("httpsTriggerUrl"),
            ingress_settings: o.get_field("ingressSettings"),
            kms_key_name: o.get_field("kmsKeyName"),
            labels: o.get_field("labels"),
            max_instances: o.get_field("maxInstances"),
            min_instances: o.get_field("minInstances"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            runtime: o.get_field("runtime"),
            secret_environment_variables: o.get_field("secretEnvironmentVariables"),
            secret_volumes: o.get_field("secretVolumes"),
            service_account_email: o.get_field("serviceAccountEmail"),
            source_archive_bucket: o.get_field("sourceArchiveBucket"),
            source_archive_object: o.get_field("sourceArchiveObject"),
            source_repository: o.get_field("sourceRepository"),
            status: o.get_field("status"),
            timeout: o.get_field("timeout"),
            trigger_http: o.get_field("triggerHttp"),
            version_id: o.get_field("versionId"),
            vpc_connector: o.get_field("vpcConnector"),
            vpc_connector_egress_settings: o.get_field("vpcConnectorEgressSettings"),
        }
    }
}
