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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetWorkloadIdentityPoolProviderArgs,
    ) -> GetWorkloadIdentityPoolProviderResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_output(context).get_inner();
        let workload_identity_pool_id_binding = args
            .workload_identity_pool_id
            .get_output(context)
            .get_inner();
        let workload_identity_pool_provider_id_binding = args
            .workload_identity_pool_provider_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:iam/getWorkloadIdentityPoolProvider:getWorkloadIdentityPoolProvider"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "workloadIdentityPoolId".into(),
                    value: &workload_identity_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "workloadIdentityPoolProviderId".into(),
                    value: &workload_identity_pool_provider_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetWorkloadIdentityPoolProviderResult {
            attribute_condition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attributeCondition"),
            ),
            attribute_mapping: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attributeMapping"),
            ),
            aws: pulumi_gestalt_rust::__private::into_domain(o.extract_field("aws")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            oidcs: pulumi_gestalt_rust::__private::into_domain(o.extract_field("oidcs")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            samls: pulumi_gestalt_rust::__private::into_domain(o.extract_field("samls")),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            workload_identity_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadIdentityPoolId"),
            ),
            workload_identity_pool_provider_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadIdentityPoolProviderId"),
            ),
            x509s: pulumi_gestalt_rust::__private::into_domain(o.extract_field("x509s")),
        }
    }
}
