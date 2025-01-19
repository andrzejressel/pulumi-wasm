pub mod get_solution_stack {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSolutionStackArgs {
        /// If more than one result is returned, use the most
        /// recent solution stack.
        #[builder(into, default)]
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        /// Regex string to apply to the solution stack list returned
        /// by AWS. See [Elastic Beanstalk Supported Platforms][beanstalk-platforms] from
        /// AWS documentation for reference solution stack names.
        ///
        /// > **NOTE:** If more or less than a single match is returned by the search,
        /// this call will fail. Ensure that your search is specific enough to return
        /// a single solution stack, or use `most_recent` to choose the most recent one.
        #[builder(into)]
        pub name_regex: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSolutionStackResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the solution stack.
        pub name: pulumi_wasm_rust::Output<String>,
        pub name_regex: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSolutionStackArgs) -> GetSolutionStackResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let most_recent_binding = args.most_recent.get_inner();
        let name_regex_binding = args.name_regex.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:elasticbeanstalk/getSolutionStack:getSolutionStack".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding,
                },
                register_interface::ObjectField {
                    name: "nameRegex".into(),
                    value: &name_regex_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "mostRecent".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nameRegex".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSolutionStackResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            most_recent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mostRecent").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_regex: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nameRegex").unwrap(),
            ),
        }
    }
}
