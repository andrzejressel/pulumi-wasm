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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: KeystoresAliasesPkcs12Args,
    ) -> KeystoresAliasesPkcs12Result {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let alias_binding_1 = args.alias.get_output(context);
        let alias_binding = alias_binding_1.get_inner();
        let environment_binding_1 = args.environment.get_output(context);
        let environment_binding = environment_binding_1.get_inner();
        let file_binding_1 = args.file.get_output(context);
        let file_binding = file_binding_1.get_inner();
        let filehash_binding_1 = args.filehash.get_output(context);
        let filehash_binding = filehash_binding_1.get_inner();
        let keystore_binding_1 = args.keystore.get_output(context);
        let keystore_binding = keystore_binding_1.get_inner();
        let org_id_binding_1 = args.org_id.get_output(context);
        let org_id_binding = org_id_binding_1.get_inner();
        let password_binding_1 = args.password.get_output(context);
        let password_binding = password_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/keystoresAliasesPkcs12:KeystoresAliasesPkcs12".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alias".into(),
                    value: &alias_binding,
                },
                register_interface::ObjectField {
                    name: "environment".into(),
                    value: &environment_binding,
                },
                register_interface::ObjectField {
                    name: "file".into(),
                    value: &file_binding,
                },
                register_interface::ObjectField {
                    name: "filehash".into(),
                    value: &filehash_binding,
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
        KeystoresAliasesPkcs12Result {
            alias: pulumi_gestalt_rust::__private::into_domain(o.extract_field("alias")),
            certs_infos: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certsInfos"),
            ),
            environment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environment"),
            ),
            file: pulumi_gestalt_rust::__private::into_domain(o.extract_field("file")),
            filehash: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filehash"),
            ),
            keystore: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keystore"),
            ),
            org_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("orgId"),
            ),
            password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
