/// Provides a AWS Transfer AS2 Agreement resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = agreement::create(
///         "example",
///         AgreementArgs::builder()
///             .access_role("${test.arn}")
///             .base_directory("/DOC-EXAMPLE-BUCKET/home/mydirectory")
///             .description("example")
///             .local_profile_id("${local.profileId}")
///             .partner_profile_id("${partner.profileId}")
///             .server_id("${testAwsTransferServer.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Transfer AS2 Agreement using the `server_id/agreement_id`. For example:
///
/// ```sh
/// $ pulumi import aws:transfer/agreement:Agreement example s-4221a88afd5f4362a/a-4221a88afd5f4362a
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod agreement {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgreementArgs {
        /// The IAM Role which provides read and write access to the parent directory of the file location mentioned in the StartFileTransfer request.
        #[builder(into)]
        pub access_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The landing directory for the files transferred by using the AS2 protocol.
        #[builder(into)]
        pub base_directory: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Optional description of the transdfer.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The unique identifier for the AS2 local profile.
        #[builder(into)]
        pub local_profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The unique identifier for the AS2 partner profile.
        #[builder(into)]
        pub partner_profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The unique server identifier for the server instance. This is the specific server the agreement uses.
        #[builder(into)]
        pub server_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AgreementResult {
        /// The IAM Role which provides read and write access to the parent directory of the file location mentioned in the StartFileTransfer request.
        pub access_role: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the AS2 agreement.
        pub agreement_id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the agreement.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The landing directory for the files transferred by using the AS2 protocol.
        pub base_directory: pulumi_gestalt_rust::Output<String>,
        /// The Optional description of the transdfer.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The unique identifier for the AS2 local profile.
        pub local_profile_id: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the AS2 partner profile.
        pub partner_profile_id: pulumi_gestalt_rust::Output<String>,
        /// The unique server identifier for the server instance. This is the specific server the agreement uses.
        pub server_id: pulumi_gestalt_rust::Output<String>,
        pub status: pulumi_gestalt_rust::Output<String>,
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
        args: AgreementArgs,
    ) -> AgreementResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_role_binding = args.access_role.get_output(context);
        let base_directory_binding = args.base_directory.get_output(context);
        let description_binding = args.description.get_output(context);
        let local_profile_id_binding = args.local_profile_id.get_output(context);
        let partner_profile_id_binding = args.partner_profile_id.get_output(context);
        let server_id_binding = args.server_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:transfer/agreement:Agreement".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessRole".into(),
                    value: access_role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baseDirectory".into(),
                    value: base_directory_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localProfileId".into(),
                    value: local_profile_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partnerProfileId".into(),
                    value: partner_profile_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverId".into(),
                    value: server_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AgreementResult {
            access_role: o.get_field("accessRole"),
            agreement_id: o.get_field("agreementId"),
            arn: o.get_field("arn"),
            base_directory: o.get_field("baseDirectory"),
            description: o.get_field("description"),
            local_profile_id: o.get_field("localProfileId"),
            partner_profile_id: o.get_field("partnerProfileId"),
            server_id: o.get_field("serverId"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
