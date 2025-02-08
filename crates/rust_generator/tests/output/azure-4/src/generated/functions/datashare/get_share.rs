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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetShareArgs,
    ) -> GetShareResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:datashare/getShare:getShare".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetShareResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kind: pulumi_gestalt_rust::__private::into_domain(o.extract_field("kind")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            snapshot_schedules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotSchedules"),
            ),
            terms: pulumi_gestalt_rust::__private::into_domain(o.extract_field("terms")),
        }
    }
}
