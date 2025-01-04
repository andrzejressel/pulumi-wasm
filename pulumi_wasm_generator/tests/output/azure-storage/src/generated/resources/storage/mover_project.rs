/// Manages a Storage Mover Project.
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
///             .location("${example.location}")
///             .name("example-ssm")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleMoverProject = mover_project::create(
///         "exampleMoverProject",
///         MoverProjectArgs::builder()
///             .description("Example Project Description")
///             .name("example-sp")
///             .storage_mover_id("${exampleMover.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Mover Project can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/moverProject:MoverProject example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.StorageMover/storageMovers/storageMover1/projects/project1
/// ```
///
pub mod mover_project {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MoverProjectArgs {
        /// Specifies a description for this Storage Mover Project.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Storage Mover Project. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the storage mover for this Storage Mover Project. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_mover_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MoverProjectResult {
        /// Specifies a description for this Storage Mover Project.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Storage Mover Project. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the storage mover for this Storage Mover Project. Changing this forces a new resource to be created.
        pub storage_mover_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MoverProjectArgs) -> MoverProjectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let storage_mover_id_binding = args.storage_mover_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/moverProject:MoverProject".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "storageMoverId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MoverProjectResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            storage_mover_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageMoverId").unwrap(),
            ),
        }
    }
}
