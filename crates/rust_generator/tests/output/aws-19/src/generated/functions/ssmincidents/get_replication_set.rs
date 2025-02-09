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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetReplicationSetArgs,
    ) -> GetReplicationSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssmincidents/getReplicationSet:getReplicationSet".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetReplicationSetResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            created_by: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdBy"),
            ),
            deletion_protected: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionProtected"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            last_modified_by: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModifiedBy"),
            ),
            regions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("regions"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
