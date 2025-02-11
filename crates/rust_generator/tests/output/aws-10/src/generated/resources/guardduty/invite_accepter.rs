/// Provides a resource to accept a pending GuardDuty invite on creation, ensure the detector has the correct primary account on read, and disassociate with the primary account upon removal.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = invite_accepter::create(
///         "member",
///         InviteAccepterArgs::builder()
///             .detector_id("${memberDetector.id}")
///             .master_account_id("${primary.accountId}")
///             .build_struct(),
///     );
///     let memberDetector = detector::create(
///         "memberDetector",
///         DetectorArgs::builder().build_struct(),
///     );
///     let memberMember = member::create(
///         "memberMember",
///         MemberArgs::builder()
///             .account_id("${memberDetector.accountId}")
///             .detector_id("${primary.id}")
///             .email("required@example.com")
///             .invite(true)
///             .build_struct(),
///     );
///     let primary = detector::create("primary", DetectorArgs::builder().build_struct());
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_guardduty_invite_accepter` using the member GuardDuty detector ID. For example:
///
/// ```sh
/// $ pulumi import aws:guardduty/inviteAccepter:InviteAccepter member 00b00fd5aecc0ab60a708659477e9617
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod invite_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InviteAccepterArgs {
        /// The detector ID of the member GuardDuty account.
        #[builder(into)]
        pub detector_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS account ID for primary account.
        #[builder(into)]
        pub master_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InviteAccepterResult {
        /// The detector ID of the member GuardDuty account.
        pub detector_id: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID for primary account.
        pub master_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InviteAccepterArgs,
    ) -> InviteAccepterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let detector_id_binding = args.detector_id.get_output(context);
        let master_account_id_binding = args.master_account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:guardduty/inviteAccepter:InviteAccepter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "detectorId".into(),
                    value: &detector_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterAccountId".into(),
                    value: &master_account_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InviteAccepterResult {
            detector_id: o.get_field("detectorId"),
            master_account_id: o.get_field("masterAccountId"),
        }
    }
}
