/// Manages a Spring Cloud Builder.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleSpringCloudBuilder = spring_cloud_builder::create(
///         "exampleSpringCloudBuilder",
///         SpringCloudBuilderArgs::builder()
///             .build_pack_groups(
///                 vec![
///                     SpringCloudBuilderBuildPackGroup::builder()
///                     .buildPackIds(vec!["tanzu-buildpacks/java-azure",]).name("mix")
///                     .build_struct(),
///                 ],
///             )
///             .name("example")
///             .spring_cloud_service_id("${exampleSpringCloudService.id}")
///             .stack(
///                 SpringCloudBuilderStack::builder()
///                     .id("io.buildpacks.stacks.bionic")
///                     .version("base")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleSpringCloudService = spring_cloud_service::create(
///         "exampleSpringCloudService",
///         SpringCloudServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-springcloud")
///             .resource_group_name("${example.name}")
///             .sku_name("E0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Spring Cloud Builders can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudBuilder:SpringCloudBuilder example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resourceGroup1/providers/Microsoft.AppPlatform/spring/service1/buildServices/buildService1/builders/builder1
/// ```
///
pub mod spring_cloud_builder {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudBuilderArgs {
        /// One or more `build_pack_group` blocks as defined below.
        #[builder(into)]
        pub build_pack_groups: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::appplatform::SpringCloudBuilderBuildPackGroup>,
        >,
        /// The name which should be used for this Spring Cloud Builder. Changing this forces a new Spring Cloud Builder to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Builder to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A `stack` block as defined below.
        #[builder(into)]
        pub stack: pulumi_wasm_rust::InputOrOutput<
            super::super::types::appplatform::SpringCloudBuilderStack,
        >,
    }
    #[allow(dead_code)]
    pub struct SpringCloudBuilderResult {
        /// One or more `build_pack_group` blocks as defined below.
        pub build_pack_groups: pulumi_wasm_rust::Output<
            Vec<super::super::types::appplatform::SpringCloudBuilderBuildPackGroup>,
        >,
        /// The name which should be used for this Spring Cloud Builder. Changing this forces a new Spring Cloud Builder to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Builder to be created.
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
        /// A `stack` block as defined below.
        pub stack: pulumi_wasm_rust::Output<
            super::super::types::appplatform::SpringCloudBuilderStack,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SpringCloudBuilderArgs,
    ) -> SpringCloudBuilderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let build_pack_groups_binding = args
            .build_pack_groups
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let spring_cloud_service_id_binding = args
            .spring_cloud_service_id
            .get_output(context)
            .get_inner();
        let stack_binding = args.stack.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudBuilder:SpringCloudBuilder".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "buildPackGroups".into(),
                    value: &build_pack_groups_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudServiceId".into(),
                    value: &spring_cloud_service_id_binding,
                },
                register_interface::ObjectField {
                    name: "stack".into(),
                    value: &stack_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "buildPackGroups".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "springCloudServiceId".into(),
                },
                register_interface::ResultField {
                    name: "stack".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpringCloudBuilderResult {
            build_pack_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("buildPackGroups").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            spring_cloud_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudServiceId").unwrap(),
            ),
            stack: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stack").unwrap(),
            ),
        }
    }
}
