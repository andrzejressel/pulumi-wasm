/// Manages an Azure VMware Solution ExpressRoute Circuit Authorization.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod express_route_authorization {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRouteAuthorizationArgs {
        /// The name which should be used for this Azure VMware Solution ExpressRoute Circuit Authorization. Changing this forces a new Azure VMware Solution ExpressRoute Circuit Authorization to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Azure VMware Solution Private Cloud in which to create this Azure VMware Solution ExpressRoute Circuit Authorization. Changing this forces a new Azure VMware Solution ExpressRoute Circuit Authorization to be created.
        #[builder(into)]
        pub private_cloud_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ExpressRouteAuthorizationResult {
        /// The ID of the Azure VMware Solution ExpressRoute Circuit Authorization.
        pub express_route_authorization_id: pulumi_gestalt_rust::Output<String>,
        /// The key of the Azure VMware Solution ExpressRoute Circuit Authorization.
        pub express_route_authorization_key: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Azure VMware Solution ExpressRoute Circuit Authorization. Changing this forces a new Azure VMware Solution ExpressRoute Circuit Authorization to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Azure VMware Solution Private Cloud in which to create this Azure VMware Solution ExpressRoute Circuit Authorization. Changing this forces a new Azure VMware Solution ExpressRoute Circuit Authorization to be created.
        pub private_cloud_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExpressRouteAuthorizationArgs,
    ) -> ExpressRouteAuthorizationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let private_cloud_id_binding = args.private_cloud_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:avs/expressRouteAuthorization:ExpressRouteAuthorization"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateCloudId".into(),
                    value: private_cloud_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExpressRouteAuthorizationResult {
            express_route_authorization_id: o.get_field("expressRouteAuthorizationId"),
            express_route_authorization_key: o.get_field("expressRouteAuthorizationKey"),
            name: o.get_field("name"),
            private_cloud_id: o.get_field("privateCloudId"),
        }
    }
}
