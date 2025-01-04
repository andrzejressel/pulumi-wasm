/// Manages an Elastic SAN Volume Group resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   exampleElasticSan:
///     type: azure:elasticsan:ElasticSan
///     name: example
///     properties:
///       name: examplees-es
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       baseSizeInTib: 1
///       sku:
///         name: Premium_LRS
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       name: example-uai
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///       serviceEndpoints:
///         - Microsoft.Storage.Global
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: examplekv
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       enabledForDiskEncryption: true
///       tenantId: ${current.tenantId}
///       softDeleteRetentionDays: 7
///       purgeProtectionEnabled: true
///       skuName: standard
///   userAssignedIdentity:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${exampleUserAssignedIdentity.principalId}
///       keyPermissions:
///         - Get
///         - UnwrapKey
///         - WrapKey
///       secretPermissions:
///         - Get
///   client:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
///       keyPermissions:
///         - Get
///         - Create
///         - Delete
///         - List
///         - Restore
///         - Recover
///         - UnwrapKey
///         - WrapKey
///         - Purge
///         - Encrypt
///         - Decrypt
///         - Sign
///         - Verify
///         - GetRotationPolicy
///       secretPermissions:
///         - Get
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: example-kvk
///       keyVaultId: ${exampleKeyVault.id}
///       keyType: RSA
///       keySize: 2048
///       keyOpts:
///         - decrypt
///         - encrypt
///         - sign
///         - unwrapKey
///         - verify
///         - wrapKey
///     options:
///       dependsOn:
///         - ${userAssignedIdentity}
///         - ${client}
///   exampleVolumeGroup:
///     type: azure:elasticsan:VolumeGroup
///     name: example
///     properties:
///       name: example-esvg
///       elasticSanId: ${exampleElasticSan.id}
///       encryptionType: EncryptionAtRestWithCustomerManagedKey
///       encryption:
///         keyVaultKeyId: ${exampleKey.versionlessId}
///         userAssignedIdentityId: ${exampleUserAssignedIdentity.id}
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
///       networkRules:
///         - subnetId: ${exampleSubnet.id}
///           action: Allow
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// An existing Elastic SAN Volume Group can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:elasticsan/volumeGroup:VolumeGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ElasticSan/elasticSans/esan1/volumeGroups/vg1
/// ```
///
pub mod volume_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeGroupArgs {
        /// Specifies the Elastic SAN ID within which this Elastic SAN Volume Group should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub elastic_san_id: pulumi_wasm_rust::Output<String>,
        /// An `encryption` block as defined below.
        ///
        /// > **NOTE:** The `encryption` block can only be set when `encryption_type` is set to `EncryptionAtRestWithCustomerManagedKey`.
        #[builder(into, default)]
        pub encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsan::VolumeGroupEncryption>,
        >,
        /// Specifies the type of the key used to encrypt the data of the disk. Possible values are `EncryptionAtRestWithCustomerManagedKey` and `EncryptionAtRestWithPlatformKey`. Defaults to `EncryptionAtRestWithPlatformKey`.
        #[builder(into, default)]
        pub encryption_type: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below. Specifies the Managed Identity which should be assigned to this Elastic SAN Volume Group.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsan::VolumeGroupIdentity>,
        >,
        /// Specifies the name of this Elastic SAN Volume Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `network_rule` blocks as defined below.
        #[builder(into, default)]
        pub network_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::elasticsan::VolumeGroupNetworkRule>>,
        >,
        /// Specifies the type of the storage target. The only possible value is `Iscsi`. Defaults to `Iscsi`.
        #[builder(into, default)]
        pub protocol_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VolumeGroupResult {
        /// Specifies the Elastic SAN ID within which this Elastic SAN Volume Group should exist. Changing this forces a new resource to be created.
        pub elastic_san_id: pulumi_wasm_rust::Output<String>,
        /// An `encryption` block as defined below.
        ///
        /// > **NOTE:** The `encryption` block can only be set when `encryption_type` is set to `EncryptionAtRestWithCustomerManagedKey`.
        pub encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsan::VolumeGroupEncryption>,
        >,
        /// Specifies the type of the key used to encrypt the data of the disk. Possible values are `EncryptionAtRestWithCustomerManagedKey` and `EncryptionAtRestWithPlatformKey`. Defaults to `EncryptionAtRestWithPlatformKey`.
        pub encryption_type: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below. Specifies the Managed Identity which should be assigned to this Elastic SAN Volume Group.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsan::VolumeGroupIdentity>,
        >,
        /// Specifies the name of this Elastic SAN Volume Group. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `network_rule` blocks as defined below.
        pub network_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::elasticsan::VolumeGroupNetworkRule>>,
        >,
        /// Specifies the type of the storage target. The only possible value is `Iscsi`. Defaults to `Iscsi`.
        pub protocol_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VolumeGroupArgs) -> VolumeGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let elastic_san_id_binding = args.elastic_san_id.get_inner();
        let encryption_binding = args.encryption.get_inner();
        let encryption_type_binding = args.encryption_type.get_inner();
        let identity_binding = args.identity.get_inner();
        let name_binding = args.name.get_inner();
        let network_rules_binding = args.network_rules.get_inner();
        let protocol_type_binding = args.protocol_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:elasticsan/volumeGroup:VolumeGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "elasticSanId".into(),
                    value: &elastic_san_id_binding,
                },
                register_interface::ObjectField {
                    name: "encryption".into(),
                    value: &encryption_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionType".into(),
                    value: &encryption_type_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkRules".into(),
                    value: &network_rules_binding,
                },
                register_interface::ObjectField {
                    name: "protocolType".into(),
                    value: &protocol_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "elasticSanId".into(),
                },
                register_interface::ResultField {
                    name: "encryption".into(),
                },
                register_interface::ResultField {
                    name: "encryptionType".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkRules".into(),
                },
                register_interface::ResultField {
                    name: "protocolType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VolumeGroupResult {
            elastic_san_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticSanId").unwrap(),
            ),
            encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryption").unwrap(),
            ),
            encryption_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionType").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkRules").unwrap(),
            ),
            protocol_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocolType").unwrap(),
            ),
        }
    }
}
