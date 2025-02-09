#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zip_blob {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZipBlobArgs {
        #[builder(into, default)]
        pub access_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub cache_control: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub content_md5: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub encryption_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub parallelism: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub source_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub source_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub storage_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub storage_container_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZipBlobResult {
        pub access_tier: pulumi_gestalt_rust::Output<String>,
        pub cache_control: pulumi_gestalt_rust::Output<Option<String>>,
        pub content: pulumi_gestalt_rust::Output<Option<String>>,
        pub content_md5: pulumi_gestalt_rust::Output<Option<String>>,
        pub content_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub encryption_scope: pulumi_gestalt_rust::Output<Option<String>>,
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub parallelism: pulumi_gestalt_rust::Output<Option<i32>>,
        pub size: pulumi_gestalt_rust::Output<Option<i32>>,
        pub source_content: pulumi_gestalt_rust::Output<Option<String>>,
        pub source_uri: pulumi_gestalt_rust::Output<Option<String>>,
        pub storage_account_name: pulumi_gestalt_rust::Output<String>,
        pub storage_container_name: pulumi_gestalt_rust::Output<String>,
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZipBlobArgs,
    ) -> ZipBlobResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_tier_binding = args.access_tier.get_output(context);
        let cache_control_binding = args.cache_control.get_output(context);
        let content_binding = args.content.get_output(context);
        let content_md5_binding = args.content_md5.get_output(context);
        let content_type_binding = args.content_type.get_output(context);
        let encryption_scope_binding = args.encryption_scope.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let parallelism_binding = args.parallelism.get_output(context);
        let size_binding = args.size.get_output(context);
        let source_content_binding = args.source_content.get_output(context);
        let source_uri_binding = args.source_uri.get_output(context);
        let storage_account_name_binding = args.storage_account_name.get_output(context);
        let storage_container_name_binding = args
            .storage_container_name
            .get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/zipBlob:ZipBlob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessTier".into(),
                    value: access_tier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cacheControl".into(),
                    value: cache_control_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "content".into(),
                    value: content_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentMd5".into(),
                    value: content_md5_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentType".into(),
                    value: content_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionScope".into(),
                    value: encryption_scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parallelism".into(),
                    value: parallelism_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "size".into(),
                    value: size_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceContent".into(),
                    value: source_content_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceUri".into(),
                    value: source_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountName".into(),
                    value: storage_account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageContainerName".into(),
                    value: storage_container_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZipBlobResult {
            access_tier: o.get_field("accessTier"),
            cache_control: o.get_field("cacheControl"),
            content: o.get_field("content"),
            content_md5: o.get_field("contentMd5"),
            content_type: o.get_field("contentType"),
            encryption_scope: o.get_field("encryptionScope"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            parallelism: o.get_field("parallelism"),
            size: o.get_field("size"),
            source_content: o.get_field("sourceContent"),
            source_uri: o.get_field("sourceUri"),
            storage_account_name: o.get_field("storageAccountName"),
            storage_container_name: o.get_field("storageContainerName"),
            type_: o.get_field("type"),
            url: o.get_field("url"),
        }
    }
}
