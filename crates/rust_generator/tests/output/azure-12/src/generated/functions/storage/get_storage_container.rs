pub mod get_storage_container {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStorageContainerArgs {
        /// A mapping of MetaData for this Container.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Container.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The id of the Storage Account where the Container exists. This property will become Required in version 5.0 of the Provider.
        ///
        /// > **NOTE:** One of `storage_account_name` or `storage_account_id` must be specified. When specifying `storage_account_id` the resource will use the Resource Manager API, rather than the Data Plane API.
        #[builder(into, default)]
        pub storage_account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Storage Account where the Container exists. This property is deprecated in favour of `storage_account_id`.
        #[builder(into, default)]
        pub storage_account_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetStorageContainerResult {
        /// The Access Level configured for this Container.
        pub container_access_type: pulumi_wasm_rust::Output<String>,
        /// The default encryption scope in use for blobs uploaded to this container.
        pub default_encryption_scope: pulumi_wasm_rust::Output<String>,
        /// Whether blobs are allowed to override the default encryption scope for this container.
        pub encryption_scope_override_enabled: pulumi_wasm_rust::Output<bool>,
        /// Is there an Immutability Policy configured on this Storage Container?
        pub has_immutability_policy: pulumi_wasm_rust::Output<bool>,
        /// Is there a Legal Hold configured on this Storage Container?
        pub has_legal_hold: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A mapping of MetaData for this Container.
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_manager_id: pulumi_wasm_rust::Output<String>,
        pub storage_account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub storage_account_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetStorageContainerArgs,
    ) -> GetStorageContainerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let storage_account_id_binding = args
            .storage_account_id
            .get_output(context)
            .get_inner();
        let storage_account_name_binding = args
            .storage_account_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:storage/getStorageContainer:getStorageContainer".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetStorageContainerResult {
            container_access_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("containerAccessType"),
            ),
            default_encryption_scope: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultEncryptionScope"),
            ),
            encryption_scope_override_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptionScopeOverrideEnabled"),
            ),
            has_immutability_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hasImmutabilityPolicy"),
            ),
            has_legal_hold: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hasLegalHold"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            metadata: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_manager_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceManagerId"),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("storageAccountName"),
            ),
        }
    }
}
