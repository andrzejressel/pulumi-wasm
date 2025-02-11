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
        context: &pulumi_gestalt_rust::Context,
        args: GetFirewallPolicyArgs,
    ) -> GetFirewallPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:networkfirewall/getFirewallPolicy:getFirewallPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFirewallPolicyResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            firewall_policies: o.get_field("firewallPolicies"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            update_token: o.get_field("updateToken"),
        }
    }
}
