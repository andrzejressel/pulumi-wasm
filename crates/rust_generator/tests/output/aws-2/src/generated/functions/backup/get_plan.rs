#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPlanArgs {
        /// Backup plan ID.
        #[builder(into)]
        pub plan_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Metadata that you can assign to help organize the plans you create.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPlanResult {
        /// ARN of the backup plan.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Display name of a backup plan.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub plan_id: pulumi_gestalt_rust::Output<String>,
        /// Rules of a backup plan.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::backup::GetPlanRule>,
        >,
        /// Metadata that you can assign to help organize the plans you create.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Unique, randomly generated, Unicode, UTF-8 encoded string that serves as the version ID of the backup plan.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPlanArgs,
    ) -> GetPlanResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let plan_id_binding = args.plan_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:backup/getPlan:getPlan".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "planId".into(),
                    value: plan_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPlanResult {
            arn: o.get_field("arn"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            plan_id: o.get_field("planId"),
            rules: o.get_field("rules"),
            tags: o.get_field("tags"),
            version: o.get_field("version"),
        }
    }
}
