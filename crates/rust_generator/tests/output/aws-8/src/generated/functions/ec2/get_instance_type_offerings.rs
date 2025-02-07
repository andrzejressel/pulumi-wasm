pub mod get_instance_type_offerings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceTypeOfferingsArgs {
        /// One or more configuration blocks containing name-values filters. See the [EC2 API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeInstanceTypeOfferings.html) for supported filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetInstanceTypeOfferingsFilter>>,
        >,
        /// Location type. Defaults to `region`. Valid values: `availability-zone`, `availability-zone-id`, and `region`.
        #[builder(into, default)]
        pub location_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceTypeOfferingsResult {
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetInstanceTypeOfferingsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of EC2 Instance Types.
        pub instance_types: pulumi_gestalt_rust::Output<Vec<String>>,
        pub location_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of location types.
        pub location_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// List of locations.
        pub locations: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetInstanceTypeOfferingsArgs,
    ) -> GetInstanceTypeOfferingsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let location_type_binding = args.location_type.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getInstanceTypeOfferings:getInstanceTypeOfferings".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "locationType".into(),
                    value: &location_type_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInstanceTypeOfferingsResult {
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceTypes"),
            ),
            location_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("locationType"),
            ),
            location_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("locationTypes"),
            ),
            locations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("locations"),
            ),
        }
    }
}
