#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_dicom_store_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDicomStoreIamPolicyArgs {
        /// The DICOM store ID, in the form
        /// `{project_id}/{location_name}/{dataset_name}/{dicom_store_name}` or
        /// `{location_name}/{dataset_name}/{dicom_store_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        #[builder(into)]
        pub dicom_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDicomStoreIamPolicyResult {
        pub dicom_store_id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The policy data
        pub policy_data: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDicomStoreIamPolicyArgs,
    ) -> GetDicomStoreIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dicom_store_id_binding = args.dicom_store_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:healthcare/getDicomStoreIamPolicy:getDicomStoreIamPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dicomStoreId".into(),
                    value: &dicom_store_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDicomStoreIamPolicyResult {
            dicom_store_id: o.get_field("dicomStoreId"),
            etag: o.get_field("etag"),
            id: o.get_field("id"),
            policy_data: o.get_field("policyData"),
        }
    }
}
