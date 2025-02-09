#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_frontdoor_firewall_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrontdoorFirewallPolicyArgs {
        /// The name of the Front Door Firewall Policy.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFrontdoorFirewallPolicyResult {
        /// The enabled state of the Front Door Firewall Policy.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Front Door Profiles frontend endpoints associated with this Front Door Firewall Policy.
        pub frontend_endpoint_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Front Door Firewall Policy mode.
        pub mode: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The redirect URL for the client.
        pub redirect_url: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The sku's pricing tier for this Front Door Firewall Policy.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetFrontdoorFirewallPolicyArgs,
    ) -> GetFrontdoorFirewallPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:cdn/getFrontdoorFirewallPolicy:getFrontdoorFirewallPolicy"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
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
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFrontdoorFirewallPolicyResult {
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            frontend_endpoint_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frontendEndpointIds"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            mode: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mode")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            redirect_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("redirectUrl"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
        }
    }
}
