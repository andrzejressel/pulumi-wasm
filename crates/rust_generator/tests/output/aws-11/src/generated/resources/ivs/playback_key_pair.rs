/// Resource for managing an AWS IVS (Interactive Video) Playback Key Pair.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ivs:PlaybackKeyPair
///     properties:
///       publicKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: ./public-key.pem
///           return: result
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IVS (Interactive Video) Playback Key Pair using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:ivs/playbackKeyPair:PlaybackKeyPair example arn:aws:ivs:us-west-2:326937407773:playback-key/KDJRJNQhiQzA
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod playback_key_pair {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PlaybackKeyPairArgs {
        /// Playback Key Pair name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Public portion of a customer-generated key pair. Must be an ECDSA public key in PEM format.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub public_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PlaybackKeyPairResult {
        /// ARN of the Playback Key Pair.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Key-pair identifier.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Playback Key Pair name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Public portion of a customer-generated key pair. Must be an ECDSA public key in PEM format.
        ///
        /// The following arguments are optional:
        pub public_key: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: PlaybackKeyPairArgs,
    ) -> PlaybackKeyPairResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let public_key_binding_1 = args.public_key.get_output(context);
        let public_key_binding = public_key_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ivs/playbackKeyPair:PlaybackKeyPair".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
        PlaybackKeyPairResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
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
