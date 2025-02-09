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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: QueueArgs,
    ) -> QueueResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let pricing_plan_binding = args.pricing_plan.get_output(context);
        let reservation_plan_settings_binding = args
            .reservation_plan_settings
            .get_output(context);
        let status_binding = args.status.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:mediaconvert/queue:Queue".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pricingPlan".into(),
                    value: pricing_plan_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reservationPlanSettings".into(),
                    value: reservation_plan_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        QueueResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            pricing_plan: o.get_field("pricingPlan"),
            reservation_plan_settings: o.get_field("reservationPlanSettings"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
