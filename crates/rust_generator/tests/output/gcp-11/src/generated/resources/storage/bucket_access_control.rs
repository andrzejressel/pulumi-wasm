/// Bucket ACLs can be managed authoritatively using the
/// `storage_bucket_acl` resource. Do not use these two resources in conjunction to manage the same bucket.
///
/// The BucketAccessControls resource manages the Access Control List
/// (ACLs) for a single entity/role pairing on a bucket. ACLs let you specify who
/// has access to your data and to what extent.
///
/// There are three roles that can be assigned to an entity:
///
/// READERs can get the bucket, though no acl property will be returned, and
/// list the bucket's objects.  WRITERs are READERs, and they can insert
/// objects into the bucket and delete the bucket's objects.  OWNERs are
/// WRITERs, and they can get the acl property of a bucket, update a bucket,
/// and call all BucketAccessControls methods on the bucket.  For more
/// information, see Access Control, with the caveat that this API uses
/// READER, WRITER, and OWNER instead of READ, WRITE, and FULL_CONTROL.
///
///
/// To get more information about BucketAccessControl, see:
///
/// * [API documentation](https://cloud.google.com/storage/docs/json_api/v1/bucketAccessControls)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/storage/docs/access-control/lists)
///
/// ## Example Usage
///
/// ### Storage Bucket Access Control Public Bucket
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bucket = bucket::create(
///         "bucket",
///         BucketArgs::builder().location("US").name("static-content-bucket").build_struct(),
///     );
///     let publicRule = bucket_access_control::create(
///         "publicRule",
///         BucketAccessControlArgs::builder()
///             .bucket("${bucket.name}")
///             .entity("allUsers")
///             .role("READER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// BucketAccessControl can be imported using any of these accepted formats:
///
/// * `{{bucket}}/{{entity}}`
///
/// When using the `pulumi import` command, BucketAccessControl can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:storage/bucketAccessControl:BucketAccessControl default {{bucket}}/{{entity}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bucket_access_control {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketAccessControlArgs {
        /// The name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The entity holding the permission, in one of the following forms:
        /// user-userId
        /// user-email
        /// group-groupId
        /// group-email
        /// domain-domain
        /// project-team-projectId
        /// allUsers
        /// allAuthenticatedUsers
        /// Examples:
        /// The user liz@example.com would be user-liz@example.com.
        /// The group example@googlegroups.com would be
        /// group-example@googlegroups.com.
        /// To refer to all members of the Google Apps for Business domain
        /// example.com, the entity would be domain-example.com.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub entity: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The access permission for the entity.
        /// Possible values are: `OWNER`, `READER`, `WRITER`.
        #[builder(into, default)]
        pub role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BucketAccessControlResult {
        /// The name of the bucket.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// The domain associated with the entity.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// The email address associated with the entity.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The entity holding the permission, in one of the following forms:
        /// user-userId
        /// user-email
        /// group-groupId
        /// group-email
        /// domain-domain
        /// project-team-projectId
        /// allUsers
        /// allAuthenticatedUsers
        /// Examples:
        /// The user liz@example.com would be user-liz@example.com.
        /// The group example@googlegroups.com would be
        /// group-example@googlegroups.com.
        /// To refer to all members of the Google Apps for Business domain
        /// example.com, the entity would be domain-example.com.
        ///
        ///
        /// - - -
        pub entity: pulumi_gestalt_rust::Output<String>,
        /// The access permission for the entity.
        /// Possible values are: `OWNER`, `READER`, `WRITER`.
        pub role: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BucketAccessControlArgs,
    ) -> BucketAccessControlResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let entity_binding = args.entity.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:storage/bucketAccessControl:BucketAccessControl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucket".into(),
                    value: bucket_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entity".into(),
                    value: entity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        BucketAccessControlResult {
            bucket: o.get_field("bucket"),
            domain: o.get_field("domain"),
            email: o.get_field("email"),
            entity: o.get_field("entity"),
            role: o.get_field("role"),
        }
    }
}
