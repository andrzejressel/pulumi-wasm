#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_published_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPublishedVersionArgs {
        /// The name of the Blueprint Definition
        #[builder(into)]
        pub blueprint_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Management Group / Subscription where this Blueprint Definition is stored.
        #[builder(into)]
        pub scope_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Version name of the Published Version of the Blueprint Definition
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPublishedVersionResult {
        pub blueprint_name: pulumi_gestalt_rust::Output<String>,
        /// The description of the Blueprint Published Version
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The display name of the Blueprint Published Version
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub last_modified: pulumi_gestalt_rust::Output<String>,
        pub scope_id: pulumi_gestalt_rust::Output<String>,
        /// The target scope
        pub target_scope: pulumi_gestalt_rust::Output<String>,
        pub time_created: pulumi_gestalt_rust::Output<String>,
        /// The type of the Blueprint
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPublishedVersionArgs,
    ) -> GetPublishedVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let blueprint_name_binding = args.blueprint_name.get_output(context);
        let scope_id_binding = args.scope_id.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:blueprint/getPublishedVersion:getPublishedVersion".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blueprintName".into(),
                    value: &blueprint_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopeId".into(),
                    value: &scope_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPublishedVersionResult {
            blueprint_name: o.get_field("blueprintName"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            last_modified: o.get_field("lastModified"),
            scope_id: o.get_field("scopeId"),
            target_scope: o.get_field("targetScope"),
            time_created: o.get_field("timeCreated"),
            type_: o.get_field("type"),
            version: o.get_field("version"),
        }
    }
}
