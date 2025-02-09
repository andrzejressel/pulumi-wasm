/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("azureteststorage")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleShare = share::create(
///         "exampleShare",
///         ShareArgs::builder()
///             .name("sharename")
///             .quota(50)
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleShareFile = share_file::create(
///         "exampleShareFile",
///         ShareFileArgs::builder()
///             .name("my-awesome-content.zip")
///             .source("some-local-file.zip")
///             .storage_share_id("${exampleShare.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Directories within an Azure Storage File Share can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/shareFile:ShareFile example https://account1.file.core.windows.net/share1/file1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod share_file {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ShareFileArgs {
        /// Sets the file’s Content-Disposition header.
        #[builder(into, default)]
        pub content_disposition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies which content encodings have been applied to the file.
        #[builder(into, default)]
        pub content_encoding: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub content_md5: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The content type of the share file. Defaults to `application/octet-stream`.
        #[builder(into, default)]
        pub content_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of metadata to assign to this file.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name (or path) of the File that should be created within this File Share. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The storage share directory that you would like the file placed into. Changing this forces a new resource to be created. Defaults to `""`.
        #[builder(into, default)]
        pub path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An absolute path to a file on the local system. Changing this forces a new resource to be created.
        ///
        /// > **Note** The file specified with `source` can not be empty.
        #[builder(into, default)]
        pub source: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Storage Share ID in which this file will be placed into. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_share_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ShareFileResult {
        /// Sets the file’s Content-Disposition header.
        pub content_disposition: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies which content encodings have been applied to the file.
        pub content_encoding: pulumi_gestalt_rust::Output<Option<String>>,
        /// The length in bytes of the file content
        pub content_length: pulumi_gestalt_rust::Output<i32>,
        pub content_md5: pulumi_gestalt_rust::Output<Option<String>>,
        /// The content type of the share file. Defaults to `application/octet-stream`.
        pub content_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of metadata to assign to this file.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name (or path) of the File that should be created within this File Share. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The storage share directory that you would like the file placed into. Changing this forces a new resource to be created. Defaults to `""`.
        pub path: pulumi_gestalt_rust::Output<Option<String>>,
        /// An absolute path to a file on the local system. Changing this forces a new resource to be created.
        ///
        /// > **Note** The file specified with `source` can not be empty.
        pub source: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Storage Share ID in which this file will be placed into. Changing this forces a new resource to be created.
        pub storage_share_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ShareFileArgs,
    ) -> ShareFileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let content_disposition_binding = args.content_disposition.get_output(context);
        let content_encoding_binding = args.content_encoding.get_output(context);
        let content_md5_binding = args.content_md5.get_output(context);
        let content_type_binding = args.content_type.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let path_binding = args.path.get_output(context);
        let source_binding = args.source.get_output(context);
        let storage_share_id_binding = args.storage_share_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/shareFile:ShareFile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentDisposition".into(),
                    value: content_disposition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentEncoding".into(),
                    value: content_encoding_binding.get_id(),
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
                    name: "metadata".into(),
                    value: metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "path".into(),
                    value: path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageShareId".into(),
                    value: storage_share_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ShareFileResult {
            content_disposition: o.get_field("contentDisposition"),
            content_encoding: o.get_field("contentEncoding"),
            content_length: o.get_field("contentLength"),
            content_md5: o.get_field("contentMd5"),
            content_type: o.get_field("contentType"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            path: o.get_field("path"),
            source: o.get_field("source"),
            storage_share_id: o.get_field("storageShareId"),
        }
    }
}
