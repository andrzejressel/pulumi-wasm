#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_reserved_instance_offering {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReservedInstanceOfferingArgs {
        /// DB instance class for the reserved DB instance.
        #[builder(into)]
        pub db_instance_class: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Duration of the reservation in years or seconds. Valid values are `1`, `3`, `31536000`, `94608000`
        #[builder(into)]
        pub duration: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Whether the reservation applies to Multi-AZ deployments.
        #[builder(into)]
        pub multi_az: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Offering type of this reserved DB instance. Valid values are `No Upfront`, `Partial Upfront`, `All Upfront`.
        #[builder(into)]
        pub offering_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the reserved DB instance.
        #[builder(into)]
        pub product_description: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetReservedInstanceOfferingResult {
        /// Currency code for the reserved DB instance.
        pub currency_code: pulumi_gestalt_rust::Output<String>,
        pub db_instance_class: pulumi_gestalt_rust::Output<String>,
        pub duration: pulumi_gestalt_rust::Output<i32>,
        /// Fixed price charged for this reserved DB instance.
        pub fixed_price: pulumi_gestalt_rust::Output<f64>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub multi_az: pulumi_gestalt_rust::Output<bool>,
        /// Unique identifier for the reservation.
        pub offering_id: pulumi_gestalt_rust::Output<String>,
        pub offering_type: pulumi_gestalt_rust::Output<String>,
        pub product_description: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetReservedInstanceOfferingArgs,
    ) -> GetReservedInstanceOfferingResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let db_instance_class_binding = args
            .db_instance_class
            .get_output(context)
            .get_inner();
        let duration_binding = args.duration.get_output(context).get_inner();
        let multi_az_binding = args.multi_az.get_output(context).get_inner();
        let offering_type_binding = args.offering_type.get_output(context).get_inner();
        let product_description_binding = args
            .product_description
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:rds/getReservedInstanceOffering:getReservedInstanceOffering"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dbInstanceClass".into(),
                    value: &db_instance_class_binding,
                },
                register_interface::ObjectField {
                    name: "duration".into(),
                    value: &duration_binding,
                },
                register_interface::ObjectField {
                    name: "multiAz".into(),
                    value: &multi_az_binding,
                },
                register_interface::ObjectField {
                    name: "offeringType".into(),
                    value: &offering_type_binding,
                },
                register_interface::ObjectField {
                    name: "productDescription".into(),
                    value: &product_description_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetReservedInstanceOfferingResult {
            currency_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("currencyCode"),
            ),
            db_instance_class: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbInstanceClass"),
            ),
            duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("duration"),
            ),
            fixed_price: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fixedPrice"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            multi_az: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multiAz"),
            ),
            offering_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("offeringId"),
            ),
            offering_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("offeringType"),
            ),
            product_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("productDescription"),
            ),
        }
    }
}
