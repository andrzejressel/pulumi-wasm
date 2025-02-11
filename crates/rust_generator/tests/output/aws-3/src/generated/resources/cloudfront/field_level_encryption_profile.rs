/// Provides a CloudFront Field-level Encryption Profile resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudfront:PublicKey
///     properties:
///       comment: test public key
///       encodedKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: public_key.pem
///           return: result
///       name: test_key
///   test:
///     type: aws:cloudfront:FieldLevelEncryptionProfile
///     properties:
///       comment: test comment
///       name: test profile
///       encryptionEntities:
///         items:
///           - publicKeyId: ${example.id}
///             providerId: test provider
///             fieldPatterns:
///               items:
///                 - DateOfBirth
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cloudfront Field Level Encryption Profile using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/fieldLevelEncryptionProfile:FieldLevelEncryptionProfile profile K3D5EWEUDCCXON
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod field_level_encryption_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FieldLevelEncryptionProfileArgs {
        /// An optional comment about the Field Level Encryption Profile.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The encryption entities config block for field-level encryption profiles that contains an attribute `items` which includes the encryption key and field pattern specifications.
        #[builder(into)]
        pub encryption_entities: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudfront::FieldLevelEncryptionProfileEncryptionEntities,
        >,
        /// The name of the Field Level Encryption Profile.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FieldLevelEncryptionProfileResult {
        /// Internal value used by CloudFront to allow future updates to the Field Level Encryption Profile.
        pub caller_reference: pulumi_gestalt_rust::Output<String>,
        /// An optional comment about the Field Level Encryption Profile.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// The encryption entities config block for field-level encryption profiles that contains an attribute `items` which includes the encryption key and field pattern specifications.
        pub encryption_entities: pulumi_gestalt_rust::Output<
            super::super::types::cloudfront::FieldLevelEncryptionProfileEncryptionEntities,
        >,
        /// The current version of the Field Level Encryption Profile. For example: `E2QWRUHAPOMQZL`.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The name of the Field Level Encryption Profile.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FieldLevelEncryptionProfileArgs,
    ) -> FieldLevelEncryptionProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let comment_binding = args.comment.get_output(context);
        let encryption_entities_binding = args.encryption_entities.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudfront/fieldLevelEncryptionProfile:FieldLevelEncryptionProfile"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: &comment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionEntities".into(),
                    value: &encryption_entities_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FieldLevelEncryptionProfileResult {
            caller_reference: o.get_field("callerReference"),
            comment: o.get_field("comment"),
            encryption_entities: o.get_field("encryptionEntities"),
            etag: o.get_field("etag"),
            name: o.get_field("name"),
        }
    }
}
