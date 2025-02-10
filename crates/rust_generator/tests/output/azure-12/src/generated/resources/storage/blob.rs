/// Manages a Blob within a Storage Container.
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
///       name: examplestoracc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: content
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
///   exampleBlob:
///     type: azure:storage:Blob
///     name: example
///     properties:
///       name: my-awesome-content.zip
///       storageAccountName: ${exampleAccount.name}
///       storageContainerName: ${exampleContainer.name}
///       type: Block
///       source:
///         fn::FileAsset: some-local-file.zip
/// ```
///
/// ## Import
///
/// Storage Blob's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/blob:Blob blob1 https://example.blob.core.windows.net/container/blob.vhd
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod blob {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BlobArgs {
        /// The access tier of the storage blob. Possible values are `Archive`, `Cool` and `Hot`.
        #[builder(into, default)]
        pub access_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Controls the [cache control header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control) content of the response when blob is requested .
        #[builder(into, default)]
        pub cache_control: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The MD5 sum of the blob contents. Cannot be defined if `source_uri` is defined, or if blob type is Append or Page. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub content_md5: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The content type of the storage blob. Cannot be defined if `source_uri` is defined. Defaults to `application/octet-stream`.
        #[builder(into, default)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The encryption scope to use for this blob.
        #[builder(into, default)]
        pub encryption_scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of custom blob metadata.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the storage blob. Must be unique within the storage container the blob is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of workers per CPU core to run for concurrent uploads. Defaults to `8`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `parallelism` is only applicable for Page blobs - support for [Block Blobs is blocked on the upstream issue](https://github.com/tombuildsstuff/giovanni/issues/15).
        #[builder(into, default)]
        pub parallelism: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Used only for `page` blobs to specify the size in bytes of the blob to be created. Must be a multiple of 512. Defaults to `0`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `size` is required if `source_uri` is not set.
        #[builder(into, default)]
        pub size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// An absolute path to a file on the local system. This field cannot be specified for Append blobs and cannot be specified if `source_content` or `source_uri` is specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The content for this blob which should be defined inline. This field can only be specified for Block blobs and cannot be specified if `source` or `source_uri` is specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URI of an existing blob, or a file in the Azure File service, to use as the source contents for the blob to be created. Changing this forces a new resource to be created. This field cannot be specified for Append blobs and cannot be specified if `source` or `source_content` is specified.
        #[builder(into, default)]
        pub source_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the storage account in which to create the storage container. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the storage container in which this blob should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_container_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the storage blob to be created. Possible values are `Append`, `Block` or `Page`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BlobResult {
        /// The access tier of the storage blob. Possible values are `Archive`, `Cool` and `Hot`.
        pub access_tier: pulumi_gestalt_rust::Output<String>,
        /// Controls the [cache control header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control) content of the response when blob is requested .
        pub cache_control: pulumi_gestalt_rust::Output<Option<String>>,
        /// The MD5 sum of the blob contents. Cannot be defined if `source_uri` is defined, or if blob type is Append or Page. Changing this forces a new resource to be created.
        pub content_md5: pulumi_gestalt_rust::Output<Option<String>>,
        /// The content type of the storage blob. Cannot be defined if `source_uri` is defined. Defaults to `application/octet-stream`.
        pub content_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The encryption scope to use for this blob.
        pub encryption_scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of custom blob metadata.
        pub metadata: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the storage blob. Must be unique within the storage container the blob is located. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of workers per CPU core to run for concurrent uploads. Defaults to `8`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `parallelism` is only applicable for Page blobs - support for [Block Blobs is blocked on the upstream issue](https://github.com/tombuildsstuff/giovanni/issues/15).
        pub parallelism: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Used only for `page` blobs to specify the size in bytes of the blob to be created. Must be a multiple of 512. Defaults to `0`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `size` is required if `source_uri` is not set.
        pub size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// An absolute path to a file on the local system. This field cannot be specified for Append blobs and cannot be specified if `source_content` or `source_uri` is specified. Changing this forces a new resource to be created.
        pub source: pulumi_gestalt_rust::Output<Option<String>>,
        /// The content for this blob which should be defined inline. This field can only be specified for Block blobs and cannot be specified if `source` or `source_uri` is specified. Changing this forces a new resource to be created.
        pub source_content: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI of an existing blob, or a file in the Azure File service, to use as the source contents for the blob to be created. Changing this forces a new resource to be created. This field cannot be specified for Append blobs and cannot be specified if `source` or `source_content` is specified.
        pub source_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the storage account in which to create the storage container. Changing this forces a new resource to be created.
        pub storage_account_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the storage container in which this blob should be created. Changing this forces a new resource to be created.
        pub storage_container_name: pulumi_gestalt_rust::Output<String>,
        /// The type of the storage blob to be created. Possible values are `Append`, `Block` or `Page`. Changing this forces a new resource to be created.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The URL of the blob
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BlobArgs,
    ) -> BlobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_tier_binding = args.access_tier.get_output(context);
        let cache_control_binding = args.cache_control.get_output(context);
        let content_md5_binding = args.content_md5.get_output(context);
        let content_type_binding = args.content_type.get_output(context);
        let encryption_scope_binding = args.encryption_scope.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let parallelism_binding = args.parallelism.get_output(context);
        let size_binding = args.size.get_output(context);
        let source_binding = args.source.get_output(context);
        let source_content_binding = args.source_content.get_output(context);
        let source_uri_binding = args.source_uri.get_output(context);
        let storage_account_name_binding = args.storage_account_name.get_output(context);
        let storage_container_name_binding = args
            .storage_container_name
            .get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/blob:Blob".into(),
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
                    name: "source".into(),
                    value: source_binding.get_id(),
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
        BlobResult {
            access_tier: o.get_field("accessTier"),
            cache_control: o.get_field("cacheControl"),
            content_md5: o.get_field("contentMd5"),
            content_type: o.get_field("contentType"),
            encryption_scope: o.get_field("encryptionScope"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            parallelism: o.get_field("parallelism"),
            size: o.get_field("size"),
            source: o.get_field("source"),
            source_content: o.get_field("sourceContent"),
            source_uri: o.get_field("sourceUri"),
            storage_account_name: o.get_field("storageAccountName"),
            storage_container_name: o.get_field("storageContainerName"),
            type_: o.get_field("type"),
            url: o.get_field("url"),
        }
    }
}
