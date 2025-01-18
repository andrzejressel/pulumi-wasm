/// Definition of custom Cloud Build WorkerPools for running jobs with custom configuration and custom networking.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pool = worker_pool::create(
///         "pool",
///         WorkerPoolArgs::builder()
///             .location("europe-west1")
///             .name("my-pool")
///             .worker_config(
///                 WorkerPoolWorkerConfig::builder()
///                     .diskSizeGb(100)
///                     .machineType("e2-standard-4")
///                     .noExternalIp(false)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Network Config
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let network = network::create(
///         "network",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-network")
///             .build_struct(),
///     );
///     let pool = worker_pool::create(
///         "pool",
///         WorkerPoolArgs::builder()
///             .location("europe-west1")
///             .name("my-pool")
///             .network_config(
///                 WorkerPoolNetworkConfig::builder()
///                     .peeredNetwork("${network.id}")
///                     .peeredNetworkIpRange("/29")
///                     .build_struct(),
///             )
///             .worker_config(
///                 WorkerPoolWorkerConfig::builder()
///                     .diskSizeGb(100)
///                     .machineType("e2-standard-4")
///                     .noExternalIp(false)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let servicenetworking = service::create(
///         "servicenetworking",
///         ServiceArgs::builder()
///             .disable_on_destroy(false)
///             .service("servicenetworking.googleapis.com")
///             .build_struct(),
///     );
///     let workerPoolConn = connection::create(
///         "workerPoolConn",
///         ConnectionArgs::builder()
///             .network("${network.id}")
///             .reserved_peering_ranges(vec!["${workerRange.name}",])
///             .service("servicenetworking.googleapis.com")
///             .build_struct(),
///     );
///     let workerRange = global_address::create(
///         "workerRange",
///         GlobalAddressArgs::builder()
///             .address_type("INTERNAL")
///             .name("worker-pool-range")
///             .network("${network.id}")
///             .prefix_length(16)
///             .purpose("VPC_PEERING")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// WorkerPool can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/workerPools/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, WorkerPool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudbuild/workerPool:WorkerPool default projects/{{project}}/locations/{{location}}/workerPools/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuild/workerPool:WorkerPool default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudbuild/workerPool:WorkerPool default {{location}}/{{name}}
/// ```
///
pub mod worker_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkerPoolArgs {
        /// User specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size
        /// limitations. **Note**: This field is non-authoritative, and will only manage the annotations present in your
        /// configuration. Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// User-defined name of the `WorkerPool`.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Network configuration for the `WorkerPool`. Structure is documented below.
        #[builder(into, default)]
        pub network_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudbuild::WorkerPoolNetworkConfig>,
        >,
        /// Private Service Connect configuration for the pool.
        #[builder(into, default)]
        pub private_service_connect: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudbuild::WorkerPoolPrivateServiceConnect>,
        >,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration to be used for a creating workers in the `WorkerPool`. Structure is documented below.
        #[builder(into, default)]
        pub worker_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudbuild::WorkerPoolWorkerConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkerPoolResult {
        /// User specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size
        /// limitations. **Note**: This field is non-authoritative, and will only manage the annotations present in your
        /// configuration. Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Output only. Time at which the request to create the `WorkerPool` was received.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Output only. Time at which the request to delete the `WorkerPool` was received.
        pub delete_time: pulumi_wasm_rust::Output<String>,
        /// A user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// User-defined name of the `WorkerPool`.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// Network configuration for the `WorkerPool`. Structure is documented below.
        pub network_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudbuild::WorkerPoolNetworkConfig>,
        >,
        /// Private Service Connect configuration for the pool.
        pub private_service_connect: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudbuild::WorkerPoolPrivateServiceConnect>,
        >,
        /// The project for the resource
        pub project: pulumi_wasm_rust::Output<String>,
        /// Output only. WorkerPool state. Possible values: STATE_UNSPECIFIED, PENDING, APPROVED, REJECTED, CANCELLED
        pub state: pulumi_wasm_rust::Output<String>,
        /// Output only. A unique identifier for the `WorkerPool`.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. Time at which the request to update the `WorkerPool` was received.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// Configuration to be used for a creating workers in the `WorkerPool`. Structure is documented below.
        pub worker_config: pulumi_wasm_rust::Output<
            super::super::types::cloudbuild::WorkerPoolWorkerConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkerPoolArgs) -> WorkerPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let network_config_binding = args.network_config.get_inner();
        let private_service_connect_binding = args.private_service_connect.get_inner();
        let project_binding = args.project.get_inner();
        let worker_config_binding = args.worker_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:cloudbuild/workerPool:WorkerPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
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
                    name: "networkConfig".into(),
                    value: &network_config_binding,
                },
                register_interface::ObjectField {
                    name: "privateServiceConnect".into(),
                    value: &private_service_connect_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "workerConfig".into(),
                    value: &worker_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deleteTime".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkConfig".into(),
                },
                register_interface::ResultField {
                    name: "privateServiceConnect".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "workerConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkerPoolResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteTime").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkConfig").unwrap(),
            ),
            private_service_connect: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateServiceConnect").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            worker_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workerConfig").unwrap(),
            ),
        }
    }
}
