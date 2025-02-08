/// Provides an AWS Elemental MediaConvert Queue.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = queue::create(
///         "test",
///         QueueArgs::builder().name("tf-test-queue").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Media Convert Queue using the queue name. For example:
///
/// ```sh
/// $ pulumi import aws:mediaconvert/queue:Queue test tf-test-queue
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod queue {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueueArgs {
        /// A description of the queue
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A unique identifier describing the queue
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the pricing plan for the queue is on-demand or reserved. Valid values are `ON_DEMAND` or `RESERVED`. Default to `ON_DEMAND`.
        #[builder(into, default)]
        pub pricing_plan: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A detail pricing plan of the  reserved queue. See below.
        #[builder(into, default)]
        pub reservation_plan_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mediaconvert::QueueReservationPlanSettings>,
        >,
        /// A status of the queue. Valid values are `ACTIVE` or `RESERVED`. Default to `PAUSED`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct QueueResult {
        /// The Arn of the queue
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description of the queue
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A unique identifier describing the queue
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the pricing plan for the queue is on-demand or reserved. Valid values are `ON_DEMAND` or `RESERVED`. Default to `ON_DEMAND`.
        pub pricing_plan: pulumi_gestalt_rust::Output<Option<String>>,
        /// A detail pricing plan of the  reserved queue. See below.
        pub reservation_plan_settings: pulumi_gestalt_rust::Output<
            super::super::types::mediaconvert::QueueReservationPlanSettings,
        >,
        /// A status of the queue. Valid values are `ACTIVE` or `RESERVED`. Default to `PAUSED`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: QueueArgs,
    ) -> QueueResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let pricing_plan_binding = args.pricing_plan.get_output(context).get_inner();
        let reservation_plan_settings_binding = args
            .reservation_plan_settings
            .get_output(context)
            .get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:mediaconvert/queue:Queue".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pricingPlan".into(),
                    value: &pricing_plan_binding,
                },
                register_interface::ObjectField {
                    name: "reservationPlanSettings".into(),
                    value: &reservation_plan_settings_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        QueueResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            pricing_plan: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pricingPlan"),
            ),
            reservation_plan_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reservationPlanSettings"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
