/// Provides a CloudFront Field-level Encryption Config resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = field_level_encryption_config::create(
///         "test",
///         FieldLevelEncryptionConfigArgs::builder()
///             .comment("test comment")
///             .content_type_profile_config(
///                 FieldLevelEncryptionConfigContentTypeProfileConfig::builder()
///                     .contentTypeProfiles(
///                         FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfiles::builder()
///                             .items(
///                                 vec![
///                                     FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesItem::builder()
///                                     .contentType("application/x-www-form-urlencoded")
///                                     .format("URLEncoded").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .forwardWhenContentTypeIsUnknown(true)
///                     .build_struct(),
///             )
///             .query_arg_profile_config(
///                 FieldLevelEncryptionConfigQueryArgProfileConfig::builder()
///                     .forwardWhenQueryArgProfileIsUnknown(true)
///                     .queryArgProfiles(
///                         FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfiles::builder()
///                             .items(
///                                 vec![
///                                     FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesItem::builder()
///                                     .profileId("${testAwsCloudfrontFieldLevelEncryptionProfile.id}")
///                                     .queryArg("Arg1").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cloudfront Field Level Encryption Config using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/fieldLevelEncryptionConfig:FieldLevelEncryptionConfig config E74FTE3AEXAMPLE
/// ```
pub mod field_level_encryption_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FieldLevelEncryptionConfigArgs {
        /// An optional comment about the Field Level Encryption Config.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// Content Type Profile Config specifies when to forward content if a content type isn't recognized and profiles to use as by default in a request if a query argument doesn't specify a profile to use.
        #[builder(into)]
        pub content_type_profile_config: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::FieldLevelEncryptionConfigContentTypeProfileConfig,
        >,
        /// Query Arg Profile Config that specifies when to forward content if a profile isn't found and the profile that can be provided as a query argument in a request.
        #[builder(into)]
        pub query_arg_profile_config: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::FieldLevelEncryptionConfigQueryArgProfileConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct FieldLevelEncryptionConfigResult {
        /// Internal value used by CloudFront to allow future updates to the Field Level Encryption Config.
        pub caller_reference: pulumi_wasm_rust::Output<String>,
        /// An optional comment about the Field Level Encryption Config.
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// Content Type Profile Config specifies when to forward content if a content type isn't recognized and profiles to use as by default in a request if a query argument doesn't specify a profile to use.
        pub content_type_profile_config: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::FieldLevelEncryptionConfigContentTypeProfileConfig,
        >,
        /// The current version of the Field Level Encryption Config. For example: `E2QWRUHAPOMQZL`.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Query Arg Profile Config that specifies when to forward content if a profile isn't found and the profile that can be provided as a query argument in a request.
        pub query_arg_profile_config: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::FieldLevelEncryptionConfigQueryArgProfileConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: FieldLevelEncryptionConfigArgs,
    ) -> FieldLevelEncryptionConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let comment_binding = args.comment.get_inner();
        let content_type_profile_config_binding = args
            .content_type_profile_config
            .get_inner();
        let query_arg_profile_config_binding = args.query_arg_profile_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/fieldLevelEncryptionConfig:FieldLevelEncryptionConfig"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "contentTypeProfileConfig".into(),
                    value: &content_type_profile_config_binding,
                },
                register_interface::ObjectField {
                    name: "queryArgProfileConfig".into(),
                    value: &query_arg_profile_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "callerReference".into(),
                },
                register_interface::ResultField {
                    name: "comment".into(),
                },
                register_interface::ResultField {
                    name: "contentTypeProfileConfig".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "queryArgProfileConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FieldLevelEncryptionConfigResult {
            caller_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("callerReference").unwrap(),
            ),
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            content_type_profile_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contentTypeProfileConfig").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            query_arg_profile_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryArgProfileConfig").unwrap(),
            ),
        }
    }
}