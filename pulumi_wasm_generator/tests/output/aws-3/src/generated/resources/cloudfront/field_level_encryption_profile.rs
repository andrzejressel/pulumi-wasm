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
pub mod field_level_encryption_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FieldLevelEncryptionProfileArgs {
        /// An optional comment about the Field Level Encryption Profile.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The encryption entities config block for field-level encryption profiles that contains an attribute `items` which includes the encryption key and field pattern specifications.
        #[builder(into)]
        pub encryption_entities: pulumi_wasm_rust::InputOrOutput<
            super::super::types::cloudfront::FieldLevelEncryptionProfileEncryptionEntities,
        >,
        /// The name of the Field Level Encryption Profile.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FieldLevelEncryptionProfileResult {
        /// Internal value used by CloudFront to allow future updates to the Field Level Encryption Profile.
        pub caller_reference: pulumi_wasm_rust::Output<String>,
        /// An optional comment about the Field Level Encryption Profile.
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// The encryption entities config block for field-level encryption profiles that contains an attribute `items` which includes the encryption key and field pattern specifications.
        pub encryption_entities: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::FieldLevelEncryptionProfileEncryptionEntities,
        >,
        /// The current version of the Field Level Encryption Profile. For example: `E2QWRUHAPOMQZL`.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The name of the Field Level Encryption Profile.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FieldLevelEncryptionProfileArgs,
    ) -> FieldLevelEncryptionProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let comment_binding = args.comment.get_output(context).get_inner();
        let encryption_entities_binding = args
            .encryption_entities
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/fieldLevelEncryptionProfile:FieldLevelEncryptionProfile"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionEntities".into(),
                    value: &encryption_entities_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FieldLevelEncryptionProfileResult {
            caller_reference: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("callerReference"),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("comment"),
            ),
            encryption_entities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptionEntities"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
