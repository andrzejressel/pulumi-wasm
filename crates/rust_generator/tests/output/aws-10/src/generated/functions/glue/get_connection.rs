#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectionArgs {
        /// Concatenation of the catalog ID and connection name. For example, if your account ID is
        /// `123456789123` and the connection name is `conn` then the ID is `123456789123:conn`.
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tags assigned to the resource
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetConnectionResult {
        /// ARN of the Glue Connection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Catalog ID of the Glue Connection.
        pub catalog_id: pulumi_gestalt_rust::Output<String>,
        pub connection_properties: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of Glue Connection.
        pub connection_type: pulumi_gestalt_rust::Output<String>,
        /// Description of the connection.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of criteria that can be used in selecting this connection.
        pub match_criterias: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Name of the Glue Connection.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of physical connection requirements, such as VPC and SecurityGroup.
        pub physical_connection_requirements: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::glue::GetConnectionPhysicalConnectionRequirement,
            >,
        >,
        /// Tags assigned to the resource
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConnectionArgs,
    ) -> GetConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:glue/getConnection:getConnection".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConnectionResult {
            arn: o.get_field("arn"),
            catalog_id: o.get_field("catalogId"),
            connection_properties: o.get_field("connectionProperties"),
            connection_type: o.get_field("connectionType"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            match_criterias: o.get_field("matchCriterias"),
            name: o.get_field("name"),
            physical_connection_requirements: o
                .get_field("physicalConnectionRequirements"),
            tags: o.get_field("tags"),
        }
    }
}
