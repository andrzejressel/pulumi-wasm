/// Manages an ExpressRoute Port Authorization.
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
///             .name("exprtTest")
///             .build_struct(),
///     );
///     let exampleExpressRoutePort = express_route_port::create(
///         "exampleExpressRoutePort",
///         ExpressRoutePortArgs::builder()
///             .bandwidth_in_gbps(10)
///             .encapsulation("Dot1Q")
///             .location("${example.location}")
///             .name("port1")
///             .peering_location("Airtel-Chennai-CLS")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleExpressRoutePortAuthorization = express_route_port_authorization::create(
///         "exampleExpressRoutePortAuthorization",
///         ExpressRoutePortAuthorizationArgs::builder()
///             .express_route_port_name("${exampleExpressRoutePort.name}")
///             .name("exampleERCAuth")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ExpressRoute Port Authorizations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/expressRoutePortAuthorization:ExpressRoutePortAuthorization auth1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/expressRoutePorts/myExpressPort/authorizations/auth1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod express_route_port_authorization {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExpressRoutePortAuthorizationArgs {
        /// The name of the Express Route Port in which to create the Authorization. Changing this forces a new resource to be created.
        #[builder(into)]
        pub express_route_port_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the ExpressRoute Port. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the ExpressRoute Port. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ExpressRoutePortAuthorizationResult {
        /// The Authorization Key.
        pub authorization_key: pulumi_gestalt_rust::Output<String>,
        /// The authorization use status.
        pub authorization_use_status: pulumi_gestalt_rust::Output<String>,
        /// The name of the Express Route Port in which to create the Authorization. Changing this forces a new resource to be created.
        pub express_route_port_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the ExpressRoute Port. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the ExpressRoute Port. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ExpressRoutePortAuthorizationArgs,
    ) -> ExpressRoutePortAuthorizationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let express_route_port_name_binding = args
            .express_route_port_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/expressRoutePortAuthorization:ExpressRoutePortAuthorization"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "expressRoutePortName".into(),
                    value: &express_route_port_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ExpressRoutePortAuthorizationResult {
            authorization_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizationKey"),
            ),
            authorization_use_status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizationUseStatus"),
            ),
            express_route_port_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expressRoutePortName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
