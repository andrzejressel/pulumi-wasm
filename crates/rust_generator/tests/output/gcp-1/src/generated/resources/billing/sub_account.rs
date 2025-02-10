/// Allows creation and management of a Google Cloud Billing Subaccount.
///
/// !> **WARNING:** Deleting this resource will not delete or close the billing subaccount.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let subaccount = sub_account::create(
///         "subaccount",
///         SubAccountArgs::builder()
///             .display_name("My Billing Account")
///             .master_billing_account("012345-567890-ABCDEF")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Billing Subaccounts can be imported using any of these accepted formats:
///
/// * `billingAccounts/{billing_account_id}`
///
/// When using the `pulumi import` command, Billing Subaccounts can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:billing/subAccount:SubAccount default billingAccounts/{billing_account_id}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sub_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubAccountArgs {
        /// If set to "RENAME_ON_DESTROY" the billing account display_name
        /// will be changed to "Destroyed" along with a timestamp.  If set to "" this will not occur.
        /// Default is "".
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of the billing account.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the master billing account that the subaccount
        /// will be created under in the form `{billing_account_id}` or `billingAccounts/{billing_account_id}`.
        #[builder(into)]
        pub master_billing_account: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SubAccountResult {
        /// The billing account id.
        pub billing_account_id: pulumi_gestalt_rust::Output<String>,
        /// If set to "RENAME_ON_DESTROY" the billing account display_name
        /// will be changed to "Destroyed" along with a timestamp.  If set to "" this will not occur.
        /// Default is "".
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name of the billing account.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the master billing account that the subaccount
        /// will be created under in the form `{billing_account_id}` or `billingAccounts/{billing_account_id}`.
        pub master_billing_account: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the billing account in the form `billingAccounts/{billing_account_id}`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// `true` if the billing account is open, `false` if the billing account is closed.
        pub open: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubAccountArgs,
    ) -> SubAccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deletion_policy_binding = args.deletion_policy.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let master_billing_account_binding = args
            .master_billing_account
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:billing/subAccount:SubAccount".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionPolicy".into(),
                    value: deletion_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterBillingAccount".into(),
                    value: master_billing_account_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SubAccountResult {
            billing_account_id: o.get_field("billingAccountId"),
            deletion_policy: o.get_field("deletionPolicy"),
            display_name: o.get_field("displayName"),
            master_billing_account: o.get_field("masterBillingAccount"),
            name: o.get_field("name"),
            open: o.get_field("open"),
        }
    }
}
