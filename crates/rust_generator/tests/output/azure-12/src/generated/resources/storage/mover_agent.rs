/// Manages a Storage Mover Agent.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mover_agent {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MoverAgentArgs {
        /// Specifies the fully qualified ID of the Hybrid Compute resource for the Storage Mover Agent. Changing this forces a new resource to be created.
        #[builder(into)]
        pub arc_virtual_machine_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Hybrid Compute resource's unique SMBIOS ID. Changing this forces a new resource to be created.
        #[builder(into)]
        pub arc_virtual_machine_uuid: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a description for this Storage Mover Agent.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Storage Mover Agent. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Storage Mover that this Agent should be connected to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_mover_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MoverAgentResult {
        /// Specifies the fully qualified ID of the Hybrid Compute resource for the Storage Mover Agent. Changing this forces a new resource to be created.
        pub arc_virtual_machine_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Hybrid Compute resource's unique SMBIOS ID. Changing this forces a new resource to be created.
        pub arc_virtual_machine_uuid: pulumi_gestalt_rust::Output<String>,
        /// Specifies a description for this Storage Mover Agent.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Storage Mover Agent. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Storage Mover that this Agent should be connected to. Changing this forces a new resource to be created.
        pub storage_mover_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MoverAgentArgs,
    ) -> MoverAgentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arc_virtual_machine_id_binding = args
            .arc_virtual_machine_id
            .get_output(context);
        let arc_virtual_machine_uuid_binding = args
            .arc_virtual_machine_uuid
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let storage_mover_id_binding = args.storage_mover_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:storage/moverAgent:MoverAgent".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arcVirtualMachineId".into(),
                    value: arc_virtual_machine_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arcVirtualMachineUuid".into(),
                    value: arc_virtual_machine_uuid_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageMoverId".into(),
                    value: storage_mover_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MoverAgentResult {
            arc_virtual_machine_id: o.get_field("arcVirtualMachineId"),
            arc_virtual_machine_uuid: o.get_field("arcVirtualMachineUuid"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            storage_mover_id: o.get_field("storageMoverId"),
        }
    }
}
