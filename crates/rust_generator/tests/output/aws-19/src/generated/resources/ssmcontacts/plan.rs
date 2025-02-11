/// Resource for managing an AWS SSM Contact Plan.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = plan::create(
///         "example",
///         PlanArgs::builder()
///             .contact_id(
///                 "arn:aws:ssm-contacts:us-west-2:123456789012:contact/contactalias",
///             )
///             .stages(vec![PlanStage::builder().durationInMinutes(1).build_struct(),])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Usage with SSM Contact
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let contact = contact::create(
///         "contact",
///         ContactArgs::builder().alias("alias").type_("PERSONAL").build_struct(),
///     );
///     let plan = plan::create(
///         "plan",
///         PlanArgs::builder()
///             .contact_id("${contact.arn}")
///             .stages(vec![PlanStage::builder().durationInMinutes(1).build_struct(),])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Usage With All Fields
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let contactOne = contact::create(
///         "contactOne",
///         ContactArgs::builder().alias("alias").type_("PERSONAL").build_struct(),
///     );
///     let contactTwo = contact::create(
///         "contactTwo",
///         ContactArgs::builder().alias("alias").type_("PERSONAL").build_struct(),
///     );
///     let escalationPlan = contact::create(
///         "escalationPlan",
///         ContactArgs::builder()
///             .alias("escalation-plan-alias")
///             .type_("ESCALATION")
///             .build_struct(),
///     );
///     let test = plan::create(
///         "test",
///         PlanArgs::builder()
///             .contact_id("${escalationPlan.arn}")
///             .stages(
///                 vec![
///                     PlanStage::builder().durationInMinutes(0)
///                     .targets(vec![PlanStageTarget::builder()
///                     .contactTargetInfo(PlanStageTargetContactTargetInfo::builder()
///                     .contactId("${contactOne.arn}").isEssential(false).build_struct())
///                     .build_struct(), PlanStageTarget::builder()
///                     .contactTargetInfo(PlanStageTargetContactTargetInfo::builder()
///                     .contactId("${contactTwo.arn}").isEssential(true).build_struct())
///                     .build_struct(), PlanStageTarget::builder()
///                     .channelTargetInfo(PlanStageTargetChannelTargetInfo::builder()
///                     .contactChannelId("${channel.arn}").retryIntervalInMinutes(2)
///                     .build_struct()).build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSM Contact Plan using the Contact ARN. For example:
///
/// ```sh
/// $ pulumi import aws:ssmcontacts/plan:Plan example {ARNValue}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PlanArgs {
        /// The Amazon Resource Name (ARN) of the contact or escalation plan.
        #[builder(into)]
        pub contact_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more configuration blocks for specifying a list of stages that the escalation plan or engagement plan uses to engage contacts and contact methods. See Stage below for more details.
        #[builder(into)]
        pub stages: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::ssmcontacts::PlanStage>,
        >,
    }
    #[allow(dead_code)]
    pub struct PlanResult {
        /// The Amazon Resource Name (ARN) of the contact or escalation plan.
        pub contact_id: pulumi_gestalt_rust::Output<String>,
        /// One or more configuration blocks for specifying a list of stages that the escalation plan or engagement plan uses to engage contacts and contact methods. See Stage below for more details.
        pub stages: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ssmcontacts::PlanStage>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PlanArgs,
    ) -> PlanResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let contact_id_binding = args.contact_id.get_output(context);
        let stages_binding = args.stages.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssmcontacts/plan:Plan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contactId".into(),
                    value: &contact_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stages".into(),
                    value: &stages_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PlanResult {
            contact_id: o.get_field("contactId"),
            stages: o.get_field("stages"),
        }
    }
}
