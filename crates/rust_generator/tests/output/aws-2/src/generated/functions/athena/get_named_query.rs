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
        context: &pulumi_gestalt_rust::Context,
        args: GetNamedQueryArgs,
    ) -> GetNamedQueryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let workgroup_binding = args.workgroup.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:athena/getNamedQuery:getNamedQuery".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workgroup".into(),
                    value: workgroup_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNamedQueryResult {
            database: o.get_field("database"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            querystring: o.get_field("querystring"),
            workgroup: o.get_field("workgroup"),
        }
    }
}
