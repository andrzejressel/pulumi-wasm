/// Manages an RDS DB Reserved Instance.
///
/// > **NOTE:** Once created, a reservation is valid for the `duration` of the provided `offering_id` and cannot be deleted. Performing a `destroy` will only remove the resource from state. For more information see [RDS Reserved Instances Documentation](https://aws.amazon.com/rds/reserved-instances/) and [PurchaseReservedDBInstancesOffering](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_PurchaseReservedDBInstancesOffering.html).
///
/// > **NOTE:** Due to the expense of testing this resource, we provide it as best effort. If you find it useful, and have the ability to help test or notice issues, consider reaching out to us on GitHub.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = get_reserved_instance_offering::invoke(
///         GetReservedInstanceOfferingArgs::builder()
///             .db_instance_class("db.t2.micro")
///             .duration(31536000)
///             .multi_az(false)
///             .offering_type("All Upfront")
///             .product_description("mysql")
///             .build_struct(),
///     );
///     let example = reserved_instance::create(
///         "example",
///         ReservedInstanceArgs::builder()
///             .instance_count(3)
///             .offering_id("${test.offeringId}")
///             .reservation_id("optionalCustomReservationID")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RDS DB Instance Reservations using the `instance_id`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/reservedInstance:ReservedInstance reservation_instance CustomReservationID
/// ```
pub mod reserved_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReservedInstanceArgs {
        /// Number of instances to reserve. Default value is `1`.
        #[builder(into, default)]
        pub instance_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// ID of the Reserved DB instance offering to purchase. To determine an `offering_id`, see the `aws.rds.getReservedInstanceOffering` data source.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub offering_id: pulumi_wasm_rust::Output<String>,
        /// Customer-specified identifier to track this reservation.
        #[builder(into, default)]
        pub reservation_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the DB reservation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReservedInstanceResult {
        /// ARN for the reserved DB instance.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Currency code for the reserved DB instance.
        pub currency_code: pulumi_wasm_rust::Output<String>,
        /// DB instance class for the reserved DB instance.
        pub db_instance_class: pulumi_wasm_rust::Output<String>,
        /// Duration of the reservation in seconds.
        pub duration: pulumi_wasm_rust::Output<i32>,
        /// Fixed price charged for this reserved DB instance.
        pub fixed_price: pulumi_wasm_rust::Output<f64>,
        /// Number of instances to reserve. Default value is `1`.
        pub instance_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Unique identifier for the lease associated with the reserved DB instance. Amazon Web Services Support might request the lease ID for an issue related to a reserved DB instance.
        pub lease_id: pulumi_wasm_rust::Output<String>,
        /// Whether the reservation applies to Multi-AZ deployments.
        pub multi_az: pulumi_wasm_rust::Output<bool>,
        /// ID of the Reserved DB instance offering to purchase. To determine an `offering_id`, see the `aws.rds.getReservedInstanceOffering` data source.
        ///
        /// The following arguments are optional:
        pub offering_id: pulumi_wasm_rust::Output<String>,
        /// Offering type of this reserved DB instance.
        pub offering_type: pulumi_wasm_rust::Output<String>,
        /// Description of the reserved DB instance.
        pub product_description: pulumi_wasm_rust::Output<String>,
        /// Recurring price charged to run this reserved DB instance.
        pub recurring_charges: pulumi_wasm_rust::Output<
            Vec<super::super::types::rds::ReservedInstanceRecurringCharge>,
        >,
        /// Customer-specified identifier to track this reservation.
        pub reservation_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Time the reservation started.
        pub start_time: pulumi_wasm_rust::Output<String>,
        /// State of the reserved DB instance.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the DB reservation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Hourly price charged for this reserved DB instance.
        pub usage_price: pulumi_wasm_rust::Output<f64>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ReservedInstanceArgs) -> ReservedInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_count_binding = args.instance_count.get_inner();
        let offering_id_binding = args.offering_id.get_inner();
        let reservation_id_binding = args.reservation_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/reservedInstance:ReservedInstance".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding,
                },
                register_interface::ObjectField {
                    name: "offeringId".into(),
                    value: &offering_id_binding,
                },
                register_interface::ObjectField {
                    name: "reservationId".into(),
                    value: &reservation_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
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
                    name: "instanceCount".into(),
                },
                register_interface::ResultField {
                    name: "leaseId".into(),
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
                register_interface::ResultField {
                    name: "recurringCharges".into(),
                },
                register_interface::ResultField {
                    name: "reservationId".into(),
                },
                register_interface::ResultField {
                    name: "startTime".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "usagePrice".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReservedInstanceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
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
            instance_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceCount").unwrap(),
            ),
            lease_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("leaseId").unwrap(),
            ),
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
            recurring_charges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recurringCharges").unwrap(),
            ),
            reservation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservationId").unwrap(),
            ),
            start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startTime").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            usage_price: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("usagePrice").unwrap(),
            ),
        }
    }
}