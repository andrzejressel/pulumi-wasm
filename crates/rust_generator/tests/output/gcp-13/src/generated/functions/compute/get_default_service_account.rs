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
        /// Email address of the default service account used by VMs running in this project
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
        context: &pulumi_gestalt_rust::Context,
        args: GetDefaultServiceAccountArgs,
    ) -> GetDefaultServiceAccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getDefaultServiceAccount:getDefaultServiceAccount"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDefaultServiceAccountResult {
            display_name: o.get_field("displayName"),
            email: o.get_field("email"),
            id: o.get_field("id"),
            member: o.get_field("member"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            unique_id: o.get_field("uniqueId"),
        }
    }
}
