#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_managed_folder_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedFolderIamPolicyArgs {
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub managed_folder: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagedFolderIamPolicyResult {
        pub bucket: pulumi_gestalt_rust::Output<String>,
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub managed_folder: pulumi_gestalt_rust::Output<String>,
        pub policy_data: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetManagedFolderIamPolicyArgs,
    ) -> GetManagedFolderIamPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let managed_folder_binding = args.managed_folder.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:storage/getManagedFolderIamPolicy:getManagedFolderIamPolicy"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedFolder".into(),
                    value: managed_folder_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetManagedFolderIamPolicyResult {
            bucket: o.get_field("bucket"),
            etag: o.get_field("etag"),
            id: o.get_field("id"),
            managed_folder: o.get_field("managedFolder"),
            policy_data: o.get_field("policyData"),
        }
    }
}
