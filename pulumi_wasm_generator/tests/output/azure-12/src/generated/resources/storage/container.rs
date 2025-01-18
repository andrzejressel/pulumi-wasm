/// Manages a Container within an Azure Storage Account.
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestoraccount
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       tags:
///         environment: staging
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: vhds
///       storageAccountId: ${exampleAccount.id}
///       containerAccessType: private
/// ```
///
/// ## Import
///
/// Storage Containers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/container:Container container1 https://example.blob.core.windows.net/container
/// ```
///
pub mod container {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerArgs {
        /// The Access Level configured for this Container. Possible values are `blob`, `container` or `private`. Defaults to `private`.
        ///
        /// > **Note** When updating `container_access_type` for an existing storage container resource, Shared Key authentication will always be used, as AzureAD authentication is not supported.
        #[builder(into, default)]
        pub container_access_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The default encryption scope to use for blobs uploaded to this container. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub default_encryption_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to allow blobs to override the default encryption scope for this container. Can only be set when specifying `default_encryption_scope`. Defaults to `true`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub encryption_scope_override_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of MetaData for this Container. All metadata keys should be lowercase.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Container which should be created within the Storage Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Storage Account where the Container should be created. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of `storage_account_name` or `storage_account_id` must be specified. When specifying `storage_account_id` the resource will use the Resource Manager API, rather than the Data Plane API.
        #[builder(into, default)]
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Storage Account where the Container should be created. Changing this forces a new resource to be created. This property is deprecated in favour of `storage_account_id`.
        #[builder(into, default)]
        pub storage_account_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ContainerResult {
        /// The Access Level configured for this Container. Possible values are `blob`, `container` or `private`. Defaults to `private`.
        ///
        /// > **Note** When updating `container_access_type` for an existing storage container resource, Shared Key authentication will always be used, as AzureAD authentication is not supported.
        pub container_access_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The default encryption scope to use for blobs uploaded to this container. Changing this forces a new resource to be created.
        pub default_encryption_scope: pulumi_wasm_rust::Output<String>,
        /// Whether to allow blobs to override the default encryption scope for this container. Can only be set when specifying `default_encryption_scope`. Defaults to `true`. Changing this forces a new resource to be created.
        pub encryption_scope_override_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is there an Immutability Policy configured on this Storage Container?
        pub has_immutability_policy: pulumi_wasm_rust::Output<bool>,
        /// Is there a Legal Hold configured on this Storage Container?
        pub has_legal_hold: pulumi_wasm_rust::Output<bool>,
        /// A mapping of MetaData for this Container. All metadata keys should be lowercase.
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the Container which should be created within the Storage Account. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Resource Manager ID of this Storage Container.
        pub resource_manager_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Storage Account where the Container should be created. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** One of `storage_account_name` or `storage_account_id` must be specified. When specifying `storage_account_id` the resource will use the Resource Manager API, rather than the Data Plane API.
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Storage Account where the Container should be created. Changing this forces a new resource to be created. This property is deprecated in favour of `storage_account_id`.
        pub storage_account_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ContainerArgs) -> ContainerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_access_type_binding = args.container_access_type.get_inner();
        let default_encryption_scope_binding = args.default_encryption_scope.get_inner();
        let encryption_scope_override_enabled_binding = args
            .encryption_scope_override_enabled
            .get_inner();
        let metadata_binding = args.metadata.get_inner();
        let name_binding = args.name.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let storage_account_name_binding = args.storage_account_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/container:Container".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerAccessType".into(),
                    value: &container_access_type_binding,
                },
                register_interface::ObjectField {
                    name: "defaultEncryptionScope".into(),
                    value: &default_encryption_scope_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionScopeOverrideEnabled".into(),
                    value: &encryption_scope_override_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "containerAccessType".into(),
                },
                register_interface::ResultField {
                    name: "defaultEncryptionScope".into(),
                },
                register_interface::ResultField {
                    name: "encryptionScopeOverrideEnabled".into(),
                },
                register_interface::ResultField {
                    name: "hasImmutabilityPolicy".into(),
                },
                register_interface::ResultField {
                    name: "hasLegalHold".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceManagerId".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContainerResult {
            container_access_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerAccessType").unwrap(),
            ),
            default_encryption_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultEncryptionScope").unwrap(),
            ),
            encryption_scope_override_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionScopeOverrideEnabled").unwrap(),
            ),
            has_immutability_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasImmutabilityPolicy").unwrap(),
            ),
            has_legal_hold: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasLegalHold").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_manager_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceManagerId").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountName").unwrap(),
            ),
        }
    }
}
