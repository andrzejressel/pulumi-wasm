pub mod get_firewall {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFirewallArgs {
        /// ARN of the firewall.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Descriptive name of the firewall.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetFirewallResult {
        /// ARN of the firewall.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A flag indicating whether the firewall is protected against deletion.
        pub delete_protection: pulumi_wasm_rust::Output<bool>,
        /// Description of the firewall.
        pub description: pulumi_wasm_rust::Output<String>,
        /// AWS Key Management Service (AWS KMS) encryption settings for the firewall.
        pub encryption_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::networkfirewall::GetFirewallEncryptionConfiguration,
            >,
        >,
        /// ARN of the VPC Firewall policy.
        pub firewall_policy_arn: pulumi_wasm_rust::Output<String>,
        /// A flag indicating whether the firewall is protected against a change to the firewall policy association.
        pub firewall_policy_change_protection: pulumi_wasm_rust::Output<bool>,
        /// Nested list of information about the current status of the firewall.
        pub firewall_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::networkfirewall::GetFirewallFirewallStatus>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Descriptive name of the firewall.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A flag indicating whether the firewall is protected against changes to the subnet associations.
        pub subnet_change_protection: pulumi_wasm_rust::Output<bool>,
        /// Set of configuration blocks describing the public subnets. Each subnet must belong to a different Availability Zone in the VPC. AWS Network Firewall creates a firewall endpoint in each subnet.
        pub subnet_mappings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::networkfirewall::GetFirewallSubnetMapping>,
        >,
        /// Map of resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// String token used when updating a firewall.
        pub update_token: pulumi_wasm_rust::Output<String>,
        /// Unique identifier of the VPC where AWS Network Firewall should create the firewall.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetFirewallArgs) -> GetFirewallResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:networkfirewall/getFirewall:getFirewall".into(),
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
                    name: "encryptionConfigurations".into(),
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
                    name: "id".into(),
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
                    name: "updateToken".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFirewallResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            delete_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteProtection").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            encryption_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfigurations").unwrap(),
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
            update_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateToken").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
