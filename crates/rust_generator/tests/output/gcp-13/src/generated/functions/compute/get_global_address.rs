#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_global_address {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGlobalAddressArgs {
        /// A unique name for the resource, required by GCE.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetGlobalAddressResult {
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetGlobalAddressArgs,
    ) -> GetGlobalAddressResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getGlobalAddress:getGlobalAddress".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGlobalAddressResult {
            address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("address"),
            ),
            address_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addressType"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            network_tier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkTier"),
            ),
            prefix_length: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("prefixLength"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            purpose: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("purpose"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            subnetwork: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetwork"),
            ),
            users: pulumi_gestalt_rust::__private::into_domain(o.extract_field("users")),
        }
    }
}
