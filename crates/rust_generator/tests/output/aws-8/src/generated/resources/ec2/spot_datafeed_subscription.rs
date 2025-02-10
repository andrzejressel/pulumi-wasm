/// > **Note:** There is only a single subscription allowed per account.
///
/// To help you understand the charges for your Spot instances, Amazon EC2 provides a data feed that describes your Spot instance usage and pricing.
/// This data feed is sent to an Amazon S3 bucket that you specify when you subscribe to the data feed.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = bucket_v_2::create(
///         "default",
///         BucketV2Args::builder().bucket("tf-spot-datafeed").build_struct(),
///     );
///     let defaultSpotDatafeedSubscription = spot_datafeed_subscription::create(
///         "defaultSpotDatafeedSubscription",
///         SpotDatafeedSubscriptionArgs::builder()
///             .bucket("${default.id}")
///             .prefix("my_subdirectory")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a Spot Datafeed Subscription using the word `spot-datafeed-subscription`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/spotDatafeedSubscription:SpotDatafeedSubscription mysubscription spot-datafeed-subscription
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spot_datafeed_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpotDatafeedSubscriptionArgs {
        /// The Amazon S3 bucket in which to store the Spot instance data feed.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Path of folder inside bucket to place spot pricing data.
        #[builder(into, default)]
        pub prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SpotDatafeedSubscriptionResult {
        /// The Amazon S3 bucket in which to store the Spot instance data feed.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Path of folder inside bucket to place spot pricing data.
        pub prefix: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SpotDatafeedSubscriptionArgs,
    ) -> SpotDatafeedSubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let prefix_binding = args.prefix.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/spotDatafeedSubscription:SpotDatafeedSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefix".into(),
                    value: prefix_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SpotDatafeedSubscriptionResult {
            bucket: o.get_field("bucket"),
            prefix: o.get_field("prefix"),
        }
    }
}
