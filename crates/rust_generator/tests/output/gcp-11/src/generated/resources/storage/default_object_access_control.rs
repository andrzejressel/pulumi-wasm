/// The DefaultObjectAccessControls resources represent the Access Control
/// Lists (ACLs) applied to a new object within a Google Cloud Storage bucket
/// when no ACL was provided for that object. ACLs let you specify who has
/// access to your bucket contents and to what extent.
///
/// There are two roles that can be assigned to an entity:
///
/// READERs can get an object, though the acl property will not be revealed.
/// OWNERs are READERs, and they can get the acl property, update an object,
/// and call all objectAccessControls methods on the object. The owner of an
/// object is always an OWNER.
/// For more information, see Access Control, with the caveat that this API
/// uses READER and OWNER instead of READ and FULL_CONTROL.
///
///
/// To get more information about DefaultObjectAccessControl, see:
///
/// * [API documentation](https://cloud.google.com/storage/docs/json_api/v1/defaultObjectAccessControls)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/storage/docs/access-control/create-manage-lists)
///
/// ## Example Usage
///
/// ### Storage Default Object Access Control Public
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
///     let publicRule = default_object_access_control::create(
///         "publicRule",
///         DefaultObjectAccessControlArgs::builder()
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
/// DefaultObjectAccessControl can be imported using any of these accepted formats:
///
/// * `{{bucket}}/{{entity}}`
///
/// When using the `pulumi import` command, DefaultObjectAccessControl can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:storage/defaultObjectAccessControl:DefaultObjectAccessControl default {{bucket}}/{{entity}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod default_object_access_control {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultObjectAccessControlArgs {
        /// The name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The entity holding the permission, in one of the following forms:
        /// * user-{{userId}}
        /// * user-{{email}} (such as "user-liz@example.com")
        /// * group-{{groupId}}
        /// * group-{{email}} (such as "group-example@googlegroups.com")
        /// * domain-{{domain}} (such as "domain-example.com")
        /// * project-team-{{projectId}}
        /// * allUsers
        /// * allAuthenticatedUsers
        #[builder(into)]
        pub entity: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the object, if applied to an object.
        #[builder(into, default)]
        pub object: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The access permission for the entity.
        /// Possible values are: `OWNER`, `READER`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DefaultObjectAccessControlResult {
        /// The name of the bucket.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// The domain associated with the entity.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// The email address associated with the entity.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The entity holding the permission, in one of the following forms:
        /// * user-{{userId}}
        /// * user-{{email}} (such as "user-liz@example.com")
        /// * group-{{groupId}}
        /// * group-{{email}} (such as "group-example@googlegroups.com")
        /// * domain-{{domain}} (such as "domain-example.com")
        /// * project-team-{{projectId}}
        /// * allUsers
        /// * allAuthenticatedUsers
        pub entity: pulumi_gestalt_rust::Output<String>,
        /// The ID for the entity
        pub entity_id: pulumi_gestalt_rust::Output<String>,
        /// The content generation of the object, if applied to an object.
        pub generation: pulumi_gestalt_rust::Output<i32>,
        /// The name of the object, if applied to an object.
        pub object: pulumi_gestalt_rust::Output<Option<String>>,
        /// The project team associated with the entity
        /// Structure is documented below.
        pub project_teams: pulumi_gestalt_rust::Output<
            Vec<super::super::types::storage::DefaultObjectAccessControlProjectTeam>,
        >,
        /// The access permission for the entity.
        /// Possible values are: `OWNER`, `READER`.
        ///
        ///
        /// - - -
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultObjectAccessControlArgs,
    ) -> DefaultObjectAccessControlResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_binding = args.bucket.get_output(context);
        let entity_binding = args.entity.get_output(context);
        let object_binding = args.object.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:storage/defaultObjectAccessControl:DefaultObjectAccessControl"
                .into(),
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
                    name: "object".into(),
                    value: object_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DefaultObjectAccessControlResult {
            bucket: o.get_field("bucket"),
            domain: o.get_field("domain"),
            email: o.get_field("email"),
            entity: o.get_field("entity"),
            entity_id: o.get_field("entityId"),
            generation: o.get_field("generation"),
            object: o.get_field("object"),
            project_teams: o.get_field("projectTeams"),
            role: o.get_field("role"),
        }
    }
}
