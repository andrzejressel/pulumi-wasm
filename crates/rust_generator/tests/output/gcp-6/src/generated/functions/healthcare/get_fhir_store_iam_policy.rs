#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_fhir_store_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFhirStoreIamPolicyArgs {
        /// The FHIR store ID, in the form
        /// `{project_id}/{location_name}/{dataset_name}/{fhir_store_name}` or
        /// `{location_name}/{dataset_name}/{fhir_store_name}`. In the second form, the provider's
        /// project setting will be used as a fallback.
        #[builder(into)]
        pub fhir_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFhirStoreIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub fhir_store_id: pulumi_gestalt_rust::Output<String>,
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetFhirStoreIamPolicyArgs,
    ) -> GetFhirStoreIamPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let fhir_store_id_binding_1 = args.fhir_store_id.get_output(context);
        let fhir_store_id_binding = fhir_store_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:healthcare/getFhirStoreIamPolicy:getFhirStoreIamPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fhirStoreId".into(),
                    value: &fhir_store_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFhirStoreIamPolicyResult {
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            fhir_store_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fhirStoreId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            policy_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
        }
    }
}
