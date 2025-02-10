#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_address {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAddressArgs {
        /// A unique name for the resource, required by GCE.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Region in which the created address reside.
        /// If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAddressResult {
        /// The IP of the created resource.
        pub address: pulumi_gestalt_rust::Output<String>,
        pub address_type: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network: pulumi_gestalt_rust::Output<String>,
        pub network_tier: pulumi_gestalt_rust::Output<String>,
        pub prefix_length: pulumi_gestalt_rust::Output<i32>,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub purpose: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Indicates if the address is used. Possible values are: RESERVED or IN_USE.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub subnetwork: pulumi_gestalt_rust::Output<String>,
        pub users: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAddressArgs,
    ) -> GetAddressResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getAddress:getAddress".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAddressResult {
            address: o.get_field("address"),
            address_type: o.get_field("addressType"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            network_tier: o.get_field("networkTier"),
            prefix_length: o.get_field("prefixLength"),
            project: o.get_field("project"),
            purpose: o.get_field("purpose"),
            region: o.get_field("region"),
            self_link: o.get_field("selfLink"),
            status: o.get_field("status"),
            subnetwork: o.get_field("subnetwork"),
            users: o.get_field("users"),
        }
    }
}
