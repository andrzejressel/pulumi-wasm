/// Provides a AWS Transfer AS2 Profile resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:transfer:Profile
///     properties:
///       as2Id: example
///       certificateIds:
///         - ${exampleAwsTransferCertificate.certificateId}
///       usage: LOCAL
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Transfer AS2 Profile using the `profile_id`. For example:
///
/// ```sh
/// $ pulumi import aws:transfer/profile:Profile example p-4221a88afd5f4362a
/// ```
pub mod profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProfileArgs {
        /// The As2Id is the AS2 name as defined in the RFC 4130. For inbound ttransfers this is the AS2 From Header for the AS2 messages sent from the partner. For Outbound messages this is the AS2 To Header for the AS2 messages sent to the partner. his ID cannot include spaces.
        #[builder(into)]
        pub as2_id: pulumi_wasm_rust::Output<String>,
        /// The list of certificate Ids from the imported certificate operation.
        #[builder(into, default)]
        pub certificate_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The profile type should be LOCAL or PARTNER.
        #[builder(into)]
        pub profile_type: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProfileResult {
        /// The ARN of the profile.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The As2Id is the AS2 name as defined in the RFC 4130. For inbound ttransfers this is the AS2 From Header for the AS2 messages sent from the partner. For Outbound messages this is the AS2 To Header for the AS2 messages sent to the partner. his ID cannot include spaces.
        pub as2_id: pulumi_wasm_rust::Output<String>,
        /// The list of certificate Ids from the imported certificate operation.
        pub certificate_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The unique identifier for the AS2 profile.
        pub profile_id: pulumi_wasm_rust::Output<String>,
        /// The profile type should be LOCAL or PARTNER.
        pub profile_type: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: ProfileArgs) -> ProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let as2_id_binding = args.as2_id.get_inner();
        let certificate_ids_binding = args.certificate_ids.get_inner();
        let profile_type_binding = args.profile_type.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:transfer/profile:Profile".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "as2Id".into(),
                    value: &as2_id_binding,
                },
                register_interface::ObjectField {
                    name: "certificateIds".into(),
                    value: &certificate_ids_binding,
                },
                register_interface::ObjectField {
                    name: "profileType".into(),
                    value: &profile_type_binding,
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
                    name: "as2Id".into(),
                },
                register_interface::ResultField {
                    name: "certificateIds".into(),
                },
                register_interface::ResultField {
                    name: "profileId".into(),
                },
                register_interface::ResultField {
                    name: "profileType".into(),
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
        ProfileResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            as2_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("as2Id").unwrap(),
            ),
            certificate_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateIds").unwrap(),
            ),
            profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profileId").unwrap(),
            ),
            profile_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profileType").unwrap(),
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
