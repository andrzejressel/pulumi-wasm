/// Manages an RDS DB Reserved Instance.
///
/// > **NOTE:** Once created, a reservation is valid for the `duration` of the provided `offering_id` and cannot be deleted. Performing a `destroy` will only remove the resource from state. For more information see [RDS Reserved Instances Documentation](https://aws.amazon.com/rds/reserved-instances/) and [PurchaseReservedDBInstancesOffering](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_PurchaseReservedDBInstancesOffering.html).
///
/// > **NOTE:** Due to the expense of testing this resource, we provide it as best effort. If you find it useful, and have the ability to help test or notice issues, consider reaching out to us on GitHub.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:rds:ReservedInstance
///     properties:
///       offeringId: ${test.offeringId}
///       reservationId: optionalCustomReservationID
///       instanceCount: 3
/// variables:
///   test:
///     fn::invoke:
///       function: aws:rds:getReservedInstanceOffering
///       arguments:
///         dbInstanceClass: db.t2.micro
///         duration: 3.1536e+07
///         multiAz: false
///         offeringType: All Upfront
///         productDescription: mysql
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RDS DB Instance Reservations using the `instance_id`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/reservedInstance:ReservedInstance reservation_instance CustomReservationID
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod reserved_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReservedInstanceArgs {
        /// Number of instances to reserve. Default value is `1`.
        #[builder(into, default)]
        pub instance_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// ID of the Reserved DB instance offering to purchase. To determine an `offering_id`, see the `aws.rds.getReservedInstanceOffering` data source.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub offering_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Customer-specified identifier to track this reservation.
        #[builder(into, default)]
        pub reservation_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the DB reservation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReservedInstanceResult {
        /// ARN for the reserved DB instance.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Currency code for the reserved DB instance.
        pub currency_code: pulumi_gestalt_rust::Output<String>,
        /// DB instance class for the reserved DB instance.
        pub db_instance_class: pulumi_gestalt_rust::Output<String>,
        /// Duration of the reservation in seconds.
        pub duration: pulumi_gestalt_rust::Output<i32>,
        /// Fixed price charged for this reserved DB instance.
        pub fixed_price: pulumi_gestalt_rust::Output<f64>,
        /// Number of instances to reserve. Default value is `1`.
        pub instance_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Unique identifier for the lease associated with the reserved DB instance. Amazon Web Services Support might request the lease ID for an issue related to a reserved DB instance.
        pub lease_id: pulumi_gestalt_rust::Output<String>,
        /// Whether the reservation applies to Multi-AZ deployments.
        pub multi_az: pulumi_gestalt_rust::Output<bool>,
        /// ID of the Reserved DB instance offering to purchase. To determine an `offering_id`, see the `aws.rds.getReservedInstanceOffering` data source.
        ///
        /// The following arguments are optional:
        pub offering_id: pulumi_gestalt_rust::Output<String>,
        /// Offering type of this reserved DB instance.
        pub offering_type: pulumi_gestalt_rust::Output<String>,
        /// Description of the reserved DB instance.
        pub product_description: pulumi_gestalt_rust::Output<String>,
        /// Recurring price charged to run this reserved DB instance.
        pub recurring_charges: pulumi_gestalt_rust::Output<
            Vec<super::super::types::rds::ReservedInstanceRecurringCharge>,
        >,
        /// Customer-specified identifier to track this reservation.
        pub reservation_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Time the reservation started.
        pub start_time: pulumi_gestalt_rust::Output<String>,
        /// State of the reserved DB instance.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the DB reservation. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Hourly price charged for this reserved DB instance.
        pub usage_price: pulumi_gestalt_rust::Output<f64>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReservedInstanceArgs,
    ) -> ReservedInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_count_binding = args.instance_count.get_output(context);
        let offering_id_binding = args.offering_id.get_output(context);
        let reservation_id_binding = args.reservation_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/reservedInstance:ReservedInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceCount".into(),
                    value: &instance_count_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "offeringId".into(),
                    value: &offering_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reservationId".into(),
                    value: &reservation_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReservedInstanceResult {
            arn: o.get_field("arn"),
            currency_code: o.get_field("currencyCode"),
            db_instance_class: o.get_field("dbInstanceClass"),
            duration: o.get_field("duration"),
            fixed_price: o.get_field("fixedPrice"),
            instance_count: o.get_field("instanceCount"),
            lease_id: o.get_field("leaseId"),
            multi_az: o.get_field("multiAz"),
            offering_id: o.get_field("offeringId"),
            offering_type: o.get_field("offeringType"),
            product_description: o.get_field("productDescription"),
            recurring_charges: o.get_field("recurringCharges"),
            reservation_id: o.get_field("reservationId"),
            start_time: o.get_field("startTime"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            usage_price: o.get_field("usagePrice"),
        }
    }
}
