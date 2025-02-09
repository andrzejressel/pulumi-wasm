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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetFirewallArgs,
    ) -> GetFirewallResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding_1 = args.arn.get_output(context);
        let arn_binding = arn_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:networkfirewall/getFirewall:getFirewall".into(),
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
        GetFirewallResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            delete_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteProtection"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            encryption_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionConfigurations"),
            ),
            firewall_policy_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("firewallPolicyArn"),
            ),
            firewall_policy_change_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("firewallPolicyChangeProtection"),
            ),
            firewall_statuses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("firewallStatuses"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            subnet_change_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetChangeProtection"),
            ),
            subnet_mappings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetMappings"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            update_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateToken"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
