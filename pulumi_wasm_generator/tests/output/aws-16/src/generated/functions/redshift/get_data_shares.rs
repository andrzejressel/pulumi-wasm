pub mod get_data_shares {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataSharesArgs {
        /// An array of all data shares in the current region. See `data_shares` below.
        #[builder(into, default)]
        pub data_shares: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::redshift::GetDataSharesDataShare>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDataSharesResult {
        /// An array of all data shares in the current region. See `data_shares` below.
        pub data_shares: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::redshift::GetDataSharesDataShare>>,
        >,
        /// AWS region.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDataSharesArgs,
    ) -> GetDataSharesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_shares_binding = args.data_shares.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:redshift/getDataShares:getDataShares".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataShares".into(),
                    value: &data_shares_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dataShares".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDataSharesResult {
            data_shares: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataShares").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}
