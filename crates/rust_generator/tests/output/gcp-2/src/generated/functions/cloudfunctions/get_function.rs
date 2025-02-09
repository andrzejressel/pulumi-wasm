#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_function {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFunctionArgs {
        /// The name of a Cloud Function.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region in which the resource belongs. If it
        /// is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetFunctionResult {
        /// Available memory (in MB) to the function.
        pub available_memory_mb: pulumi_gestalt_rust::Output<i32>,
        pub build_environment_variables: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub build_service_account: pulumi_gestalt_rust::Output<String>,
        pub build_worker_pool: pulumi_gestalt_rust::Output<String>,
        /// Description of the function.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub docker_registry: pulumi_gestalt_rust::Output<String>,
        pub docker_repository: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Name of a JavaScript function that will be executed when the Google Cloud Function is triggered.
        pub entry_point: pulumi_gestalt_rust::Output<String>,
        pub environment_variables: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A source that fires events in response to a condition in another service. Structure is documented below.
        pub event_triggers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudfunctions::GetFunctionEventTrigger>,
        >,
        pub https_trigger_security_level: pulumi_gestalt_rust::Output<String>,
        /// If function is triggered by HTTP, trigger URL is set here.
        pub https_trigger_url: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Controls what traffic can reach the function.
        pub ingress_settings: pulumi_gestalt_rust::Output<String>,
        pub kms_key_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The limit on the maximum number of function instances that may coexist at a given time. If unset or set to `0`, the API default will be used.
        pub max_instances: pulumi_gestalt_rust::Output<i32>,
        pub min_instances: pulumi_gestalt_rust::Output<i32>,
        /// The name of the Cloud Function.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The runtime in which the function is running.
        pub runtime: pulumi_gestalt_rust::Output<String>,
        pub secret_environment_variables: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudfunctions::GetFunctionSecretEnvironmentVariable,
            >,
        >,
        pub secret_volumes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudfunctions::GetFunctionSecretVolume>,
        >,
        /// The service account email to be assumed by the cloud function.
        pub service_account_email: pulumi_gestalt_rust::Output<String>,
        /// The GCS bucket containing the zip archive which contains the function.
        pub source_archive_bucket: pulumi_gestalt_rust::Output<String>,
        /// The source archive object (file) in archive bucket.
        pub source_archive_object: pulumi_gestalt_rust::Output<String>,
        /// The URL of the Cloud Source Repository that the function is deployed from. Structure is documented below.
        pub source_repositories: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudfunctions::GetFunctionSourceRepository>,
        >,
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Function execution timeout (in seconds).
        pub timeout: pulumi_gestalt_rust::Output<i32>,
        /// If function is triggered by HTTP, this boolean is set.
        pub trigger_http: pulumi_gestalt_rust::Output<bool>,
        pub version_id: pulumi_gestalt_rust::Output<String>,
        /// The VPC Network Connector that this cloud function can connect to.
        pub vpc_connector: pulumi_gestalt_rust::Output<String>,
        /// The egress settings for the connector, controlling what traffic is diverted through it.
        pub vpc_connector_egress_settings: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetFunctionArgs,
    ) -> GetFunctionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudfunctions/getFunction:getFunction".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFunctionResult {
            available_memory_mb: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availableMemoryMb"),
            ),
            build_environment_variables: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("buildEnvironmentVariables"),
            ),
            build_service_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("buildServiceAccount"),
            ),
            build_worker_pool: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("buildWorkerPool"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            docker_registry: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dockerRegistry"),
            ),
            docker_repository: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dockerRepository"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            entry_point: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("entryPoint"),
            ),
            environment_variables: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environmentVariables"),
            ),
            event_triggers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventTriggers"),
            ),
            https_trigger_security_level: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpsTriggerSecurityLevel"),
            ),
            https_trigger_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpsTriggerUrl"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ingress_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ingressSettings"),
            ),
            kms_key_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyName"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            max_instances: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxInstances"),
            ),
            min_instances: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minInstances"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            runtime: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runtime"),
            ),
            secret_environment_variables: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretEnvironmentVariables"),
            ),
            secret_volumes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretVolumes"),
            ),
            service_account_email: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccountEmail"),
            ),
            source_archive_bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceArchiveBucket"),
            ),
            source_archive_object: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceArchiveObject"),
            ),
            source_repositories: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceRepositories"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            timeout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeout"),
            ),
            trigger_http: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("triggerHttp"),
            ),
            version_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionId"),
            ),
            vpc_connector: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcConnector"),
            ),
            vpc_connector_egress_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcConnectorEgressSettings"),
            ),
        }
    }
}
