/// EdgeCacheKeyset represents a collection of public keys used for validating signed requests.
///
///
/// To get more information about EdgeCacheKeyset, see:
///
/// * [API documentation](https://cloud.google.com/media-cdn/docs/reference/rest/v1/projects.locations.edgeCacheKeysets)
/// * How-to Guides
///     * [Create keysets](https://cloud.google.com/media-cdn/docs/create-keyset)
///
///
///
/// ## Example Usage
///
/// ### Network Services Edge Cache Keyset Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = edge_cache_keyset::create(
///         "default",
///         EdgeCacheKeysetArgs::builder()
///             .description("The default keyset")
///             .name("my-keyset")
///             .public_keys(
///                 vec![
///                     EdgeCacheKeysetPublicKey::builder().id("my-public-key")
///                     .value("FHsTyFHNmvNpw4o7-rp-M1yqMyBF8vXSBRkZtkQ0RKY").build_struct(),
///                     EdgeCacheKeysetPublicKey::builder().id("my-public-key-2")
///                     .value("hzd03llxB1u5FOLKFkZ6_wCJqC7jtN0bg7xlBqS6WVM").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Services Edge Cache Keyset Dual Token
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret-name
///       replication:
///         auto: {}
///   secret-version-basic:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: secret-data
///   default:
///     type: gcp:networkservices:EdgeCacheKeyset
///     properties:
///       name: my-keyset
///       description: The default keyset
///       publicKeys:
///         - id: my-public-key
///           managed: true
///       validationSharedKeys:
///         - secretVersion: ${["secret-version-basic"].id}
/// ```
///
/// ## Import
///
/// EdgeCacheKeyset can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/edgeCacheKeysets/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, EdgeCacheKeyset can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/edgeCacheKeyset:EdgeCacheKeyset default projects/{{project}}/locations/global/edgeCacheKeysets/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/edgeCacheKeyset:EdgeCacheKeyset default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/edgeCacheKeyset:EdgeCacheKeyset default {{name}}
/// ```
///
pub mod edge_cache_keyset {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EdgeCacheKeysetArgs {
        /// A human-readable description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of label tags associated with the EdgeCache resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// An ordered list of Ed25519 public keys to use for validating signed requests.
        /// You must specify `public_keys` or `validation_shared_keys` (or both). The keys in `public_keys` are checked first.
        /// You may specify no more than one Google-managed public key.
        /// If you specify `public_keys`, you must specify at least one (1) key and may specify up to three (3) keys.
        /// Ed25519 public keys are not secret, and only allow Google to validate a request was signed by your corresponding private key.
        /// Ensure that the private key is kept secret, and that only authorized users can add public keys to a keyset.
        /// Structure is documented below.
        #[builder(into, default)]
        pub public_keys: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::networkservices::EdgeCacheKeysetPublicKey>>,
        >,
        /// An ordered list of shared keys to use for validating signed requests.
        /// Shared keys are secret.  Ensure that only authorized users can add `validation_shared_keys` to a keyset.
        /// You can rotate keys by appending (pushing) a new key to the list of `validation_shared_keys` and removing any superseded keys.
        /// You must specify `public_keys` or `validation_shared_keys` (or both). The keys in `public_keys` are checked first.
        /// Structure is documented below.
        #[builder(into, default)]
        pub validation_shared_keys: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::networkservices::EdgeCacheKeysetValidationSharedKey,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct EdgeCacheKeysetResult {
        /// A human-readable description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of label tags associated with the EdgeCache resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// An ordered list of Ed25519 public keys to use for validating signed requests.
        /// You must specify `public_keys` or `validation_shared_keys` (or both). The keys in `public_keys` are checked first.
        /// You may specify no more than one Google-managed public key.
        /// If you specify `public_keys`, you must specify at least one (1) key and may specify up to three (3) keys.
        /// Ed25519 public keys are not secret, and only allow Google to validate a request was signed by your corresponding private key.
        /// Ensure that the private key is kept secret, and that only authorized users can add public keys to a keyset.
        /// Structure is documented below.
        pub public_keys: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::networkservices::EdgeCacheKeysetPublicKey>>,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// An ordered list of shared keys to use for validating signed requests.
        /// Shared keys are secret.  Ensure that only authorized users can add `validation_shared_keys` to a keyset.
        /// You can rotate keys by appending (pushing) a new key to the list of `validation_shared_keys` and removing any superseded keys.
        /// You must specify `public_keys` or `validation_shared_keys` (or both). The keys in `public_keys` are checked first.
        /// Structure is documented below.
        pub validation_shared_keys: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::networkservices::EdgeCacheKeysetValidationSharedKey,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EdgeCacheKeysetArgs) -> EdgeCacheKeysetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let public_keys_binding = args.public_keys.get_inner();
        let validation_shared_keys_binding = args.validation_shared_keys.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkservices/edgeCacheKeyset:EdgeCacheKeyset".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "publicKeys".into(),
                    value: &public_keys_binding,
                },
                register_interface::ObjectField {
                    name: "validationSharedKeys".into(),
                    value: &validation_shared_keys_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "publicKeys".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "validationSharedKeys".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EdgeCacheKeysetResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            public_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKeys").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            validation_shared_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validationSharedKeys").unwrap(),
            ),
        }
    }
}
