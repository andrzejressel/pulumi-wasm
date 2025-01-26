pub mod get_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDefinitionArgs {
        /// The name of the Blueprint.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Subscription or Management Group, as the scope at which the blueprint definition is stored.
        #[builder(into)]
        pub scope_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDefinitionResult {
        /// The description of the Blueprint Definition.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The display name of the Blueprint Definition.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The timestamp of when this last modification was saved to the Blueprint Definition.
        pub last_modified: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub scope_id: pulumi_wasm_rust::Output<String>,
        /// The target scope.
        pub target_scope: pulumi_wasm_rust::Output<String>,
        /// The timestamp of when this Blueprint Definition was created.
        pub time_created: pulumi_wasm_rust::Output<String>,
        /// A list of versions published for this Blueprint Definition.
        pub versions: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDefinitionArgs,
    ) -> GetDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let scope_id_binding = args.scope_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:blueprint/getDefinition:getDefinition".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scopeId".into(),
                    value: &scope_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDefinitionResult {
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            last_modified: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastModified"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            scope_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scopeId"),
            ),
            target_scope: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetScope"),
            ),
            time_created: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeCreated"),
            ),
            versions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versions"),
            ),
        }
    }
}
