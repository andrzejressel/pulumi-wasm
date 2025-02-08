#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_firewall_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFirewallPolicyArgs {
        /// ARN of the firewall policy.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Descriptive name of the firewall policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the firewall policy.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetFirewallPolicyResult {
        pub arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Description of the firewall policy.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The [policy][2] for the specified firewall policy.
        pub firewall_policies: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::networkfirewall::GetFirewallPolicyFirewallPolicy,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value tags for the firewall policy.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Token used for optimistic locking.
        pub update_token: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetFirewallPolicyArgs,
    ) -> GetFirewallPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:networkfirewall/getFirewallPolicy:getFirewallPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFirewallPolicyResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            firewall_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("firewallPolicies"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            update_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateToken"),
            ),
        }
    }
}
