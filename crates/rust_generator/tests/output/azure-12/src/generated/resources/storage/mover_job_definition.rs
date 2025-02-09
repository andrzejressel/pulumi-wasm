/// Manages a Storage Mover Job Definition.
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
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .allow_nested_items_to_be_public(true)
///             .location("${example.location}")
///             .name("examplesa")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleContainer = container::create(
///         "exampleContainer",
///         ContainerArgs::builder()
///             .container_access_type("blob")
///             .name("acccontainer")
///             .storage_account_name("${exampleAccount.name}")
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
///     let exampleMoverAgent = mover_agent::create(
///         "exampleMoverAgent",
///         MoverAgentArgs::builder()
///             .arc_virtual_machine_id(
///                 "${example.id}/providers/Microsoft.HybridCompute/machines/examples-hybridComputeName",
///             )
///             .arc_virtual_machine_uuid("3bb2c024-eba9-4d18-9e7a-1d772fcc5fe9")
///             .name("example-agent")
///             .storage_mover_id("${exampleMover.id}")
///             .build_struct(),
///     );
///     let exampleMoverJobDefinition = mover_job_definition::create(
///         "exampleMoverJobDefinition",
///         MoverJobDefinitionArgs::builder()
///             .agent_name("${exampleMoverAgent.name}")
///             .copy_mode("Additive")
///             .description("Example Job Definition Description")
///             .name("example-sjd")
///             .source_name("${exampleMoverSourceEndpoint.name}")
///             .source_sub_path("/")
///             .storage_mover_project_id("${exampleMoverProject.id}")
///             .target_name("${exampleMoverTargetEndpoint.name}")
///             .target_sub_path("/")
///             .build_struct(),
///     );
///     let exampleMoverProject = mover_project::create(
///         "exampleMoverProject",
///         MoverProjectArgs::builder()
///             .name("example-sp")
///             .storage_mover_id("${exampleMover.id}")
///             .build_struct(),
///     );
///     let exampleMoverSourceEndpoint = mover_source_endpoint::create(
///         "exampleMoverSourceEndpoint",
///         MoverSourceEndpointArgs::builder()
///             .host("192.168.0.1")
///             .name("example-smse")
///             .storage_mover_id("${exampleMover.id}")
///             .build_struct(),
///     );
///     let exampleMoverTargetEndpoint = mover_target_endpoint::create(
///         "exampleMoverTargetEndpoint",
///         MoverTargetEndpointArgs::builder()
///             .name("example-smte")
///             .storage_account_id("${exampleAccount.id}")
///             .storage_container_name("${exampleContainer.name}")
///             .storage_mover_id("${exampleMover.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Mover Job Definition can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/moverJobDefinition:MoverJobDefinition example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.StorageMover/storageMovers/storageMover1/projects/project1/jobDefinitions/jobDefinition1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mover_job_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MoverJobDefinitionArgs {
        /// Specifies the name of the Storage Mover Agent to assign for new Job Runs of this Storage Mover Job Definition.
        #[builder(into, default)]
        pub agent_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the strategy to use for copy. Possible values are `Additive` and `Mirror`.
        #[builder(into)]
        pub copy_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a description for this Storage Mover Job Definition.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Storage Mover Job Definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub source_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the sub path to use when reading from the Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_sub_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the Storage Mover Project. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_mover_project_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Storage Mover target Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the sub path to use when writing to the Storage Mover Target Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub target_sub_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MoverJobDefinitionResult {
        /// Specifies the name of the Storage Mover Agent to assign for new Job Runs of this Storage Mover Job Definition.
        pub agent_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the strategy to use for copy. Possible values are `Additive` and `Mirror`.
        pub copy_mode: pulumi_gestalt_rust::Output<String>,
        /// Specifies a description for this Storage Mover Job Definition.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Storage Mover Job Definition. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        pub source_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the sub path to use when reading from the Storage Mover Source Endpoint. Changing this forces a new resource to be created.
        pub source_sub_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the ID of the Storage Mover Project. Changing this forces a new resource to be created.
        pub storage_mover_project_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Storage Mover target Endpoint. Changing this forces a new resource to be created.
        pub target_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the sub path to use when writing to the Storage Mover Target Endpoint. Changing this forces a new resource to be created.
        pub target_sub_path: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MoverJobDefinitionArgs,
    ) -> MoverJobDefinitionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let agent_name_binding_1 = args.agent_name.get_output(context);
        let agent_name_binding = agent_name_binding_1.get_inner();
        let copy_mode_binding_1 = args.copy_mode.get_output(context);
        let copy_mode_binding = copy_mode_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let source_name_binding_1 = args.source_name.get_output(context);
        let source_name_binding = source_name_binding_1.get_inner();
        let source_sub_path_binding_1 = args.source_sub_path.get_output(context);
        let source_sub_path_binding = source_sub_path_binding_1.get_inner();
        let storage_mover_project_id_binding_1 = args
            .storage_mover_project_id
            .get_output(context);
        let storage_mover_project_id_binding = storage_mover_project_id_binding_1
            .get_inner();
        let target_name_binding_1 = args.target_name.get_output(context);
        let target_name_binding = target_name_binding_1.get_inner();
        let target_sub_path_binding_1 = args.target_sub_path.get_output(context);
        let target_sub_path_binding = target_sub_path_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/moverJobDefinition:MoverJobDefinition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "agentName".into(),
                    value: &agent_name_binding,
                },
                register_interface::ObjectField {
                    name: "copyMode".into(),
                    value: &copy_mode_binding,
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
                    name: "sourceName".into(),
                    value: &source_name_binding,
                },
                register_interface::ObjectField {
                    name: "sourceSubPath".into(),
                    value: &source_sub_path_binding,
                },
                register_interface::ObjectField {
                    name: "storageMoverProjectId".into(),
                    value: &storage_mover_project_id_binding,
                },
                register_interface::ObjectField {
                    name: "targetName".into(),
                    value: &target_name_binding,
                },
                register_interface::ObjectField {
                    name: "targetSubPath".into(),
                    value: &target_sub_path_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MoverJobDefinitionResult {
            agent_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("agentName"),
            ),
            copy_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("copyMode"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            source_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceName"),
            ),
            source_sub_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceSubPath"),
            ),
            storage_mover_project_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageMoverProjectId"),
            ),
            target_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetName"),
            ),
            target_sub_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetSubPath"),
            ),
        }
    }
}
