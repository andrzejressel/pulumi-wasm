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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FirewallArgs,
    ) -> FirewallResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let delete_protection_binding_1 = args.delete_protection.get_output(context);
        let delete_protection_binding = delete_protection_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let encryption_configuration_binding_1 = args
            .encryption_configuration
            .get_output(context);
        let encryption_configuration_binding = encryption_configuration_binding_1
            .get_inner();
        let firewall_policy_arn_binding_1 = args.firewall_policy_arn.get_output(context);
        let firewall_policy_arn_binding = firewall_policy_arn_binding_1.get_inner();
        let firewall_policy_change_protection_binding_1 = args
            .firewall_policy_change_protection
            .get_output(context);
        let firewall_policy_change_protection_binding = firewall_policy_change_protection_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let subnet_change_protection_binding_1 = args
            .subnet_change_protection
            .get_output(context);
        let subnet_change_protection_binding = subnet_change_protection_binding_1
            .get_inner();
        let subnet_mappings_binding_1 = args.subnet_mappings.get_output(context);
        let subnet_mappings_binding = subnet_mappings_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let vpc_id_binding_1 = args.vpc_id.get_output(context);
        let vpc_id_binding = vpc_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:networkfirewall/firewall:Firewall".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deleteProtection".into(),
                    value: &delete_protection_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "firewallPolicyArn".into(),
                    value: &firewall_policy_arn_binding,
                },
                register_interface::ObjectField {
                    name: "firewallPolicyChangeProtection".into(),
                    value: &firewall_policy_change_protection_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "subnetChangeProtection".into(),
                    value: &subnet_change_protection_binding,
                },
                register_interface::ObjectField {
                    name: "subnetMappings".into(),
                    value: &subnet_mappings_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FirewallResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            delete_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteProtection"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            encryption_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionConfiguration"),
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
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            subnet_change_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetChangeProtection"),
            ),
            subnet_mappings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetMappings"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            update_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateToken"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
