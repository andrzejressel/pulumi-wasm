pub mod get_log_groups {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLogGroupsArgs {
        /// Group prefix of the Cloudwatch log groups to list
        #[builder(into, default)]
        pub log_group_name_prefix: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetLogGroupsResult {
        /// Set of ARNs of the Cloudwatch log groups
        pub arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub log_group_name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of names of the Cloudwatch log groups
        pub log_group_names: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetLogGroupsArgs) -> GetLogGroupsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let log_group_name_prefix_binding = args.log_group_name_prefix.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudwatch/getLogGroups:getLogGroups".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "logGroupNamePrefix".into(),
                    value: &log_group_name_prefix_binding,
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
                    name: "logGroupNamePrefix".into(),
                },
                register_interface::ResultField {
                    name: "logGroupNames".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLogGroupsResult {
            arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arns").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            log_group_name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logGroupNamePrefix").unwrap(),
            ),
            log_group_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logGroupNames").unwrap(),
            ),
        }
    }
}
