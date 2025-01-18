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
pub mod firewall {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallArgs {
        /// A flag indicating whether the firewall is protected against deletion. Use this setting to protect against accidentally deleting a firewall that is in use. Defaults to `false`.
        #[builder(into, default)]
        pub delete_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// A friendly description of the firewall.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// KMS encryption configuration settings. See Encryption Configuration below for details.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::networkfirewall::FirewallEncryptionConfiguration>,
        >,
        /// The Amazon Resource Name (ARN) of the VPC Firewall policy.
        #[builder(into)]
        pub firewall_policy_arn: pulumi_wasm_rust::Output<String>,
        /// A flag indicating whether the firewall is protected against a change to the firewall policy association. Use this setting to protect against accidentally modifying the firewall policy for a firewall that is in use. Defaults to `false`.
        #[builder(into, default)]
        pub firewall_policy_change_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// A friendly name of the firewall.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A flag indicating whether the firewall is protected against changes to the subnet associations. Use this setting to protect against accidentally modifying the subnet associations for a firewall that is in use. Defaults to `false`.
        #[builder(into, default)]
        pub subnet_change_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Set of configuration blocks describing the public subnets. Each subnet must belong to a different Availability Zone in the VPC. AWS Network Firewall creates a firewall endpoint in each subnet. See Subnet Mapping below for details.
        #[builder(into)]
        pub subnet_mappings: pulumi_wasm_rust::Output<
            Vec<super::super::types::networkfirewall::FirewallSubnetMapping>,
        >,
        /// Map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The unique identifier of the VPC where AWS Network Firewall should create the firewall.
        #[builder(into)]
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FirewallResult {
        /// The Amazon Resource Name (ARN) that identifies the firewall.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A flag indicating whether the firewall is protected against deletion. Use this setting to protect against accidentally deleting a firewall that is in use. Defaults to `false`.
        pub delete_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// A friendly description of the firewall.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// KMS encryption configuration settings. See Encryption Configuration below for details.
        pub encryption_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::networkfirewall::FirewallEncryptionConfiguration>,
        >,
        /// The Amazon Resource Name (ARN) of the VPC Firewall policy.
        pub firewall_policy_arn: pulumi_wasm_rust::Output<String>,
        /// A flag indicating whether the firewall is protected against a change to the firewall policy association. Use this setting to protect against accidentally modifying the firewall policy for a firewall that is in use. Defaults to `false`.
        pub firewall_policy_change_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Nested list of information about the current status of the firewall.
        pub firewall_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::networkfirewall::FirewallFirewallStatus>,
        >,
        /// A friendly name of the firewall.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A flag indicating whether the firewall is protected against changes to the subnet associations. Use this setting to protect against accidentally modifying the subnet associations for a firewall that is in use. Defaults to `false`.
        pub subnet_change_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Set of configuration blocks describing the public subnets. Each subnet must belong to a different Availability Zone in the VPC. AWS Network Firewall creates a firewall endpoint in each subnet. See Subnet Mapping below for details.
        pub subnet_mappings: pulumi_wasm_rust::Output<
            Vec<super::super::types::networkfirewall::FirewallSubnetMapping>,
        >,
        /// Map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A string token used when updating a firewall.
        pub update_token: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the VPC where AWS Network Firewall should create the firewall.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FirewallArgs) -> FirewallResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let delete_protection_binding = args.delete_protection.get_inner();
        let description_binding = args.description.get_inner();
        let encryption_configuration_binding = args.encryption_configuration.get_inner();
        let firewall_policy_arn_binding = args.firewall_policy_arn.get_inner();
        let firewall_policy_change_protection_binding = args
            .firewall_policy_change_protection
            .get_inner();
        let name_binding = args.name.get_inner();
        let subnet_change_protection_binding = args.subnet_change_protection.get_inner();
        let subnet_mappings_binding = args.subnet_mappings.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_id_binding = args.vpc_id.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "deleteProtection".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "firewallPolicyArn".into(),
                },
                register_interface::ResultField {
                    name: "firewallPolicyChangeProtection".into(),
                },
                register_interface::ResultField {
                    name: "firewallStatuses".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "subnetChangeProtection".into(),
                },
                register_interface::ResultField {
                    name: "subnetMappings".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "updateToken".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FirewallResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            delete_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteProtection").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            encryption_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfiguration").unwrap(),
            ),
            firewall_policy_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallPolicyArn").unwrap(),
            ),
            firewall_policy_change_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallPolicyChangeProtection").unwrap(),
            ),
            firewall_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firewallStatuses").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            subnet_change_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetChangeProtection").unwrap(),
            ),
            subnet_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetMappings").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            update_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateToken").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
