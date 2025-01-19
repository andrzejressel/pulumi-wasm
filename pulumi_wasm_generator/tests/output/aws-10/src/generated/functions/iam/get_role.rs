pub mod get_role {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRoleArgs {
        /// Friendly IAM role name to match.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Tags attached to the role.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRoleResult {
        /// ARN of the role.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Policy document associated with the role.
        pub assume_role_policy: pulumi_wasm_rust::Output<String>,
        /// Creation date of the role in RFC 3339 format.
        pub create_date: pulumi_wasm_rust::Output<String>,
        /// Description for the role.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Maximum session duration.
        pub max_session_duration: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Path to the role.
        pub path: pulumi_wasm_rust::Output<String>,
        /// The ARN of the policy that is used to set the permissions boundary for the role.
        pub permissions_boundary: pulumi_wasm_rust::Output<String>,
        /// Contains information about the last time that an IAM role was used. See `role_last_used` for details.
        pub role_last_useds: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::iam::GetRoleRoleLastUsed>,
        >,
        /// Tags attached to the role.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Stable and unique string identifying the role.
        pub unique_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRoleArgs) -> GetRoleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getRole:getRole".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "assumeRolePolicy".into(),
                },
                register_interface::ResultField {
                    name: "createDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "maxSessionDuration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "path".into(),
                },
                register_interface::ResultField {
                    name: "permissionsBoundary".into(),
                },
                register_interface::ResultField {
                    name: "roleLastUseds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "uniqueId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRoleResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            assume_role_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assumeRolePolicy").unwrap(),
            ),
            create_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            max_session_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxSessionDuration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("path").unwrap(),
            ),
            permissions_boundary: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissionsBoundary").unwrap(),
            ),
            role_last_useds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleLastUseds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            unique_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uniqueId").unwrap(),
            ),
        }
    }
}
