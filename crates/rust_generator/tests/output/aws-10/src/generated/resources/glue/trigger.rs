/// Manages a Glue Trigger resource.
///
/// ## Example Usage
///
/// ### Conditional Trigger
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = trigger::create(
///         "example",
///         TriggerArgs::builder()
///             .actions(
///                 vec![
///                     TriggerAction::builder().jobName("${example1.name}").build_struct(),
///                 ],
///             )
///             .name("example")
///             .predicate(
///                 TriggerPredicate::builder()
///                     .conditions(
///                         vec![
///                             TriggerPredicateCondition::builder()
///                             .jobName("${example2.name}").state("SUCCEEDED")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .type_("CONDITIONAL")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### On-Demand Trigger
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = trigger::create(
///         "example",
///         TriggerArgs::builder()
///             .actions(
///                 vec![
///                     TriggerAction::builder().jobName("${exampleAwsGlueJob.name}")
///                     .build_struct(),
///                 ],
///             )
///             .name("example")
///             .type_("ON_DEMAND")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Scheduled Trigger
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = trigger::create(
///         "example",
///         TriggerArgs::builder()
///             .actions(
///                 vec![
///                     TriggerAction::builder().jobName("${exampleAwsGlueJob.name}")
///                     .build_struct(),
///                 ],
///             )
///             .name("example")
///             .schedule("cron(15 12 * * ? *)")
///             .type_("SCHEDULED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Conditional Trigger with Crawler Action
///
/// **Note:** Triggers can have both a crawler action and a crawler condition, just no example provided.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = trigger::create(
///         "example",
///         TriggerArgs::builder()
///             .actions(
///                 vec![
///                     TriggerAction::builder().crawlerName("${example1.name}")
///                     .build_struct(),
///                 ],
///             )
///             .name("example")
///             .predicate(
///                 TriggerPredicate::builder()
///                     .conditions(
///                         vec![
///                             TriggerPredicateCondition::builder()
///                             .jobName("${example2.name}").state("SUCCEEDED")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .type_("CONDITIONAL")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Conditional Trigger with Crawler Condition
///
/// **Note:** Triggers can have both a crawler action and a crawler condition, just no example provided.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = trigger::create(
///         "example",
///         TriggerArgs::builder()
///             .actions(
///                 vec![
///                     TriggerAction::builder().jobName("${example1.name}").build_struct(),
///                 ],
///             )
///             .name("example")
///             .predicate(
///                 TriggerPredicate::builder()
///                     .conditions(
///                         vec![
///                             TriggerPredicateCondition::builder().crawlState("SUCCEEDED")
///                             .crawlerName("${example2.name}").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .type_("CONDITIONAL")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Triggers using `name`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/trigger:Trigger MyTrigger MyTrigger
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trigger {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerArgs {
        /// List of actions initiated by this trigger when it fires. See Actions Below.
        #[builder(into)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::glue::TriggerAction>,
        >,
        /// A description of the new trigger.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Start the trigger. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Batch condition that must be met (specified number of events received or batch time window expired) before EventBridge event trigger fires. See Event Batching Condition.
        #[builder(into, default)]
        pub event_batching_conditions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::glue::TriggerEventBatchingCondition>>,
        >,
        /// The name of the trigger.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A predicate to specify when the new trigger should fire. Required when trigger type is `CONDITIONAL`. See Predicate Below.
        #[builder(into, default)]
        pub predicate: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::TriggerPredicate>,
        >,
        /// A cron expression used to specify the schedule. [Time-Based Schedules for Jobs and Crawlers](https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html)
        #[builder(into, default)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set to true to start `SCHEDULED` and `CONDITIONAL` triggers when created. True is not supported for `ON_DEMAND` triggers.
        #[builder(into, default)]
        pub start_on_creation: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of trigger. Valid values are `CONDITIONAL`, `EVENT`, `ON_DEMAND`, and `SCHEDULED`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A workflow to which the trigger should be associated to. Every workflow graph (DAG) needs a starting trigger (`ON_DEMAND` or `SCHEDULED` type) and can contain multiple additional `CONDITIONAL` triggers.
        #[builder(into, default)]
        pub workflow_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TriggerResult {
        /// List of actions initiated by this trigger when it fires. See Actions Below.
        pub actions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::glue::TriggerAction>,
        >,
        /// Amazon Resource Name (ARN) of Glue Trigger
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description of the new trigger.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Start the trigger. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Batch condition that must be met (specified number of events received or batch time window expired) before EventBridge event trigger fires. See Event Batching Condition.
        pub event_batching_conditions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::glue::TriggerEventBatchingCondition>>,
        >,
        /// The name of the trigger.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A predicate to specify when the new trigger should fire. Required when trigger type is `CONDITIONAL`. See Predicate Below.
        pub predicate: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::TriggerPredicate>,
        >,
        /// A cron expression used to specify the schedule. [Time-Based Schedules for Jobs and Crawlers](https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html)
        pub schedule: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set to true to start `SCHEDULED` and `CONDITIONAL` triggers when created. True is not supported for `ON_DEMAND` triggers.
        pub start_on_creation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The current state of the trigger.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of trigger. Valid values are `CONDITIONAL`, `EVENT`, `ON_DEMAND`, and `SCHEDULED`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// A workflow to which the trigger should be associated to. Every workflow graph (DAG) needs a starting trigger (`ON_DEMAND` or `SCHEDULED` type) and can contain multiple additional `CONDITIONAL` triggers.
        pub workflow_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TriggerArgs,
    ) -> TriggerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let description_binding = args.description.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let event_batching_conditions_binding = args
            .event_batching_conditions
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let predicate_binding = args.predicate.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let start_on_creation_binding = args.start_on_creation.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let workflow_name_binding = args.workflow_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/trigger:Trigger".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: &actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventBatchingConditions".into(),
                    value: &event_batching_conditions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "predicate".into(),
                    value: &predicate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startOnCreation".into(),
                    value: &start_on_creation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workflowName".into(),
                    value: &workflow_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TriggerResult {
            actions: o.get_field("actions"),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            enabled: o.get_field("enabled"),
            event_batching_conditions: o.get_field("eventBatchingConditions"),
            name: o.get_field("name"),
            predicate: o.get_field("predicate"),
            schedule: o.get_field("schedule"),
            start_on_creation: o.get_field("startOnCreation"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
            workflow_name: o.get_field("workflowName"),
        }
    }
}
