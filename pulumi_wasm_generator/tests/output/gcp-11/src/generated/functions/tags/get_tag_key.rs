pub mod get_tag_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTagKeyArgs {
        /// The resource name of the parent organization or project. It can be in format `organizations/{org_id}` or `projects/{project_id_or_number}`.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The tag key's short_name.
        #[builder(into)]
        pub short_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetTagKeyResult {
        /// Creation time.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        /// an identifier for the resource with format `tagKeys/{{name}}`
        pub id: pulumi_wasm_rust::Output<String>,
        /// The generated numeric id for the TagKey.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Namespaced name of the TagKey which is in the format `{parentNamespace}/{shortName}`.
        pub namespaced_name: pulumi_wasm_rust::Output<String>,
        pub parent: pulumi_wasm_rust::Output<String>,
        pub short_name: pulumi_wasm_rust::Output<String>,
        /// Update time.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTagKeyArgs) -> GetTagKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_binding = args.parent.get_inner();
        let short_name_binding = args.short_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:tags/getTagKey:getTagKey".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "shortName".into(),
                    value: &short_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namespacedName".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "shortName".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTagKeyResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            namespaced_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespacedName").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            short_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shortName").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
