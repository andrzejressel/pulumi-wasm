#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_project_service_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProjectServiceAccountArgs {
        /// The project the unique service account was created for. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project the lookup originates from. This field is used if you are making the request
        /// from a different account than the one you are finding the service account for.
        #[builder(into, default)]
        pub user_project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetProjectServiceAccountResult {
        /// The email address of the service account. This value is often used to refer to the service account
        /// in order to grant IAM permissions.
        pub email_address: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Identity of the service account in the form `serviceAccount:{email_address}`. This value is often used to refer to the service account in order to grant IAM permissions.
        pub member: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub user_project: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetProjectServiceAccountArgs,
    ) -> GetProjectServiceAccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let user_project_binding_1 = args.user_project.get_output(context);
        let user_project_binding = user_project_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:storage/getProjectServiceAccount:getProjectServiceAccount"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "userProject".into(),
                    value: &user_project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetProjectServiceAccountResult {
            email_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("emailAddress"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            member: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("member"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            user_project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userProject"),
            ),
        }
    }
}
