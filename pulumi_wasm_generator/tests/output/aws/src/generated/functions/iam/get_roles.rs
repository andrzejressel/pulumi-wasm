pub mod get_roles {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRolesArgs {
        /// Regex string to apply to the IAM roles list returned by AWS. This allows more advanced filtering not supported from the AWS API. This filtering is done locally on what AWS returns, and could have a performance impact if the result is large. Combine this with other options to narrow down the list AWS returns.
        #[builder(into, default)]
        pub name_regex: pulumi_wasm_rust::Output<Option<String>>,
        /// Path prefix for filtering the results. For example, the prefix `/application_abc/component_xyz/` gets all roles whose path starts with `/application_abc/component_xyz/`. If it is not included, it defaults to a slash (`/`), listing all roles. For more details, check out [list-roles in the AWS CLI reference][1].
        #[builder(into, default)]
        pub path_prefix: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRolesResult {
        /// Set of ARNs of the matched IAM roles.
        pub arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name_regex: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of Names of the matched IAM roles.
        pub names: pulumi_wasm_rust::Output<Vec<String>>,
        pub path_prefix: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRolesArgs) -> GetRolesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_regex_binding = args.name_regex.get_inner();
        let path_prefix_binding = args.path_prefix.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getRoles:getRoles".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "nameRegex".into(),
                    value: &name_regex_binding,
                },
                register_interface::ObjectField {
                    name: "pathPrefix".into(),
                    value: &path_prefix_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arns".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "nameRegex".into(),
                },
                register_interface::ResultField {
                    name: "names".into(),
                },
                register_interface::ResultField {
                    name: "pathPrefix".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRolesResult {
            arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arns").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name_regex: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nameRegex").unwrap(),
            ),
            names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("names").unwrap(),
            ),
            path_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pathPrefix").unwrap(),
            ),
        }
    }
}
