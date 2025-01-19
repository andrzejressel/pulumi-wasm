/// Provides a resource to manage an [AWS Organizations Delegated Administrator](https://docs.aws.amazon.com/organizations/latest/APIReference/API_RegisterDelegatedAdministrator.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod delegated_administrator {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DelegatedAdministratorArgs {
        /// The account ID number of the member account in the organization to register as a delegated administrator.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The service principal of the AWS service for which you want to make the member account a delegated administrator.
        #[builder(into)]
        pub service_principal: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DelegatedAdministratorResult {
        /// The account ID number of the member account in the organization to register as a delegated administrator.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the delegated administrator's account.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The date when the account was made a delegated administrator.
        pub delegation_enabled_date: pulumi_wasm_rust::Output<String>,
        /// The email address that is associated with the delegated administrator's AWS account.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The method by which the delegated administrator's account joined the organization.
        pub joined_method: pulumi_wasm_rust::Output<String>,
        /// The date when the delegated administrator's account became a part of the organization.
        pub joined_timestamp: pulumi_wasm_rust::Output<String>,
        /// The friendly name of the delegated administrator's account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The service principal of the AWS service for which you want to make the member account a delegated administrator.
        pub service_principal: pulumi_wasm_rust::Output<String>,
        /// The status of the delegated administrator's account in the organization.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DelegatedAdministratorArgs,
    ) -> DelegatedAdministratorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let service_principal_binding = args.service_principal.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "delegationEnabledDate".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "joinedMethod".into(),
                },
                register_interface::ResultField {
                    name: "joinedTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "servicePrincipal".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DelegatedAdministratorResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            delegation_enabled_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("delegationEnabledDate").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            joined_method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("joinedMethod").unwrap(),
            ),
            joined_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("joinedTimestamp").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            service_principal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePrincipal").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
