/// The ObjectAccessControls resources represent the Access Control Lists
/// (ACLs) for objects within Google Cloud Storage. ACLs let you specify
/// who has access to your data and to what extent.
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
/// To get more information about ObjectAccessControl, see:
///
/// * [API documentation](https://cloud.google.com/storage/docs/json_api/v1/objectAccessControls)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/storage/docs/access-control/create-manage-lists)
///
/// ## Example Usage
///
/// ### Storage Object Access Control Public Object
///
///
/// ```yaml
/// resources:
///   publicRule:
///     type: gcp:storage:ObjectAccessControl
///     name: public_rule
///     properties:
///       object: ${object.outputName}
///       bucket: ${bucket.name}
///       role: READER
///       entity: allUsers
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: static-content-bucket
///       location: US
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: public-object
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: ../static/img/header-logo.png
/// ```
///
/// ## Import
///
/// ObjectAccessControl can be imported using any of these accepted formats:
///
/// * `{{bucket}}/{{object}}/{{entity}}`
///
/// When using the `pulumi import` command, ObjectAccessControl can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:storage/objectAccessControl:ObjectAccessControl default {{bucket}}/{{object}}/{{entity}}
/// ```
///
pub mod object_access_control {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObjectAccessControlArgs {
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
        /// The name of the object to apply the access control to.
        #[builder(into)]
        pub object: pulumi_wasm_rust::InputOrOutput<String>,
        /// The access permission for the entity.
        /// Possible values are: `OWNER`, `READER`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub role: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ObjectAccessControlResult {
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
        /// The name of the object to apply the access control to.
        pub object: pulumi_wasm_rust::Output<String>,
        /// The project team associated with the entity
        /// Structure is documented below.
        pub project_teams: pulumi_wasm_rust::Output<
            Vec<super::super::types::storage::ObjectAccessControlProjectTeam>,
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
        args: ObjectAccessControlArgs,
    ) -> ObjectAccessControlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let entity_binding = args.entity.get_output(context).get_inner();
        let object_binding = args.object.get_output(context).get_inner();
        let role_binding = args.role.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:storage/objectAccessControl:ObjectAccessControl".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "bucket".into(),
                },
                register_interface::ResultField {
                    name: "domain".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "entity".into(),
                },
                register_interface::ResultField {
                    name: "entityId".into(),
                },
                register_interface::ResultField {
                    name: "generation".into(),
                },
                register_interface::ResultField {
                    name: "object".into(),
                },
                register_interface::ResultField {
                    name: "projectTeams".into(),
                },
                register_interface::ResultField {
                    name: "role".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ObjectAccessControlResult {
            bucket: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bucket").unwrap(),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domain").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            entity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entity").unwrap(),
            ),
            entity_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entityId").unwrap(),
            ),
            generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("generation").unwrap(),
            ),
            object: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("object").unwrap(),
            ),
            project_teams: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectTeams").unwrap(),
            ),
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
        }
    }
}
