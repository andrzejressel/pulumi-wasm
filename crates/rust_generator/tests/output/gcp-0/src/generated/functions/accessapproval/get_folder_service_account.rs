#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_folder_service_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFolderServiceAccountArgs {
        /// The folder ID the service account was created for.
        #[builder(into)]
        pub folder_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFolderServiceAccountResult {
        /// The email address of the service account. This value is
        /// often used to refer to the service account in order to grant IAM permissions.
        pub account_email: pulumi_gestalt_rust::Output<String>,
        pub folder_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Access Approval service account resource name. Format is "folders/{folder_id}/serviceAccount".
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFolderServiceAccountArgs,
    ) -> GetFolderServiceAccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let folder_id_binding = args.folder_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:accessapproval/getFolderServiceAccount:getFolderServiceAccount"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folderId".into(),
                    value: &folder_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFolderServiceAccountResult {
            account_email: o.get_field("accountEmail"),
            folder_id: o.get_field("folderId"),
            id: o.get_field("id"),
            name: o.get_field("name"),
        }
    }
}
