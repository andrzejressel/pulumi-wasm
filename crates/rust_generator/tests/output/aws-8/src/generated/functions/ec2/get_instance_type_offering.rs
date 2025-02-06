pub mod get_instance_type_offering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceTypeOfferingArgs {
        /// One or more configuration blocks containing name-values filters. See the [EC2 API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeInstanceTypeOfferings.html) for supported filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetInstanceTypeOfferingFilter>>,
        >,
        /// Location type. Defaults to `region`. Valid values: `availability-zone`, `availability-zone-id`, and `region`.
        #[builder(into, default)]
        pub location_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Ordered list of preferred EC2 Instance Types. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned.
        #[builder(into, default)]
        pub preferred_instance_types: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetInstanceTypeOfferingResult {
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetInstanceTypeOfferingFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// EC2 Instance Type.
        pub instance_type: pulumi_wasm_rust::Output<String>,
        pub location_type: pulumi_wasm_rust::Output<Option<String>>,
        pub preferred_instance_types: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetInstanceTypeOfferingArgs,
    ) -> GetInstanceTypeOfferingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let location_type_binding = args.location_type.get_output(context).get_inner();
        let preferred_instance_types_binding = args
            .preferred_instance_types
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getInstanceTypeOffering:getInstanceTypeOffering".into(),
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
                register_interface::ObjectField {
                    name: "preferredInstanceTypes".into(),
                    value: &preferred_instance_types_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInstanceTypeOfferingResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceType"),
            ),
            location_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("locationType"),
            ),
            preferred_instance_types: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("preferredInstanceTypes"),
            ),
        }
    }
}
