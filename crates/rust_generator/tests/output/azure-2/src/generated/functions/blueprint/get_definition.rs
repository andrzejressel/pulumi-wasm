#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDefinitionArgs {
        /// The name of the Blueprint.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Subscription or Management Group, as the scope at which the blueprint definition is stored.
        #[builder(into)]
        pub scope_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDefinitionResult {
        /// The description of the Blueprint Definition.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The display name of the Blueprint Definition.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The timestamp of when this last modification was saved to the Blueprint Definition.
        pub last_modified: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub scope_id: pulumi_gestalt_rust::Output<String>,
        /// The target scope.
        pub target_scope: pulumi_gestalt_rust::Output<String>,
        /// The timestamp of when this Blueprint Definition was created.
        pub time_created: pulumi_gestalt_rust::Output<String>,
        /// A list of versions published for this Blueprint Definition.
        pub versions: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDefinitionArgs,
    ) -> GetDefinitionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let scope_id_binding = args.scope_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:blueprint/getDefinition:getDefinition".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopeId".into(),
                    value: scope_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDefinitionResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            last_modified: o.get_field("lastModified"),
            name: o.get_field("name"),
            scope_id: o.get_field("scopeId"),
            target_scope: o.get_field("targetScope"),
            time_created: o.get_field("timeCreated"),
            versions: o.get_field("versions"),
        }
    }
}
