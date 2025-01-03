/// Apigee NAT (network address translation) address. A NAT address is a static external IP address used for Internet egress traffic. This is not avaible for Apigee hybrid.
///
///
/// To get more information about NatAddress, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.instances.natAddresses)
/// * How-to Guides
///     * [Provisioning NAT IPs](https://cloud.google.com/apigee/docs/api-platform/security/nat-provisioning)
///
/// ## Example Usage
///
/// ### Apigee Nat Address Basic
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 21
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeKeyring:
///     type: gcp:kms:KeyRing
///     name: apigee_keyring
///     properties:
///       name: apigee-keyring
///       location: us-central1
///   apigeeKey:
///     type: gcp:kms:CryptoKey
///     name: apigee_key
///     properties:
///       name: apigee-key
///       keyRing: ${apigeeKeyring.id}
///   apigeeSa:
///     type: gcp:projects:ServiceIdentity
///     name: apigee_sa
///     properties:
///       project: ${project.projectId}
///       service: ${apigee.service}
///   apigeeSaKeyuser:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: apigee_sa_keyuser
///     properties:
///       cryptoKeyId: ${apigeeKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: ${apigeeSa.member}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       displayName: apigee-org
///       description: Terraform-provisioned Apigee Org.
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///       runtimeDatabaseEncryptionKeyName: ${apigeeKey.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///         - ${apigeeSaKeyuser}
///   apigeeInstance:
///     type: gcp:apigee:Instance
///     name: apigee_instance
///     properties:
///       name: apigee-instance
///       location: us-central1
///       description: Terraform-managed Apigee Runtime Instance
///       displayName: apigee-instance
///       orgId: ${apigeeOrg.id}
///       diskEncryptionKeyName: ${apigeeKey.id}
///   apigee-nat:
///     type: gcp:apigee:NatAddress
///     properties:
///       name: my-nat-address
///       instanceId: ${apigeeInstance.id}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
/// ### Apigee Nat Address With Activate
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 21
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeKeyring:
///     type: gcp:kms:KeyRing
///     name: apigee_keyring
///     properties:
///       name: apigee-keyring
///       location: us-central1
///   apigeeKey:
///     type: gcp:kms:CryptoKey
///     name: apigee_key
///     properties:
///       name: apigee-key
///       keyRing: ${apigeeKeyring.id}
///   apigeeSa:
///     type: gcp:projects:ServiceIdentity
///     name: apigee_sa
///     properties:
///       project: ${project.projectId}
///       service: ${apigee.service}
///   apigeeSaKeyuser:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: apigee_sa_keyuser
///     properties:
///       cryptoKeyId: ${apigeeKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: ${apigeeSa.member}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       displayName: apigee-org
///       description: Terraform-provisioned Apigee Org.
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///       runtimeDatabaseEncryptionKeyName: ${apigeeKey.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///         - ${apigeeSaKeyuser}
///   apigeeInstance:
///     type: gcp:apigee:Instance
///     name: apigee_instance
///     properties:
///       name: apigee-instance
///       location: us-central1
///       description: Terraform-managed Apigee Runtime Instance
///       displayName: apigee-instance
///       orgId: ${apigeeOrg.id}
///       diskEncryptionKeyName: ${apigeeKey.id}
///   apigee-nat:
///     type: gcp:apigee:NatAddress
///     properties:
///       name: my-nat-address
///       activate: 'true'
///       instanceId: ${apigeeInstance.id}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// NatAddress can be imported using any of these accepted formats:
///
/// * `{{instance_id}}/natAddresses/{{name}}`
///
/// * `{{instance_id}}/{{name}}`
///
/// When using the `pulumi import` command, NatAddress can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/natAddress:NatAddress default {{instance_id}}/natAddresses/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/natAddress:NatAddress default {{instance_id}}/{{name}}
/// ```
///
pub mod nat_address {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NatAddressArgs {
        /// Flag that specifies whether the reserved NAT address should be activate.
        #[builder(into, default)]
        pub activate: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Apigee instance associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/instances/{{instance_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Resource ID of the NAT address.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NatAddressResult {
        /// Flag that specifies whether the reserved NAT address should be activate.
        pub activate: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Apigee instance associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/instances/{{instance_name}}`.
        ///
        ///
        /// - - -
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// The allocated NAT IP address.
        pub ip_address: pulumi_wasm_rust::Output<String>,
        /// Resource ID of the NAT address.
        pub name: pulumi_wasm_rust::Output<String>,
        /// State of the NAT IP address.
        pub state: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NatAddressArgs) -> NatAddressResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let activate_binding = args.activate.get_inner();
        let instance_id_binding = args.instance_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/natAddress:NatAddress".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activate".into(),
                    value: &activate_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "activate".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NatAddressResult {
            activate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activate").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
        }
    }
}
