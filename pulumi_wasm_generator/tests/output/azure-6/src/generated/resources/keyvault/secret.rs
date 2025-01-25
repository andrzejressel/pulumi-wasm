/// Manages a Key Vault Secret.
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
///             - Get
///           secretPermissions:
///             - Set
///             - Get
///             - Delete
///             - Purge
///             - Recover
///   exampleSecret:
///     type: azure:keyvault:Secret
///     name: example
///     properties:
///       name: secret-sauce
///       value: szechuan
///       keyVaultId: ${exampleKeyVault.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Key Vault Secrets which are Enabled can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:keyvault/secret:Secret example "https://example-keyvault.vault.azure.net/secrets/example/fdf067c93bbb4b22bff4d8b7a9a56217"
/// ```
///
pub mod secret {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretArgs {
        /// Specifies the content type for the Key Vault Secret.
        #[builder(into, default)]
        pub content_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Expiration UTC datetime (Y-m-d'T'H:M:S'Z').
        #[builder(into, default)]
        pub expiration_date: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Key Vault where the Secret should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key_vault_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Key Vault Secret. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key not usable before the provided UTC datetime (Y-m-d'T'H:M:S'Z').
        #[builder(into, default)]
        pub not_before_date: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the value of the Key Vault Secret. Changing this will create a new version of the Key Vault Secret.
        ///
        /// > **Note:** Key Vault strips newlines. To preserve newlines in multi-line secrets try replacing them with `\n` or by base 64 encoding them with `replace(file("my_secret_file"), "/\n/", "\n")` or `base64encode(file("my_secret_file"))`, respectively.
        #[builder(into)]
        pub value: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SecretResult {
        /// Specifies the content type for the Key Vault Secret.
        pub content_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Expiration UTC datetime (Y-m-d'T'H:M:S'Z').
        pub expiration_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Key Vault where the Secret should be created. Changing this forces a new resource to be created.
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Key Vault Secret. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key not usable before the provided UTC datetime (Y-m-d'T'H:M:S'Z').
        pub not_before_date: pulumi_wasm_rust::Output<Option<String>>,
        /// The (Versioned) ID for this Key Vault Secret. This property points to a specific version of a Key Vault Secret, as such using this won't auto-rotate values if used in other Azure Services.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// The Versionless ID of the Key Vault Secret. This property allows other Azure Services (that support it) to auto-rotate their value when the Key Vault Secret is updated.
        pub resource_versionless_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the value of the Key Vault Secret. Changing this will create a new version of the Key Vault Secret.
        ///
        /// > **Note:** Key Vault strips newlines. To preserve newlines in multi-line secrets try replacing them with `\n` or by base 64 encoding them with `replace(file("my_secret_file"), "/\n/", "\n")` or `base64encode(file("my_secret_file"))`, respectively.
        pub value: pulumi_wasm_rust::Output<String>,
        /// The current version of the Key Vault Secret.
        pub version: pulumi_wasm_rust::Output<String>,
        /// The Base ID of the Key Vault Secret.
        pub versionless_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SecretArgs,
    ) -> SecretResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_type_binding = args.content_type.get_output(context).get_inner();
        let expiration_date_binding = args
            .expiration_date
            .get_output(context)
            .get_inner();
        let key_vault_id_binding = args.key_vault_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let not_before_date_binding = args
            .not_before_date
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let value_binding = args.value.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:keyvault/secret:Secret".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding,
                },
                register_interface::ObjectField {
                    name: "expirationDate".into(),
                    value: &expiration_date_binding,
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "contentType".into(),
                },
                register_interface::ResultField {
                    name: "expirationDate".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notBeforeDate".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "resourceVersionlessId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "value".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "versionlessId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SecretResult {
            content_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentType").unwrap(),
            ),
            expiration_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expirationDate").unwrap(),
            ),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            not_before_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notBeforeDate").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            resource_versionless_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceVersionlessId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            versionless_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionlessId").unwrap(),
            ),
        }
    }
}
