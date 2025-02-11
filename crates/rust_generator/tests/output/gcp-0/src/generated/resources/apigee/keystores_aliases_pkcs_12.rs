/// An alias from a pkcs12 file.
///
/// To get more information about KeystoresAliasesPkcs12, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.keystores.aliases)
/// * How-to Guides
///     * [Keystores Aliases](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.keystores.aliases)
///
/// ## Import
///
/// KeystoresAliasesPkcs12 can be imported using any of these accepted formats:
///
/// * `organizations/{{org_id}}/environments/{{environment}}/keystores/{{keystore}}/aliases/{{alias}}`
///
/// * `{{org_id}}/{{environment}}/{{keystore}}/{{alias}}`
///
/// When using the `pulumi import` command, KeystoresAliasesPkcs12 can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/keystoresAliasesPkcs12:KeystoresAliasesPkcs12 default organizations/{{org_id}}/environments/{{environment}}/keystores/{{keystore}}/aliases/{{alias}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/keystoresAliasesPkcs12:KeystoresAliasesPkcs12 default {{org_id}}/{{environment}}/{{keystore}}/{{alias}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod keystores_aliases_pkcs_12 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeystoresAliasesPkcs12Args {
        /// Alias Name
        #[builder(into)]
        pub alias: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Environment associated with the alias
        #[builder(into)]
        pub environment: pulumi_gestalt_rust::InputOrOutput<String>,
        /// PKCS12 file content
        ///
        /// - - -
        #[builder(into)]
        pub file: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Hash of the pkcs file
        #[builder(into)]
        pub filehash: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Keystore Name
        #[builder(into)]
        pub keystore: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Organization ID associated with the alias, without organization/ prefix
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Password for the PKCS12 file if it's encrypted
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeystoresAliasesPkcs12Result {
        /// Alias Name
        pub alias: pulumi_gestalt_rust::Output<String>,
        /// Chain of certificates under this alias.
        /// Structure is documented below.
        pub certs_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::types::apigee::KeystoresAliasesPkcs12CertsInfo>,
        >,
        /// Environment associated with the alias
        pub environment: pulumi_gestalt_rust::Output<String>,
        /// PKCS12 file content
        ///
        /// - - -
        pub file: pulumi_gestalt_rust::Output<String>,
        /// Hash of the pkcs file
        pub filehash: pulumi_gestalt_rust::Output<String>,
        /// Keystore Name
        pub keystore: pulumi_gestalt_rust::Output<String>,
        /// Organization ID associated with the alias, without organization/ prefix
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// Password for the PKCS12 file if it's encrypted
        pub password: pulumi_gestalt_rust::Output<String>,
        /// Optional.Type of Alias
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeystoresAliasesPkcs12Args,
    ) -> KeystoresAliasesPkcs12Result {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alias_binding = args.alias.get_output(context);
        let environment_binding = args.environment.get_output(context);
        let file_binding = args.file.get_output(context);
        let filehash_binding = args.filehash.get_output(context);
        let keystore_binding = args.keystore.get_output(context);
        let org_id_binding = args.org_id.get_output(context);
        let password_binding = args.password.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/keystoresAliasesPkcs12:KeystoresAliasesPkcs12".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alias".into(),
                    value: &alias_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environment".into(),
                    value: &environment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "file".into(),
                    value: &file_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filehash".into(),
                    value: &filehash_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keystore".into(),
                    value: &keystore_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: &password_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        KeystoresAliasesPkcs12Result {
            alias: o.get_field("alias"),
            certs_infos: o.get_field("certsInfos"),
            environment: o.get_field("environment"),
            file: o.get_field("file"),
            filehash: o.get_field("filehash"),
            keystore: o.get_field("keystore"),
            org_id: o.get_field("orgId"),
            password: o.get_field("password"),
            type_: o.get_field("type"),
        }
    }
}
