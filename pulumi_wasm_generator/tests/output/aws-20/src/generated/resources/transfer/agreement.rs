/// Provides a AWS Transfer AS2 Agreement resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod agreement {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AgreementArgs {
        /// The IAM Role which provides read and write access to the parent directory of the file location mentioned in the StartFileTransfer request.
        #[builder(into)]
        pub access_role: pulumi_wasm_rust::InputOrOutput<String>,
        /// The landing directory for the files transferred by using the AS2 protocol.
        #[builder(into)]
        pub base_directory: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Optional description of the transdfer.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The unique identifier for the AS2 local profile.
        #[builder(into)]
        pub local_profile_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The unique identifier for the AS2 partner profile.
        #[builder(into)]
        pub partner_profile_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The unique server identifier for the server instance. This is the specific server the agreement uses.
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AgreementResult {
        /// The IAM Role which provides read and write access to the parent directory of the file location mentioned in the StartFileTransfer request.
        pub access_role: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the AS2 agreement.
        pub agreement_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the agreement.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The landing directory for the files transferred by using the AS2 protocol.
        pub base_directory: pulumi_wasm_rust::Output<String>,
        /// The Optional description of the transdfer.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The unique identifier for the AS2 local profile.
        pub local_profile_id: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the AS2 partner profile.
        pub partner_profile_id: pulumi_wasm_rust::Output<String>,
        /// The unique server identifier for the server instance. This is the specific server the agreement uses.
        pub server_id: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AgreementArgs,
    ) -> AgreementResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_role_binding = args.access_role.get_output(context).get_inner();
        let base_directory_binding = args.base_directory.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let local_profile_id_binding = args
            .local_profile_id
            .get_output(context)
            .get_inner();
        let partner_profile_id_binding = args
            .partner_profile_id
            .get_output(context)
            .get_inner();
        let server_id_binding = args.server_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:transfer/agreement:Agreement".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessRole".into(),
                    value: &access_role_binding,
                },
                register_interface::ObjectField {
                    name: "baseDirectory".into(),
                    value: &base_directory_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "localProfileId".into(),
                    value: &local_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "partnerProfileId".into(),
                    value: &partner_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessRole".into(),
                },
                register_interface::ResultField {
                    name: "agreementId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "baseDirectory".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "localProfileId".into(),
                },
                register_interface::ResultField {
                    name: "partnerProfileId".into(),
                },
                register_interface::ResultField {
                    name: "serverId".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AgreementResult {
            access_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessRole").unwrap(),
            ),
            agreement_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agreementId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            base_directory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseDirectory").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            local_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localProfileId").unwrap(),
            ),
            partner_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partnerProfileId").unwrap(),
            ),
            server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverId").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
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
