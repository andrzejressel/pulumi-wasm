/// Resource for managing an AWS SSM Contact Plan.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PlanArgs {
        /// The Amazon Resource Name (ARN) of the contact or escalation plan.
        #[builder(into)]
        pub contact_id: pulumi_wasm_rust::Output<String>,
        /// One or more configuration blocks for specifying a list of stages that the escalation plan or engagement plan uses to engage contacts and contact methods. See Stage below for more details.
        #[builder(into)]
        pub stages: pulumi_wasm_rust::Output<
            Vec<super::super::types::ssmcontacts::PlanStage>,
        >,
    }
    #[allow(dead_code)]
    pub struct PlanResult {
        /// The Amazon Resource Name (ARN) of the contact or escalation plan.
        pub contact_id: pulumi_wasm_rust::Output<String>,
        /// One or more configuration blocks for specifying a list of stages that the escalation plan or engagement plan uses to engage contacts and contact methods. See Stage below for more details.
        pub stages: pulumi_wasm_rust::Output<
            Vec<super::super::types::ssmcontacts::PlanStage>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PlanArgs) -> PlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let contact_id_binding = args.contact_id.get_inner();
        let stages_binding = args.stages.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssmcontacts/plan:Plan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contactId".into(),
                    value: &contact_id_binding,
                },
                register_interface::ObjectField {
                    name: "stages".into(),
                    value: &stages_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "contactId".into(),
                },
                register_interface::ResultField {
                    name: "stages".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PlanResult {
            contact_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contactId").unwrap(),
            ),
            stages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stages").unwrap(),
            ),
        }
    }
}
