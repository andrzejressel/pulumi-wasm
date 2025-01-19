/// Manages a Spring Cloud API Portal Domain.
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
///     let exampleSpringCloudApiPortal = spring_cloud_api_portal::create(
///         "exampleSpringCloudApiPortal",
///         SpringCloudApiPortalArgs::builder()
///             .gateway_ids(vec!["${exampleSpringCloudGateway.id}",])
///             .name("default")
///             .spring_cloud_service_id("${exampleSpringCloudService.id}")
///             .build_struct(),
///     );
///     let exampleSpringCloudApiPortalCustomDomain = spring_cloud_api_portal_custom_domain::create(
///         "exampleSpringCloudApiPortalCustomDomain",
///         SpringCloudApiPortalCustomDomainArgs::builder()
///             .name("example.com")
///             .spring_cloud_api_portal_id("${exampleSpringCloudApiPortal.id}")
///             .build_struct(),
///     );
///     let exampleSpringCloudGateway = spring_cloud_gateway::create(
///         "exampleSpringCloudGateway",
///         SpringCloudGatewayArgs::builder()
///             .name("default")
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
/// Spring Cloud API Portal Domains can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudApiPortalCustomDomain:SpringCloudApiPortalCustomDomain example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.AppPlatform/spring/service1/apiPortals/apiPortal1/domains/domain1
/// ```
///
pub mod spring_cloud_api_portal_custom_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudApiPortalCustomDomainArgs {
        /// The name which should be used for this Spring Cloud API Portal Domain. Changing this forces a new Spring Cloud API Portal Domain to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Spring Cloud API Portal. Changing this forces a new Spring Cloud API Portal Domain to be created.
        #[builder(into)]
        pub spring_cloud_api_portal_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the thumbprint of the Spring Cloud Certificate that binds to the Spring Cloud API Portal Domain.
        #[builder(into, default)]
        pub thumbprint: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudApiPortalCustomDomainResult {
        /// The name which should be used for this Spring Cloud API Portal Domain. Changing this forces a new Spring Cloud API Portal Domain to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Spring Cloud API Portal. Changing this forces a new Spring Cloud API Portal Domain to be created.
        pub spring_cloud_api_portal_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the thumbprint of the Spring Cloud Certificate that binds to the Spring Cloud API Portal Domain.
        pub thumbprint: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SpringCloudApiPortalCustomDomainArgs,
    ) -> SpringCloudApiPortalCustomDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let spring_cloud_api_portal_id_binding = args
            .spring_cloud_api_portal_id
            .get_inner();
        let thumbprint_binding = args.thumbprint.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudApiPortalCustomDomain:SpringCloudApiPortalCustomDomain"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudApiPortalId".into(),
                    value: &spring_cloud_api_portal_id_binding,
                },
                register_interface::ObjectField {
                    name: "thumbprint".into(),
                    value: &thumbprint_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "springCloudApiPortalId".into(),
                },
                register_interface::ResultField {
                    name: "thumbprint".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpringCloudApiPortalCustomDomainResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            spring_cloud_api_portal_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudApiPortalId").unwrap(),
            ),
            thumbprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thumbprint").unwrap(),
            ),
        }
    }
}
