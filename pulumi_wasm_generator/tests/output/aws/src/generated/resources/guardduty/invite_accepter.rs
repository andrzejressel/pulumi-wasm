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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InviteAccepterArgs {
        /// The detector ID of the member GuardDuty account.
        #[builder(into)]
        pub detector_id: pulumi_wasm_rust::Output<String>,
        /// AWS account ID for primary account.
        #[builder(into)]
        pub master_account_id: pulumi_wasm_rust::Output<String>,
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
    pub fn create(name: &str, args: InviteAccepterArgs) -> InviteAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let detector_id_binding = args.detector_id.get_inner();
        let master_account_id_binding = args.master_account_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:guardduty/inviteAccepter:InviteAccepter".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "detectorId".into(),
                },
                register_interface::ResultField {
                    name: "masterAccountId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InviteAccepterResult {
            detector_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("detectorId").unwrap(),
            ),
            master_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterAccountId").unwrap(),
            ),
        }
    }
}