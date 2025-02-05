pub mod get_user_hierarchy_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserHierarchyGroupArgs {
        /// Returns information on a specific hierarchy group by hierarchy group id
        #[builder(into, default)]
        pub hierarchy_group_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Returns information on a specific hierarchy group by name
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the hierarchy group.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetUserHierarchyGroupResult {
        /// ARN of the hierarchy group.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub hierarchy_group_id: pulumi_wasm_rust::Output<String>,
        /// Block that contains information about the levels in the hierarchy group. The `hierarchy_path` block is documented below.
        pub hierarchy_paths: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::connect::GetUserHierarchyGroupHierarchyPath>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the level in the hierarchy group.
        pub level_id: pulumi_wasm_rust::Output<String>,
        /// Name of the hierarchy group.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the hierarchy group.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetUserHierarchyGroupArgs,
    ) -> GetUserHierarchyGroupResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hierarchy_group_id_binding = args
            .hierarchy_group_id
            .get_output(context)
            .get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getUserHierarchyGroup:getUserHierarchyGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hierarchyGroupId".into(),
                    value: &hierarchy_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUserHierarchyGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            hierarchy_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hierarchyGroupId"),
            ),
            hierarchy_paths: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hierarchyPaths"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            level_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("levelId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
