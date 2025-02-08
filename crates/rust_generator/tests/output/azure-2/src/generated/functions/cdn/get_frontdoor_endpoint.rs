#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_frontdoor_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrontdoorEndpointArgs {
        /// Specifies the name of the Front Door Endpoint.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Front Door Profile within which Front Door Endpoint exists.
        #[builder(into)]
        pub profile_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Front Door Profile exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFrontdoorEndpointResult {
        /// Specifies whether this Front Door Endpoint is enabled or not.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the host name of the Front Door Endpoint, in the format `{endpointName}.{dnsZone}` (for example, `contoso.azureedge.net`).
        pub host_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub profile_name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies a mapping of Tags assigned to this Front Door Endpoint.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetFrontdoorEndpointArgs,
    ) -> GetFrontdoorEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let profile_name_binding = args.profile_name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:cdn/getFrontdoorEndpoint:getFrontdoorEndpoint".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "profileName".into(),
                    value: &profile_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFrontdoorEndpointResult {
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            host_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            profile_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("profileName"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
