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
pub mod keystores_aliases_pkcs_12 {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeystoresAliasesPkcs12Args {
        /// Alias Name
        #[builder(into)]
        pub alias: pulumi_wasm_rust::Output<String>,
        /// Environment associated with the alias
        #[builder(into)]
        pub environment: pulumi_wasm_rust::Output<String>,
        /// PKCS12 file content
        ///
        /// - - -
        #[builder(into)]
        pub file: pulumi_wasm_rust::Output<String>,
        /// Hash of the pkcs file
        #[builder(into)]
        pub filehash: pulumi_wasm_rust::Output<String>,
        /// Keystore Name
        #[builder(into)]
        pub keystore: pulumi_wasm_rust::Output<String>,
        /// Organization ID associated with the alias, without organization/ prefix
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// Password for the PKCS12 file if it's encrypted
        #[builder(into, default)]
        pub password: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeystoresAliasesPkcs12Result {
        /// Alias Name
        pub alias: pulumi_wasm_rust::Output<String>,
        /// Chain of certificates under this alias.
        /// Structure is documented below.
        pub certs_infos: pulumi_wasm_rust::Output<
            Vec<super::super::types::apigee::KeystoresAliasesPkcs12CertsInfo>,
        >,
        /// Environment associated with the alias
        pub environment: pulumi_wasm_rust::Output<String>,
        /// PKCS12 file content
        ///
        /// - - -
        pub file: pulumi_wasm_rust::Output<String>,
        /// Hash of the pkcs file
        pub filehash: pulumi_wasm_rust::Output<String>,
        /// Keystore Name
        pub keystore: pulumi_wasm_rust::Output<String>,
        /// Organization ID associated with the alias, without organization/ prefix
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// Password for the PKCS12 file if it's encrypted
        pub password: pulumi_wasm_rust::Output<String>,
        /// Optional.Type of Alias
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: KeystoresAliasesPkcs12Args,
    ) -> KeystoresAliasesPkcs12Result {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_inner();
        let environment_binding = args.environment.get_inner();
        let file_binding = args.file.get_inner();
        let filehash_binding = args.filehash.get_inner();
        let keystore_binding = args.keystore.get_inner();
        let org_id_binding = args.org_id.get_inner();
        let password_binding = args.password.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "certsInfos".into(),
                },
                register_interface::ResultField {
                    name: "environment".into(),
                },
                register_interface::ResultField {
                    name: "file".into(),
                },
                register_interface::ResultField {
                    name: "filehash".into(),
                },
                register_interface::ResultField {
                    name: "keystore".into(),
                },
                register_interface::ResultField {
                    name: "orgId".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KeystoresAliasesPkcs12Result {
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            certs_infos: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certsInfos").unwrap(),
            ),
            environment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environment").unwrap(),
            ),
            file: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("file").unwrap(),
            ),
            filehash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filehash").unwrap(),
            ),
            keystore: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keystore").unwrap(),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orgId").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
