#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_blob {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBlobArgs {
        /// A map of custom blob metadata.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the Blob.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Storage Account where the Container exists.
        #[builder(into)]
        pub storage_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Storage Container where the Blob exists.
        #[builder(into)]
        pub storage_container_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetBlobResult {
        /// The access tier of the storage blob.
        pub access_tier: pulumi_gestalt_rust::Output<String>,
        /// The MD5 sum of the blob contents.
        pub content_md5: pulumi_gestalt_rust::Output<String>,
        /// The content type of the storage blob.
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// The encryption scope for this blob.
        pub encryption_scope: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A map of custom blob metadata.
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub storage_account_name: pulumi_gestalt_rust::Output<String>,
        pub storage_container_name: pulumi_gestalt_rust::Output<String>,
        /// The type of the storage blob
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The URL of the storage blob.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBlobArgs,
    ) -> GetBlobResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let storage_account_name_binding = args.storage_account_name.get_output(context);
        let storage_container_name_binding = args
            .storage_container_name
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:storage/getBlob:getBlob".into(),
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
                    name: "storageAccountName".into(),
                    value: storage_account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageContainerName".into(),
                    value: storage_container_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBlobResult {
            access_tier: o.get_field("accessTier"),
            content_md5: o.get_field("contentMd5"),
            content_type: o.get_field("contentType"),
            encryption_scope: o.get_field("encryptionScope"),
            id: o.get_field("id"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            storage_account_name: o.get_field("storageAccountName"),
            storage_container_name: o.get_field("storageContainerName"),
            type_: o.get_field("type"),
            url: o.get_field("url"),
        }
    }
}
