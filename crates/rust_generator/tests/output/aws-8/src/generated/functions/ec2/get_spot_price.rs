#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_spot_price {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSpotPriceArgs {
        /// Availability zone in which to query Spot price information.
        #[builder(into, default)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more configuration blocks containing name-values filters. See the [EC2 API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSpotPriceHistory.html) for supported filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetSpotPriceFilter>>,
        >,
        /// Type of instance for which to query Spot Price information.
        #[builder(into, default)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSpotPriceResult {
        pub availability_zone: pulumi_gestalt_rust::Output<Option<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetSpotPriceFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Most recent Spot Price value for the given instance type and AZ.
        pub spot_price: pulumi_gestalt_rust::Output<String>,
        /// The timestamp at which the Spot Price value was published.
        pub spot_price_timestamp: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSpotPriceArgs,
    ) -> GetSpotPriceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let availability_zone_binding = args.availability_zone.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let instance_type_binding = args.instance_type.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getSpotPrice:getSpotPrice".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: availability_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: instance_type_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSpotPriceResult {
            availability_zone: o.get_field("availabilityZone"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            instance_type: o.get_field("instanceType"),
            spot_price: o.get_field("spotPrice"),
            spot_price_timestamp: o.get_field("spotPriceTimestamp"),
        }
    }
}
