/// Resource for managing an AWS QuickSight Account Subscription.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let subscription = account_subscription::create(
///         "subscription",
///         AccountSubscriptionArgs::builder()
///             .account_name("quicksight-pulumi")
///             .authentication_method("IAM_AND_QUICKSIGHT")
///             .edition("ENTERPRISE")
///             .notification_email("notification@email.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// You cannot import this resource.
///
pub mod account_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountSubscriptionArgs {
        /// Name of your Amazon QuickSight account. This name is unique over all of AWS, and it appears only when users sign in.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of your Active Directory. This field is required if `ACTIVE_DIRECTORY` is the selected authentication method of the new Amazon QuickSight account.
        #[builder(into, default)]
        pub active_directory_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Admin group associated with your Active Directory. This field is required if `ACTIVE_DIRECTORY` is the selected authentication method of the new Amazon QuickSight account.
        #[builder(into, default)]
        pub admin_groups: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Method that you want to use to authenticate your Amazon QuickSight account. Currently, the valid values for this parameter are `IAM_AND_QUICKSIGHT`, `IAM_ONLY`, `IAM_IDENTITY_CENTER`, and `ACTIVE_DIRECTORY`.
        #[builder(into)]
        pub authentication_method: pulumi_wasm_rust::InputOrOutput<String>,
        /// Author group associated with your Active Directory.
        #[builder(into, default)]
        pub author_groups: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// AWS account ID hosting the QuickSight account. Default to provider account.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A 10-digit phone number for the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        #[builder(into, default)]
        pub contact_number: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Active Directory ID that is associated with your Amazon QuickSight account.
        #[builder(into, default)]
        pub directory_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Edition of Amazon QuickSight that you want your account to have. Currently, you can choose from `STANDARD`, `ENTERPRISE` or `ENTERPRISE_AND_Q`.
        #[builder(into)]
        pub edition: pulumi_wasm_rust::InputOrOutput<String>,
        /// Email address of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        #[builder(into, default)]
        pub email_address: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// First name of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        #[builder(into, default)]
        pub first_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for the IAM Identity Center instance.
        #[builder(into, default)]
        pub iam_identity_center_instance_arn: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Last name of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        #[builder(into, default)]
        pub last_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Email address that you want Amazon QuickSight to send notifications to regarding your Amazon QuickSight account or Amazon QuickSight subscription.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub notification_email: pulumi_wasm_rust::InputOrOutput<String>,
        /// Reader group associated with your Active Direcrtory.
        #[builder(into, default)]
        pub reader_groups: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Realm of the Active Directory that is associated with your Amazon QuickSight account.
        #[builder(into, default)]
        pub realm: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountSubscriptionResult {
        /// Name of your Amazon QuickSight account. This name is unique over all of AWS, and it appears only when users sign in.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// Status of the Amazon QuickSight account's subscription.
        pub account_subscription_status: pulumi_wasm_rust::Output<String>,
        /// Name of your Active Directory. This field is required if `ACTIVE_DIRECTORY` is the selected authentication method of the new Amazon QuickSight account.
        pub active_directory_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Admin group associated with your Active Directory. This field is required if `ACTIVE_DIRECTORY` is the selected authentication method of the new Amazon QuickSight account.
        pub admin_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Method that you want to use to authenticate your Amazon QuickSight account. Currently, the valid values for this parameter are `IAM_AND_QUICKSIGHT`, `IAM_ONLY`, `IAM_IDENTITY_CENTER`, and `ACTIVE_DIRECTORY`.
        pub authentication_method: pulumi_wasm_rust::Output<String>,
        /// Author group associated with your Active Directory.
        pub author_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// AWS account ID hosting the QuickSight account. Default to provider account.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// A 10-digit phone number for the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        pub contact_number: pulumi_wasm_rust::Output<Option<String>>,
        /// Active Directory ID that is associated with your Amazon QuickSight account.
        pub directory_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Edition of Amazon QuickSight that you want your account to have. Currently, you can choose from `STANDARD`, `ENTERPRISE` or `ENTERPRISE_AND_Q`.
        pub edition: pulumi_wasm_rust::Output<String>,
        /// Email address of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        pub email_address: pulumi_wasm_rust::Output<Option<String>>,
        /// First name of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        pub first_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) for the IAM Identity Center instance.
        pub iam_identity_center_instance_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Last name of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        pub last_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Email address that you want Amazon QuickSight to send notifications to regarding your Amazon QuickSight account or Amazon QuickSight subscription.
        ///
        /// The following arguments are optional:
        pub notification_email: pulumi_wasm_rust::Output<String>,
        /// Reader group associated with your Active Direcrtory.
        pub reader_groups: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Realm of the Active Directory that is associated with your Amazon QuickSight account.
        pub realm: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccountSubscriptionArgs,
    ) -> AccountSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_output(context).get_inner();
        let active_directory_name_binding = args
            .active_directory_name
            .get_output(context)
            .get_inner();
        let admin_groups_binding = args.admin_groups.get_output(context).get_inner();
        let authentication_method_binding = args
            .authentication_method
            .get_output(context)
            .get_inner();
        let author_groups_binding = args.author_groups.get_output(context).get_inner();
        let aws_account_id_binding = args.aws_account_id.get_output(context).get_inner();
        let contact_number_binding = args.contact_number.get_output(context).get_inner();
        let directory_id_binding = args.directory_id.get_output(context).get_inner();
        let edition_binding = args.edition.get_output(context).get_inner();
        let email_address_binding = args.email_address.get_output(context).get_inner();
        let first_name_binding = args.first_name.get_output(context).get_inner();
        let iam_identity_center_instance_arn_binding = args
            .iam_identity_center_instance_arn
            .get_output(context)
            .get_inner();
        let last_name_binding = args.last_name.get_output(context).get_inner();
        let notification_email_binding = args
            .notification_email
            .get_output(context)
            .get_inner();
        let reader_groups_binding = args.reader_groups.get_output(context).get_inner();
        let realm_binding = args.realm.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/accountSubscription:AccountSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "activeDirectoryName".into(),
                    value: &active_directory_name_binding,
                },
                register_interface::ObjectField {
                    name: "adminGroups".into(),
                    value: &admin_groups_binding,
                },
                register_interface::ObjectField {
                    name: "authenticationMethod".into(),
                    value: &authentication_method_binding,
                },
                register_interface::ObjectField {
                    name: "authorGroups".into(),
                    value: &author_groups_binding,
                },
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "contactNumber".into(),
                    value: &contact_number_binding,
                },
                register_interface::ObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding,
                },
                register_interface::ObjectField {
                    name: "edition".into(),
                    value: &edition_binding,
                },
                register_interface::ObjectField {
                    name: "emailAddress".into(),
                    value: &email_address_binding,
                },
                register_interface::ObjectField {
                    name: "firstName".into(),
                    value: &first_name_binding,
                },
                register_interface::ObjectField {
                    name: "iamIdentityCenterInstanceArn".into(),
                    value: &iam_identity_center_instance_arn_binding,
                },
                register_interface::ObjectField {
                    name: "lastName".into(),
                    value: &last_name_binding,
                },
                register_interface::ObjectField {
                    name: "notificationEmail".into(),
                    value: &notification_email_binding,
                },
                register_interface::ObjectField {
                    name: "readerGroups".into(),
                    value: &reader_groups_binding,
                },
                register_interface::ObjectField {
                    name: "realm".into(),
                    value: &realm_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccountSubscriptionResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountName"),
            ),
            account_subscription_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountSubscriptionStatus"),
            ),
            active_directory_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("activeDirectoryName"),
            ),
            admin_groups: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminGroups"),
            ),
            authentication_method: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authenticationMethod"),
            ),
            author_groups: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorGroups"),
            ),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("awsAccountId"),
            ),
            contact_number: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contactNumber"),
            ),
            directory_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("directoryId"),
            ),
            edition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("edition"),
            ),
            email_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("emailAddress"),
            ),
            first_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firstName"),
            ),
            iam_identity_center_instance_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("iamIdentityCenterInstanceArn"),
            ),
            last_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastName"),
            ),
            notification_email: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notificationEmail"),
            ),
            reader_groups: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("readerGroups"),
            ),
            realm: pulumi_wasm_rust::__private::into_domain(o.extract_field("realm")),
        }
    }
}
