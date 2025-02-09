/// Manages AWS Compute Optimizer enrollment status.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod enrollment_status {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnrollmentStatusArgs {
        /// Whether to enroll member accounts of the organization if the account is the management account of an organization. Default is `false`.
        #[builder(into, default)]
        pub include_member_accounts: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The enrollment status of the account. Valid values: `Active`, `Inactive`.
        #[builder(into)]
        pub status: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::computeoptimizer::EnrollmentStatusTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnrollmentStatusResult {
        /// Whether to enroll member accounts of the organization if the account is the management account of an organization. Default is `false`.
        pub include_member_accounts: pulumi_gestalt_rust::Output<bool>,
        /// The count of organization member accounts that are opted in to the service, if your account is an organization management account.
        pub number_of_member_accounts_opted_in: pulumi_gestalt_rust::Output<i32>,
        /// The enrollment status of the account. Valid values: `Active`, `Inactive`.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::computeoptimizer::EnrollmentStatusTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnrollmentStatusArgs,
    ) -> EnrollmentStatusResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let include_member_accounts_binding = args
            .include_member_accounts
            .get_output(context);
        let status_binding = args.status.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:computeoptimizer/enrollmentStatus:EnrollmentStatus".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeMemberAccounts".into(),
                    value: include_member_accounts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnrollmentStatusResult {
            include_member_accounts: o.get_field("includeMemberAccounts"),
            number_of_member_accounts_opted_in: o
                .get_field("numberOfMemberAccountsOptedIn"),
            status: o.get_field("status"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
