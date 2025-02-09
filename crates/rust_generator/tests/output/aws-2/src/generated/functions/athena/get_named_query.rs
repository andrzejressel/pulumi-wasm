#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_named_query {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamedQueryArgs {
        /// The plain language name for the query. Maximum length of 128.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The workgroup to which the query belongs. Defaults to `primary`.
        #[builder(into, default)]
        pub workgroup: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNamedQueryResult {
        /// Database to which the query belongs.
        pub database: pulumi_gestalt_rust::Output<String>,
        /// Brief explanation of the query.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub querystring: pulumi_gestalt_rust::Output<String>,
        pub workgroup: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetNamedQueryArgs,
    ) -> GetNamedQueryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let workgroup_binding_1 = args.workgroup.get_output(context);
        let workgroup_binding = workgroup_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:athena/getNamedQuery:getNamedQuery".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "workgroup".into(),
                    value: &workgroup_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetNamedQueryResult {
            database: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("database"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            querystring: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("querystring"),
            ),
            workgroup: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workgroup"),
            ),
        }
    }
}
