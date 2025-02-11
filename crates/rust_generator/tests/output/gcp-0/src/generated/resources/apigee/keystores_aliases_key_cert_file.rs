/// An alias from a key/certificate pair.
///
/// To get more information about KeystoresAliasesKeyCertFile, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.keystores.aliases)
/// * How-to Guides
///     * [Keystores Aliases](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.keystores.aliases)
///
/// ## Import
///
/// KeystoresAliasesKeyCertFile can be imported using any of these accepted formats:
///
/// * `organizations/{{org_id}}/environments/{{environment}}/keystores/{{keystore}}/aliases/{{alias}}`
///
/// * `{{org_id}}/{{environment}}/{{keystore}}/{{alias}}`
///
/// When using the `pulumi import` command, KeystoresAliasesKeyCertFile can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/keystoresAliasesKeyCertFile:KeystoresAliasesKeyCertFile default organizations/{{org_id}}/environments/{{environment}}/keystores/{{keystore}}/aliases/{{alias}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/keystoresAliasesKeyCertFile:KeystoresAliasesKeyCertFile default {{org_id}}/{{environment}}/{{keystore}}/{{alias}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod keystores_aliases_key_cert_file {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeystoresAliasesKeyCertFileArgs {
        /// Alias Name
        #[builder(into)]
        pub alias: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Cert content
        ///
        ///
        /// - - -
        #[builder(into)]
        pub cert: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Chain of certificates under this alias.
        /// Structure is documented below.
        #[builder(into, default)]
        pub certs_info: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigee::KeystoresAliasesKeyCertFileCertsInfo>,
        >,
        /// Environment associated with the alias
        #[builder(into)]
        pub environment: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Private Key content, omit if uploading to truststore
        #[builder(into, default)]
        pub key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Keystore Name
        #[builder(into)]
        pub keystore: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Organization ID associated with the alias, without organization/ prefix
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Password for the Private Key if it's encrypted
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeystoresAliasesKeyCertFileResult {
        /// Alias Name
        pub alias: pulumi_gestalt_rust::Output<String>,
        /// Cert content
        ///
        ///
        /// - - -
        pub cert: pulumi_gestalt_rust::Output<String>,
        /// Chain of certificates under this alias.
        /// Structure is documented below.
        pub certs_info: pulumi_gestalt_rust::Output<
            super::super::types::apigee::KeystoresAliasesKeyCertFileCertsInfo,
        >,
        /// Environment associated with the alias
        pub environment: pulumi_gestalt_rust::Output<String>,
        /// Private Key content, omit if uploading to truststore
        pub key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Keystore Name
        pub keystore: pulumi_gestalt_rust::Output<String>,
        /// Organization ID associated with the alias, without organization/ prefix
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// Password for the Private Key if it's encrypted
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
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
        args: KeystoresAliasesKeyCertFileArgs,
    ) -> KeystoresAliasesKeyCertFileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alias_binding = args.alias.get_output(context);
        let cert_binding = args.cert.get_output(context);
        let certs_info_binding = args.certs_info.get_output(context);
        let environment_binding = args.environment.get_output(context);
        let key_binding = args.key.get_output(context);
        let keystore_binding = args.keystore.get_output(context);
        let org_id_binding = args.org_id.get_output(context);
        let password_binding = args.password.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/keystoresAliasesKeyCertFile:KeystoresAliasesKeyCertFile"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alias".into(),
                    value: &alias_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cert".into(),
                    value: &cert_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certsInfo".into(),
                    value: &certs_info_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environment".into(),
                    value: &environment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "key".into(),
                    value: &key_binding.drop_type(),
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
        KeystoresAliasesKeyCertFileResult {
            alias: o.get_field("alias"),
            cert: o.get_field("cert"),
            certs_info: o.get_field("certsInfo"),
            environment: o.get_field("environment"),
            key: o.get_field("key"),
            keystore: o.get_field("keystore"),
            org_id: o.get_field("orgId"),
            password: o.get_field("password"),
            type_: o.get_field("type"),
        }
    }
}
