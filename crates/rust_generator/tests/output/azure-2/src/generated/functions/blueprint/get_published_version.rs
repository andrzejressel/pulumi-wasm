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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetPublishedVersionArgs,
    ) -> GetPublishedVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let blueprint_name_binding_1 = args.blueprint_name.get_output(context);
        let blueprint_name_binding = blueprint_name_binding_1.get_inner();
        let scope_id_binding_1 = args.scope_id.get_output(context);
        let scope_id_binding = scope_id_binding_1.get_inner();
        let version_binding_1 = args.version.get_output(context);
        let version_binding = version_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:blueprint/getPublishedVersion:getPublishedVersion".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "blueprintName".into(),
                    value: &blueprint_name_binding,
                },
                register_interface::ObjectField {
                    name: "scopeId".into(),
                    value: &scope_id_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPublishedVersionResult {
            blueprint_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("blueprintName"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            last_modified: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModified"),
            ),
            scope_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scopeId"),
            ),
            target_scope: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetScope"),
            ),
            time_created: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeCreated"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
