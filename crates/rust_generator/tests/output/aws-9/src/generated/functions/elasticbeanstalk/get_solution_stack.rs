#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_solution_stack {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSolutionStackArgs {
        /// If more than one result is returned, use the most
        /// recent solution stack.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Regex string to apply to the solution stack list returned
        /// by AWS. See [Elastic Beanstalk Supported Platforms][beanstalk-platforms] from
        /// AWS documentation for reference solution stack names.
        ///
        /// > **NOTE:** If more or less than a single match is returned by the search,
        /// this call will fail. Ensure that your search is specific enough to return
        /// a single solution stack, or use `most_recent` to choose the most recent one.
        #[builder(into)]
        pub name_regex: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSolutionStackResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the solution stack.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub name_regex: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSolutionStackArgs,
    ) -> GetSolutionStackResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let most_recent_binding_1 = args.most_recent.get_output(context);
        let most_recent_binding = most_recent_binding_1.get_inner();
        let name_regex_binding_1 = args.name_regex.get_output(context);
        let name_regex_binding = name_regex_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:elasticbeanstalk/getSolutionStack:getSolutionStack".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding,
                },
                register_interface::ObjectField {
                    name: "nameRegex".into(),
                    value: &name_regex_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSolutionStackResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            most_recent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mostRecent"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_regex: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nameRegex"),
            ),
        }
    }
}
