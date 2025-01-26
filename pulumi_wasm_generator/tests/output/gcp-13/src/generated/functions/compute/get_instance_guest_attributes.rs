pub mod get_instance_guest_attributes {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceGuestAttributesArgs {
        /// The name or self_link of the instance.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If `self_link` is provided, this value is ignored.  If neither `self_link`
        /// nor `project` are provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Path to query for the guest attributes. Consists of
        /// `namespace` name for the attributes followed with a `/`.
        #[builder(into, default)]
        pub query_path: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key of a variable to get the value of. Consists of
        /// `namespace` name and `key` name for the variable separated by a `/`.
        #[builder(into, default)]
        pub variable_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The zone of the instance. If `self_link` is provided, this
        /// value is ignored.  If neither `self_link` nor `zone` are provided, the
        /// provider zone is used.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceGuestAttributesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        pub query_path: pulumi_wasm_rust::Output<Option<String>>,
        /// Structure is documented below.
        pub query_values: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetInstanceGuestAttributesQueryValue,
            >,
        >,
        pub region: pulumi_wasm_rust::Output<String>,
        pub variable_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Value of the queried guest_attribute.
        pub variable_value: pulumi_wasm_rust::Output<String>,
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetInstanceGuestAttributesArgs,
    ) -> GetInstanceGuestAttributesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let query_path_binding = args.query_path.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let variable_key_binding = args.variable_key.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getInstanceGuestAttributes:getInstanceGuestAttributes"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "queryPath".into(),
                    value: &query_path_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "variableKey".into(),
                    value: &variable_key_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "queryPath".into(),
                },
                register_interface::ResultField {
                    name: "queryValues".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "variableKey".into(),
                },
                register_interface::ResultField {
                    name: "variableValue".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInstanceGuestAttributesResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            query_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryPath").unwrap(),
            ),
            query_values: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryValues").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            variable_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("variableKey").unwrap(),
            ),
            variable_value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("variableValue").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
