/// Manages a Key Vault Managed Hardware Security Module Key.
///
/// > **Note:** The Azure Provider includes a Feature Toggle which will purge a Key Vault Managed Hardware Security Module Key resource on destroy, rather than the default soft-delete. See `purge_soft_deleted_hardware_security_modules_on_destroy` for more information.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:keyvault:ManagedHardwareSecurityModule
///     properties:
///       name: example
///       resourceGroupName: ${exampleAzurermResourceGroup.name}
///       location: ${exampleAzurermResourceGroup.location}
///       skuName: Standard_B1
///       tenantId: ${current.tenantId}
///       adminObjectIds:
///         - ${current.objectId}
///       purgeProtectionEnabled: false
///       activeConfig:
///         - securityDomainCertificate:
///             - ${cert[0].id}
///             - ${cert[1].id}
///             - ${cert[2].id}
///           securityDomainQuorum: 2
///   # this gives your service principal the HSM Crypto User role which lets you create and destroy hsm keys
///   hsm-crypto-user:
///     type: azure:keyvault:ManagedHardwareSecurityModuleRoleAssignment
///     properties:
///       managedHsmId: ${test.id}
///       name: 1e243909-064c-6ac3-84e9-1c8bf8d6ad22
///       scope: /keys
///       roleDefinitionId: /Microsoft.KeyVault/providers/Microsoft.Authorization/roleDefinitions/21dbd100-6940-42c2-9190-5d6cb909625b
///       principalId: ${current.objectId}
///   # this gives your service principal the HSM Crypto Officer role which lets you purge hsm keys
///   hsm-crypto-officer:
///     type: azure:keyvault:ManagedHardwareSecurityModuleRoleAssignment
///     properties:
///       managedHsmId: ${test.id}
///       name: 1e243909-064c-6ac3-84e9-1c8bf8d6ad23
///       scope: /keys
///       roleDefinitionId: /Microsoft.KeyVault/providers/Microsoft.Authorization/roleDefinitions/515eb02d-2335-4d2d-92f2-b1cbdf9c3778
///       principalId: ${current.objectId}
///   exampleManagedHardwareSecurityModuleKey:
///     type: azure:keyvault:ManagedHardwareSecurityModuleKey
///     name: example
///     properties:
///       name: example
///       managedHsmId: ${test.id}
///       keyType: EC-HSM
///       curve: P-521
///       keyOpts:
///         - sign
///     options:
///       dependsOn:
///         - ${testAzurermKeyVaultManagedHardwareSecurityModuleRoleAssignment}
///         - ${test1}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Key Vault Managed Hardware Security Module Key can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/managedHardwareSecurityModuleKey:ManagedHardwareSecurityModuleKey example https://exampleHSM.managedhsm.azure.net/keys/exampleKey
/// ```
///
pub mod managed_hardware_security_module_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleKeyArgs {
        /// Specifies the curve to use when creating an `EC-HSM` key. Possible values are `P-256`, `P-256K`, `P-384`, and `P-521`. This field is required if `key_type` is `EC-HSM`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub curve: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Expiration UTC datetime (Y-m-d'T'H:M:S'Z'). When this parameter gets changed on reruns, if newer date is ahead of current date, an update is performed. If the newer date is before the current date, resource will be force created.
        #[builder(into, default)]
        pub expiration_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of JSON web key operations. Possible values include: `decrypt`, `encrypt`, `sign`, `unwrapKey`, `verify` and `wrapKey`. Please note these values are case-sensitive.
        #[builder(into)]
        pub key_opts: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Specifies the Size of the RSA key to create in bytes. For example, 1024 or 2048. *Note*: This field is required if `key_type` is `RSA-HSM` or `oct-HSM`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub key_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the Key Type to use for this Key Vault Managed Hardware Security Module Key. Possible values are `EC-HSM`, `oct-HSM` and `RSA-HSM`. More details see [HSM-protected keys](https://learn.microsoft.com/en-us/azure/key-vault/keys/about-keys#hsm-protected-keys). Changing this forces a new resource to be created.
        #[builder(into)]
        pub key_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the ID of the Key Vault Managed Hardware Security Module that they key will be owned by. Changing this forces a new resource to be created.
        #[builder(into)]
        pub managed_hsm_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Key Vault Managed Hardware Security Module Key. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key not usable before the provided UTC datetime (Y-m-d'T'H:M:S'Z').
        ///
        /// > **Note:** Once `expiration_date` is set, it's not possible to unset the key even if it is deleted & recreated as underlying Azure API uses the restore of the purged key.
        #[builder(into, default)]
        pub not_before_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ManagedHardwareSecurityModuleKeyResult {
        /// Specifies the curve to use when creating an `EC-HSM` key. Possible values are `P-256`, `P-256K`, `P-384`, and `P-521`. This field is required if `key_type` is `EC-HSM`. Changing this forces a new resource to be created.
        pub curve: pulumi_gestalt_rust::Output<Option<String>>,
        /// Expiration UTC datetime (Y-m-d'T'H:M:S'Z'). When this parameter gets changed on reruns, if newer date is ahead of current date, an update is performed. If the newer date is before the current date, resource will be force created.
        pub expiration_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of JSON web key operations. Possible values include: `decrypt`, `encrypt`, `sign`, `unwrapKey`, `verify` and `wrapKey`. Please note these values are case-sensitive.
        pub key_opts: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the Size of the RSA key to create in bytes. For example, 1024 or 2048. *Note*: This field is required if `key_type` is `RSA-HSM` or `oct-HSM`. Changing this forces a new resource to be created.
        pub key_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the Key Type to use for this Key Vault Managed Hardware Security Module Key. Possible values are `EC-HSM`, `oct-HSM` and `RSA-HSM`. More details see [HSM-protected keys](https://learn.microsoft.com/en-us/azure/key-vault/keys/about-keys#hsm-protected-keys). Changing this forces a new resource to be created.
        pub key_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Key Vault Managed Hardware Security Module that they key will be owned by. Changing this forces a new resource to be created.
        pub managed_hsm_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Key Vault Managed Hardware Security Module Key. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key not usable before the provided UTC datetime (Y-m-d'T'H:M:S'Z').
        ///
        /// > **Note:** Once `expiration_date` is set, it's not possible to unset the key even if it is deleted & recreated as underlying Azure API uses the restore of the purged key.
        pub not_before_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The versioned Key Vault Secret Managed Hardware Security Module Key ID.
        pub versioned_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ManagedHardwareSecurityModuleKeyArgs,
    ) -> ManagedHardwareSecurityModuleKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let curve_binding = args.curve.get_output(context).get_inner();
        let expiration_date_binding = args
            .expiration_date
            .get_output(context)
            .get_inner();
        let key_opts_binding = args.key_opts.get_output(context).get_inner();
        let key_size_binding = args.key_size.get_output(context).get_inner();
        let key_type_binding = args.key_type.get_output(context).get_inner();
        let managed_hsm_id_binding = args.managed_hsm_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let not_before_date_binding = args
            .not_before_date
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:keyvault/managedHardwareSecurityModuleKey:ManagedHardwareSecurityModuleKey"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "curve".into(),
                    value: &curve_binding,
                },
                register_interface::ObjectField {
                    name: "expirationDate".into(),
                    value: &expiration_date_binding,
                },
                register_interface::ObjectField {
                    name: "keyOpts".into(),
                    value: &key_opts_binding,
                },
                register_interface::ObjectField {
                    name: "keySize".into(),
                    value: &key_size_binding,
                },
                register_interface::ObjectField {
                    name: "keyType".into(),
                    value: &key_type_binding,
                },
                register_interface::ObjectField {
                    name: "managedHsmId".into(),
                    value: &managed_hsm_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notBeforeDate".into(),
                    value: &not_before_date_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ManagedHardwareSecurityModuleKeyResult {
            curve: pulumi_gestalt_rust::__private::into_domain(o.extract_field("curve")),
            expiration_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expirationDate"),
            ),
            key_opts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyOpts"),
            ),
            key_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keySize"),
            ),
            key_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyType"),
            ),
            managed_hsm_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedHsmId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            not_before_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notBeforeDate"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            versioned_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionedId"),
            ),
        }
    }
}
