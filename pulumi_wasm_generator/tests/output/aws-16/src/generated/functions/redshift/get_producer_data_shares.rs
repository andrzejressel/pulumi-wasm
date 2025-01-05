pub mod get_producer_data_shares {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProducerDataSharesArgs {
        /// An array of all data shares in the producer. See `data_shares` below.
        #[builder(into, default)]
        pub data_shares: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::redshift::GetProducerDataSharesDataShare>,
            >,
        >,
        /// Amazon Resource Name (ARN) of the producer namespace that returns in the list of datashares.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub producer_arn: pulumi_wasm_rust::Output<String>,
        /// Status of a datashare in the producer. Valid values are `ACTIVE`, `AUTHORIZED`, `PENDING_AUTHORIZATION`, `DEAUTHORIZED`, and `REJECTED`. Omit this argument to return all statuses.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetProducerDataSharesResult {
        /// An array of all data shares in the producer. See `data_shares` below.
        pub data_shares: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::redshift::GetProducerDataSharesDataShare>,
            >,
        >,
        /// Producer ARN.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN (Amazon Resource Name) of the producer.
        pub producer_arn: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetProducerDataSharesArgs) -> GetProducerDataSharesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_shares_binding = args.data_shares.get_inner();
        let producer_arn_binding = args.producer_arn.get_inner();
        let status_binding = args.status.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:redshift/getProducerDataShares:getProducerDataShares".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataShares".into(),
                    value: &data_shares_binding,
                },
                register_interface::ObjectField {
                    name: "producerArn".into(),
                    value: &producer_arn_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dataShares".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "producerArn".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetProducerDataSharesResult {
            data_shares: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataShares").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            producer_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("producerArn").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
