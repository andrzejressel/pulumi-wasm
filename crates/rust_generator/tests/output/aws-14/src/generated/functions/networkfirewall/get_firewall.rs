#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_firewall {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFirewallArgs {
        /// ARN of the firewall.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Descriptive name of the firewall.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetFirewallResult {
        /// ARN of the firewall.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A flag indicating whether the firewall is protected against deletion.
        pub delete_protection: pulumi_gestalt_rust::Output<bool>,
        /// Description of the firewall.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// AWS Key Management Service (AWS KMS) encryption settings for the firewall.
        pub encryption_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::networkfirewall::GetFirewallEncryptionConfiguration,
            >,
        >,
        /// ARN of the VPC Firewall policy.
        pub firewall_policy_arn: pulumi_gestalt_rust::Output<String>,
        /// A flag indicating whether the firewall is protected against a change to the firewall policy association.
        pub firewall_policy_change_protection: pulumi_gestalt_rust::Output<bool>,
        /// Nested list of information about the current status of the firewall.
        pub firewall_statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::networkfirewall::GetFirewallFirewallStatus>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Descriptive name of the firewall.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A flag indicating whether the firewall is protected against changes to the subnet associations.
        pub subnet_change_protection: pulumi_gestalt_rust::Output<bool>,
        /// Set of configuration blocks describing the public subnets. Each subnet must belong to a different Availability Zone in the VPC. AWS Network Firewall creates a firewall endpoint in each subnet.
        pub subnet_mappings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::networkfirewall::GetFirewallSubnetMapping>,
        >,
        /// Map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// String token used when updating a firewall.
        pub update_token: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier of the VPC where AWS Network Firewall should create the firewall.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFirewallArgs,
    ) -> GetFirewallResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:networkfirewall/getFirewall:getFirewall".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFirewallResult {
            arn: o.get_field("arn"),
            delete_protection: o.get_field("deleteProtection"),
            description: o.get_field("description"),
            encryption_configurations: o.get_field("encryptionConfigurations"),
            firewall_policy_arn: o.get_field("firewallPolicyArn"),
            firewall_policy_change_protection: o
                .get_field("firewallPolicyChangeProtection"),
            firewall_statuses: o.get_field("firewallStatuses"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            subnet_change_protection: o.get_field("subnetChangeProtection"),
            subnet_mappings: o.get_field("subnetMappings"),
            tags: o.get_field("tags"),
            update_token: o.get_field("updateToken"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
