#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_storage_container {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStorageContainerArgs {
        /// A mapping of MetaData for this Container.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Container.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the Storage Account where the Container exists. This property will become Required in version 5.0 of the Provider.
        ///
        /// > **NOTE:** One of `storage_account_name` or `storage_account_id` must be specified. When specifying `storage_account_id` the resource will use the Resource Manager API, rather than the Data Plane API.
        #[builder(into, default)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Storage Account where the Container exists. This property is deprecated in favour of `storage_account_id`.
        #[builder(into, default)]
        pub storage_account_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetStorageContainerResult {
        /// The Access Level configured for this Container.
        pub container_access_type: pulumi_gestalt_rust::Output<String>,
        /// The default encryption scope in use for blobs uploaded to this container.
        pub default_encryption_scope: pulumi_gestalt_rust::Output<String>,
        /// Whether blobs are allowed to override the default encryption scope for this container.
        pub encryption_scope_override_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Is there an Immutability Policy configured on this Storage Container?
        pub has_immutability_policy: pulumi_gestalt_rust::Output<bool>,
        /// Is there a Legal Hold configured on this Storage Container?
        pub has_legal_hold: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of MetaData for this Container.
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_manager_id: pulumi_gestalt_rust::Output<String>,
        pub storage_account_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub storage_account_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetStorageContainerArgs,
    ) -> GetStorageContainerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let storage_account_id_binding = args.storage_account_id.get_output(context);
        let storage_account_name_binding = args.storage_account_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:storage/getStorageContainer:getStorageContainer".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountId".into(),
                    value: storage_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountName".into(),
                    value: storage_account_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetStorageContainerResult {
            container_access_type: o.get_field("containerAccessType"),
            default_encryption_scope: o.get_field("defaultEncryptionScope"),
            encryption_scope_override_enabled: o
                .get_field("encryptionScopeOverrideEnabled"),
            has_immutability_policy: o.get_field("hasImmutabilityPolicy"),
            has_legal_hold: o.get_field("hasLegalHold"),
            id: o.get_field("id"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            resource_manager_id: o.get_field("resourceManagerId"),
            storage_account_id: o.get_field("storageAccountId"),
            storage_account_name: o.get_field("storageAccountName"),
        }
    }
}
