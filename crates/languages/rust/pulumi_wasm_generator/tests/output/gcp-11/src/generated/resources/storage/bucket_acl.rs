/// Authoritatively manages a bucket's ACLs in Google cloud storage service (GCS). For more information see
/// [the official documentation](https://cloud.google.com/storage/docs/access-control/lists)
/// and
/// [API](https://cloud.google.com/storage/docs/json_api/v1/bucketAccessControls).
///
/// Bucket ACLs can be managed non authoritatively using the `storage_bucket_access_control` resource. Do not use these two resources in conjunction to manage the same bucket.
///
/// Permissions can be granted either by ACLs or Cloud IAM policies. In general, permissions granted by Cloud IAM policies do not appear in ACLs, and permissions granted by ACLs do not appear in Cloud IAM policies. The only exception is for ACLs applied directly on a bucket and certain bucket-level Cloud IAM policies, as described in [Cloud IAM relation to ACLs](https://cloud.google.com/storage/docs/access-control/iam#acls).
///
/// **NOTE** This resource will not remove the `project-owners-<project_id>` entity from the `OWNER` role.
///
/// ## Example Usage
///
/// Example creating an ACL on a bucket with one owner, and one reader.
///
/// ```yaml
/// resources:
///   image-store:
///     type: gcp:storage:Bucket
///     properties:
///       name: image-store-bucket
///       location: EU
///   image-store-acl:
///     type: gcp:storage:BucketACL
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
pub mod bucket_acl {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BucketACLArgs {
        /// The name of the bucket it applies to.
        ///
        /// - - -
        #[builder(into)]
        pub bucket: pulumi_wasm_rust::InputOrOutput<String>,
        /// Configure this ACL to be the default ACL.
        #[builder(into, default)]
        pub default_acl: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The [canned GCS ACL](https://cloud.google.com/storage/docs/access-control/lists#predefined-acl) to apply. Must be set if `role_entity` is not.
        #[builder(into, default)]
        pub predefined_acl: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// List of role/entity pairs in the form `ROLE:entity`. See [GCS Bucket ACL documentation](https://cloud.google.com/storage/docs/json_api/v1/bucketAccessControls)  for more details. Must be set if `predefined_acl` is not.
        #[builder(into, default)]
        pub role_entities: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct BucketACLResult {
        /// The name of the bucket it applies to.
        ///
        /// - - -
        pub bucket: pulumi_wasm_rust::Output<String>,
        /// Configure this ACL to be the default ACL.
        pub default_acl: pulumi_wasm_rust::Output<Option<String>>,
        /// The [canned GCS ACL](https://cloud.google.com/storage/docs/access-control/lists#predefined-acl) to apply. Must be set if `role_entity` is not.
        pub predefined_acl: pulumi_wasm_rust::Output<Option<String>>,
        /// List of role/entity pairs in the form `ROLE:entity`. See [GCS Bucket ACL documentation](https://cloud.google.com/storage/docs/json_api/v1/bucketAccessControls)  for more details. Must be set if `predefined_acl` is not.
        pub role_entities: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BucketACLArgs,
    ) -> BucketACLResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bucket_binding = args.bucket.get_output(context).get_inner();
        let default_acl_binding = args.default_acl.get_output(context).get_inner();
        let predefined_acl_binding = args.predefined_acl.get_output(context).get_inner();
        let role_entities_binding = args.role_entities.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:storage/bucketACL:BucketACL".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "defaultAcl".into(),
                    value: &default_acl_binding,
                },
                register_interface::ObjectField {
                    name: "predefinedAcl".into(),
                    value: &predefined_acl_binding,
                },
                register_interface::ObjectField {
                    name: "roleEntities".into(),
                    value: &role_entities_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BucketACLResult {
            bucket: pulumi_wasm_rust::__private::into_domain(o.extract_field("bucket")),
            default_acl: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultAcl"),
            ),
            predefined_acl: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("predefinedAcl"),
            ),
            role_entities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleEntities"),
            ),
        }
    }
}
