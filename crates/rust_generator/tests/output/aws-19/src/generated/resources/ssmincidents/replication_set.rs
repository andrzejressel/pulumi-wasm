/// Provides a resource for managing a replication set in AWS Systems Manager Incident Manager.
///
/// > **NOTE:** Deleting a replication set also deletes all Incident Manager related data including response plans, incident records, contacts and escalation plans.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// Create a replication set.
///
/// ```yaml
/// resources:
///   replicationSetName:
///     type: aws:ssmincidents:ReplicationSet
///     properties:
///       regions:
///         - name: us-west-2
///       tags:
///         exampleTag: exampleValue
/// ```
///
/// Add a Region to a replication set. (You can add only one Region at a time.)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let replicationSetName = replication_set::create(
///         "replicationSetName",
///         ReplicationSetArgs::builder()
///             .regions(
///                 vec![
///                     ReplicationSetRegion::builder().name("us-west-2").build_struct(),
///                     ReplicationSetRegion::builder().name("ap-southeast-2")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// Delete a Region from a replication set. (You can delete only one Region at a time.)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let replicationSetName = replication_set::create(
///         "replicationSetName",
///         ReplicationSetArgs::builder()
///             .regions(
///                 vec![ReplicationSetRegion::builder().name("us-west-2").build_struct(),],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Basic Usage with an AWS Customer Managed Key
///
/// Create a replication set with an AWS Key Management Service (AWS KMS) customer manager key:
///
/// ```yaml
/// resources:
///   exampleKey:
///     type: aws:kms:Key
///     name: example_key
///   replicationSetName:
///     type: aws:ssmincidents:ReplicationSet
///     properties:
///       regions:
///         - name: us-west-2
///           kmsKeyArn: ${exampleKey.arn}
///       tags:
///         exampleTag: exampleValue
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an Incident Manager replication. For example:
///
/// ```sh
/// $ pulumi import aws:ssmincidents/replicationSet:ReplicationSet replicationSetName import
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod replication_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationSetArgs {
        #[builder(into)]
        pub regions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::ssmincidents::ReplicationSetRegion>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReplicationSetResult {
        /// The ARN of the replication set.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the user who created the replication set.
        pub created_by: pulumi_gestalt_rust::Output<String>,
        /// If `true`, the last region in a replication set cannot be deleted.
        pub deletion_protected: pulumi_gestalt_rust::Output<bool>,
        /// A timestamp showing when the replication set was last modified.
        pub last_modified_by: pulumi_gestalt_rust::Output<String>,
        pub regions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ssmincidents::ReplicationSetRegion>,
        >,
        /// The current status of the Region.
        /// * Valid Values: `ACTIVE` | `CREATING` | `UPDATING` | `DELETING` | `FAILED`
        pub status: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReplicationSetArgs,
    ) -> ReplicationSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let regions_binding = args.regions.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssmincidents/replicationSet:ReplicationSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regions".into(),
                    value: &regions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReplicationSetResult {
            arn: o.get_field("arn"),
            created_by: o.get_field("createdBy"),
            deletion_protected: o.get_field("deletionProtected"),
            last_modified_by: o.get_field("lastModifiedBy"),
            regions: o.get_field("regions"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
