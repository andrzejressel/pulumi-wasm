pub mod get_ami_ids {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAmiIdsArgs {
        /// Limit search to users with *explicit* launch
        /// permission on  the image. Valid items are the numeric account ID or `self`.
        #[builder(into, default)]
        pub executable_users: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// One or more name/value pairs to filter off of. There
        /// are several valid keys, for a full reference, check out
        /// [describe-images in the AWS CLI reference][1].
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetAmiIdsFilter>>,
        >,
        /// If true, all deprecated AMIs are included in the response.
        /// If false, no deprecated AMIs are included in the response. If no value is specified, the default value is `false`.
        #[builder(into, default)]
        pub include_deprecated: pulumi_wasm_rust::Output<Option<bool>>,
        /// Regex string to apply to the AMI list returned
        /// by AWS. This allows more advanced filtering not supported from the AWS API.
        /// This filtering is done locally on what AWS returns, and could have a performance
        /// impact if the result is large. Combine this with other
        /// options to narrow down the list AWS returns.
        #[builder(into, default)]
        pub name_regex: pulumi_wasm_rust::Output<Option<String>>,
        /// List of AMI owners to limit search. At least 1 value must be specified. Valid values: an AWS account ID, `self` (the current account), or an AWS owner alias (e.g., `amazon`, `aws-marketplace`, `microsoft`).
        #[builder(into)]
        pub owners: pulumi_wasm_rust::Output<Vec<String>>,
        /// Used to sort AMIs by creation time.
        /// If no value is specified, the default value is `false`.
        #[builder(into, default)]
        pub sort_ascending: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetAmiIdsResult {
        pub executable_users: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetAmiIdsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub include_deprecated: pulumi_wasm_rust::Output<Option<bool>>,
        pub name_regex: pulumi_wasm_rust::Output<Option<String>>,
        pub owners: pulumi_wasm_rust::Output<Vec<String>>,
        pub sort_ascending: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAmiIdsArgs) -> GetAmiIdsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let executable_users_binding = args.executable_users.get_inner();
        let filters_binding = args.filters.get_inner();
        let include_deprecated_binding = args.include_deprecated.get_inner();
        let name_regex_binding = args.name_regex.get_inner();
        let owners_binding = args.owners.get_inner();
        let sort_ascending_binding = args.sort_ascending.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getAmiIds:getAmiIds".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "executableUsers".into(),
                    value: &executable_users_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "includeDeprecated".into(),
                    value: &include_deprecated_binding,
                },
                register_interface::ObjectField {
                    name: "nameRegex".into(),
                    value: &name_regex_binding,
                },
                register_interface::ObjectField {
                    name: "owners".into(),
                    value: &owners_binding,
                },
                register_interface::ObjectField {
                    name: "sortAscending".into(),
                    value: &sort_ascending_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "executableUsers".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ids".into(),
                },
                register_interface::ResultField {
                    name: "includeDeprecated".into(),
                },
                register_interface::ResultField {
                    name: "nameRegex".into(),
                },
                register_interface::ResultField {
                    name: "owners".into(),
                },
                register_interface::ResultField {
                    name: "sortAscending".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAmiIdsResult {
            executable_users: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executableUsers").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ids").unwrap(),
            ),
            include_deprecated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeDeprecated").unwrap(),
            ),
            name_regex: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nameRegex").unwrap(),
            ),
            owners: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owners").unwrap(),
            ),
            sort_ascending: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sortAscending").unwrap(),
            ),
        }
    }
}
