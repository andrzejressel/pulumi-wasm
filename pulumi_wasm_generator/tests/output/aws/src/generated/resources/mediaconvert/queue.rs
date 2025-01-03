/// Provides an AWS Elemental MediaConvert Queue.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod queue {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct QueueArgs {
        /// A description of the queue
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique identifier describing the queue
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies whether the pricing plan for the queue is on-demand or reserved. Valid values are `ON_DEMAND` or `RESERVED`. Default to `ON_DEMAND`.
        #[builder(into, default)]
        pub pricing_plan: pulumi_wasm_rust::Output<Option<String>>,
        /// A detail pricing plan of the  reserved queue. See below.
        #[builder(into, default)]
        pub reservation_plan_settings: pulumi_wasm_rust::Output<
            Option<super::super::types::mediaconvert::QueueReservationPlanSettings>,
        >,
        /// A status of the queue. Valid values are `ACTIVE` or `RESERVED`. Default to `PAUSED`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct QueueResult {
        /// The Arn of the queue
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A description of the queue
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique identifier describing the queue
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the pricing plan for the queue is on-demand or reserved. Valid values are `ON_DEMAND` or `RESERVED`. Default to `ON_DEMAND`.
        pub pricing_plan: pulumi_wasm_rust::Output<Option<String>>,
        /// A detail pricing plan of the  reserved queue. See below.
        pub reservation_plan_settings: pulumi_wasm_rust::Output<
            super::super::types::mediaconvert::QueueReservationPlanSettings,
        >,
        /// A status of the queue. Valid values are `ACTIVE` or `RESERVED`. Default to `PAUSED`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: QueueArgs) -> QueueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let pricing_plan_binding = args.pricing_plan.get_inner();
        let reservation_plan_settings_binding = args
            .reservation_plan_settings
            .get_inner();
        let status_binding = args.status.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:mediaconvert/queue:Queue".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pricingPlan".into(),
                },
                register_interface::ResultField {
                    name: "reservationPlanSettings".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        QueueResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pricing_plan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pricingPlan").unwrap(),
            ),
            reservation_plan_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservationPlanSettings").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
