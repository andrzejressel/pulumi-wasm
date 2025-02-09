#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_share {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetShareArgs {
        /// The ID of the Data Share account in which the Data Share is created.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of this Data Share.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetShareResult {
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The description of the Data Share.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The kind of the Data Share.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The name of the snapshot schedule.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `snapshot_schedule` block as defined below.
        pub snapshot_schedules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::datashare::GetShareSnapshotSchedule>,
        >,
        /// The terms of the Data Share.
        pub terms: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetShareArgs,
    ) -> GetShareResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:datashare/getShare:getShare".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetShareResult {
            account_id: o.get_field("accountId"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            kind: o.get_field("kind"),
            name: o.get_field("name"),
            snapshot_schedules: o.get_field("snapshotSchedules"),
            terms: o.get_field("terms"),
        }
    }
}
