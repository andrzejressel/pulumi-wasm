pub mod get_function {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFunctionArgs {
        /// The name of a Cloud Function.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region in which the resource belongs. If it
        /// is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetFunctionResult {
        /// Available memory (in MB) to the function.
        pub available_memory_mb: pulumi_wasm_rust::Output<i32>,
        pub build_environment_variables: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub build_service_account: pulumi_wasm_rust::Output<String>,
        pub build_worker_pool: pulumi_wasm_rust::Output<String>,
        /// Description of the function.
        pub description: pulumi_wasm_rust::Output<String>,
        pub docker_registry: pulumi_wasm_rust::Output<String>,
        pub docker_repository: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Name of a JavaScript function that will be executed when the Google Cloud Function is triggered.
        pub entry_point: pulumi_wasm_rust::Output<String>,
        pub environment_variables: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A source that fires events in response to a condition in another service. Structure is documented below.
        pub event_triggers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudfunctions::GetFunctionEventTrigger>,
        >,
        pub https_trigger_security_level: pulumi_wasm_rust::Output<String>,
        /// If function is triggered by HTTP, trigger URL is set here.
        pub https_trigger_url: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Controls what traffic can reach the function.
        pub ingress_settings: pulumi_wasm_rust::Output<String>,
        pub kms_key_name: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The limit on the maximum number of function instances that may coexist at a given time. If unset or set to `0`, the API default will be used.
        pub max_instances: pulumi_wasm_rust::Output<i32>,
        pub min_instances: pulumi_wasm_rust::Output<i32>,
        /// The name of the Cloud Function.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The runtime in which the function is running.
        pub runtime: pulumi_wasm_rust::Output<String>,
        pub secret_environment_variables: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::cloudfunctions::GetFunctionSecretEnvironmentVariable,
            >,
        >,
        pub secret_volumes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudfunctions::GetFunctionSecretVolume>,
        >,
        /// The service account email to be assumed by the cloud function.
        pub service_account_email: pulumi_wasm_rust::Output<String>,
        /// The GCS bucket containing the zip archive which contains the function.
        pub source_archive_bucket: pulumi_wasm_rust::Output<String>,
        /// The source archive object (file) in archive bucket.
        pub source_archive_object: pulumi_wasm_rust::Output<String>,
        /// The URL of the Cloud Source Repository that the function is deployed from. Structure is documented below.
        pub source_repositories: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudfunctions::GetFunctionSourceRepository>,
        >,
        pub status: pulumi_wasm_rust::Output<String>,
        /// Function execution timeout (in seconds).
        pub timeout: pulumi_wasm_rust::Output<i32>,
        /// If function is triggered by HTTP, this boolean is set.
        pub trigger_http: pulumi_wasm_rust::Output<bool>,
        pub version_id: pulumi_wasm_rust::Output<String>,
        /// The VPC Network Connector that this cloud function can connect to.
        pub vpc_connector: pulumi_wasm_rust::Output<String>,
        /// The egress settings for the connector, controlling what traffic is diverted through it.
        pub vpc_connector_egress_settings: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetFunctionArgs) -> GetFunctionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudfunctions/getFunction:getFunction".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "availableMemoryMb".into(),
                },
                register_interface::ResultField {
                    name: "buildEnvironmentVariables".into(),
                },
                register_interface::ResultField {
                    name: "buildServiceAccount".into(),
                },
                register_interface::ResultField {
                    name: "buildWorkerPool".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "dockerRegistry".into(),
                },
                register_interface::ResultField {
                    name: "dockerRepository".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "entryPoint".into(),
                },
                register_interface::ResultField {
                    name: "environmentVariables".into(),
                },
                register_interface::ResultField {
                    name: "eventTriggers".into(),
                },
                register_interface::ResultField {
                    name: "httpsTriggerSecurityLevel".into(),
                },
                register_interface::ResultField {
                    name: "httpsTriggerUrl".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ingressSettings".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyName".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "maxInstances".into(),
                },
                register_interface::ResultField {
                    name: "minInstances".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
                    name: "runtime".into(),
                },
                register_interface::ResultField {
                    name: "secretEnvironmentVariables".into(),
                },
                register_interface::ResultField {
                    name: "secretVolumes".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccountEmail".into(),
                },
                register_interface::ResultField {
                    name: "sourceArchiveBucket".into(),
                },
                register_interface::ResultField {
                    name: "sourceArchiveObject".into(),
                },
                register_interface::ResultField {
                    name: "sourceRepositories".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "timeout".into(),
                },
                register_interface::ResultField {
                    name: "triggerHttp".into(),
                },
                register_interface::ResultField {
                    name: "versionId".into(),
                },
                register_interface::ResultField {
                    name: "vpcConnector".into(),
                },
                register_interface::ResultField {
                    name: "vpcConnectorEgressSettings".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFunctionResult {
            available_memory_mb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availableMemoryMb").unwrap(),
            ),
            build_environment_variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("buildEnvironmentVariables").unwrap(),
            ),
            build_service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("buildServiceAccount").unwrap(),
            ),
            build_worker_pool: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("buildWorkerPool").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            docker_registry: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dockerRegistry").unwrap(),
            ),
            docker_repository: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dockerRepository").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            entry_point: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entryPoint").unwrap(),
            ),
            environment_variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentVariables").unwrap(),
            ),
            event_triggers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventTriggers").unwrap(),
            ),
            https_trigger_security_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsTriggerSecurityLevel").unwrap(),
            ),
            https_trigger_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsTriggerUrl").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ingress_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingressSettings").unwrap(),
            ),
            kms_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyName").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            max_instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxInstances").unwrap(),
            ),
            min_instances: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minInstances").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
            runtime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtime").unwrap(),
            ),
            secret_environment_variables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretEnvironmentVariables").unwrap(),
            ),
            secret_volumes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretVolumes").unwrap(),
            ),
            service_account_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccountEmail").unwrap(),
            ),
            source_archive_bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceArchiveBucket").unwrap(),
            ),
            source_archive_object: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceArchiveObject").unwrap(),
            ),
            source_repositories: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceRepositories").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeout").unwrap(),
            ),
            trigger_http: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggerHttp").unwrap(),
            ),
            version_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionId").unwrap(),
            ),
            vpc_connector: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConnector").unwrap(),
            ),
            vpc_connector_egress_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConnectorEgressSettings").unwrap(),
            ),
        }
    }
}
