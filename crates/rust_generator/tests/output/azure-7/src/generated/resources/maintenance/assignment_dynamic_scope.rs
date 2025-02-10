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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod assignment_dynamic_scope {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssignmentDynamicScopeArgs {
        /// A `filter` block as defined below.
        #[builder(into)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::maintenance::AssignmentDynamicScopeFilter,
        >,
        /// The ID of the Maintenance Configuration Resource. Changing this forces a new Dynamic Maintenance Assignment to be created.
        #[builder(into)]
        pub maintenance_configuration_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Dynamic Maintenance Assignment. Changing this forces a new Dynamic Maintenance Assignment to be created.
        ///
        /// > **Note:** The `name` must be unique per subscription.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AssignmentDynamicScopeResult {
        /// A `filter` block as defined below.
        pub filter: pulumi_gestalt_rust::Output<
            super::super::types::maintenance::AssignmentDynamicScopeFilter,
        >,
        /// The ID of the Maintenance Configuration Resource. Changing this forces a new Dynamic Maintenance Assignment to be created.
        pub maintenance_configuration_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Dynamic Maintenance Assignment. Changing this forces a new Dynamic Maintenance Assignment to be created.
        ///
        /// > **Note:** The `name` must be unique per subscription.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssignmentDynamicScopeArgs,
    ) -> AssignmentDynamicScopeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filter_binding = args.filter.get_output(context);
        let maintenance_configuration_id_binding = args
            .maintenance_configuration_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:maintenance/assignmentDynamicScope:AssignmentDynamicScope"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maintenanceConfigurationId".into(),
                    value: maintenance_configuration_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AssignmentDynamicScopeResult {
            filter: o.get_field("filter"),
            maintenance_configuration_id: o.get_field("maintenanceConfigurationId"),
            name: o.get_field("name"),
        }
    }
}
