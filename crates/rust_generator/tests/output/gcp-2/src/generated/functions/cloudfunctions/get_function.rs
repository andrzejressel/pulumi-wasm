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
        context: &pulumi_gestalt_rust::Context,
        args: GetFunctionArgs,
    ) -> GetFunctionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:cloudfunctions/getFunction:getFunction".into(),
            version: super::super::super::get_version(),
            object: &[
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
            ],
        };
        let o = context.invoke_resource(request);
        GetFunctionResult {
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
            event_triggers: o.get_field("eventTriggers"),
            https_trigger_security_level: o.get_field("httpsTriggerSecurityLevel"),
            https_trigger_url: o.get_field("httpsTriggerUrl"),
            id: o.get_field("id"),
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
            source_repositories: o.get_field("sourceRepositories"),
            status: o.get_field("status"),
            timeout: o.get_field("timeout"),
            trigger_http: o.get_field("triggerHttp"),
            version_id: o.get_field("versionId"),
            vpc_connector: o.get_field("vpcConnector"),
            vpc_connector_egress_settings: o.get_field("vpcConnectorEgressSettings"),
        }
    }
}
