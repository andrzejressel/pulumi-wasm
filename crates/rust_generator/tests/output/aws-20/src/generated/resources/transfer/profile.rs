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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProfileArgs {
        /// The As2Id is the AS2 name as defined in the RFC 4130. For inbound ttransfers this is the AS2 From Header for the AS2 messages sent from the partner. For Outbound messages this is the AS2 To Header for the AS2 messages sent to the partner. his ID cannot include spaces.
        #[builder(into)]
        pub as2_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The list of certificate Ids from the imported certificate operation.
        #[builder(into, default)]
        pub certificate_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The profile type should be LOCAL or PARTNER.
        #[builder(into)]
        pub profile_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProfileResult {
        /// The ARN of the profile.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The As2Id is the AS2 name as defined in the RFC 4130. For inbound ttransfers this is the AS2 From Header for the AS2 messages sent from the partner. For Outbound messages this is the AS2 To Header for the AS2 messages sent to the partner. his ID cannot include spaces.
        pub as2_id: pulumi_gestalt_rust::Output<String>,
        /// The list of certificate Ids from the imported certificate operation.
        pub certificate_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The unique identifier for the AS2 profile.
        pub profile_id: pulumi_gestalt_rust::Output<String>,
        /// The profile type should be LOCAL or PARTNER.
        pub profile_type: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProfileArgs,
    ) -> ProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let as2_id_binding = args.as2_id.get_output(context);
        let certificate_ids_binding = args.certificate_ids.get_output(context);
        let profile_type_binding = args.profile_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:transfer/profile:Profile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "as2Id".into(),
                    value: as2_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateIds".into(),
                    value: certificate_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "profileType".into(),
                    value: profile_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProfileResult {
            arn: o.get_field("arn"),
            as2_id: o.get_field("as2Id"),
            certificate_ids: o.get_field("certificateIds"),
            profile_id: o.get_field("profileId"),
            profile_type: o.get_field("profileType"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
