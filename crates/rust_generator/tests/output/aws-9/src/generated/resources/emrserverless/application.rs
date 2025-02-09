/// Manages an EMR Serverless Application.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// The CPU architecture of an application. Valid values are `ARM64` or `X86_64`. Default value is `X86_64`.
        #[builder(into, default)]
        pub architecture: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The configuration for an application to automatically start on job submission.
        #[builder(into, default)]
        pub auto_start_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::emrserverless::ApplicationAutoStartConfiguration>,
        >,
        /// The configuration for an application to automatically stop after a certain amount of time being idle.
        #[builder(into, default)]
        pub auto_stop_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::emrserverless::ApplicationAutoStopConfiguration>,
        >,
        /// The image configuration applied to all worker types.
        #[builder(into, default)]
        pub image_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::emrserverless::ApplicationImageConfiguration>,
        >,
        /// The capacity to initialize when the application is created.
        #[builder(into, default)]
        pub initial_capacities: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::emrserverless::ApplicationInitialCapacity>>,
        >,
        /// Enables the interactive use cases to use when running an application.
        #[builder(into, default)]
        pub interactive_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::emrserverless::ApplicationInteractiveConfiguration,
            >,
        >,
        /// The maximum capacity to allocate when the application is created. This is cumulative across all workers at any given point in time, not just when an application is created. No new resources will be created once any one of the defined limits is hit.
        #[builder(into, default)]
        pub maximum_capacity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::emrserverless::ApplicationMaximumCapacity>,
        >,
        /// The name of the application.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The network configuration for customer VPC connectivity.
        #[builder(into, default)]
        pub network_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::emrserverless::ApplicationNetworkConfiguration>,
        >,
        /// The EMR release version associated with the application.
        #[builder(into)]
        pub release_label: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of application you want to start, such as `spark` or `hive`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// The CPU architecture of an application. Valid values are `ARM64` or `X86_64`. Default value is `X86_64`.
        pub architecture: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the cluster.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The configuration for an application to automatically start on job submission.
        pub auto_start_configuration: pulumi_gestalt_rust::Output<
            super::super::types::emrserverless::ApplicationAutoStartConfiguration,
        >,
        /// The configuration for an application to automatically stop after a certain amount of time being idle.
        pub auto_stop_configuration: pulumi_gestalt_rust::Output<
            super::super::types::emrserverless::ApplicationAutoStopConfiguration,
        >,
        /// The image configuration applied to all worker types.
        pub image_configuration: pulumi_gestalt_rust::Output<
            super::super::types::emrserverless::ApplicationImageConfiguration,
        >,
        /// The capacity to initialize when the application is created.
        pub initial_capacities: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::emrserverless::ApplicationInitialCapacity>>,
        >,
        /// Enables the interactive use cases to use when running an application.
        pub interactive_configuration: pulumi_gestalt_rust::Output<
            super::super::types::emrserverless::ApplicationInteractiveConfiguration,
        >,
        /// The maximum capacity to allocate when the application is created. This is cumulative across all workers at any given point in time, not just when an application is created. No new resources will be created once any one of the defined limits is hit.
        pub maximum_capacity: pulumi_gestalt_rust::Output<
            super::super::types::emrserverless::ApplicationMaximumCapacity,
        >,
        /// The name of the application.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The network configuration for customer VPC connectivity.
        pub network_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::emrserverless::ApplicationNetworkConfiguration>,
        >,
        /// The EMR release version associated with the application.
        pub release_label: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of application you want to start, such as `spark` or `hive`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let architecture_binding = args.architecture.get_output(context);
        let auto_start_configuration_binding = args
            .auto_start_configuration
            .get_output(context);
        let auto_stop_configuration_binding = args
            .auto_stop_configuration
            .get_output(context);
        let image_configuration_binding = args.image_configuration.get_output(context);
        let initial_capacities_binding = args.initial_capacities.get_output(context);
        let interactive_configuration_binding = args
            .interactive_configuration
            .get_output(context);
        let maximum_capacity_binding = args.maximum_capacity.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_configuration_binding = args
            .network_configuration
            .get_output(context);
        let release_label_binding = args.release_label.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:emrserverless/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "architecture".into(),
                    value: architecture_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoStartConfiguration".into(),
                    value: auto_start_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoStopConfiguration".into(),
                    value: auto_stop_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageConfiguration".into(),
                    value: image_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "initialCapacities".into(),
                    value: initial_capacities_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interactiveConfiguration".into(),
                    value: interactive_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maximumCapacity".into(),
                    value: maximum_capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkConfiguration".into(),
                    value: network_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "releaseLabel".into(),
                    value: release_label_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationResult {
            architecture: o.get_field("architecture"),
            arn: o.get_field("arn"),
            auto_start_configuration: o.get_field("autoStartConfiguration"),
            auto_stop_configuration: o.get_field("autoStopConfiguration"),
            image_configuration: o.get_field("imageConfiguration"),
            initial_capacities: o.get_field("initialCapacities"),
            interactive_configuration: o.get_field("interactiveConfiguration"),
            maximum_capacity: o.get_field("maximumCapacity"),
            name: o.get_field("name"),
            network_configuration: o.get_field("networkConfiguration"),
            release_label: o.get_field("releaseLabel"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
        }
    }
}
