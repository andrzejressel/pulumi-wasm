/// Provides a resource to accept a pending GuardDuty invite on creation, ensure the detector has the correct primary account on read, and disassociate with the primary account upon removal.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod invite_accepter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InviteAccepterArgs {
        /// The detector ID of the member GuardDuty account.
        #[builder(into)]
        pub detector_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// AWS account ID for primary account.
        #[builder(into)]
        pub master_account_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InviteAccepterResult {
        /// The detector ID of the member GuardDuty account.
        pub detector_id: pulumi_wasm_rust::Output<String>,
        /// AWS account ID for primary account.
        pub master_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InviteAccepterArgs,
    ) -> InviteAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let detector_id_binding = args.detector_id.get_output(context).get_inner();
        let master_account_id_binding = args
            .master_account_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:guardduty/inviteAccepter:InviteAccepter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "detectorId".into(),
                    value: &detector_id_binding,
                },
                register_interface::ObjectField {
                    name: "masterAccountId".into(),
                    value: &master_account_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InviteAccepterResult {
            detector_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("detectorId"),
            ),
            master_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("masterAccountId"),
            ),
        }
    }
}
