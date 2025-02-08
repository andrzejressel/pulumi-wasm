#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetPlanArgs,
    ) -> GetPlanResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let contact_id_binding = args.contact_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssmcontacts/getPlan:getPlan".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contactId".into(),
                    value: &contact_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPlanResult {
            contact_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contactId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            stages: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stages"),
            ),
        }
    }
}
