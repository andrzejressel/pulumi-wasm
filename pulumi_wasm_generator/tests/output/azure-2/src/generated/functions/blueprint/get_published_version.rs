pub mod get_published_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPublishedVersionArgs {
        /// The name of the Blueprint Definition
        #[builder(into)]
        pub blueprint_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Management Group / Subscription where this Blueprint Definition is stored.
        #[builder(into)]
        pub scope_id: pulumi_wasm_rust::Output<String>,
        /// The Version name of the Published Version of the Blueprint Definition
        #[builder(into)]
        pub version: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetPublishedVersionResult {
        pub blueprint_name: pulumi_wasm_rust::Output<String>,
        /// The description of the Blueprint Published Version
        pub description: pulumi_wasm_rust::Output<String>,
        /// The display name of the Blueprint Published Version
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub last_modified: pulumi_wasm_rust::Output<String>,
        pub scope_id: pulumi_wasm_rust::Output<String>,
        /// The target scope
        pub target_scope: pulumi_wasm_rust::Output<String>,
        pub time_created: pulumi_wasm_rust::Output<String>,
        /// The type of the Blueprint
        pub type_: pulumi_wasm_rust::Output<String>,
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPublishedVersionArgs) -> GetPublishedVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let blueprint_name_binding = args.blueprint_name.get_inner();
        let scope_id_binding = args.scope_id.get_inner();
        let version_binding = args.version.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "blueprintName".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lastModified".into(),
                },
                register_interface::ResultField {
                    name: "scopeId".into(),
                },
                register_interface::ResultField {
                    name: "targetScope".into(),
                },
                register_interface::ResultField {
                    name: "timeCreated".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPublishedVersionResult {
            blueprint_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blueprintName").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            last_modified: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModified").unwrap(),
            ),
            scope_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopeId").unwrap(),
            ),
            target_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetScope").unwrap(),
            ),
            time_created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeCreated").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
