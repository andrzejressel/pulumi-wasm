#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_frontdoor_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrontdoorProfileArgs {
        /// Specifies the name of the Front Door Profile.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where this Front Door Profile exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFrontdoorProfileResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The UUID of the Front Door Profile which will be sent in the HTTP Header as the `X-Azure-FDID` attribute.
        pub resource_guid: pulumi_gestalt_rust::Output<String>,
        /// Specifies the maximum response timeout in seconds.
        pub response_timeout_seconds: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the SKU for this Front Door Profile.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies a mapping of Tags assigned to this Front Door Profile.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetFrontdoorProfileArgs,
    ) -> GetFrontdoorProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:cdn/getFrontdoorProfile:getFrontdoorProfile".into(),
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
        GetFrontdoorProfileResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            resource_guid: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGuid"),
            ),
            response_timeout_seconds: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("responseTimeoutSeconds"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
