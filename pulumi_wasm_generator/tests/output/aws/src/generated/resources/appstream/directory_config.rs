/// Provides an AppStream Directory Config.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = directory_config::create(
///         "example",
///         DirectoryConfigArgs::builder()
///             .directory_name("NAME OF DIRECTORY")
///             .organizational_unit_distinguished_names(vec!["DISTINGUISHED NAME",])
///             .service_account_credentials(
///                 DirectoryConfigServiceAccountCredentials::builder()
///                     .accountName("NAME OF ACCOUNT")
///                     .accountPassword("PASSWORD OF ACCOUNT")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appstream_directory_config` using the id. For example:
///
/// ```sh
/// $ pulumi import aws:appstream/directoryConfig:DirectoryConfig example directoryNameExample
/// ```
pub mod directory_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DirectoryConfigArgs {
        /// Fully qualified name of the directory.
        #[builder(into)]
        pub directory_name: pulumi_wasm_rust::Output<String>,
        /// Distinguished names of the organizational units for computer accounts.
        #[builder(into)]
        pub organizational_unit_distinguished_names: pulumi_wasm_rust::Output<
            Vec<String>,
        >,
        /// Configuration block for the name of the directory and organizational unit (OU) to use to join the directory config to a Microsoft Active Directory domain. See `service_account_credentials` below.
        #[builder(into)]
        pub service_account_credentials: pulumi_wasm_rust::Output<
            super::super::types::appstream::DirectoryConfigServiceAccountCredentials,
        >,
    }
    #[allow(dead_code)]
    pub struct DirectoryConfigResult {
        /// Date and time, in UTC and extended RFC 3339 format, when the directory config was created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// Fully qualified name of the directory.
        pub directory_name: pulumi_wasm_rust::Output<String>,
        /// Distinguished names of the organizational units for computer accounts.
        pub organizational_unit_distinguished_names: pulumi_wasm_rust::Output<
            Vec<String>,
        >,
        /// Configuration block for the name of the directory and organizational unit (OU) to use to join the directory config to a Microsoft Active Directory domain. See `service_account_credentials` below.
        pub service_account_credentials: pulumi_wasm_rust::Output<
            super::super::types::appstream::DirectoryConfigServiceAccountCredentials,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DirectoryConfigArgs) -> DirectoryConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let directory_name_binding = args.directory_name.get_inner();
        let organizational_unit_distinguished_names_binding = args
            .organizational_unit_distinguished_names
            .get_inner();
        let service_account_credentials_binding = args
            .service_account_credentials
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appstream/directoryConfig:DirectoryConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "directoryName".into(),
                    value: &directory_name_binding,
                },
                register_interface::ObjectField {
                    name: "organizationalUnitDistinguishedNames".into(),
                    value: &organizational_unit_distinguished_names_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccountCredentials".into(),
                    value: &service_account_credentials_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createdTime".into(),
                },
                register_interface::ResultField {
                    name: "directoryName".into(),
                },
                register_interface::ResultField {
                    name: "organizationalUnitDistinguishedNames".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccountCredentials".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DirectoryConfigResult {
            created_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTime").unwrap(),
            ),
            directory_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryName").unwrap(),
            ),
            organizational_unit_distinguished_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organizationalUnitDistinguishedNames").unwrap(),
            ),
            service_account_credentials: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccountCredentials").unwrap(),
            ),
        }
    }
}