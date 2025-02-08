/// Provides a resource to manage an [AWS Organizations Delegated Administrator](https://docs.aws.amazon.com/organizations/latest/APIReference/API_RegisterDelegatedAdministrator.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = delegated_administrator::create(
///         "example",
///         DelegatedAdministratorArgs::builder()
///             .account_id("123456789012")
///             .service_principal("principal")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_organizations_delegated_administrator` using the account ID and its service principal. For example:
///
/// ```sh
/// $ pulumi import aws:organizations/delegatedAdministrator:DelegatedAdministrator example 123456789012/config.amazonaws.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod delegated_administrator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DelegatedAdministratorArgs {
        /// The account ID number of the member account in the organization to register as a delegated administrator.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The service principal of the AWS service for which you want to make the member account a delegated administrator.
        #[builder(into)]
        pub service_principal: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DelegatedAdministratorResult {
        /// The account ID number of the member account in the organization to register as a delegated administrator.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the delegated administrator's account.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The date when the account was made a delegated administrator.
        pub delegation_enabled_date: pulumi_gestalt_rust::Output<String>,
        /// The email address that is associated with the delegated administrator's AWS account.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The method by which the delegated administrator's account joined the organization.
        pub joined_method: pulumi_gestalt_rust::Output<String>,
        /// The date when the delegated administrator's account became a part of the organization.
        pub joined_timestamp: pulumi_gestalt_rust::Output<String>,
        /// The friendly name of the delegated administrator's account.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The service principal of the AWS service for which you want to make the member account a delegated administrator.
        pub service_principal: pulumi_gestalt_rust::Output<String>,
        /// The status of the delegated administrator's account in the organization.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DelegatedAdministratorArgs,
    ) -> DelegatedAdministratorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let service_principal_binding = args
            .service_principal
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:organizations/delegatedAdministrator:DelegatedAdministrator"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "servicePrincipal".into(),
                    value: &service_principal_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DelegatedAdministratorResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            delegation_enabled_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("delegationEnabledDate"),
            ),
            email: pulumi_gestalt_rust::__private::into_domain(o.extract_field("email")),
            joined_method: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("joinedMethod"),
            ),
            joined_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("joinedTimestamp"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            service_principal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("servicePrincipal"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}
