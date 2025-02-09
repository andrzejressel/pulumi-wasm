/// Provides an [EC2 key pair](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html) resource. A key pair is used to control login access to EC2 instances.
///
/// Currently this resource requires an existing user-supplied key pair. This key pair's public key will be registered with AWS to allow logging-in to EC2 instances.
///
/// When importing an existing key pair the public key material may be in any format supported by AWS. Supported formats (per the [AWS documentation](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws)) are:
///
/// * OpenSSH public key format (the format in ~/.ssh/authorized_keys)
/// * Base64 encoded DER format
/// * SSH public key file format as specified in RFC4716
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let deployer = key_pair::create(
///         "deployer",
///         KeyPairArgs::builder()
///             .key_name("deployer-key")
///             .public_key(
///                 "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQD3F6tyPEFEzV0LX3X8BsXdMsQz1x2cEikKDEY0aIj41qgxMCP/iteneqXSIFZBp5vizPvaoIR3Um9xK7PGoW8giupGn+EPuxIA4cDM4vzOqOkiMPhz5XK0whEjkVzTo4+S0puvDZuwIsdiW9mxhJc7tgBNL0cYlWSYVkz4G/fslNfRPW5mYAM49f4fhtxPb5ok4Q2Lg9dPKVHO/Bgeu5woMc7RY0p1ej6D4CKFE6lymSDJpW0YHX/wqE9+cfEauh7xZcG0q9t2ta6F6fmX0agvpFyZo8aFbXeUBr7osSCJNgvavWbM/06niWrOvYX2xwWdhXmXSrbX8ZbabVohBK41 email@example.com",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Key Pairs using the `key_name`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/keyPair:KeyPair deployer deployer-key
/// ```
/// ~> __NOTE:__ The AWS API does not include the public key in the response, so `pulumi up` will attempt to replace the key pair. There is currently no supported workaround for this limitation.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod key_pair {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeyPairArgs {
        /// The name for the key pair. If neither `key_name` nor `key_name_prefix` is provided, the provider will create a unique key name.
        #[builder(into, default)]
        pub key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `key_name`. If neither `key_name` nor `key_name_prefix` is provided, the provider will create a unique key name.
        #[builder(into, default)]
        pub key_name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The public key material.
        #[builder(into)]
        pub public_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct KeyPairResult {
        /// The key pair ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The MD5 public key fingerprint as specified in section 4 of RFC 4716.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The name for the key pair. If neither `key_name` nor `key_name_prefix` is provided, the provider will create a unique key name.
        pub key_name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `key_name`. If neither `key_name` nor `key_name_prefix` is provided, the provider will create a unique key name.
        pub key_name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The key pair ID.
        pub key_pair_id: pulumi_gestalt_rust::Output<String>,
        /// The type of key pair.
        pub key_type: pulumi_gestalt_rust::Output<String>,
        /// The public key material.
        pub public_key: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: KeyPairArgs,
    ) -> KeyPairResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let key_name_binding_1 = args.key_name.get_output(context);
        let key_name_binding = key_name_binding_1.get_inner();
        let key_name_prefix_binding_1 = args.key_name_prefix.get_output(context);
        let key_name_prefix_binding = key_name_prefix_binding_1.get_inner();
        let public_key_binding_1 = args.public_key.get_output(context);
        let public_key_binding = public_key_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/keyPair:KeyPair".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keyName".into(),
                    value: &key_name_binding,
                },
                register_interface::ObjectField {
                    name: "keyNamePrefix".into(),
                    value: &key_name_prefix_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        KeyPairResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            key_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyName"),
            ),
            key_name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyNamePrefix"),
            ),
            key_pair_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyPairId"),
            ),
            key_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyType"),
            ),
            public_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicKey"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
