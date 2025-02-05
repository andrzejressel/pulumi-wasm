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
pub mod keystores_aliases_key_cert_file {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeystoresAliasesKeyCertFileArgs {
        /// Alias Name
        #[builder(into)]
        pub alias: pulumi_wasm_rust::InputOrOutput<String>,
        /// Cert content
        ///
        ///
        /// - - -
        #[builder(into)]
        pub cert: pulumi_wasm_rust::InputOrOutput<String>,
        /// Chain of certificates under this alias.
        /// Structure is documented below.
        #[builder(into, default)]
        pub certs_info: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::apigee::KeystoresAliasesKeyCertFileCertsInfo>,
        >,
        /// Environment associated with the alias
        #[builder(into)]
        pub environment: pulumi_wasm_rust::InputOrOutput<String>,
        /// Private Key content, omit if uploading to truststore
        #[builder(into, default)]
        pub key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Keystore Name
        #[builder(into)]
        pub keystore: pulumi_wasm_rust::InputOrOutput<String>,
        /// Organization ID associated with the alias, without organization/ prefix
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Password for the Private Key if it's encrypted
        #[builder(into, default)]
        pub password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeystoresAliasesKeyCertFileResult {
        /// Alias Name
        pub alias: pulumi_wasm_rust::Output<String>,
        /// Cert content
        ///
        ///
        /// - - -
        pub cert: pulumi_wasm_rust::Output<String>,
        /// Chain of certificates under this alias.
        /// Structure is documented below.
        pub certs_info: pulumi_wasm_rust::Output<
            super::super::types::apigee::KeystoresAliasesKeyCertFileCertsInfo,
        >,
        /// Environment associated with the alias
        pub environment: pulumi_wasm_rust::Output<String>,
        /// Private Key content, omit if uploading to truststore
        pub key: pulumi_wasm_rust::Output<Option<String>>,
        /// Keystore Name
        pub keystore: pulumi_wasm_rust::Output<String>,
        /// Organization ID associated with the alias, without organization/ prefix
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// Password for the Private Key if it's encrypted
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional.Type of Alias
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: KeystoresAliasesKeyCertFileArgs,
    ) -> KeystoresAliasesKeyCertFileResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_output(context).get_inner();
        let cert_binding = args.cert.get_output(context).get_inner();
        let certs_info_binding = args.certs_info.get_output(context).get_inner();
        let environment_binding = args.environment.get_output(context).get_inner();
        let key_binding = args.key.get_output(context).get_inner();
        let keystore_binding = args.keystore.get_output(context).get_inner();
        let org_id_binding = args.org_id.get_output(context).get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/keystoresAliasesKeyCertFile:KeystoresAliasesKeyCertFile"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alias".into(),
                    value: &alias_binding,
                },
                register_interface::ObjectField {
                    name: "cert".into(),
                    value: &cert_binding,
                },
                register_interface::ObjectField {
                    name: "certsInfo".into(),
                    value: &certs_info_binding,
                },
                register_interface::ObjectField {
                    name: "environment".into(),
                    value: &environment_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "keystore".into(),
                    value: &keystore_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        KeystoresAliasesKeyCertFileResult {
            alias: pulumi_wasm_rust::__private::into_domain(o.extract_field("alias")),
            cert: pulumi_wasm_rust::__private::into_domain(o.extract_field("cert")),
            certs_info: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certsInfo"),
            ),
            environment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("environment"),
            ),
            key: pulumi_wasm_rust::__private::into_domain(o.extract_field("key")),
            keystore: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keystore"),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("orgId")),
            password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
