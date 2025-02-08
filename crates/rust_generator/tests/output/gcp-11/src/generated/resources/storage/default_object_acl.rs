/// Authoritatively manages the default object ACLs for a Google Cloud Storage bucket
/// without managing the bucket itself.
///
/// > Note that for each object, its creator will have the `"OWNER"` role in addition
/// to the default ACL that has been defined.
///
/// For more information see
/// [the official documentation](https://cloud.google.com/storage/docs/access-control/lists)
/// and
/// [API](https://cloud.google.com/storage/docs/json_api/v1/defaultObjectAccessControls).
///
/// > Want fine-grained control over default object ACLs? Use `gcp.storage.DefaultObjectAccessControl`
/// to control individual role entity pairs.
///
/// ## Example Usage
///
/// Example creating a default object ACL on a bucket with one owner, and one reader.
///
/// ```yaml
/// resources:
///   image-store:
///     type: gcp:storage:Bucket
///     properties:
///       name: image-store-bucket
///       location: EU
///   image-store-default-acl:
///     type: gcp:storage:DefaultObjectACL
///     properties:
///       bucket: ${["image-store"].name}
///       roleEntities:
///         - OWNER:user-my.email@gmail.com
///         - READER:group-mygroup
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
#[allow(clippy::doc_lazy_continuation)]
pub mod default_object_acl {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultObjectACLArgs {
        /// The name of the bucket it applies to.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of role/entity pairs in the form `ROLE:entity`.
        /// See [GCS Object ACL documentation](https://cloud.google.com/storage/docs/json_api/v1/objectAccessControls) for more details.
        /// Omitting the field is the same as providing an empty list.
        #[builder(into, default)]
        pub role_entities: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct DefaultObjectACLResult {
        /// The name of the bucket it applies to.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// List of role/entity pairs in the form `ROLE:entity`.
        /// See [GCS Object ACL documentation](https://cloud.google.com/storage/docs/json_api/v1/objectAccessControls) for more details.
        /// Omitting the field is the same as providing an empty list.
        pub role_entities: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DefaultObjectACLArgs,
    ) -> DefaultObjectACLResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let role_entities_binding = args.role_entities.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:storage/defaultObjectACL:DefaultObjectACL".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "roleEntities".into(),
                    value: &role_entities_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DefaultObjectACLResult {
            bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucket"),
            ),
            role_entities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleEntities"),
            ),
        }
    }
}
