#[allow(clippy::doc_lazy_continuation)]
pub mod get_data_shares {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataSharesArgs {
        /// An array of all data shares in the current region. See `data_shares` below.
        #[builder(into, default)]
        pub data_shares: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::redshift::GetDataSharesDataShare>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDataSharesResult {
        /// An array of all data shares in the current region. See `data_shares` below.
        pub data_shares: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::redshift::GetDataSharesDataShare>>,
        >,
        /// AWS region.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDataSharesArgs,
    ) -> GetDataSharesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDataSharesResult {
            data_shares: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataShares"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
