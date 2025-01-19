pub mod get_reserved_instance_offering {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReservedInstanceOfferingArgs {
        /// DB instance class for the reserved DB instance.
        #[builder(into)]
        pub db_instance_class: pulumi_wasm_rust::Output<String>,
        /// Duration of the reservation in years or seconds. Valid values are `1`, `3`, `31536000`, `94608000`
        #[builder(into)]
        pub duration: pulumi_wasm_rust::Output<i32>,
        /// Whether the reservation applies to Multi-AZ deployments.
        #[builder(into)]
        pub multi_az: pulumi_wasm_rust::Output<bool>,
        /// Offering type of this reserved DB instance. Valid values are `No Upfront`, `Partial Upfront`, `All Upfront`.
        #[builder(into)]
        pub offering_type: pulumi_wasm_rust::Output<String>,
        /// Description of the reserved DB instance.
        #[builder(into)]
        pub product_description: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetReservedInstanceOfferingResult {
        /// Currency code for the reserved DB instance.
        pub currency_code: pulumi_wasm_rust::Output<String>,
        pub db_instance_class: pulumi_wasm_rust::Output<String>,
        pub duration: pulumi_wasm_rust::Output<i32>,
        /// Fixed price charged for this reserved DB instance.
        pub fixed_price: pulumi_wasm_rust::Output<f64>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub multi_az: pulumi_wasm_rust::Output<bool>,
        /// Unique identifier for the reservation.
        pub offering_id: pulumi_wasm_rust::Output<String>,
        pub offering_type: pulumi_wasm_rust::Output<String>,
        pub product_description: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetReservedInstanceOfferingArgs,
    ) -> GetReservedInstanceOfferingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let db_instance_class_binding = args.db_instance_class.get_inner();
        let duration_binding = args.duration.get_inner();
        let multi_az_binding = args.multi_az.get_inner();
        let offering_type_binding = args.offering_type.get_inner();
        let product_description_binding = args.product_description.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "currencyCode".into(),
                },
                register_interface::ResultField {
                    name: "dbInstanceClass".into(),
                },
                register_interface::ResultField {
                    name: "duration".into(),
                },
                register_interface::ResultField {
                    name: "fixedPrice".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "multiAz".into(),
                },
                register_interface::ResultField {
                    name: "offeringId".into(),
                },
                register_interface::ResultField {
                    name: "offeringType".into(),
                },
                register_interface::ResultField {
                    name: "productDescription".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetReservedInstanceOfferingResult {
            currency_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("currencyCode").unwrap(),
            ),
            db_instance_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbInstanceClass").unwrap(),
            ),
            duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("duration").unwrap(),
            ),
            fixed_price: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fixedPrice").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            multi_az: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiAz").unwrap(),
            ),
            offering_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("offeringId").unwrap(),
            ),
            offering_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("offeringType").unwrap(),
            ),
            product_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productDescription").unwrap(),
            ),
        }
    }
}
