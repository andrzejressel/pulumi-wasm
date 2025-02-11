#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_project {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProjectArgs {
        /// The project ID. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetProjectResult {
        pub auto_create_network: pulumi_gestalt_rust::Output<bool>,
        pub billing_account: pulumi_gestalt_rust::Output<String>,
        pub deletion_policy: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub folder_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The numeric identifier of the project.
        pub number: pulumi_gestalt_rust::Output<String>,
        pub org_id: pulumi_gestalt_rust::Output<String>,
        pub project_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetProjectArgs,
    ) -> GetProjectResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let project_id_binding = args.project_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:organizations/getProject:getProject".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "projectId".into(),
                    value: &project_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetProjectResult {
            auto_create_network: o.get_field("autoCreateNetwork"),
            billing_account: o.get_field("billingAccount"),
            deletion_policy: o.get_field("deletionPolicy"),
            effective_labels: o.get_field("effectiveLabels"),
            folder_id: o.get_field("folderId"),
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            number: o.get_field("number"),
            org_id: o.get_field("orgId"),
            project_id: o.get_field("projectId"),
            pulumi_labels: o.get_field("pulumiLabels"),
            tags: o.get_field("tags"),
        }
    }
}
