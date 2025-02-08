/// Manages a Disk Encryption Set.
///
/// > **NOTE:** At this time the Key Vault used to store the Active Key for this Disk Encryption Set must have both Soft Delete & Purge Protection enabled - which are not yet supported by this provider.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: des-example-keyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       enabledForDiskEncryption: true
///       purgeProtectionEnabled: true
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: des-example-key
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
///         - ${["example-user"]}
///   exampleDiskEncryptionSet:
///     type: azure:compute:DiskEncryptionSet
///     name: example
///     properties:
///       name: des
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       keyVaultKeyId: ${exampleKey.id}
///       identity:
///         type: SystemAssigned
///   example-disk:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${exampleDiskEncryptionSet.identity.tenantId}
///       objectId: ${exampleDiskEncryptionSet.identity.principalId}
///       keyPermissions:
///         - Create
///         - Delete
///         - Get
///         - Purge
///         - Recover
///         - Update
///         - List
///         - Decrypt
///         - Sign
///   example-user:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
///       keyPermissions:
///         - Create
///         - Delete
///         - Get
///         - Purge
///         - Recover
///         - Update
///         - List
///         - Decrypt
///         - Sign
///         - GetRotationPolicy
///   example-diskAssignment:
///     type: azure:authorization:Assignment
///     name: example-disk
///     properties:
///       scope: ${exampleKeyVault.id}
///       roleDefinitionName: Key Vault Crypto Service Encryption User
///       principalId: ${exampleDiskEncryptionSet.identity.principalId}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
///
/// ### With Automatic Key Rotation Enabled
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: des-example-keyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       enabledForDiskEncryption: true
///       purgeProtectionEnabled: true
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: des-example-key
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
///         - ${["example-user"]}
///   exampleDiskEncryptionSet:
///     type: azure:compute:DiskEncryptionSet
///     name: example
///     properties:
///       name: des
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       keyVaultKeyId: ${exampleKey.versionlessId}
///       autoKeyRotationEnabled: true
///       identity:
///         type: SystemAssigned
///   example-disk:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${exampleDiskEncryptionSet.identity.tenantId}
///       objectId: ${exampleDiskEncryptionSet.identity.principalId}
///       keyPermissions:
///         - Create
///         - Delete
///         - Get
///         - Purge
///         - Recover
///         - Update
///         - List
///         - Decrypt
///         - Sign
///   example-user:
///     type: azure:keyvault:AccessPolicy
///     properties:
///       keyVaultId: ${exampleKeyVault.id}
///       tenantId: ${current.tenantId}
///       objectId: ${current.objectId}
///       keyPermissions:
///         - Create
///         - Delete
///         - Get
///         - Purge
///         - Recover
///         - Update
///         - List
///         - Decrypt
///         - Sign
///         - GetRotationPolicy
///   example-diskAssignment:
///     type: azure:authorization:Assignment
///     name: example-disk
///     properties:
///       scope: ${exampleKeyVault.id}
///       roleDefinitionName: Key Vault Crypto Service Encryption User
///       principalId: ${exampleDiskEncryptionSet.identity.principalId}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Disk Encryption Sets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/diskEncryptionSet:DiskEncryptionSet example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Compute/diskEncryptionSets/encryptionSet1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod disk_encryption_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DiskEncryptionSetArgs {
        #[builder(into, default)]
        pub auto_key_rotation_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The type of key used to encrypt the data of the disk. Possible values are `EncryptionAtRestWithCustomerKey`, `EncryptionAtRestWithPlatformAndCustomerKeys` and `ConfidentialVmEncryptedWithCustomerKey`. Defaults to `EncryptionAtRestWithCustomerKey`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub encryption_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Multi-tenant application client id to access key vault in a different tenant.
        #[builder(into, default)]
        pub federated_client_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::DiskEncryptionSetIdentity,
        >,
        /// Specifies the URL to a Key Vault Key (either from a Key Vault Key, or the Key URL for the Key Vault Secret). Exactly one of `managed_hsm_key_id`, `key_vault_key_id` must be specified.
        ///
        /// > **NOTE** Access to the KeyVault must be granted for this Disk Encryption Set, if you want to further use this Disk Encryption Set in a Managed Disk or Virtual Machine, or Virtual Machine Scale Set. For instructions, please refer to the doc of [Server side encryption of Azure managed disks](https://docs.microsoft.com/azure/virtual-machines/linux/disk-encryption).
        ///
        /// > **NOTE** A KeyVault or Managed HSM using enable_rbac_authorization requires to use `azure.authorization.Assignment` to assign the role `Key Vault Crypto Service Encryption User` to this Disk Encryption Set.
        /// In this case, `azure.keyvault.AccessPolicy` is not needed.
        #[builder(into, default)]
        pub key_vault_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Azure Region where the Disk Encryption Set exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key ID of a key in a managed HSM.  Exactly one of `managed_hsm_key_id`, `key_vault_key_id` must be specified.
        #[builder(into, default)]
        pub managed_hsm_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Disk Encryption Set. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group where the Disk Encryption Set should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the Disk Encryption Set.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DiskEncryptionSetResult {
        pub auto_key_rotation_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The type of key used to encrypt the data of the disk. Possible values are `EncryptionAtRestWithCustomerKey`, `EncryptionAtRestWithPlatformAndCustomerKeys` and `ConfidentialVmEncryptedWithCustomerKey`. Defaults to `EncryptionAtRestWithCustomerKey`. Changing this forces a new resource to be created.
        pub encryption_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Multi-tenant application client id to access key vault in a different tenant.
        pub federated_client_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            super::super::types::compute::DiskEncryptionSetIdentity,
        >,
        /// Specifies the URL to a Key Vault Key (either from a Key Vault Key, or the Key URL for the Key Vault Secret). Exactly one of `managed_hsm_key_id`, `key_vault_key_id` must be specified.
        ///
        /// > **NOTE** Access to the KeyVault must be granted for this Disk Encryption Set, if you want to further use this Disk Encryption Set in a Managed Disk or Virtual Machine, or Virtual Machine Scale Set. For instructions, please refer to the doc of [Server side encryption of Azure managed disks](https://docs.microsoft.com/azure/virtual-machines/linux/disk-encryption).
        ///
        /// > **NOTE** A KeyVault or Managed HSM using enable_rbac_authorization requires to use `azure.authorization.Assignment` to assign the role `Key Vault Crypto Service Encryption User` to this Disk Encryption Set.
        /// In this case, `azure.keyvault.AccessPolicy` is not needed.
        pub key_vault_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URL for the Key Vault Key or Key Vault Secret that is currently being used by the service.
        pub key_vault_key_url: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Azure Region where the Disk Encryption Set exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Key ID of a key in a managed HSM.  Exactly one of `managed_hsm_key_id`, `key_vault_key_id` must be specified.
        pub managed_hsm_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Disk Encryption Set. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group where the Disk Encryption Set should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the Disk Encryption Set.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DiskEncryptionSetArgs,
    ) -> DiskEncryptionSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_key_rotation_enabled_binding = args
            .auto_key_rotation_enabled
            .get_output(context)
            .get_inner();
        let encryption_type_binding = args
            .encryption_type
            .get_output(context)
            .get_inner();
        let federated_client_id_binding = args
            .federated_client_id
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let key_vault_key_id_binding = args
            .key_vault_key_id
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let managed_hsm_key_id_binding = args
            .managed_hsm_key_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/diskEncryptionSet:DiskEncryptionSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoKeyRotationEnabled".into(),
                    value: &auto_key_rotation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionType".into(),
                    value: &encryption_type_binding,
                },
                register_interface::ObjectField {
                    name: "federatedClientId".into(),
                    value: &federated_client_id_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultKeyId".into(),
                    value: &key_vault_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managedHsmKeyId".into(),
                    value: &managed_hsm_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DiskEncryptionSetResult {
            auto_key_rotation_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoKeyRotationEnabled"),
            ),
            encryption_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionType"),
            ),
            federated_client_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("federatedClientId"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            key_vault_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultKeyId"),
            ),
            key_vault_key_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultKeyUrl"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            managed_hsm_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedHsmKeyId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
