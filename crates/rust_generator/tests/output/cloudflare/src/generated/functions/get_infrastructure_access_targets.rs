pub mod get_infrastructure_access_targets {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInfrastructureAccessTargetsArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A date and time after a target was created to filter on.
        #[builder(into, default)]
        pub created_after: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The hostname of the target.
        #[builder(into, default)]
        pub hostname: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Partial match to the hostname of a target
        #[builder(into, default)]
        pub hostname_contains: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The target's IPv4 address.
        #[builder(into, default)]
        pub ipv4: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The target's IPv6 address.
        #[builder(into, default)]
        pub ipv6: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A date and time after a target was modified to filter on.
        #[builder(into, default)]
        pub modified_after: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The private virtual network identifier for the target.
        #[builder(into, default)]
        pub virtual_network_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInfrastructureAccessTargetsResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// A date and time after a target was created to filter on.
        pub created_after: pulumi_gestalt_rust::Output<Option<String>>,
        /// The hostname of the target.
        pub hostname: pulumi_gestalt_rust::Output<Option<String>>,
        /// Partial match to the hostname of a target
        pub hostname_contains: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The target's IPv4 address.
        pub ipv4: pulumi_gestalt_rust::Output<Option<String>>,
        /// The target's IPv6 address.
        pub ipv6: pulumi_gestalt_rust::Output<Option<String>>,
        /// A date and time after a target was modified to filter on.
        pub modified_after: pulumi_gestalt_rust::Output<Option<String>>,
        pub targets: pulumi_gestalt_rust::Output<
            Vec<super::super::types::GetInfrastructureAccessTargetsTarget>,
        >,
        /// The private virtual network identifier for the target.
        pub virtual_network_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetInfrastructureAccessTargetsArgs,
    ) -> GetInfrastructureAccessTargetsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let created_after_binding = args.created_after.get_output(context).get_inner();
        let hostname_binding = args.hostname.get_output(context).get_inner();
        let hostname_contains_binding = args
            .hostname_contains
            .get_output(context)
            .get_inner();
        let ipv4_binding = args.ipv4.get_output(context).get_inner();
        let ipv6_binding = args.ipv6.get_output(context).get_inner();
        let modified_after_binding = args.modified_after.get_output(context).get_inner();
        let virtual_network_id_binding = args
            .virtual_network_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getInfrastructureAccessTargets:getInfrastructureAccessTargets"
                .into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "createdAfter".into(),
                    value: &created_after_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "hostnameContains".into(),
                    value: &hostname_contains_binding,
                },
                register_interface::ObjectField {
                    name: "ipv4".into(),
                    value: &ipv4_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6".into(),
                    value: &ipv6_binding,
                },
                register_interface::ObjectField {
                    name: "modifiedAfter".into(),
                    value: &modified_after_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkId".into(),
                    value: &virtual_network_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInfrastructureAccessTargetsResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            created_after: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdAfter"),
            ),
            hostname: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            hostname_contains: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostnameContains"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ipv4: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ipv4")),
            ipv6: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ipv6")),
            modified_after: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modifiedAfter"),
            ),
            targets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targets"),
            ),
            virtual_network_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualNetworkId"),
            ),
        }
    }
}
