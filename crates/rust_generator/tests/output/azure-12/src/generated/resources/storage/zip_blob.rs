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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ZipBlobArgs,
    ) -> ZipBlobResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let access_tier_binding = args.access_tier.get_output(context).get_inner();
        let cache_control_binding = args.cache_control.get_output(context).get_inner();
        let content_binding = args.content.get_output(context).get_inner();
        let content_md5_binding = args.content_md5.get_output(context).get_inner();
        let content_type_binding = args.content_type.get_output(context).get_inner();
        let encryption_scope_binding = args
            .encryption_scope
            .get_output(context)
            .get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parallelism_binding = args.parallelism.get_output(context).get_inner();
        let size_binding = args.size.get_output(context).get_inner();
        let source_content_binding = args.source_content.get_output(context).get_inner();
        let source_uri_binding = args.source_uri.get_output(context).get_inner();
        let storage_account_name_binding = args
            .storage_account_name
            .get_output(context)
            .get_inner();
        let storage_container_name_binding = args
            .storage_container_name
            .get_output(context)
            .get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/zipBlob:ZipBlob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessTier".into(),
                    value: &access_tier_binding,
                },
                register_interface::ObjectField {
                    name: "cacheControl".into(),
                    value: &cache_control_binding,
                },
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "contentMd5".into(),
                    value: &content_md5_binding,
                },
                register_interface::ObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionScope".into(),
                    value: &encryption_scope_binding,
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
                    name: "parallelism".into(),
                    value: &parallelism_binding,
                },
                register_interface::ObjectField {
                    name: "size".into(),
                    value: &size_binding,
                },
                register_interface::ObjectField {
                    name: "sourceContent".into(),
                    value: &source_content_binding,
                },
                register_interface::ObjectField {
                    name: "sourceUri".into(),
                    value: &source_uri_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageContainerName".into(),
                    value: &storage_container_name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZipBlobResult {
            access_tier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessTier"),
            ),
            cache_control: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cacheControl"),
            ),
            content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            content_md5: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentMd5"),
            ),
            content_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentType"),
            ),
            encryption_scope: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionScope"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parallelism: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parallelism"),
            ),
            size: pulumi_gestalt_rust::__private::into_domain(o.extract_field("size")),
            source_content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceContent"),
            ),
            source_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceUri"),
            ),
            storage_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountName"),
            ),
            storage_container_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageContainerName"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            url: pulumi_gestalt_rust::__private::into_domain(o.extract_field("url")),
        }
    }
}
