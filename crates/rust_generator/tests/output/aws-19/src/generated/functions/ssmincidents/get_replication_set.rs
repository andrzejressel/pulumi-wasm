#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_replication_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReplicationSetArgs {
        /// All tags applied to the replication set.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetReplicationSetResult {
        /// The Amazon Resouce Name (ARN) of the replication set.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the user who created the replication set.
        pub created_by: pulumi_gestalt_rust::Output<String>,
        /// If `true`, the last remaining Region in a replication set can’t be deleted.
        pub deletion_protected: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the user who last modified the replication set.
        pub last_modified_by: pulumi_gestalt_rust::Output<String>,
        pub regions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ssmincidents::GetReplicationSetRegion>,
        >,
        /// The current status of the Region.
        /// * Valid Values: `ACTIVE` | `CREATING` | `UPDATING` | `DELETING` | `FAILED`
        pub status: pulumi_gestalt_rust::Output<String>,
        /// All tags applied to the replication set.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetReplicationSetArgs,
    ) -> GetReplicationSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ssmincidents/getReplicationSet:getReplicationSet".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetReplicationSetResult {
            arn: o.get_field("arn"),
            created_by: o.get_field("createdBy"),
            deletion_protected: o.get_field("deletionProtected"),
            id: o.get_field("id"),
            last_modified_by: o.get_field("lastModifiedBy"),
            regions: o.get_field("regions"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
        }
    }
}
