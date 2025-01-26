/// Manages a Dynamic Maintenance Assignment.
///
/// > **Note:** Only valid for `InGuestPatch` Maintenance Configuration Scopes.
///
/// ## Import
///
/// Dynamic Maintenance Assignments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:maintenance/assignmentDynamicScope:AssignmentDynamicScope example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Maintenance/configurationAssignments/assignmentName
/// ```
///
pub mod assignment_dynamic_scope {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssignmentDynamicScopeArgs {
        /// A `filter` block as defined below.
        #[builder(into)]
        pub filter: pulumi_wasm_rust::InputOrOutput<
            super::super::types::maintenance::AssignmentDynamicScopeFilter,
        >,
        /// The ID of the Maintenance Configuration Resource. Changing this forces a new Dynamic Maintenance Assignment to be created.
        #[builder(into)]
        pub maintenance_configuration_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Dynamic Maintenance Assignment. Changing this forces a new Dynamic Maintenance Assignment to be created.
        ///
        /// > **Note:** The `name` must be unique per subscription.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AssignmentDynamicScopeResult {
        /// A `filter` block as defined below.
        pub filter: pulumi_wasm_rust::Output<
            super::super::types::maintenance::AssignmentDynamicScopeFilter,
        >,
        /// The ID of the Maintenance Configuration Resource. Changing this forces a new Dynamic Maintenance Assignment to be created.
        pub maintenance_configuration_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Dynamic Maintenance Assignment. Changing this forces a new Dynamic Maintenance Assignment to be created.
        ///
        /// > **Note:** The `name` must be unique per subscription.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AssignmentDynamicScopeArgs,
    ) -> AssignmentDynamicScopeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filter_binding = args.filter.get_output(context).get_inner();
        let maintenance_configuration_id_binding = args
            .maintenance_configuration_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:maintenance/assignmentDynamicScope:AssignmentDynamicScope"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceConfigurationId".into(),
                    value: &maintenance_configuration_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceConfigurationId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AssignmentDynamicScopeResult {
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            maintenance_configuration_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceConfigurationId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
