/// Resource for managing an AWS QuickSight Account Subscription.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountSubscriptionArgs {
        /// Name of your Amazon QuickSight account. This name is unique over all of AWS, and it appears only when users sign in.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of your Active Directory. This field is required if `ACTIVE_DIRECTORY` is the selected authentication method of the new Amazon QuickSight account.
        #[builder(into, default)]
        pub active_directory_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Admin group associated with your Active Directory. This field is required if `ACTIVE_DIRECTORY` is the selected authentication method of the new Amazon QuickSight account.
        #[builder(into, default)]
        pub admin_groups: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Method that you want to use to authenticate your Amazon QuickSight account. Currently, the valid values for this parameter are `IAM_AND_QUICKSIGHT`, `IAM_ONLY`, `IAM_IDENTITY_CENTER`, and `ACTIVE_DIRECTORY`.
        #[builder(into)]
        pub authentication_method: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Author group associated with your Active Directory.
        #[builder(into, default)]
        pub author_groups: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// AWS account ID hosting the QuickSight account. Default to provider account.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A 10-digit phone number for the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        #[builder(into, default)]
        pub contact_number: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Active Directory ID that is associated with your Amazon QuickSight account.
        #[builder(into, default)]
        pub directory_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Edition of Amazon QuickSight that you want your account to have. Currently, you can choose from `STANDARD`, `ENTERPRISE` or `ENTERPRISE_AND_Q`.
        #[builder(into)]
        pub edition: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Email address of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        #[builder(into, default)]
        pub email_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// First name of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        #[builder(into, default)]
        pub first_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for the IAM Identity Center instance.
        #[builder(into, default)]
        pub iam_identity_center_instance_arn: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Last name of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        #[builder(into, default)]
        pub last_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Email address that you want Amazon QuickSight to send notifications to regarding your Amazon QuickSight account or Amazon QuickSight subscription.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub notification_email: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Reader group associated with your Active Direcrtory.
        #[builder(into, default)]
        pub reader_groups: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Realm of the Active Directory that is associated with your Amazon QuickSight account.
        #[builder(into, default)]
        pub realm: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountSubscriptionResult {
        /// Name of your Amazon QuickSight account. This name is unique over all of AWS, and it appears only when users sign in.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// Status of the Amazon QuickSight account's subscription.
        pub account_subscription_status: pulumi_gestalt_rust::Output<String>,
        /// Name of your Active Directory. This field is required if `ACTIVE_DIRECTORY` is the selected authentication method of the new Amazon QuickSight account.
        pub active_directory_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Admin group associated with your Active Directory. This field is required if `ACTIVE_DIRECTORY` is the selected authentication method of the new Amazon QuickSight account.
        pub admin_groups: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Method that you want to use to authenticate your Amazon QuickSight account. Currently, the valid values for this parameter are `IAM_AND_QUICKSIGHT`, `IAM_ONLY`, `IAM_IDENTITY_CENTER`, and `ACTIVE_DIRECTORY`.
        pub authentication_method: pulumi_gestalt_rust::Output<String>,
        /// Author group associated with your Active Directory.
        pub author_groups: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// AWS account ID hosting the QuickSight account. Default to provider account.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// A 10-digit phone number for the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        pub contact_number: pulumi_gestalt_rust::Output<Option<String>>,
        /// Active Directory ID that is associated with your Amazon QuickSight account.
        pub directory_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Edition of Amazon QuickSight that you want your account to have. Currently, you can choose from `STANDARD`, `ENTERPRISE` or `ENTERPRISE_AND_Q`.
        pub edition: pulumi_gestalt_rust::Output<String>,
        /// Email address of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        pub email_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// First name of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        pub first_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) for the IAM Identity Center instance.
        pub iam_identity_center_instance_arn: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Last name of the author of the Amazon QuickSight account to use for future communications. This field is required if `ENTERPPRISE_AND_Q` is the selected edition of the new Amazon QuickSight account.
        pub last_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Email address that you want Amazon QuickSight to send notifications to regarding your Amazon QuickSight account or Amazon QuickSight subscription.
        ///
        /// The following arguments are optional:
        pub notification_email: pulumi_gestalt_rust::Output<String>,
        /// Reader group associated with your Active Direcrtory.
        pub reader_groups: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Realm of the Active Directory that is associated with your Amazon QuickSight account.
        pub realm: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountSubscriptionArgs,
    ) -> AccountSubscriptionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_name_binding = args.account_name.get_output(context);
        let active_directory_name_binding = args
            .active_directory_name
            .get_output(context);
        let admin_groups_binding = args.admin_groups.get_output(context);
        let authentication_method_binding = args
            .authentication_method
            .get_output(context);
        let author_groups_binding = args.author_groups.get_output(context);
        let aws_account_id_binding = args.aws_account_id.get_output(context);
        let contact_number_binding = args.contact_number.get_output(context);
        let directory_id_binding = args.directory_id.get_output(context);
        let edition_binding = args.edition.get_output(context);
        let email_address_binding = args.email_address.get_output(context);
        let first_name_binding = args.first_name.get_output(context);
        let iam_identity_center_instance_arn_binding = args
            .iam_identity_center_instance_arn
            .get_output(context);
        let last_name_binding = args.last_name.get_output(context);
        let notification_email_binding = args.notification_email.get_output(context);
        let reader_groups_binding = args.reader_groups.get_output(context);
        let realm_binding = args.realm.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:quicksight/accountSubscription:AccountSubscription".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "activeDirectoryName".into(),
                    value: active_directory_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminGroups".into(),
                    value: admin_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationMethod".into(),
                    value: authentication_method_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorGroups".into(),
                    value: author_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: aws_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contactNumber".into(),
                    value: contact_number_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "directoryId".into(),
                    value: directory_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edition".into(),
                    value: edition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailAddress".into(),
                    value: email_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firstName".into(),
                    value: first_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamIdentityCenterInstanceArn".into(),
                    value: iam_identity_center_instance_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lastName".into(),
                    value: last_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationEmail".into(),
                    value: notification_email_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readerGroups".into(),
                    value: reader_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "realm".into(),
                    value: realm_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountSubscriptionResult {
            account_name: o.get_field("accountName"),
            account_subscription_status: o.get_field("accountSubscriptionStatus"),
            active_directory_name: o.get_field("activeDirectoryName"),
            admin_groups: o.get_field("adminGroups"),
            authentication_method: o.get_field("authenticationMethod"),
            author_groups: o.get_field("authorGroups"),
            aws_account_id: o.get_field("awsAccountId"),
            contact_number: o.get_field("contactNumber"),
            directory_id: o.get_field("directoryId"),
            edition: o.get_field("edition"),
            email_address: o.get_field("emailAddress"),
            first_name: o.get_field("firstName"),
            iam_identity_center_instance_arn: o
                .get_field("iamIdentityCenterInstanceArn"),
            last_name: o.get_field("lastName"),
            notification_email: o.get_field("notificationEmail"),
            reader_groups: o.get_field("readerGroups"),
            realm: o.get_field("realm"),
        }
    }
}
