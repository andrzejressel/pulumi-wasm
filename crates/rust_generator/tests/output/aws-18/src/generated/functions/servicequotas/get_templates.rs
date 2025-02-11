#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_templates {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTemplatesArgs {
        /// AWS Region to which the quota increases apply.
        #[builder(into)]
        pub region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of quota increase templates for specified region. See `templates`.
        #[builder(into, default)]
        pub templates: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::servicequotas::GetTemplatesTemplate>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetTemplatesResult {
        pub id: pulumi_gestalt_rust::Output<String>,
        /// AWS Region to which the template applies.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// A list of quota increase templates for specified region. See `templates`.
        pub templates: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::servicequotas::GetTemplatesTemplate>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTemplatesArgs,
    ) -> GetTemplatesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let region_binding = args.region.get_output(context);
        let templates_binding = args.templates.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:servicequotas/getTemplates:getTemplates".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templates".into(),
                    value: &templates_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTemplatesResult {
            id: o.get_field("id"),
            region: o.get_field("region"),
            templates: o.get_field("templates"),
        }
    }
}
