/// Manages a Spring Cloud Configuration Service.
///
/// > **NOTE:** This resource is applicable only for Spring Cloud Service with enterprise tier.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod spring_cloud_configuration_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudConfigurationServiceArgs {
        /// The generation of the Spring Cloud Configuration Service. Possible values are `Gen1` and `Gen2`.
        #[builder(into, default)]
        pub generation: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Spring Cloud Configuration Service. The only possible value is `default`. Changing this forces a new Spring Cloud Configuration Service to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies how often to check repository updates. Minimum value is 0.
        #[builder(into, default)]
        pub refresh_interval_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// One or more `repository` blocks as defined below.
        #[builder(into, default)]
        pub repositories: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::appplatform::SpringCloudConfigurationServiceRepository,
                >,
            >,
        >,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Configuration Service to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudConfigurationServiceResult {
        /// The generation of the Spring Cloud Configuration Service. Possible values are `Gen1` and `Gen2`.
        pub generation: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Spring Cloud Configuration Service. The only possible value is `default`. Changing this forces a new Spring Cloud Configuration Service to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies how often to check repository updates. Minimum value is 0.
        pub refresh_interval_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// One or more `repository` blocks as defined below.
        pub repositories: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::appplatform::SpringCloudConfigurationServiceRepository,
                >,
            >,
        >,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Configuration Service to be created.
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SpringCloudConfigurationServiceArgs,
    ) -> SpringCloudConfigurationServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let generation_binding = args.generation.get_inner();
        let name_binding = args.name.get_inner();
        let refresh_interval_in_seconds_binding = args
            .refresh_interval_in_seconds
            .get_inner();
        let repositories_binding = args.repositories.get_inner();
        let spring_cloud_service_id_binding = args.spring_cloud_service_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudConfigurationService:SpringCloudConfigurationService"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "generation".into(),
                    value: &generation_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "refreshIntervalInSeconds".into(),
                    value: &refresh_interval_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "repositories".into(),
                    value: &repositories_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudServiceId".into(),
                    value: &spring_cloud_service_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "generation".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "refreshIntervalInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "repositories".into(),
                },
                register_interface::ResultField {
                    name: "springCloudServiceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpringCloudConfigurationServiceResult {
            generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("generation").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            refresh_interval_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("refreshIntervalInSeconds").unwrap(),
            ),
            repositories: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositories").unwrap(),
            ),
            spring_cloud_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudServiceId").unwrap(),
            ),
        }
    }
}