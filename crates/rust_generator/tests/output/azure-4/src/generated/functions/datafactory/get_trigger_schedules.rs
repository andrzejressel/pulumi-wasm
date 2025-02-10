#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_trigger_schedules {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTriggerSchedulesArgs {
        /// The ID of the Azure Data Factory to fetch trigger schedules from.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTriggerSchedulesResult {
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of trigger schedule names available in this Azure Data Factory.
        pub items: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTriggerSchedulesArgs,
    ) -> GetTriggerSchedulesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:datafactory/getTriggerSchedules:getTriggerSchedules".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFactoryId".into(),
                    value: data_factory_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTriggerSchedulesResult {
            data_factory_id: o.get_field("dataFactoryId"),
            id: o.get_field("id"),
            items: o.get_field("items"),
        }
    }
}
