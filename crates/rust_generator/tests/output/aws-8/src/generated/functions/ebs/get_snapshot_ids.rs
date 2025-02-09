#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_snapshot_ids {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotIdsArgs {
        /// One or more name/value pairs to filter off of. There are
        /// several valid keys, for a full reference, check out
        /// [describe-volumes in the AWS CLI reference][1].
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ebs::GetSnapshotIdsFilter>>,
        >,
        /// Returns the snapshots owned by the specified owner id. Multiple owners can be specified.
        #[builder(into, default)]
        pub owners: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more AWS accounts IDs that can create volumes from the snapshot.
        #[builder(into, default)]
        pub restorable_by_user_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotIdsResult {
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ebs::GetSnapshotIdsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Set of EBS snapshot IDs, sorted by creation time in descending order.
        pub ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub owners: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub restorable_by_user_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSnapshotIdsArgs,
    ) -> GetSnapshotIdsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let owners_binding = args.owners.get_output(context);
        let restorable_by_user_ids_binding = args
            .restorable_by_user_ids
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ebs/getSnapshotIds:getSnapshotIds".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "owners".into(),
                    value: owners_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restorableByUserIds".into(),
                    value: restorable_by_user_ids_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSnapshotIdsResult {
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            ids: o.get_field("ids"),
            owners: o.get_field("owners"),
            restorable_by_user_ids: o.get_field("restorableByUserIds"),
        }
    }
}
