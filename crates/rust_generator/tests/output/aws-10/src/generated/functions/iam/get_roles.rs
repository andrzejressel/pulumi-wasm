#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_roles {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRolesArgs {
        /// Regex string to apply to the IAM roles list returned by AWS. This allows more advanced filtering not supported from the AWS API. This filtering is done locally on what AWS returns, and could have a performance impact if the result is large. Combine this with other options to narrow down the list AWS returns.
        #[builder(into, default)]
        pub name_regex: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Path prefix for filtering the results. For example, the prefix `/application_abc/component_xyz/` gets all roles whose path starts with `/application_abc/component_xyz/`. If it is not included, it defaults to a slash (`/`), listing all roles. For more details, check out [list-roles in the AWS CLI reference][1].
        #[builder(into, default)]
        pub path_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRolesResult {
        /// Set of ARNs of the matched IAM roles.
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name_regex: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of Names of the matched IAM roles.
        pub names: pulumi_gestalt_rust::Output<Vec<String>>,
        pub path_prefix: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRolesArgs,
    ) -> GetRolesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_regex_binding = args.name_regex.get_output(context);
        let path_prefix_binding = args.path_prefix.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:iam/getRoles:getRoles".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nameRegex".into(),
                    value: name_regex_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pathPrefix".into(),
                    value: path_prefix_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRolesResult {
            arns: o.get_field("arns"),
            id: o.get_field("id"),
            name_regex: o.get_field("nameRegex"),
            names: o.get_field("names"),
            path_prefix: o.get_field("pathPrefix"),
        }
    }
}
