/// Provides a Lightsail Key Pair, for use with Lightsail Instances. These key pairs
/// are separate from EC2 Key Pairs, and must be created or imported for use with
/// Lightsail.
///
/// > **Note:** Lightsail is currently only supported in a limited number of AWS Regions, please see ["Regions and Availability Zones in Amazon Lightsail"](https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail) for more details
///
/// ## Example Usage
///
/// ### Create New Key Pair
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let lgKeyPair = key_pair::create(
///         "lgKeyPair",
///         KeyPairArgs::builder().name("lg_key_pair").build_struct(),
///     );
/// }
/// ```
///
/// ### Create New Key Pair with PGP Encrypted Private Key
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let lgKeyPair = key_pair::create(
///         "lgKeyPair",
///         KeyPairArgs::builder()
///             .name("lg_key_pair")
///             .pgp_key("keybase:keybaseusername")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Existing Public Key Import
///
/// ```yaml
/// resources:
///   lgKeyPair:
///     type: aws:lightsail:KeyPair
///     name: lg_key_pair
///     properties:
///       name: importing
///       publicKey:
///         fn::invoke:
///           Function: std:file
///           Arguments:
///             input: ~/.ssh/id_rsa.pub
///           Return: result
/// ```
///
/// ## Import
///
/// You cannot import Lightsail Key Pairs because the private and public key are only available on initial creation.
///
pub mod key_pair {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyPairArgs {
        /// The name of the Lightsail Key Pair. If omitted, a unique name will be generated by this provider
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// An optional PGP key to encrypt the resulting private key material. Only used when creating a new key pair
        #[builder(into, default)]
        pub pgp_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The public key material. This public key will be imported into Lightsail
        #[builder(into, default)]
        pub public_key: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the collection. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// > **NOTE:** a PGP key is not required, however it is strongly encouraged. Without a PGP key, the private key material will be stored in state unencrypted.`pgp_key` is ignored if `public_key` is supplied.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct KeyPairResult {
        /// The ARN of the Lightsail key pair.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The MD5 public key fingerprint for the encrypted private key.
        pub encrypted_fingerprint: pulumi_wasm_rust::Output<String>,
        /// the private key material, base 64 encoded and encrypted with the given `pgp_key`. This is only populated when creating a new key and `pgp_key` is supplied.
        pub encrypted_private_key: pulumi_wasm_rust::Output<String>,
        /// The MD5 public key fingerprint as specified in section 4 of RFC 4716.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The name of the Lightsail Key Pair. If omitted, a unique name will be generated by this provider
        pub name: pulumi_wasm_rust::Output<String>,
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// An optional PGP key to encrypt the resulting private key material. Only used when creating a new key pair
        pub pgp_key: pulumi_wasm_rust::Output<Option<String>>,
        /// the private key, base64 encoded. This is only populated when creating a new key, and when no `pgp_key` is provided.
        pub private_key: pulumi_wasm_rust::Output<String>,
        /// The public key material. This public key will be imported into Lightsail
        pub public_key: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the collection. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// > **NOTE:** a PGP key is not required, however it is strongly encouraged. Without a PGP key, the private key material will be stored in state unencrypted.`pgp_key` is ignored if `public_key` is supplied.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: KeyPairArgs) -> KeyPairResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let pgp_key_binding = args.pgp_key.get_inner();
        let public_key_binding = args.public_key.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/keyPair:KeyPair".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "pgpKey".into(),
                    value: &pgp_key_binding,
                },
                register_interface::ObjectField {
                    name: "publicKey".into(),
                    value: &public_key_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "encryptedFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "encryptedPrivateKey".into(),
                },
                register_interface::ResultField {
                    name: "fingerprint".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "pgpKey".into(),
                },
                register_interface::ResultField {
                    name: "privateKey".into(),
                },
                register_interface::ResultField {
                    name: "publicKey".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KeyPairResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            encrypted_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptedFingerprint").unwrap(),
            ),
            encrypted_private_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptedPrivateKey").unwrap(),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            pgp_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pgpKey").unwrap(),
            ),
            private_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateKey").unwrap(),
            ),
            public_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKey").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}