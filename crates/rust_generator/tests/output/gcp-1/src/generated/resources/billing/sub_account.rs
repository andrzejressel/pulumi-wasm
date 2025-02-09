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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SubAccountArgs,
    ) -> SubAccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let deletion_policy_binding_1 = args.deletion_policy.get_output(context);
        let deletion_policy_binding = deletion_policy_binding_1.get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let master_billing_account_binding_1 = args
            .master_billing_account
            .get_output(context);
        let master_billing_account_binding = master_billing_account_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:billing/subAccount:SubAccount".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "masterBillingAccount".into(),
                    value: &master_billing_account_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubAccountResult {
            billing_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("billingAccountId"),
            ),
            deletion_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionPolicy"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            master_billing_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("masterBillingAccount"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            open: pulumi_gestalt_rust::__private::into_domain(o.extract_field("open")),
        }
    }
}
