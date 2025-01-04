/// Manages an Azure VMware Solution ExpressRoute Circuit Authorization.
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
///     let exampleExpressRouteAuthorization = express_route_authorization::create(
///         "exampleExpressRouteAuthorization",
///         ExpressRouteAuthorizationArgs::builder()
///             .name("example-authorization")
///             .private_cloud_id("${examplePrivateCloud.id}")
///             .build_struct(),
///     );
///     let examplePrivateCloud = private_cloud::create(
///         "examplePrivateCloud",
///         PrivateCloudArgs::builder()
///             .internet_connection_enabled(false)
///             .location("${example.location}")
///             .management_cluster(
///                 PrivateCloudManagementCluster::builder().size(3).build_struct(),
///             )
///             .name("example-vmware-private-cloud")
///             .network_subnet_cidr("192.168.48.0/22")
///             .nsxt_password("QazWsx13$Edc")
///             .resource_group_name("${example.name}")
///             .sku_name("av36")
///             .vcenter_password("WsxEdc23$Rfv")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Azure VMware Solution ExpressRoute Circuit Authorizations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:avs/expressRouteAuthorization:ExpressRouteAuthorization example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.AVS/privateClouds/privateCloud1/authorizations/authorization1
/// ```
///
pub mod express_route_authorization {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRouteAuthorizationArgs {
        /// The name which should be used for this Azure VMware Solution ExpressRoute Circuit Authorization. Changing this forces a new Azure VMware Solution ExpressRoute Circuit Authorization to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Azure VMware Solution Private Cloud in which to create this Azure VMware Solution ExpressRoute Circuit Authorization. Changing this forces a new Azure VMware Solution ExpressRoute Circuit Authorization to be created.
        #[builder(into)]
        pub private_cloud_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ExpressRouteAuthorizationResult {
        /// The ID of the Azure VMware Solution ExpressRoute Circuit Authorization.
        pub express_route_authorization_id: pulumi_wasm_rust::Output<String>,
        /// The key of the Azure VMware Solution ExpressRoute Circuit Authorization.
        pub express_route_authorization_key: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Azure VMware Solution ExpressRoute Circuit Authorization. Changing this forces a new Azure VMware Solution ExpressRoute Circuit Authorization to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Azure VMware Solution Private Cloud in which to create this Azure VMware Solution ExpressRoute Circuit Authorization. Changing this forces a new Azure VMware Solution ExpressRoute Circuit Authorization to be created.
        pub private_cloud_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ExpressRouteAuthorizationArgs,
    ) -> ExpressRouteAuthorizationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let private_cloud_id_binding = args.private_cloud_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:avs/expressRouteAuthorization:ExpressRouteAuthorization"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "privateCloudId".into(),
                    value: &private_cloud_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "expressRouteAuthorizationId".into(),
                },
                register_interface::ResultField {
                    name: "expressRouteAuthorizationKey".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateCloudId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExpressRouteAuthorizationResult {
            express_route_authorization_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressRouteAuthorizationId").unwrap(),
            ),
            express_route_authorization_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expressRouteAuthorizationKey").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_cloud_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateCloudId").unwrap(),
            ),
        }
    }
}
