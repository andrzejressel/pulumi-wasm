#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_apis {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApisArgs {
        /// API name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// API protocol.
        #[builder(into, default)]
        pub protocol_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags, each pair of which must exactly match
        /// a pair on the desired APIs.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetApisResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of API identifiers.
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        pub protocol_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetApisArgs,
    ) -> GetApisResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let protocol_type_binding = args.protocol_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:apigatewayv2/getApis:getApis".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocolType".into(),
                    value: protocol_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetApisResult {
            id: o.get_field("id"),
            ids: o.get_field("ids"),
            name: o.get_field("name"),
            protocol_type: o.get_field("protocolType"),
            tags: o.get_field("tags"),
        }
    }
}
