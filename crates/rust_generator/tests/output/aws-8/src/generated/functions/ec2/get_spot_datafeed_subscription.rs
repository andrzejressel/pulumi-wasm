#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_spot_datafeed_subscription {
    #[allow(dead_code)]
    pub struct GetSpotDatafeedSubscriptionResult {
        /// The name of the Amazon S3 bucket where the spot instance data feed is located.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The prefix for the data feed files.
        pub prefix: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
    ) -> GetSpotDatafeedSubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getSpotDatafeedSubscription:getSpotDatafeedSubscription"
                .into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetSpotDatafeedSubscriptionResult {
            bucket: o.get_field("bucket"),
            id: o.get_field("id"),
            prefix: o.get_field("prefix"),
        }
    }
}
