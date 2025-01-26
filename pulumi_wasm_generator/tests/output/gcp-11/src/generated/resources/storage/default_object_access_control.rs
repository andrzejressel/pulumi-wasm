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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod default_object_access_control {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultObjectAccessControlArgs {
        /// The name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::InputOrOutput<String>,
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
        pub entity: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the object, if applied to an object.
        #[builder(into, default)]
        pub object: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The access permission for the entity.
        /// Possible values are: `OWNER`, `READER`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub role: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DefaultObjectAccessControlResult {
        /// The name of the bucket.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// The domain associated with the entity.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// The email address associated with the entity.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The entity holding the permission, in one of the following forms:
        /// * user-{{userId}}
        /// * user-{{email}} (such as "user-liz@example.com")
        /// * group-{{groupId}}
        /// * group-{{email}} (such as "group-example@googlegroups.com")
        /// * domain-{{domain}} (such as "domain-example.com")
        /// * project-team-{{projectId}}
        /// * allUsers
        /// * allAuthenticatedUsers
        pub entity: pulumi_wasm_rust::Output<String>,
        /// The ID for the entity
        pub entity_id: pulumi_wasm_rust::Output<String>,
        /// The content generation of the object, if applied to an object.
        pub generation: pulumi_wasm_rust::Output<i32>,
        /// The name of the object, if applied to an object.
        pub object: pulumi_wasm_rust::Output<Option<String>>,
        /// The project team associated with the entity
        /// Structure is documented below.
        pub project_teams: pulumi_wasm_rust::Output<
            Vec<super::super::types::storage::DefaultObjectAccessControlProjectTeam>,
        >,
        /// The access permission for the entity.
        /// Possible values are: `OWNER`, `READER`.
        ///
        ///
        /// - - -
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DefaultObjectAccessControlArgs,
    ) -> DefaultObjectAccessControlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let entity_binding = args.entity.get_output(context).get_inner();
        let object_binding = args.object.get_output(context).get_inner();
        let role_binding = args.role.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:storage/defaultObjectAccessControl:DefaultObjectAccessControl"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "entity".into(),
                    value: &entity_binding,
                },
                register_interface::ObjectField {
                    name: "object".into(),
                    value: &object_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DefaultObjectAccessControlResult {
            bucket: pulumi_wasm_rust::__private::into_domain(o.extract_field("bucket")),
            domain: pulumi_wasm_rust::__private::into_domain(o.extract_field("domain")),
            email: pulumi_wasm_rust::__private::into_domain(o.extract_field("email")),
            entity: pulumi_wasm_rust::__private::into_domain(o.extract_field("entity")),
            entity_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("entityId"),
            ),
            generation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("generation"),
            ),
            object: pulumi_wasm_rust::__private::into_domain(o.extract_field("object")),
            project_teams: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("projectTeams"),
            ),
            role: pulumi_wasm_rust::__private::into_domain(o.extract_field("role")),
        }
    }
}
