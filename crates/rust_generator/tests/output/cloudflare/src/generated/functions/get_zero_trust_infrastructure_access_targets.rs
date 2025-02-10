#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_zero_trust_infrastructure_access_targets {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZeroTrustInfrastructureAccessTargetsArgs {
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
    pub struct GetZeroTrustInfrastructureAccessTargetsResult {
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
            Vec<super::super::types::GetZeroTrustInfrastructureAccessTargetsTarget>,
        >,
        /// The private virtual network identifier for the target.
        pub virtual_network_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetZeroTrustInfrastructureAccessTargetsArgs,
    ) -> GetZeroTrustInfrastructureAccessTargetsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let created_after_binding = args.created_after.get_output(context);
        let hostname_binding = args.hostname.get_output(context);
        let hostname_contains_binding = args.hostname_contains.get_output(context);
        let ipv4_binding = args.ipv4.get_output(context);
        let ipv6_binding = args.ipv6.get_output(context);
        let modified_after_binding = args.modified_after.get_output(context);
        let virtual_network_id_binding = args.virtual_network_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getZeroTrustInfrastructureAccessTargets:getZeroTrustInfrastructureAccessTargets"
                .into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createdAfter".into(),
                    value: created_after_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostname".into(),
                    value: hostname_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostnameContains".into(),
                    value: hostname_contains_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv4".into(),
                    value: ipv4_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv6".into(),
                    value: ipv6_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "modifiedAfter".into(),
                    value: modified_after_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkId".into(),
                    value: virtual_network_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetZeroTrustInfrastructureAccessTargetsResult {
            account_id: o.get_field("accountId"),
            created_after: o.get_field("createdAfter"),
            hostname: o.get_field("hostname"),
            hostname_contains: o.get_field("hostnameContains"),
            id: o.get_field("id"),
            ipv4: o.get_field("ipv4"),
            ipv6: o.get_field("ipv6"),
            modified_after: o.get_field("modifiedAfter"),
            targets: o.get_field("targets"),
            virtual_network_id: o.get_field("virtualNetworkId"),
        }
    }
}
