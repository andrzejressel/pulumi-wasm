#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_default_service_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDefaultServiceAccountArgs {
        /// The project ID. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDefaultServiceAccountResult {
        /// The display name for the service account.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Email address of the default service account used by App Engine in this project.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Identity of the service account in the form `serviceAccount:{email}`. This value is often used to refer to the service account in order to grant IAM permissions.
        pub member: pulumi_gestalt_rust::Output<String>,
        /// The fully-qualified name of the service account.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The unique id of the service account.
        pub unique_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDefaultServiceAccountArgs,
    ) -> GetDefaultServiceAccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:appengine/getDefaultServiceAccount:getDefaultServiceAccount"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDefaultServiceAccountResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            email: pulumi_gestalt_rust::__private::into_domain(o.extract_field("email")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            member: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("member"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            unique_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("uniqueId"),
            ),
        }
    }
}
