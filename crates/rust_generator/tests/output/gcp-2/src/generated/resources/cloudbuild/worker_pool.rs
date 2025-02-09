/// Definition of custom Cloud Build WorkerPools for running jobs with custom configuration and custom networking.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod worker_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkerPoolArgs {
        /// User specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size
        /// limitations. **Note**: This field is non-authoritative, and will only manage the annotations present in your
        /// configuration. Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User-defined name of the `WorkerPool`.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Network configuration for the `WorkerPool`. Structure is documented below.
        #[builder(into, default)]
        pub network_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuild::WorkerPoolNetworkConfig>,
        >,
        /// Private Service Connect configuration for the pool.
        #[builder(into, default)]
        pub private_service_connect: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuild::WorkerPoolPrivateServiceConnect>,
        >,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration to be used for a creating workers in the `WorkerPool`. Structure is documented below.
        #[builder(into, default)]
        pub worker_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudbuild::WorkerPoolWorkerConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkerPoolResult {
        /// User specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size
        /// limitations. **Note**: This field is non-authoritative, and will only manage the annotations present in your
        /// configuration. Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Output only. Time at which the request to create the `WorkerPool` was received.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Output only. Time at which the request to delete the `WorkerPool` was received.
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// A user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// User-defined name of the `WorkerPool`.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Network configuration for the `WorkerPool`. Structure is documented below.
        pub network_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuild::WorkerPoolNetworkConfig>,
        >,
        /// Private Service Connect configuration for the pool.
        pub private_service_connect: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudbuild::WorkerPoolPrivateServiceConnect>,
        >,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. WorkerPool state. Possible values: STATE_UNSPECIFIED, PENDING, APPROVED, REJECTED, CANCELLED
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Output only. A unique identifier for the `WorkerPool`.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. Time at which the request to update the `WorkerPool` was received.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Configuration to be used for a creating workers in the `WorkerPool`. Structure is documented below.
        pub worker_config: pulumi_gestalt_rust::Output<
            super::super::types::cloudbuild::WorkerPoolWorkerConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkerPoolArgs,
    ) -> WorkerPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_config_binding = args.network_config.get_output(context);
        let private_service_connect_binding = args
            .private_service_connect
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let worker_config_binding = args.worker_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:cloudbuild/workerPool:WorkerPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: annotations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkConfig".into(),
                    value: network_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateServiceConnect".into(),
                    value: private_service_connect_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workerConfig".into(),
                    value: worker_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkerPoolResult {
            annotations: o.get_field("annotations"),
            create_time: o.get_field("createTime"),
            delete_time: o.get_field("deleteTime"),
            display_name: o.get_field("displayName"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_config: o.get_field("networkConfig"),
            private_service_connect: o.get_field("privateServiceConnect"),
            project: o.get_field("project"),
            state: o.get_field("state"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            worker_config: o.get_field("workerConfig"),
        }
    }
}
