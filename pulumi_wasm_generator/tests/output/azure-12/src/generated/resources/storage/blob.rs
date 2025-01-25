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
pub mod blob {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BlobArgs {
        /// The access tier of the storage blob. Possible values are `Archive`, `Cool` and `Hot`.
        #[builder(into, default)]
        pub access_tier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Controls the [cache control header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control) content of the response when blob is requested .
        #[builder(into, default)]
        pub cache_control: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The MD5 sum of the blob contents. Cannot be defined if `source_uri` is defined, or if blob type is Append or Page. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub content_md5: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The content type of the storage blob. Cannot be defined if `source_uri` is defined. Defaults to `application/octet-stream`.
        #[builder(into, default)]
        pub content_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The encryption scope to use for this blob.
        #[builder(into, default)]
        pub encryption_scope: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of custom blob metadata.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the storage blob. Must be unique within the storage container the blob is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The number of workers per CPU core to run for concurrent uploads. Defaults to `8`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `parallelism` is only applicable for Page blobs - support for [Block Blobs is blocked on the upstream issue](https://github.com/tombuildsstuff/giovanni/issues/15).
        #[builder(into, default)]
        pub parallelism: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Used only for `page` blobs to specify the size in bytes of the blob to be created. Must be a multiple of 512. Defaults to `0`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `size` is required if `source_uri` is not set.
        #[builder(into, default)]
        pub size: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// An absolute path to a file on the local system. This field cannot be specified for Append blobs and cannot be specified if `source_content` or `source_uri` is specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The content for this blob which should be defined inline. This field can only be specified for Block blobs and cannot be specified if `source` or `source_uri` is specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_content: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The URI of an existing blob, or a file in the Azure File service, to use as the source contents for the blob to be created. Changing this forces a new resource to be created. This field cannot be specified for Append blobs and cannot be specified if `source` or `source_content` is specified.
        #[builder(into, default)]
        pub source_uri: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the storage account in which to create the storage container. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the storage container in which this blob should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_container_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of the storage blob to be created. Possible values are `Append`, `Block` or `Page`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct BlobResult {
        /// The access tier of the storage blob. Possible values are `Archive`, `Cool` and `Hot`.
        pub access_tier: pulumi_wasm_rust::Output<String>,
        /// Controls the [cache control header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control) content of the response when blob is requested .
        pub cache_control: pulumi_wasm_rust::Output<Option<String>>,
        /// The MD5 sum of the blob contents. Cannot be defined if `source_uri` is defined, or if blob type is Append or Page. Changing this forces a new resource to be created.
        pub content_md5: pulumi_wasm_rust::Output<Option<String>>,
        /// The content type of the storage blob. Cannot be defined if `source_uri` is defined. Defaults to `application/octet-stream`.
        pub content_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The encryption scope to use for this blob.
        pub encryption_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of custom blob metadata.
        pub metadata: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the storage blob. Must be unique within the storage container the blob is located. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of workers per CPU core to run for concurrent uploads. Defaults to `8`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `parallelism` is only applicable for Page blobs - support for [Block Blobs is blocked on the upstream issue](https://github.com/tombuildsstuff/giovanni/issues/15).
        pub parallelism: pulumi_wasm_rust::Output<Option<i32>>,
        /// Used only for `page` blobs to specify the size in bytes of the blob to be created. Must be a multiple of 512. Defaults to `0`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** `size` is required if `source_uri` is not set.
        pub size: pulumi_wasm_rust::Output<Option<i32>>,
        /// An absolute path to a file on the local system. This field cannot be specified for Append blobs and cannot be specified if `source_content` or `source_uri` is specified. Changing this forces a new resource to be created.
        pub source: pulumi_wasm_rust::Output<Option<String>>,
        /// The content for this blob which should be defined inline. This field can only be specified for Block blobs and cannot be specified if `source` or `source_uri` is specified. Changing this forces a new resource to be created.
        pub source_content: pulumi_wasm_rust::Output<Option<String>>,
        /// The URI of an existing blob, or a file in the Azure File service, to use as the source contents for the blob to be created. Changing this forces a new resource to be created. This field cannot be specified for Append blobs and cannot be specified if `source` or `source_content` is specified.
        pub source_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the storage account in which to create the storage container. Changing this forces a new resource to be created.
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
        /// The name of the storage container in which this blob should be created. Changing this forces a new resource to be created.
        pub storage_container_name: pulumi_wasm_rust::Output<String>,
        /// The type of the storage blob to be created. Possible values are `Append`, `Block` or `Page`. Changing this forces a new resource to be created.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The URL of the blob
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BlobArgs,
    ) -> BlobResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_tier_binding = args.access_tier.get_output(context).get_inner();
        let cache_control_binding = args.cache_control.get_output(context).get_inner();
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
        let source_binding = args.source.get_output(context).get_inner();
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
            type_: "azure:storage/blob:Blob".into(),
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
                    name: "source".into(),
                    value: &source_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessTier".into(),
                },
                register_interface::ResultField {
                    name: "cacheControl".into(),
                },
                register_interface::ResultField {
                    name: "contentMd5".into(),
                },
                register_interface::ResultField {
                    name: "contentType".into(),
                },
                register_interface::ResultField {
                    name: "encryptionScope".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parallelism".into(),
                },
                register_interface::ResultField {
                    name: "size".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
                register_interface::ResultField {
                    name: "sourceContent".into(),
                },
                register_interface::ResultField {
                    name: "sourceUri".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountName".into(),
                },
                register_interface::ResultField {
                    name: "storageContainerName".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BlobResult {
            access_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessTier").unwrap(),
            ),
            cache_control: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cacheControl").unwrap(),
            ),
            content_md5: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentMd5").unwrap(),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentType").unwrap(),
            ),
            encryption_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionScope").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parallelism: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parallelism").unwrap(),
            ),
            size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("size").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
            source_content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceContent").unwrap(),
            ),
            source_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceUri").unwrap(),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountName").unwrap(),
            ),
            storage_container_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageContainerName").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
