/// Provides an AWS Network Firewall Firewall Resource
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:networkfirewall:Firewall
///     properties:
///       name: example
///       firewallPolicyArn: ${exampleAwsNetworkfirewallFirewallPolicy.arn}
///       vpcId: ${exampleAwsVpc.id}
///       subnetMappings:
///         - subnetId: ${exampleAwsSubnet.id}
///       tags:
///         Tag1: Value1
///         Tag2: Value2
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Network Firewall Firewalls using their `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:networkfirewall/firewall:Firewall example arn:aws:network-firewall:us-west-1:123456789012:firewall/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod firewall {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallArgs {
        /// A flag indicating whether the firewall is protected against deletion. Use this setting to protect against accidentally deleting a firewall that is in use. Defaults to `false`.
        #[builder(into, default)]
        pub delete_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A friendly description of the firewall.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// KMS encryption configuration settings. See Encryption Configuration below for details.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networkfirewall::FirewallEncryptionConfiguration>,
        >,
        /// The Amazon Resource Name (ARN) of the VPC Firewall policy.
        #[builder(into)]
        pub firewall_policy_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A flag indicating whether the firewall is protected against a change to the firewall policy association. Use this setting to protect against accidentally modifying the firewall policy for a firewall that is in use. Defaults to `false`.
        #[builder(into, default)]
        pub firewall_policy_change_protection: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A friendly name of the firewall.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A flag indicating whether the firewall is protected against changes to the subnet associations. Use this setting to protect against accidentally modifying the subnet associations for a firewall that is in use. Defaults to `false`.
        #[builder(into, default)]
        pub subnet_change_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Set of configuration blocks describing the public subnets. Each subnet must belong to a different Availability Zone in the VPC. AWS Network Firewall creates a firewall endpoint in each subnet. See Subnet Mapping below for details.
        #[builder(into)]
        pub subnet_mappings: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::networkfirewall::FirewallSubnetMapping>,
        >,
        /// Map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The unique identifier of the VPC where AWS Network Firewall should create the firewall.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FirewallResult {
        /// The Amazon Resource Name (ARN) that identifies the firewall.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A flag indicating whether the firewall is protected against deletion. Use this setting to protect against accidentally deleting a firewall that is in use. Defaults to `false`.
        pub delete_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A friendly description of the firewall.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// KMS encryption configuration settings. See Encryption Configuration below for details.
        pub encryption_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::networkfirewall::FirewallEncryptionConfiguration>,
        >,
        /// The Amazon Resource Name (ARN) of the VPC Firewall policy.
        pub firewall_policy_arn: pulumi_gestalt_rust::Output<String>,
        /// A flag indicating whether the firewall is protected against a change to the firewall policy association. Use this setting to protect against accidentally modifying the firewall policy for a firewall that is in use. Defaults to `false`.
        pub firewall_policy_change_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Nested list of information about the current status of the firewall.
        pub firewall_statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::networkfirewall::FirewallFirewallStatus>,
        >,
        /// A friendly name of the firewall.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A flag indicating whether the firewall is protected against changes to the subnet associations. Use this setting to protect against accidentally modifying the subnet associations for a firewall that is in use. Defaults to `false`.
        pub subnet_change_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Set of configuration blocks describing the public subnets. Each subnet must belong to a different Availability Zone in the VPC. AWS Network Firewall creates a firewall endpoint in each subnet. See Subnet Mapping below for details.
        pub subnet_mappings: pulumi_gestalt_rust::Output<
            Vec<super::super::types::networkfirewall::FirewallSubnetMapping>,
        >,
        /// Map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A string token used when updating a firewall.
        pub update_token: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier of the VPC where AWS Network Firewall should create the firewall.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FirewallArgs,
    ) -> FirewallResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let delete_protection_binding = args.delete_protection.get_output(context);
        let description_binding = args.description.get_output(context);
        let encryption_configuration_binding = args
            .encryption_configuration
            .get_output(context);
        let firewall_policy_arn_binding = args.firewall_policy_arn.get_output(context);
        let firewall_policy_change_protection_binding = args
            .firewall_policy_change_protection
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let subnet_change_protection_binding = args
            .subnet_change_protection
            .get_output(context);
        let subnet_mappings_binding = args.subnet_mappings.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:networkfirewall/firewall:Firewall".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deleteProtection".into(),
                    value: &delete_protection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallPolicyArn".into(),
                    value: &firewall_policy_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firewallPolicyChangeProtection".into(),
                    value: &firewall_policy_change_protection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetChangeProtection".into(),
                    value: &subnet_change_protection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetMappings".into(),
                    value: &subnet_mappings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FirewallResult {
            arn: o.get_field("arn"),
            delete_protection: o.get_field("deleteProtection"),
            description: o.get_field("description"),
            encryption_configuration: o.get_field("encryptionConfiguration"),
            firewall_policy_arn: o.get_field("firewallPolicyArn"),
            firewall_policy_change_protection: o
                .get_field("firewallPolicyChangeProtection"),
            firewall_statuses: o.get_field("firewallStatuses"),
            name: o.get_field("name"),
            subnet_change_protection: o.get_field("subnetChangeProtection"),
            subnet_mappings: o.get_field("subnetMappings"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            update_token: o.get_field("updateToken"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
