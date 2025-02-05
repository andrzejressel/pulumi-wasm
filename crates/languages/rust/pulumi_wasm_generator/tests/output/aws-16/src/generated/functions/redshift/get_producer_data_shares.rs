pub mod get_producer_data_shares {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProducerDataSharesArgs {
        /// An array of all data shares in the producer. See `data_shares` below.
        #[builder(into, default)]
        pub data_shares: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::redshift::GetProducerDataSharesDataShare>,
            >,
        >,
        /// Amazon Resource Name (ARN) of the producer namespace that returns in the list of datashares.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub producer_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Status of a datashare in the producer. Valid values are `ACTIVE`, `AUTHORIZED`, `PENDING_AUTHORIZATION`, `DEAUTHORIZED`, and `REJECTED`. Omit this argument to return all statuses.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetProducerDataSharesArgs,
    ) -> GetProducerDataSharesResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_shares_binding = args.data_shares.get_output(context).get_inner();
        let producer_arn_binding = args.producer_arn.get_output(context).get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:redshift/getProducerDataShares:getProducerDataShares".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetProducerDataSharesResult {
            data_shares: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataShares"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            producer_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("producerArn"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
        }
    }
}
