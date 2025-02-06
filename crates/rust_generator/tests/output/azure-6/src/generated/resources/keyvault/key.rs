/// Manages a Key Vault Key.
///
/// ## Example Usage
///
/// > **Note:** To use this resource, your client should have RBAC roles with permissions like `Key Vault Crypto Officer` or `Key Vault Administrator` or an assigned Key Vault Access Policy with permissions `Create`,`Delete`,`Get`,`Purge`,`Recover`,`Update` and `GetRotationPolicy` for keys without Rotation Policy. Include `SetRotationPolicy` for keys with Rotation Policy.
///
/// > **Note:** The Azure Provider includes a Feature Toggle which will purge a Key Vault Key resource on destroy, rather than the default soft-delete. See `purge_soft_deleted_keys_on_destroy` for more information.
///
///
/// ### Additional Examples
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
///       name: examplekeyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       softDeleteRetentionDays: 7
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           keyPermissions:
///             - Create
///             - Delete
///             - Get
///             - Purge
///             - Recover
///             - Update
///             - GetRotationPolicy
///             - SetRotationPolicy
///           secretPermissions:
///             - Set
///   generated:
///     type: azure:keyvault:Key
///     properties:
///       name: generated-certificate
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
///       rotationPolicy:
///         automatic:
///           timeBeforeExpiry: P30D
///         expireAfter: P90D
///         notifyBeforeExpiry: P29D
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Key Vault Key which is Enabled can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/key:Key example "https://example-keyvault.vault.azure.net/keys/example/fdf067c93bbb4b22bff4d8b7a9a56217"
/// ```
///
pub mod key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyArgs {
        /// Specifies the curve to use when creating an `EC` key. Possible values are `P-256`, `P-256K`, `P-384`, and `P-521`. This field will be required in a future release if `key_type` is `EC` or `EC-HSM`. The API will default to `P-256` if nothing is specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub curve: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Expiration UTC datetime (Y-m-d'T'H:M:S'Z').
        ///
        /// > **Note:** Removing this field from the config forces a new resource to be created.
        #[builder(into, default)]
        pub expiration_date: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of JSON web key operations. Possible values include: `decrypt`, `encrypt`, `sign`, `unwrapKey`, `verify` and `wrapKey`. Please note these values are case sensitive.
        #[builder(into)]
        pub key_opts: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Specifies the Size of the RSA key to create in bytes. For example, 1024 or 2048. *Note*: This field is required if `key_type` is `RSA` or `RSA-HSM`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub key_size: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Specifies the Key Type to use for this Key Vault Key. Possible values are `EC` (Elliptic Curve), `EC-HSM`, `RSA` and `RSA-HSM`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Key Vault where the Key should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key_vault_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Key Vault Key. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key not usable before the provided UTC datetime (Y-m-d'T'H:M:S'Z').
        ///
        /// > **Note:** Once `expiration_date` is set, it's not possible to unset the key even if it is deleted & recreated as underlying Azure API uses the restore of the purged key.
        #[builder(into, default)]
        pub not_before_date: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `rotation_policy` block as defined below.
        #[builder(into, default)]
        pub rotation_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::keyvault::KeyRotationPolicy>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct KeyResult {
        /// Specifies the curve to use when creating an `EC` key. Possible values are `P-256`, `P-256K`, `P-384`, and `P-521`. This field will be required in a future release if `key_type` is `EC` or `EC-HSM`. The API will default to `P-256` if nothing is specified. Changing this forces a new resource to be created.
        pub curve: pulumi_wasm_rust::Output<String>,
        /// The RSA public exponent of this Key Vault Key.
        pub e: pulumi_wasm_rust::Output<String>,
        /// Expiration UTC datetime (Y-m-d'T'H:M:S'Z').
        ///
        /// > **Note:** Removing this field from the config forces a new resource to be created.
        pub expiration_date: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of JSON web key operations. Possible values include: `decrypt`, `encrypt`, `sign`, `unwrapKey`, `verify` and `wrapKey`. Please note these values are case sensitive.
        pub key_opts: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the Size of the RSA key to create in bytes. For example, 1024 or 2048. *Note*: This field is required if `key_type` is `RSA` or `RSA-HSM`. Changing this forces a new resource to be created.
        pub key_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the Key Type to use for this Key Vault Key. Possible values are `EC` (Elliptic Curve), `EC-HSM`, `RSA` and `RSA-HSM`. Changing this forces a new resource to be created.
        pub key_type: pulumi_wasm_rust::Output<String>,
        /// The ID of the Key Vault where the Key should be created. Changing this forces a new resource to be created.
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// The RSA modulus of this Key Vault Key.
        pub n: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Key Vault Key. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key not usable before the provided UTC datetime (Y-m-d'T'H:M:S'Z').
        ///
        /// > **Note:** Once `expiration_date` is set, it's not possible to unset the key even if it is deleted & recreated as underlying Azure API uses the restore of the purged key.
        pub not_before_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The OpenSSH encoded public key of this Key Vault Key.
        pub public_key_openssh: pulumi_wasm_rust::Output<String>,
        /// The PEM encoded public key of this Key Vault Key.
        pub public_key_pem: pulumi_wasm_rust::Output<String>,
        /// The (Versioned) ID for this Key Vault Key. This property points to a specific version of a Key Vault Key, as such using this won't auto-rotate values if used in other Azure Services.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// The Versionless ID of the Key Vault Key. This property allows other Azure Services (that support it) to auto-rotate their value when the Key Vault Key is updated.
        pub resource_versionless_id: pulumi_wasm_rust::Output<String>,
        /// A `rotation_policy` block as defined below.
        pub rotation_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::keyvault::KeyRotationPolicy>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The current version of the Key Vault Key.
        pub version: pulumi_wasm_rust::Output<String>,
        /// The Base ID of the Key Vault Key.
        pub versionless_id: pulumi_wasm_rust::Output<String>,
        /// The EC X component of this Key Vault Key.
        pub x: pulumi_wasm_rust::Output<String>,
        /// The EC Y component of this Key Vault Key.
        pub y: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: KeyArgs,
    ) -> KeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let curve_binding = args.curve.get_output(context).get_inner();
        let expiration_date_binding = args
            .expiration_date
            .get_output(context)
            .get_inner();
        let key_opts_binding = args.key_opts.get_output(context).get_inner();
        let key_size_binding = args.key_size.get_output(context).get_inner();
        let key_type_binding = args.key_type.get_output(context).get_inner();
        let key_vault_id_binding = args.key_vault_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let not_before_date_binding = args
            .not_before_date
            .get_output(context)
            .get_inner();
        let rotation_policy_binding = args
            .rotation_policy
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:keyvault/key:Key".into(),
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
                    name: "keyVaultId".into(),
                    value: &key_vault_id_binding,
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
                    name: "rotationPolicy".into(),
                    value: &rotation_policy_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        KeyResult {
            curve: pulumi_wasm_rust::__private::into_domain(o.extract_field("curve")),
            e: pulumi_wasm_rust::__private::into_domain(o.extract_field("e")),
            expiration_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expirationDate"),
            ),
            key_opts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyOpts"),
            ),
            key_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keySize"),
            ),
            key_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyType"),
            ),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keyVaultId"),
            ),
            n: pulumi_wasm_rust::__private::into_domain(o.extract_field("n")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            not_before_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notBeforeDate"),
            ),
            public_key_openssh: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicKeyOpenssh"),
            ),
            public_key_pem: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicKeyPem"),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            resource_versionless_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceVersionlessId"),
            ),
            rotation_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rotationPolicy"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            versionless_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionlessId"),
            ),
            x: pulumi_wasm_rust::__private::into_domain(o.extract_field("x")),
            y: pulumi_wasm_rust::__private::into_domain(o.extract_field("y")),
        }
    }
}
