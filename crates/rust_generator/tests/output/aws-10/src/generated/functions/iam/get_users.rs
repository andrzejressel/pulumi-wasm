#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_users {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUsersArgs {
        /// Regex string to apply to the IAM users list returned by AWS. This allows more advanced filtering not supported from the AWS API. This filtering is done locally on what AWS returns, and could have a performance impact if the result is large. Combine this with other options to narrow down the list AWS returns.
        #[builder(into, default)]
        pub name_regex: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Path prefix for filtering the results. For example, the prefix `/division_abc/subdivision_xyz/` gets all users whose path starts with `/division_abc/subdivision_xyz/`. If it is not included, it defaults to a slash (`/`), listing all users. For more details, check out [list-users in the AWS CLI reference][1].
        #[builder(into, default)]
        pub path_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetUsersResult {
        /// Set of ARNs of the matched IAM users.
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name_regex: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of Names of the matched IAM users.
        pub names: pulumi_gestalt_rust::Output<Vec<String>>,
        pub path_prefix: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetUsersArgs,
    ) -> GetUsersResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_regex_binding = args.name_regex.get_output(context).get_inner();
        let path_prefix_binding = args.path_prefix.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getUsers:getUsers".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "nameRegex".into(),
                    value: &name_regex_binding,
                },
                register_interface::ObjectField {
                    name: "pathPrefix".into(),
                    value: &path_prefix_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUsersResult {
            arns: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arns")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name_regex: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nameRegex"),
            ),
            names: pulumi_gestalt_rust::__private::into_domain(o.extract_field("names")),
            path_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pathPrefix"),
            ),
        }
    }
}
