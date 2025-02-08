#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_workspaces {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkspacesArgs {
        /// Limits results to workspaces with aliases that begin with this value.
        #[builder(into, default)]
        pub alias_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetWorkspacesResult {
        pub alias_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of aliases of the matched Prometheus workspaces.
        pub aliases: pulumi_gestalt_rust::Output<Vec<String>>,
        /// List of ARNs of the matched Prometheus workspaces.
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of workspace IDs of the matched Prometheus workspaces.
        pub workspace_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetWorkspacesArgs,
    ) -> GetWorkspacesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let alias_prefix_binding = args.alias_prefix.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:amp/getWorkspaces:getWorkspaces".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aliasPrefix".into(),
                    value: &alias_prefix_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetWorkspacesResult {
            alias_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("aliasPrefix"),
            ),
            aliases: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("aliases"),
            ),
            arns: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arns")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            workspace_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceIds"),
            ),
        }
    }
}
