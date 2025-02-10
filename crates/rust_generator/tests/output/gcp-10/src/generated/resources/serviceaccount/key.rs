/// ## Example Usage
///
/// ### Creating A New Key
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myaccount = account::create(
///         "myaccount",
///         AccountArgs::builder()
///             .account_id("myaccount")
///             .display_name("My Service Account")
///             .build_struct(),
///     );
///     let mykey = key::create(
///         "mykey",
///         KeyArgs::builder()
///             .public_key_type("TYPE_X509_PEM_FILE")
///             .service_account_id("${myaccount.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### Creating And Regularly Rotating A Key
///
/// ```yaml
/// resources:
///   myaccount:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: myaccount
///       displayName: My Service Account
///   # note this requires the terraform to be run regularly
///   mykeyRotation:
///     type: time:Rotating
///     name: mykey_rotation
///     properties:
///       rotationDays: 30
///   mykey:
///     type: gcp:serviceaccount:Key
///     properties:
///       serviceAccountId: ${myaccount.name}
///       keepers:
///         rotation_time: ${mykeyRotation.rotationRfc3339}
/// ```
///
///
/// ### Save Key In Kubernetes Secret - DEPRECATED
///
/// ```yaml
/// resources:
///   # Workload Identity is the recommended way of accessing Google Cloud APIs from pods.
///   # https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity
///   myaccount:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: myaccount
///       displayName: My Service Account
///   mykey:
///     type: gcp:serviceaccount:Key
///     properties:
///       serviceAccountId: ${myaccount.name}
///   google-application-credentials:
///     type: kubernetes:core/v1:Secret
///     properties:
///       metadata:
///         name: google-application-credentials
///       data:
///         credentials.json:
///           fn::invoke:
///             function: std:base64decode
///             arguments:
///               input: ${mykey.privateKey}
///             return: result
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyArgs {
        /// Arbitrary map of values that, when changed, will trigger a new key to be generated.
        #[builder(into, default)]
        pub keepers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The algorithm used to generate the key. KEY_ALG_RSA_2048 is the default algorithm.
        /// Valid values are listed at
        /// [ServiceAccountPrivateKeyType](https://cloud.google.com/iam/reference/rest/v1/projects.serviceAccounts.keys#ServiceAccountKeyAlgorithm)
        /// (only used on create)
        #[builder(into, default)]
        pub key_algorithm: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The output format of the private key. TYPE_GOOGLE_CREDENTIALS_FILE is the default output format.
        #[builder(into, default)]
        pub private_key_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Public key data to create a service account key for given service account. The expected format for this field is a base64 encoded X509_PEM and it conflicts with `public_key_type` and `private_key_type`.
        #[builder(into, default)]
        pub public_key_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The output format of the public key requested. TYPE_X509_PEM_FILE is the default output format.
        #[builder(into, default)]
        pub public_key_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Service account id of the Key. This can be a string in the format
        /// `{ACCOUNT}` or `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`. If the `{ACCOUNT}`-only syntax is used, either
        /// the **full** email address of the service account or its name can be specified as a value, in which case the project will
        /// automatically be inferred from the account. Otherwise, if the `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`
        /// syntax is used, the `{ACCOUNT}` specified can be the full email address of the service account or the service account's
        /// unique id. Substituting `-` as a wildcard for the `{PROJECT_ID}` will infer the project from the account.
        #[builder(into)]
        pub service_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct KeyResult {
        /// Arbitrary map of values that, when changed, will trigger a new key to be generated.
        pub keepers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The algorithm used to generate the key. KEY_ALG_RSA_2048 is the default algorithm.
        /// Valid values are listed at
        /// [ServiceAccountPrivateKeyType](https://cloud.google.com/iam/reference/rest/v1/projects.serviceAccounts.keys#ServiceAccountKeyAlgorithm)
        /// (only used on create)
        pub key_algorithm: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name used for this key pair
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The private key in JSON format, base64 encoded. This is what you normally get as a file when creating
        /// service account keys through the CLI or web console. This is only populated when creating a new key.
        pub private_key: pulumi_gestalt_rust::Output<String>,
        /// The output format of the private key. TYPE_GOOGLE_CREDENTIALS_FILE is the default output format.
        pub private_key_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The public key, base64 encoded
        pub public_key: pulumi_gestalt_rust::Output<String>,
        /// Public key data to create a service account key for given service account. The expected format for this field is a base64 encoded X509_PEM and it conflicts with `public_key_type` and `private_key_type`.
        pub public_key_data: pulumi_gestalt_rust::Output<Option<String>>,
        /// The output format of the public key requested. TYPE_X509_PEM_FILE is the default output format.
        pub public_key_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Service account id of the Key. This can be a string in the format
        /// `{ACCOUNT}` or `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`. If the `{ACCOUNT}`-only syntax is used, either
        /// the **full** email address of the service account or its name can be specified as a value, in which case the project will
        /// automatically be inferred from the account. Otherwise, if the `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`
        /// syntax is used, the `{ACCOUNT}` specified can be the full email address of the service account or the service account's
        /// unique id. Substituting `-` as a wildcard for the `{PROJECT_ID}` will infer the project from the account.
        pub service_account_id: pulumi_gestalt_rust::Output<String>,
        /// The key can be used after this timestamp. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
        pub valid_after: pulumi_gestalt_rust::Output<String>,
        /// The key can be used before this timestamp.
        /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
        pub valid_before: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeyArgs,
    ) -> KeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let keepers_binding = args.keepers.get_output(context);
        let key_algorithm_binding = args.key_algorithm.get_output(context);
        let private_key_type_binding = args.private_key_type.get_output(context);
        let public_key_data_binding = args.public_key_data.get_output(context);
        let public_key_type_binding = args.public_key_type.get_output(context);
        let service_account_id_binding = args.service_account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:serviceaccount/key:Key".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keepers".into(),
                    value: keepers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyAlgorithm".into(),
                    value: key_algorithm_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateKeyType".into(),
                    value: private_key_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicKeyData".into(),
                    value: public_key_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicKeyType".into(),
                    value: public_key_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccountId".into(),
                    value: service_account_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        KeyResult {
            keepers: o.get_field("keepers"),
            key_algorithm: o.get_field("keyAlgorithm"),
            name: o.get_field("name"),
            private_key: o.get_field("privateKey"),
            private_key_type: o.get_field("privateKeyType"),
            public_key: o.get_field("publicKey"),
            public_key_data: o.get_field("publicKeyData"),
            public_key_type: o.get_field("publicKeyType"),
            service_account_id: o.get_field("serviceAccountId"),
            valid_after: o.get_field("validAfter"),
            valid_before: o.get_field("validBefore"),
        }
    }
}
