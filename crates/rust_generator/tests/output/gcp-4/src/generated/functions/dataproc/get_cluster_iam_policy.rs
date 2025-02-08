#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_cluster_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterIamPolicyArgs {
        /// The name or relative resource id of the cluster to manage IAM policies for.
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetClusterIamPolicyResult {
        pub cluster: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The policy data
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetClusterIamPolicyArgs,
    ) -> GetClusterIamPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_binding = args.cluster.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:dataproc/getClusterIamPolicy:getClusterIamPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClusterIamPolicyResult {
            cluster: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cluster"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            policy_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
        }
    }
}
