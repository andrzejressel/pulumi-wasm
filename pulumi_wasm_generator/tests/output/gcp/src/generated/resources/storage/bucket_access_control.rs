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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod bucket_access_control {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketAccessControlArgs {
        /// The name of the bucket.
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::Output<String>,
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
        pub entity: pulumi_wasm_rust::Output<String>,
        /// The access permission for the entity.
        /// Possible values are: `OWNER`, `READER`, `WRITER`.
        #[builder(into, default)]
        pub role: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BucketAccessControlResult {
        /// The name of the bucket.
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// The domain associated with the entity.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// The email address associated with the entity.
        pub email: pulumi_wasm_rust::Output<String>,
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
        pub entity: pulumi_wasm_rust::Output<String>,
        /// The access permission for the entity.
        /// Possible values are: `OWNER`, `READER`, `WRITER`.
        pub role: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: BucketAccessControlArgs,
    ) -> BucketAccessControlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_inner();
        let entity_binding = args.entity.get_inner();
        let role_binding = args.role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:storage/bucketAccessControl:BucketAccessControl".into(),
            name: name.to_string(),
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
                    name: "role".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BucketAccessControlResult {
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
            role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("role").unwrap(),
            ),
        }
    }
}
