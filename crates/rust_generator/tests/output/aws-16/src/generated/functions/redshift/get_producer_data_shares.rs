#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_producer_data_shares {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProducerDataSharesArgs {
        /// An array of all data shares in the producer. See `data_shares` below.
        #[builder(into, default)]
        pub data_shares: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::redshift::GetProducerDataSharesDataShare>,
            >,
        >,
        /// Amazon Resource Name (ARN) of the producer namespace that returns in the list of datashares.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub producer_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Status of a datashare in the producer. Valid values are `ACTIVE`, `AUTHORIZED`, `PENDING_AUTHORIZATION`, `DEAUTHORIZED`, and `REJECTED`. Omit this argument to return all statuses.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetProducerDataSharesResult {
        /// An array of all data shares in the producer. See `data_shares` below.
        pub data_shares: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::redshift::GetProducerDataSharesDataShare>,
            >,
        >,
        /// Producer ARN.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN (Amazon Resource Name) of the producer.
        pub producer_arn: pulumi_gestalt_rust::Output<String>,
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetProducerDataSharesArgs,
    ) -> GetProducerDataSharesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_shares_binding = args.data_shares.get_output(context);
        let producer_arn_binding = args.producer_arn.get_output(context);
        let status_binding = args.status.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:redshift/getProducerDataShares:getProducerDataShares".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataShares".into(),
                    value: data_shares_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "producerArn".into(),
                    value: producer_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetProducerDataSharesResult {
            data_shares: o.get_field("dataShares"),
            id: o.get_field("id"),
            producer_arn: o.get_field("producerArn"),
            status: o.get_field("status"),
        }
    }
}
