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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod volume_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeGroupArgs {
        /// Specifies the Elastic SAN ID within which this Elastic SAN Volume Group should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub elastic_san_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An `encryption` block as defined below.
        ///
        /// > **NOTE:** The `encryption` block can only be set when `encryption_type` is set to `EncryptionAtRestWithCustomerManagedKey`.
        #[builder(into, default)]
        pub encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elasticsan::VolumeGroupEncryption>,
        >,
        /// Specifies the type of the key used to encrypt the data of the disk. Possible values are `EncryptionAtRestWithCustomerManagedKey` and `EncryptionAtRestWithPlatformKey`. Defaults to `EncryptionAtRestWithPlatformKey`.
        #[builder(into, default)]
        pub encryption_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below. Specifies the Managed Identity which should be assigned to this Elastic SAN Volume Group.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elasticsan::VolumeGroupIdentity>,
        >,
        /// Specifies the name of this Elastic SAN Volume Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `network_rule` blocks as defined below.
        #[builder(into, default)]
        pub network_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::elasticsan::VolumeGroupNetworkRule>>,
        >,
        /// Specifies the type of the storage target. The only possible value is `Iscsi`. Defaults to `Iscsi`.
        #[builder(into, default)]
        pub protocol_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VolumeGroupResult {
        /// Specifies the Elastic SAN ID within which this Elastic SAN Volume Group should exist. Changing this forces a new resource to be created.
        pub elastic_san_id: pulumi_gestalt_rust::Output<String>,
        /// An `encryption` block as defined below.
        ///
        /// > **NOTE:** The `encryption` block can only be set when `encryption_type` is set to `EncryptionAtRestWithCustomerManagedKey`.
        pub encryption: pulumi_gestalt_rust::Output<
            Option<super::super::types::elasticsan::VolumeGroupEncryption>,
        >,
        /// Specifies the type of the key used to encrypt the data of the disk. Possible values are `EncryptionAtRestWithCustomerManagedKey` and `EncryptionAtRestWithPlatformKey`. Defaults to `EncryptionAtRestWithPlatformKey`.
        pub encryption_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below. Specifies the Managed Identity which should be assigned to this Elastic SAN Volume Group.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::elasticsan::VolumeGroupIdentity>,
        >,
        /// Specifies the name of this Elastic SAN Volume Group. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `network_rule` blocks as defined below.
        pub network_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::elasticsan::VolumeGroupNetworkRule>>,
        >,
        /// Specifies the type of the storage target. The only possible value is `Iscsi`. Defaults to `Iscsi`.
        pub protocol_type: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VolumeGroupArgs,
    ) -> VolumeGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let elastic_san_id_binding = args.elastic_san_id.get_output(context);
        let encryption_binding = args.encryption.get_output(context);
        let encryption_type_binding = args.encryption_type.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_rules_binding = args.network_rules.get_output(context);
        let protocol_type_binding = args.protocol_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:elasticsan/volumeGroup:VolumeGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "elasticSanId".into(),
                    value: &elastic_san_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryption".into(),
                    value: &encryption_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionType".into(),
                    value: &encryption_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkRules".into(),
                    value: &network_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocolType".into(),
                    value: &protocol_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VolumeGroupResult {
            elastic_san_id: o.get_field("elasticSanId"),
            encryption: o.get_field("encryption"),
            encryption_type: o.get_field("encryptionType"),
            identity: o.get_field("identity"),
            name: o.get_field("name"),
            network_rules: o.get_field("networkRules"),
            protocol_type: o.get_field("protocolType"),
        }
    }
}
