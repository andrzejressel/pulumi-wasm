/// Manages AWS Compute Optimizer enrollment status.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = enrollment_status::create(
///         "example",
///         EnrollmentStatusArgs::builder().status("Active").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import enrollment status using the account ID. For example:
///
/// ```sh
/// $ pulumi import aws:computeoptimizer/enrollmentStatus:EnrollmentStatus example 123456789012
/// ```
pub mod enrollment_status {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnrollmentStatusArgs {
        /// Whether to enroll member accounts of the organization if the account is the management account of an organization. Default is `false`.
        #[builder(into, default)]
        pub include_member_accounts: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The enrollment status of the account. Valid values: `Active`, `Inactive`.
        #[builder(into)]
        pub status: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::computeoptimizer::EnrollmentStatusTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnrollmentStatusResult {
        /// Whether to enroll member accounts of the organization if the account is the management account of an organization. Default is `false`.
        pub include_member_accounts: pulumi_wasm_rust::Output<bool>,
        /// The count of organization member accounts that are opted in to the service, if your account is an organization management account.
        pub number_of_member_accounts_opted_in: pulumi_wasm_rust::Output<i32>,
        /// The enrollment status of the account. Valid values: `Active`, `Inactive`.
        pub status: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::computeoptimizer::EnrollmentStatusTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EnrollmentStatusArgs,
    ) -> EnrollmentStatusResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let include_member_accounts_binding = args
            .include_member_accounts
            .get_output(context)
            .get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:computeoptimizer/enrollmentStatus:EnrollmentStatus".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "includeMemberAccounts".into(),
                    value: &include_member_accounts_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EnrollmentStatusResult {
            include_member_accounts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("includeMemberAccounts"),
            ),
            number_of_member_accounts_opted_in: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("numberOfMemberAccountsOptedIn"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
