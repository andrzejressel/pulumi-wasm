#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_activity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetActivityArgs {
        /// ARN that identifies the activity.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name that identifies the activity.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetActivityResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date the activity was created.
        pub creation_date: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetActivityArgs,
    ) -> GetActivityResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:sfn/getActivity:getActivity".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetActivityResult {
            arn: o.get_field("arn"),
            creation_date: o.get_field("creationDate"),
            id: o.get_field("id"),
            name: o.get_field("name"),
        }
    }
}
