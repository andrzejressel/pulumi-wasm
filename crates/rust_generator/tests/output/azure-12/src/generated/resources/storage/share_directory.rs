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
///             .name("azuretest")
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
///     let exampleShareDirectory = share_directory::create(
///         "exampleShareDirectory",
///         ShareDirectoryArgs::builder()
///             .name("example")
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
/// $ pulumi import azure:storage/shareDirectory:ShareDirectory example https://tomdevsa20.file.core.windows.net/share1/directory1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod share_directory {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ShareDirectoryArgs {
        /// A mapping of metadata to assign to this Directory.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name (or path) of the Directory that should be created within this File Share. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Storage Share ID in which this file will be placed into. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_share_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ShareDirectoryResult {
        /// A mapping of metadata to assign to this Directory.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name (or path) of the Directory that should be created within this File Share. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Storage Share ID in which this file will be placed into. Changing this forces a new resource to be created.
        pub storage_share_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ShareDirectoryArgs,
    ) -> ShareDirectoryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let metadata_binding_1 = args.metadata.get_output(context);
        let metadata_binding = metadata_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let storage_share_id_binding_1 = args.storage_share_id.get_output(context);
        let storage_share_id_binding = storage_share_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/shareDirectory:ShareDirectory".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "storageShareId".into(),
                    value: &storage_share_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ShareDirectoryResult {
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            storage_share_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageShareId"),
            ),
        }
    }
}
