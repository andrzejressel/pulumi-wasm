/// Provides a resource to create a member account in the current organization.
///
/// > **Note:** Account management must be done from the organization's root account.
///
/// > **Note:** By default, deleting this resource will only remove an AWS account from an organization. You must set the `close_on_deletion` flag to true to close the account. It is worth noting that quotas are enforced when using the `close_on_deletion` argument, which can produce a [CLOSE_ACCOUNT_QUOTA_EXCEEDED](https://docs.aws.amazon.com/organizations/latest/APIReference/API_CloseAccount.html) error, and require you to close the account manually.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let account = account::create(
///         "account",
///         AccountArgs::builder()
///             .email("john@doe.org")
///             .name("my_new_account")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the AWS member account using the `account_id`. For example:
///
/// ```sh
/// $ pulumi import aws:organizations/account:Account my_account 111111111111
/// ```
/// To import accounts that have set iam_user_access_to_billing, use the following:
///
/// ```sh
/// $ pulumi import aws:organizations/account:Account my_account 111111111111_ALLOW
/// ```
/// Certain resource arguments, like `role_name`, do not have an Organizations API method for reading the information after account creation. If the argument is set in the Pulumi program on an imported resource, Pulumi will always show a difference. To workaround this behavior, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:
///
pub mod account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// If true, a deletion event will close the account. Otherwise, it will only remove from the organization. This is not supported for GovCloud accounts.
        #[builder(into, default)]
        pub close_on_deletion: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to also create a GovCloud account. The GovCloud account is tied to the main (commercial) account this resource creates. If `true`, the GovCloud account ID is available in the `govcloud_id` attribute. The only way to manage the GovCloud account with the provider is to subsequently import the account using this resource.
        #[builder(into, default)]
        pub create_govcloud: pulumi_wasm_rust::Output<Option<bool>>,
        /// Email address of the owner to assign to the new member account. This email address must not already be associated with another AWS account.
        #[builder(into)]
        pub email: pulumi_wasm_rust::Output<String>,
        /// If set to `ALLOW`, the new account enables IAM users and roles to access account billing information if they have the required permissions. If set to `DENY`, then only the root user (and no roles) of the new account can access account billing information. If this is unset, the AWS API will default this to `ALLOW`. If the resource is created and this option is changed, it will try to recreate the account.
        #[builder(into, default)]
        pub iam_user_access_to_billing: pulumi_wasm_rust::Output<Option<String>>,
        /// Friendly name for the member account.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Parent Organizational Unit ID or Root ID for the account. Defaults to the Organization default Root ID. A configuration must be present for this argument to perform drift detection.
        #[builder(into, default)]
        pub parent_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of an IAM role that Organizations automatically preconfigures in the new member account. This role trusts the root account, allowing users in the root account to assume the role, as permitted by the root account administrator. The role has administrator permissions in the new member account. The Organizations API provides no method for reading this information after account creation, so the provider cannot perform drift detection on its value and will always show a difference for a configured value after import unless `ignoreChanges` is used.
        #[builder(into, default)]
        pub role_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// The ARN for this account.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// If true, a deletion event will close the account. Otherwise, it will only remove from the organization. This is not supported for GovCloud accounts.
        pub close_on_deletion: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to also create a GovCloud account. The GovCloud account is tied to the main (commercial) account this resource creates. If `true`, the GovCloud account ID is available in the `govcloud_id` attribute. The only way to manage the GovCloud account with the provider is to subsequently import the account using this resource.
        pub create_govcloud: pulumi_wasm_rust::Output<Option<bool>>,
        /// Email address of the owner to assign to the new member account. This email address must not already be associated with another AWS account.
        pub email: pulumi_wasm_rust::Output<String>,
        /// ID for a GovCloud account created with the account.
        pub govcloud_id: pulumi_wasm_rust::Output<String>,
        /// If set to `ALLOW`, the new account enables IAM users and roles to access account billing information if they have the required permissions. If set to `DENY`, then only the root user (and no roles) of the new account can access account billing information. If this is unset, the AWS API will default this to `ALLOW`. If the resource is created and this option is changed, it will try to recreate the account.
        pub iam_user_access_to_billing: pulumi_wasm_rust::Output<Option<String>>,
        pub joined_method: pulumi_wasm_rust::Output<String>,
        pub joined_timestamp: pulumi_wasm_rust::Output<String>,
        /// Friendly name for the member account.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Parent Organizational Unit ID or Root ID for the account. Defaults to the Organization default Root ID. A configuration must be present for this argument to perform drift detection.
        pub parent_id: pulumi_wasm_rust::Output<String>,
        /// The name of an IAM role that Organizations automatically preconfigures in the new member account. This role trusts the root account, allowing users in the root account to assume the role, as permitted by the root account administrator. The role has administrator permissions in the new member account. The Organizations API provides no method for reading this information after account creation, so the provider cannot perform drift detection on its value and will always show a difference for a configured value after import unless `ignoreChanges` is used.
        pub role_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The status of the account in the organization.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccountArgs) -> AccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let close_on_deletion_binding = args.close_on_deletion.get_inner();
        let create_govcloud_binding = args.create_govcloud.get_inner();
        let email_binding = args.email.get_inner();
        let iam_user_access_to_billing_binding = args
            .iam_user_access_to_billing
            .get_inner();
        let name_binding = args.name.get_inner();
        let parent_id_binding = args.parent_id.get_inner();
        let role_name_binding = args.role_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:organizations/account:Account".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "closeOnDeletion".into(),
                    value: &close_on_deletion_binding,
                },
                register_interface::ObjectField {
                    name: "createGovcloud".into(),
                    value: &create_govcloud_binding,
                },
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
                register_interface::ObjectField {
                    name: "iamUserAccessToBilling".into(),
                    value: &iam_user_access_to_billing_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding,
                },
                register_interface::ObjectField {
                    name: "roleName".into(),
                    value: &role_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "closeOnDeletion".into(),
                },
                register_interface::ResultField {
                    name: "createGovcloud".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "govcloudId".into(),
                },
                register_interface::ResultField {
                    name: "iamUserAccessToBilling".into(),
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
                    name: "parentId".into(),
                },
                register_interface::ResultField {
                    name: "roleName".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            close_on_deletion: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("closeOnDeletion").unwrap(),
            ),
            create_govcloud: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createGovcloud").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            govcloud_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("govcloudId").unwrap(),
            ),
            iam_user_access_to_billing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamUserAccessToBilling").unwrap(),
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
            parent_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentId").unwrap(),
            ),
            role_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleName").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}