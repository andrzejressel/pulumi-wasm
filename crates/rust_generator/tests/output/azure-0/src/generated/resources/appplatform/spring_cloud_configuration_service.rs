/// Manages a Spring Cloud Configuration Service.
///
/// > **NOTE:** This resource is applicable only for Spring Cloud Service with enterprise tier.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleSpringCloudConfigurationService = spring_cloud_configuration_service::create(
///         "exampleSpringCloudConfigurationService",
///         SpringCloudConfigurationServiceArgs::builder()
///             .name("default")
///             .repositories(
///                 vec![
///                     SpringCloudConfigurationServiceRepository::builder().label("master")
///                     .name("fake").password("H@Sh1CoR3!").patterns(vec!["app/dev",])
///                     .searchPaths(vec!["dir1", "dir2",]).strictHostKeyChecking(false)
///                     .uri("https://github.com/Azure-Samples/piggymetrics")
///                     .username("adminuser").build_struct(),
///                 ],
///             )
///             .spring_cloud_service_id("${exampleSpringCloudService.id}")
///             .build_struct(),
///     );
///     let exampleSpringCloudService = spring_cloud_service::create(
///         "exampleSpringCloudService",
///         SpringCloudServiceArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .sku_name("E0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Spring Cloud Configuration Services can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudConfigurationService:SpringCloudConfigurationService example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resourceGroup1/providers/Microsoft.AppPlatform/spring/service1/configurationServices/configurationService1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_configuration_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudConfigurationServiceArgs {
        /// The generation of the Spring Cloud Configuration Service. Possible values are `Gen1` and `Gen2`.
        #[builder(into, default)]
        pub generation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Spring Cloud Configuration Service. The only possible value is `default`. Changing this forces a new Spring Cloud Configuration Service to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies how often to check repository updates. Minimum value is 0.
        #[builder(into, default)]
        pub refresh_interval_in_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// One or more `repository` blocks as defined below.
        #[builder(into, default)]
        pub repositories: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::appplatform::SpringCloudConfigurationServiceRepository,
                >,
            >,
        >,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Configuration Service to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudConfigurationServiceResult {
        /// The generation of the Spring Cloud Configuration Service. Possible values are `Gen1` and `Gen2`.
        pub generation: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Spring Cloud Configuration Service. The only possible value is `default`. Changing this forces a new Spring Cloud Configuration Service to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies how often to check repository updates. Minimum value is 0.
        pub refresh_interval_in_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// One or more `repository` blocks as defined below.
        pub repositories: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::appplatform::SpringCloudConfigurationServiceRepository,
                >,
            >,
        >,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Configuration Service to be created.
        pub spring_cloud_service_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpringCloudConfigurationServiceArgs,
    ) -> SpringCloudConfigurationServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let generation_binding = args.generation.get_output(context);
        let name_binding = args.name.get_output(context);
        let refresh_interval_in_seconds_binding = args
            .refresh_interval_in_seconds
            .get_output(context);
        let repositories_binding = args.repositories.get_output(context);
        let spring_cloud_service_id_binding = args
            .spring_cloud_service_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudConfigurationService:SpringCloudConfigurationService"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "generation".into(),
                    value: generation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "refreshIntervalInSeconds".into(),
                    value: refresh_interval_in_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositories".into(),
                    value: repositories_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "springCloudServiceId".into(),
                    value: spring_cloud_service_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpringCloudConfigurationServiceResult {
            generation: o.get_field("generation"),
            name: o.get_field("name"),
            refresh_interval_in_seconds: o.get_field("refreshIntervalInSeconds"),
            repositories: o.get_field("repositories"),
            spring_cloud_service_id: o.get_field("springCloudServiceId"),
        }
    }
}
