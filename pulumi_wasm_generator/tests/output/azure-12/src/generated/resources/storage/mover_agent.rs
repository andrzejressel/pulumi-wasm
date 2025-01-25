/// Manages a Storage Mover Agent.
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
///             .location("East US")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleMover = mover::create(
///         "exampleMover",
///         MoverArgs::builder()
///             .name("example-ssm")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleMoverAgent = mover_agent::create(
///         "exampleMoverAgent",
///         MoverAgentArgs::builder()
///             .arc_virtual_machine_id(
///                 "${example.id}/providers/Microsoft.HybridCompute/machines/examples-hybridComputeName",
///             )
///             .arc_virtual_machine_uuid("3bb2c024-eba9-4d18-9e7a-1d772fcc5fe9")
///             .description("Example Agent Description")
///             .name("example-sa")
///             .storage_mover_id("${exampleMover.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Mover Agent can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/moverAgent:MoverAgent example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.StorageMover/storageMovers/storageMover1/agents/agent1
/// ```
///
pub mod mover_agent {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MoverAgentArgs {
        /// Specifies the fully qualified ID of the Hybrid Compute resource for the Storage Mover Agent. Changing this forces a new resource to be created.
        #[builder(into)]
        pub arc_virtual_machine_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the Hybrid Compute resource's unique SMBIOS ID. Changing this forces a new resource to be created.
        #[builder(into)]
        pub arc_virtual_machine_uuid: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies a description for this Storage Mover Agent.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Storage Mover Agent. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Storage Mover that this Agent should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_mover_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MoverAgentResult {
        /// Specifies the fully qualified ID of the Hybrid Compute resource for the Storage Mover Agent. Changing this forces a new resource to be created.
        pub arc_virtual_machine_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Hybrid Compute resource's unique SMBIOS ID. Changing this forces a new resource to be created.
        pub arc_virtual_machine_uuid: pulumi_wasm_rust::Output<String>,
        /// Specifies a description for this Storage Mover Agent.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Storage Mover Agent. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Storage Mover that this Agent should be connected to. Changing this forces a new resource to be created.
        pub storage_mover_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MoverAgentArgs,
    ) -> MoverAgentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arc_virtual_machine_id_binding = args
            .arc_virtual_machine_id
            .get_output(context)
            .get_inner();
        let arc_virtual_machine_uuid_binding = args
            .arc_virtual_machine_uuid
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let storage_mover_id_binding = args
            .storage_mover_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/moverAgent:MoverAgent".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arcVirtualMachineId".into(),
                    value: &arc_virtual_machine_id_binding,
                },
                register_interface::ObjectField {
                    name: "arcVirtualMachineUuid".into(),
                    value: &arc_virtual_machine_uuid_binding,
                },
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
                    name: "arcVirtualMachineId".into(),
                },
                register_interface::ResultField {
                    name: "arcVirtualMachineUuid".into(),
                },
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
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MoverAgentResult {
            arc_virtual_machine_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arcVirtualMachineId").unwrap(),
            ),
            arc_virtual_machine_uuid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arcVirtualMachineUuid").unwrap(),
            ),
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
