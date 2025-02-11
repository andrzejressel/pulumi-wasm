#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeyArgs {
        /// ID of the API Key to look up.
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetKeyResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date and time when the API Key was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Amazon Web Services Marketplace customer identifier, when integrating with the Amazon Web Services SaaS Marketplace.
        pub customer_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the API Key.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Whether the API Key is enabled.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// Set to the ID of the API Key.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Date and time when the API Key was last updated.
        pub last_updated_date: pulumi_gestalt_rust::Output<String>,
        /// Set to the name of the API Key.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Map of tags for the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Set to the value of the API Key.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetKeyArgs,
    ) -> GetKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:apigateway/getKey:getKey".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKeyResult {
            arn: o.get_field("arn"),
            created_date: o.get_field("createdDate"),
            customer_id: o.get_field("customerId"),
            description: o.get_field("description"),
            enabled: o.get_field("enabled"),
            id: o.get_field("id"),
            last_updated_date: o.get_field("lastUpdatedDate"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            value: o.get_field("value"),
        }
    }
}
