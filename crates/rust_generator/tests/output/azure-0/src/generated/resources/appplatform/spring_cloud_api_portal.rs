/// Manages a Spring Cloud API Portal.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleSpringCloudApiPortal = spring_cloud_api_portal::create(
///         "exampleSpringCloudApiPortal",
///         SpringCloudApiPortalArgs::builder()
///             .api_try_out_enabled(true)
///             .gateway_ids(vec!["${exampleSpringCloudGateway.id}",])
///             .https_only_enabled(false)
///             .instance_count(1)
///             .name("default")
///             .public_network_access_enabled(true)
///             .spring_cloud_service_id("${exampleSpringCloudService.id}")
///             .sso(
///                 SpringCloudApiPortalSso::builder()
///                     .clientId("test")
///                     .clientSecret("secret")
///                     .issuerUri("https://www.example.com/issueToken")
///                     .scopes(vec!["read",])
///                     .build_struct(),
///             )
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
/// Spring Cloud API Portals can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudApiPortal:SpringCloudApiPortal example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.AppPlatform/spring/service1/apiPortals/apiPortal1
/// ```
///
pub mod spring_cloud_api_portal {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudApiPortalArgs {
        /// Specifies whether the API try-out feature is enabled. When enabled, users can try out the API by sending requests and viewing responses in API portal.
        #[builder(into, default)]
        pub api_try_out_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies a list of Spring Cloud Gateway.
        #[builder(into, default)]
        pub gateway_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// is only https is allowed?
        #[builder(into, default)]
        pub https_only_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the required instance count of the Spring Cloud API Portal. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        #[builder(into, default)]
        pub instance_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name which should be used for this Spring Cloud API Portal. Changing this forces a new Spring Cloud API Portal to be created. The only possible value is `default`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Is the public network access enabled?
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud API Portal to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `sso` block as defined below.
        #[builder(into, default)]
        pub sso: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appplatform::SpringCloudApiPortalSso>,
        >,
    }
    #[allow(dead_code)]
    pub struct SpringCloudApiPortalResult {
        /// Specifies whether the API try-out feature is enabled. When enabled, users can try out the API by sending requests and viewing responses in API portal.
        pub api_try_out_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies a list of Spring Cloud Gateway.
        pub gateway_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// is only https is allowed?
        pub https_only_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the required instance count of the Spring Cloud API Portal. Possible Values are between `1` and `500`. Defaults to `1` if not specified.
        pub instance_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name which should be used for this Spring Cloud API Portal. Changing this forces a new Spring Cloud API Portal to be created. The only possible value is `default`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Is the public network access enabled?
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud API Portal to be created.
        pub spring_cloud_service_id: pulumi_gestalt_rust::Output<String>,
        /// A `sso` block as defined below.
        pub sso: pulumi_gestalt_rust::Output<
            Option<super::super::types::appplatform::SpringCloudApiPortalSso>,
        >,
        /// TODO.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SpringCloudApiPortalArgs,
    ) -> SpringCloudApiPortalResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_try_out_enabled_binding = args
            .api_try_out_enabled
            .get_output(context)
            .get_inner();
        let gateway_ids_binding = args.gateway_ids.get_output(context).get_inner();
        let https_only_enabled_binding = args
            .https_only_enabled
            .get_output(context)
            .get_inner();
        let instance_count_binding = args.instance_count.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context)
            .get_inner();
        let spring_cloud_service_id_binding = args
            .spring_cloud_service_id
            .get_output(context)
            .get_inner();
        let sso_binding = args.sso.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudApiPortal:SpringCloudApiPortal".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiTryOutEnabled".into(),
                    value: &api_try_out_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayIds".into(),
                    value: &gateway_ids_binding,
                },
                register_interface::ObjectField {
                    name: "httpsOnlyEnabled".into(),
                    value: &https_only_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudServiceId".into(),
                    value: &spring_cloud_service_id_binding,
                },
                register_interface::ObjectField {
                    name: "sso".into(),
                    value: &sso_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SpringCloudApiPortalResult {
            api_try_out_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiTryOutEnabled"),
            ),
            gateway_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayIds"),
            ),
            https_only_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpsOnlyEnabled"),
            ),
            instance_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceCount"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            public_network_access_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            spring_cloud_service_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("springCloudServiceId"),
            ),
            sso: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sso")),
            url: pulumi_gestalt_rust::__private::into_domain(o.extract_field("url")),
        }
    }
}
