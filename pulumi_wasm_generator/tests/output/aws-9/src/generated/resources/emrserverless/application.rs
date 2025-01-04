/// Manages an EMR Serverless Application.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application::create(
///         "example",
///         ApplicationArgs::builder()
///             .name("example")
///             .release_label("emr-6.6.0")
///             .type_("hive")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Initial Capacity Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application::create(
///         "example",
///         ApplicationArgs::builder()
///             .initial_capacities(
///                 vec![
///                     ApplicationInitialCapacity::builder()
///                     .initialCapacityConfig(ApplicationInitialCapacityInitialCapacityConfig::builder()
///                     .workerConfiguration(ApplicationInitialCapacityInitialCapacityConfigWorkerConfiguration::builder()
///                     .cpu("2 vCPU").memory("10 GB").build_struct()).workerCount(1)
///                     .build_struct()).initialCapacityType("HiveDriver").build_struct(),
///                 ],
///             )
///             .name("example")
///             .release_label("emr-6.6.0")
///             .type_("hive")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Maximum Capacity Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application::create(
///         "example",
///         ApplicationArgs::builder()
///             .maximum_capacity(
///                 ApplicationMaximumCapacity::builder()
///                     .cpu("2 vCPU")
///                     .memory("10 GB")
///                     .build_struct(),
///             )
///             .name("example")
///             .release_label("emr-6.6.0")
///             .type_("hive")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EMR Severless applications using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:emrserverless/application:Application example id
/// ```
pub mod application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// The CPU architecture of an application. Valid values are `ARM64` or `X86_64`. Default value is `X86_64`.
        #[builder(into, default)]
        pub architecture: pulumi_wasm_rust::Output<Option<String>>,
        /// The configuration for an application to automatically start on job submission.
        #[builder(into, default)]
        pub auto_start_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::emrserverless::ApplicationAutoStartConfiguration>,
        >,
        /// The configuration for an application to automatically stop after a certain amount of time being idle.
        #[builder(into, default)]
        pub auto_stop_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::emrserverless::ApplicationAutoStopConfiguration>,
        >,
        /// The image configuration applied to all worker types.
        #[builder(into, default)]
        pub image_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::emrserverless::ApplicationImageConfiguration>,
        >,
        /// The capacity to initialize when the application is created.
        #[builder(into, default)]
        pub initial_capacities: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::emrserverless::ApplicationInitialCapacity>>,
        >,
        /// Enables the interactive use cases to use when running an application.
        #[builder(into, default)]
        pub interactive_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::emrserverless::ApplicationInteractiveConfiguration,
            >,
        >,
        /// The maximum capacity to allocate when the application is created. This is cumulative across all workers at any given point in time, not just when an application is created. No new resources will be created once any one of the defined limits is hit.
        #[builder(into, default)]
        pub maximum_capacity: pulumi_wasm_rust::Output<
            Option<super::super::types::emrserverless::ApplicationMaximumCapacity>,
        >,
        /// The name of the application.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The network configuration for customer VPC connectivity.
        #[builder(into, default)]
        pub network_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::emrserverless::ApplicationNetworkConfiguration>,
        >,
        /// The EMR release version associated with the application.
        #[builder(into)]
        pub release_label: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of application you want to start, such as `spark` or `hive`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// The CPU architecture of an application. Valid values are `ARM64` or `X86_64`. Default value is `X86_64`.
        pub architecture: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the cluster.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The configuration for an application to automatically start on job submission.
        pub auto_start_configuration: pulumi_wasm_rust::Output<
            super::super::types::emrserverless::ApplicationAutoStartConfiguration,
        >,
        /// The configuration for an application to automatically stop after a certain amount of time being idle.
        pub auto_stop_configuration: pulumi_wasm_rust::Output<
            super::super::types::emrserverless::ApplicationAutoStopConfiguration,
        >,
        /// The image configuration applied to all worker types.
        pub image_configuration: pulumi_wasm_rust::Output<
            super::super::types::emrserverless::ApplicationImageConfiguration,
        >,
        /// The capacity to initialize when the application is created.
        pub initial_capacities: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::emrserverless::ApplicationInitialCapacity>>,
        >,
        /// Enables the interactive use cases to use when running an application.
        pub interactive_configuration: pulumi_wasm_rust::Output<
            super::super::types::emrserverless::ApplicationInteractiveConfiguration,
        >,
        /// The maximum capacity to allocate when the application is created. This is cumulative across all workers at any given point in time, not just when an application is created. No new resources will be created once any one of the defined limits is hit.
        pub maximum_capacity: pulumi_wasm_rust::Output<
            super::super::types::emrserverless::ApplicationMaximumCapacity,
        >,
        /// The name of the application.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The network configuration for customer VPC connectivity.
        pub network_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::emrserverless::ApplicationNetworkConfiguration>,
        >,
        /// The EMR release version associated with the application.
        pub release_label: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of application you want to start, such as `spark` or `hive`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApplicationArgs) -> ApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let architecture_binding = args.architecture.get_inner();
        let auto_start_configuration_binding = args.auto_start_configuration.get_inner();
        let auto_stop_configuration_binding = args.auto_stop_configuration.get_inner();
        let image_configuration_binding = args.image_configuration.get_inner();
        let initial_capacities_binding = args.initial_capacities.get_inner();
        let interactive_configuration_binding = args
            .interactive_configuration
            .get_inner();
        let maximum_capacity_binding = args.maximum_capacity.get_inner();
        let name_binding = args.name.get_inner();
        let network_configuration_binding = args.network_configuration.get_inner();
        let release_label_binding = args.release_label.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:emrserverless/application:Application".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "architecture".into(),
                    value: &architecture_binding,
                },
                register_interface::ObjectField {
                    name: "autoStartConfiguration".into(),
                    value: &auto_start_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "autoStopConfiguration".into(),
                    value: &auto_stop_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "imageConfiguration".into(),
                    value: &image_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "initialCapacities".into(),
                    value: &initial_capacities_binding,
                },
                register_interface::ObjectField {
                    name: "interactiveConfiguration".into(),
                    value: &interactive_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "maximumCapacity".into(),
                    value: &maximum_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfiguration".into(),
                    value: &network_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "releaseLabel".into(),
                    value: &release_label_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "architecture".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoStartConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "autoStopConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "imageConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "initialCapacities".into(),
                },
                register_interface::ResultField {
                    name: "interactiveConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "maximumCapacity".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "releaseLabel".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationResult {
            architecture: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("architecture").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_start_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoStartConfiguration").unwrap(),
            ),
            auto_stop_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoStopConfiguration").unwrap(),
            ),
            image_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageConfiguration").unwrap(),
            ),
            initial_capacities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("initialCapacities").unwrap(),
            ),
            interactive_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interactiveConfiguration").unwrap(),
            ),
            maximum_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumCapacity").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkConfiguration").unwrap(),
            ),
            release_label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseLabel").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
