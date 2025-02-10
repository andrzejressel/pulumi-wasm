/// Manages a Storage Mover Source Endpoint.
///
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
///     let exampleMover = mover::create(
///         "exampleMover",
///         MoverArgs::builder()
///             .location("West Europe")
///             .name("example-ssm")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleMoverSourceEndpoint = mover_source_endpoint::create(
///         "exampleMoverSourceEndpoint",
///         MoverSourceEndpointArgs::builder()
///             .export("/")
///             .host("192.168.0.1")
///             .name("example-se")
///             .nfs_version("NFSv3")
///             .storage_mover_id("${exampleMover.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Mover Source Endpoint can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/moverSourceEndpoint:MoverSourceEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.StorageMover/storageMovers/storageMover1/endpoints/endpoint1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mover_source_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MoverSourceEndpointArgs {
        /// Specifies a description for the Storage Mover Source Endpoint.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the directory being exported from the server. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub export: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the host name or IP address of the server exporting the file system. Changing this forces a new resource to be created.
        #[builder(into)]
        pub host: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the NFS protocol version. Possible values are `NFSauto`, `NFSv3` and `NFSv4`. Defaults to `NFSauto`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub nfs_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Storage Mover for this Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_mover_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MoverSourceEndpointResult {
        /// Specifies a description for the Storage Mover Source Endpoint.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the directory being exported from the server. Changing this forces a new resource to be created.
        pub export: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the host name or IP address of the server exporting the file system. Changing this forces a new resource to be created.
        pub host: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the NFS protocol version. Possible values are `NFSauto`, `NFSv3` and `NFSv4`. Defaults to `NFSauto`. Changing this forces a new resource to be created.
        pub nfs_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the ID of the Storage Mover for this Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        pub storage_mover_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MoverSourceEndpointArgs,
    ) -> MoverSourceEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let export_binding = args.export.get_output(context);
        let host_binding = args.host.get_output(context);
        let name_binding = args.name.get_output(context);
        let nfs_version_binding = args.nfs_version.get_output(context);
        let storage_mover_id_binding = args.storage_mover_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/moverSourceEndpoint:MoverSourceEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "export".into(),
                    value: export_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "host".into(),
                    value: host_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nfsVersion".into(),
                    value: nfs_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageMoverId".into(),
                    value: storage_mover_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MoverSourceEndpointResult {
            description: o.get_field("description"),
            export: o.get_field("export"),
            host: o.get_field("host"),
            name: o.get_field("name"),
            nfs_version: o.get_field("nfsVersion"),
            storage_mover_id: o.get_field("storageMoverId"),
        }
    }
}
