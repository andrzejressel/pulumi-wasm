pub mod get_spot_datafeed_subscription {
    #[allow(dead_code)]
    pub struct GetSpotDatafeedSubscriptionResult {
        /// The name of the Amazon S3 bucket where the spot instance data feed is located.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The prefix for the data feed files.
        pub prefix: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
    ) -> GetSpotDatafeedSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getSpotDatafeedSubscription:getSpotDatafeedSubscription"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSpotDatafeedSubscriptionResult {
            bucket: pulumi_wasm_rust::__private::into_domain(o.extract_field("bucket")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            prefix: pulumi_wasm_rust::__private::into_domain(o.extract_field("prefix")),
        }
    }
}
