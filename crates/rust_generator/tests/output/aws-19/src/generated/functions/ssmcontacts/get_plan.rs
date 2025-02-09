#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPlanArgs {
        /// The Amazon Resource Name (ARN) of the contact or escalation plan.
        #[builder(into)]
        pub contact_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPlanResult {
        pub contact_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of stages. A contact has an engagement plan with stages that contact specified contact channels. An escalation plan uses stages that contact specified contacts.
        pub stages: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ssmcontacts::GetPlanStage>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPlanArgs,
    ) -> GetPlanResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let contact_id_binding = args.contact_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ssmcontacts/getPlan:getPlan".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contactId".into(),
                    value: contact_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPlanResult {
            contact_id: o.get_field("contactId"),
            id: o.get_field("id"),
            stages: o.get_field("stages"),
        }
    }
}
