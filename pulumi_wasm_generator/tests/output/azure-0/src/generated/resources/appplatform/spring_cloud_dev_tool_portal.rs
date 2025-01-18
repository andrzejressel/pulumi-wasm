/// > **NOTE:** This resource is applicable only for Spring Cloud Service with enterprise tier.
///
/// Manages a Spring Cloud Dev Tool Portal.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example
///       location: West Europe
///   exampleSpringCloudService:
///     type: azure:appplatform:SpringCloudService
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: E0
///   exampleSpringCloudDevToolPortal:
///     type: azure:appplatform:SpringCloudDevToolPortal
///     name: example
///     properties:
///       name: default
///       springCloudServiceId: ${exampleSpringCloudService.id}
///       publicNetworkAccessEnabled: true
///       sso:
///         clientId: example id
///         clientSecret: example secret
///         metadataUrl: https://login.microsoftonline.com/${current.tenantId}/v2.0/.well-known/openid-configuration
///         scopes:
///           - openid
///           - profile
///           - email
///       applicationAcceleratorEnabled: true
///       applicationLiveViewEnabled: true
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Spring Cloud Dev Tool Portals can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudDevToolPortal:SpringCloudDevToolPortal example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resourceGroup1/providers/Microsoft.AppPlatform/Spring/service1/DevToolPortals/default
/// ```
///
pub mod spring_cloud_dev_tool_portal {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudDevToolPortalArgs {
        /// Should the Accelerator plugin be enabled?
        #[builder(into, default)]
        pub application_accelerator_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Should the Application Live View be enabled?
        #[builder(into, default)]
        pub application_live_view_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name which should be used for this Spring Cloud Dev Tool Portal. The only possible value is `default`. Changing this forces a new Spring Cloud Dev Tool Portal to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Is public network access enabled?
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Dev Tool Portal to be created.
        #[builder(into)]
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
        /// A `sso` block as defined below.
        #[builder(into, default)]
        pub sso: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudDevToolPortalSso>,
        >,
    }
    #[allow(dead_code)]
    pub struct SpringCloudDevToolPortalResult {
        /// Should the Accelerator plugin be enabled?
        pub application_accelerator_enabled: pulumi_wasm_rust::Output<bool>,
        /// Should the Application Live View be enabled?
        pub application_live_view_enabled: pulumi_wasm_rust::Output<bool>,
        /// The name which should be used for this Spring Cloud Dev Tool Portal. The only possible value is `default`. Changing this forces a new Spring Cloud Dev Tool Portal to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Is public network access enabled?
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Spring Cloud Service. Changing this forces a new Spring Cloud Dev Tool Portal to be created.
        pub spring_cloud_service_id: pulumi_wasm_rust::Output<String>,
        /// A `sso` block as defined below.
        pub sso: pulumi_wasm_rust::Output<
            Option<super::super::types::appplatform::SpringCloudDevToolPortalSso>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SpringCloudDevToolPortalArgs,
    ) -> SpringCloudDevToolPortalResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_accelerator_enabled_binding = args
            .application_accelerator_enabled
            .get_inner();
        let application_live_view_enabled_binding = args
            .application_live_view_enabled
            .get_inner();
        let name_binding = args.name.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let spring_cloud_service_id_binding = args.spring_cloud_service_id.get_inner();
        let sso_binding = args.sso.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudDevToolPortal:SpringCloudDevToolPortal"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationAcceleratorEnabled".into(),
                    value: &application_accelerator_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "applicationLiveViewEnabled".into(),
                    value: &application_live_view_enabled_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationAcceleratorEnabled".into(),
                },
                register_interface::ResultField {
                    name: "applicationLiveViewEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "springCloudServiceId".into(),
                },
                register_interface::ResultField {
                    name: "sso".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SpringCloudDevToolPortalResult {
            application_accelerator_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationAcceleratorEnabled").unwrap(),
            ),
            application_live_view_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationLiveViewEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            spring_cloud_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("springCloudServiceId").unwrap(),
            ),
            sso: pulumi_wasm_rust::__private::into_domain(hashmap.remove("sso").unwrap()),
        }
    }
}
