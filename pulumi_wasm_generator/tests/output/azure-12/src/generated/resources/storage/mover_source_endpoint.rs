/// Manages a Storage Mover Source Endpoint.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod mover_source_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MoverSourceEndpointArgs {
        /// Specifies a description for the Storage Mover Source Endpoint.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the directory being exported from the server. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub export: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the host name or IP address of the server exporting the file system. Changing this forces a new resource to be created.
        #[builder(into)]
        pub host: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the NFS protocol version. Possible values are `NFSauto`, `NFSv3` and `NFSv4`. Defaults to `NFSauto`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub nfs_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Storage Mover for this Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_mover_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MoverSourceEndpointResult {
        /// Specifies a description for the Storage Mover Source Endpoint.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the directory being exported from the server. Changing this forces a new resource to be created.
        pub export: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the host name or IP address of the server exporting the file system. Changing this forces a new resource to be created.
        pub host: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the NFS protocol version. Possible values are `NFSauto`, `NFSv3` and `NFSv4`. Defaults to `NFSauto`. Changing this forces a new resource to be created.
        pub nfs_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the Storage Mover for this Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        pub storage_mover_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MoverSourceEndpointArgs,
    ) -> MoverSourceEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let export_binding = args.export.get_output(context).get_inner();
        let host_binding = args.host.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let nfs_version_binding = args.nfs_version.get_output(context).get_inner();
        let storage_mover_id_binding = args
            .storage_mover_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/moverSourceEndpoint:MoverSourceEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "export".into(),
                    value: &export_binding,
                },
                register_interface::ObjectField {
                    name: "host".into(),
                    value: &host_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nfsVersion".into(),
                    value: &nfs_version_binding,
                },
                register_interface::ObjectField {
                    name: "storageMoverId".into(),
                    value: &storage_mover_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "export".into(),
                },
                register_interface::ResultField {
                    name: "host".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nfsVersion".into(),
                },
                register_interface::ResultField {
                    name: "storageMoverId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MoverSourceEndpointResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            export: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("export").unwrap(),
            ),
            host: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("host").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nfs_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nfsVersion").unwrap(),
            ),
            storage_mover_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageMoverId").unwrap(),
            ),
        }
    }
}
