pub mod get_prefix_list {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPrefixListArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetPrefixListFilter>>,
        >,
        /// Name of the prefix list to select.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the prefix list to select.
        #[builder(into, default)]
        pub prefix_list_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPrefixListResult {
        /// List of CIDR blocks for the AWS service associated with the prefix list.
        pub cidr_blocks: pulumi_wasm_rust::Output<Vec<String>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetPrefixListFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the selected prefix list.
        pub name: pulumi_wasm_rust::Output<String>,
        pub prefix_list_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPrefixListArgs) -> GetPrefixListResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let name_binding = args.name.get_inner();
        let prefix_list_id_binding = args.prefix_list_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getPrefixList:getPrefixList".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "prefixListId".into(),
                    value: &prefix_list_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cidrBlocks".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "prefixListId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPrefixListResult {
            cidr_blocks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlocks").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            prefix_list_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefixListId").unwrap(),
            ),
        }
    }
}
