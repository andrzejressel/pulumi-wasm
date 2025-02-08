#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_billing_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBillingAccountArgs {
        /// The name of the billing account in the form `{billing_account_id}` or `billingAccounts/{billing_account_id}`.
        #[builder(into, default)]
        pub billing_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of the billing account.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// `true` if projects associated with the billing account should be read, `false` if this step
        /// should be skipped. Setting `false` may be useful if the user permissions do not allow listing projects. Defaults to `true`.
        ///
        /// > **NOTE:** One of `billing_account` or `display_name` must be specified.
        #[builder(into, default)]
        pub lookup_projects: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// `true` if the billing account is open, `false` if the billing account is closed.
        #[builder(into, default)]
        pub open: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetBillingAccountResult {
        pub billing_account: pulumi_gestalt_rust::Output<Option<String>>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub lookup_projects: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The resource name of the billing account in the form `billingAccounts/{billing_account_id}`.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub open: pulumi_gestalt_rust::Output<bool>,
        /// The IDs of any projects associated with the billing account. `lookup_projects` must not be false
        /// for this to be populated.
        pub project_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetBillingAccountArgs,
    ) -> GetBillingAccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let billing_account_binding = args
            .billing_account
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let lookup_projects_binding = args
            .lookup_projects
            .get_output(context)
            .get_inner();
        let open_binding = args.open.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:organizations/getBillingAccount:getBillingAccount".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "billingAccount".into(),
                    value: &billing_account_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "lookupProjects".into(),
                    value: &lookup_projects_binding,
                },
                register_interface::ObjectField {
                    name: "open".into(),
                    value: &open_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetBillingAccountResult {
            billing_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("billingAccount"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            lookup_projects: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lookupProjects"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            open: pulumi_gestalt_rust::__private::into_domain(o.extract_field("open")),
            project_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("projectIds"),
            ),
        }
    }
}
