#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_workload_identity_pool_provider {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkloadIdentityPoolProviderArgs {
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The id of the pool which is the
        /// final component of the pool resource name.
        #[builder(into)]
        pub workload_identity_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the provider which is the
        /// final component of the resource name.
        ///
        /// - - -
        #[builder(into)]
        pub workload_identity_pool_provider_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct GetWorkloadIdentityPoolProviderResult {
        pub attribute_condition: pulumi_gestalt_rust::Output<String>,
        pub attribute_mapping: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub aws: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::iam::GetWorkloadIdentityPoolProviderAw>,
        >,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub disabled: pulumi_gestalt_rust::Output<bool>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub oidcs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::iam::GetWorkloadIdentityPoolProviderOidc>,
        >,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub samls: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::iam::GetWorkloadIdentityPoolProviderSaml>,
        >,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub workload_identity_pool_id: pulumi_gestalt_rust::Output<String>,
        pub workload_identity_pool_provider_id: pulumi_gestalt_rust::Output<String>,
        pub x509s: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::iam::GetWorkloadIdentityPoolProviderX509>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetWorkloadIdentityPoolProviderArgs,
    ) -> GetWorkloadIdentityPoolProviderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let project_binding = args.project.get_output(context);
        let workload_identity_pool_id_binding = args
            .workload_identity_pool_id
            .get_output(context);
        let workload_identity_pool_provider_id_binding = args
            .workload_identity_pool_provider_id
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:iam/getWorkloadIdentityPoolProvider:getWorkloadIdentityPoolProvider"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workloadIdentityPoolId".into(),
                    value: &workload_identity_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workloadIdentityPoolProviderId".into(),
                    value: &workload_identity_pool_provider_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetWorkloadIdentityPoolProviderResult {
            attribute_condition: o.get_field("attributeCondition"),
            attribute_mapping: o.get_field("attributeMapping"),
            aws: o.get_field("aws"),
            description: o.get_field("description"),
            disabled: o.get_field("disabled"),
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            oidcs: o.get_field("oidcs"),
            project: o.get_field("project"),
            samls: o.get_field("samls"),
            state: o.get_field("state"),
            workload_identity_pool_id: o.get_field("workloadIdentityPoolId"),
            workload_identity_pool_provider_id: o
                .get_field("workloadIdentityPoolProviderId"),
            x509s: o.get_field("x509s"),
        }
    }
}
