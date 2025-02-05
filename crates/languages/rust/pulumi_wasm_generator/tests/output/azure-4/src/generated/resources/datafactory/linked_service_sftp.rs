/// Manages a Linked Service (connection) between a SFTP Server and Azure Data Factory.
///
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLinkedServiceSftp = linked_service_sftp::create(
///         "exampleLinkedServiceSftp",
///         LinkedServiceSftpArgs::builder()
///             .authentication_type("Basic")
///             .data_factory_id("${exampleFactory.id}")
///             .host("http://www.bing.com")
///             .name("example")
///             .password("bar")
///             .port(22)
///             .username("foo")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory Linked Service's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServiceSftp:LinkedServiceSftp example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
pub mod linked_service_sftp {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceSftpArgs {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to SFTP Linked Service:
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The type of authentication used to connect to the web table source. Valid options are `Anonymous`, `Basic` and `ClientCertificate`.
        #[builder(into)]
        pub authentication_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The description for the Data Factory Linked Service.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The SFTP server hostname.
        #[builder(into)]
        pub host: pulumi_wasm_rust::InputOrOutput<String>,
        /// The host key fingerprint of the SFTP server.
        #[builder(into, default)]
        pub host_key_fingerprint: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Password to logon to the SFTP Server for Basic Authentication.
        #[builder(into)]
        pub password: pulumi_wasm_rust::InputOrOutput<String>,
        /// The TCP port number that the SFTP server uses to listen for client connection. Default value is 22.
        #[builder(into)]
        pub port: pulumi_wasm_rust::InputOrOutput<i32>,
        /// Whether to validate host key fingerprint while connecting. If set to `false`, `host_key_fingerprint` must also be set.
        #[builder(into, default)]
        pub skip_host_key_validation: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The username used to log on to the SFTP server.
        #[builder(into)]
        pub username: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceSftpResult {
        /// A map of additional properties to associate with the Data Factory Linked Service.
        ///
        /// The following supported arguments are specific to SFTP Linked Service:
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The type of authentication used to connect to the web table source. Valid options are `Anonymous`, `Basic` and `ClientCertificate`.
        pub authentication_type: pulumi_wasm_rust::Output<String>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Linked Service.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The SFTP server hostname.
        pub host: pulumi_wasm_rust::Output<String>,
        /// The host key fingerprint of the SFTP server.
        pub host_key_fingerprint: pulumi_wasm_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service.
        pub integration_runtime_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Linked Service.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Password to logon to the SFTP Server for Basic Authentication.
        pub password: pulumi_wasm_rust::Output<String>,
        /// The TCP port number that the SFTP server uses to listen for client connection. Default value is 22.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// Whether to validate host key fingerprint while connecting. If set to `false`, `host_key_fingerprint` must also be set.
        pub skip_host_key_validation: pulumi_wasm_rust::Output<Option<bool>>,
        /// The username used to log on to the SFTP server.
        pub username: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LinkedServiceSftpArgs,
    ) -> LinkedServiceSftpResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding = args
            .additional_properties
            .get_output(context)
            .get_inner();
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let authentication_type_binding = args
            .authentication_type
            .get_output(context)
            .get_inner();
        let data_factory_id_binding = args
            .data_factory_id
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let host_binding = args.host.get_output(context).get_inner();
        let host_key_fingerprint_binding = args
            .host_key_fingerprint
            .get_output(context)
            .get_inner();
        let integration_runtime_name_binding = args
            .integration_runtime_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let skip_host_key_validation_binding = args
            .skip_host_key_validation
            .get_output(context)
            .get_inner();
        let username_binding = args.username.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceSftp:LinkedServiceSftp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalProperties".into(),
                    value: &additional_properties_binding,
                },
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "authenticationType".into(),
                    value: &authentication_type_binding,
                },
                register_interface::ObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "host".into(),
                    value: &host_binding,
                },
                register_interface::ObjectField {
                    name: "hostKeyFingerprint".into(),
                    value: &host_key_fingerprint_binding,
                },
                register_interface::ObjectField {
                    name: "integrationRuntimeName".into(),
                    value: &integration_runtime_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "skipHostKeyValidation".into(),
                    value: &skip_host_key_validation_binding,
                },
                register_interface::ObjectField {
                    name: "username".into(),
                    value: &username_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LinkedServiceSftpResult {
            additional_properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("additionalProperties"),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            authentication_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authenticationType"),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataFactoryId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            host: pulumi_wasm_rust::__private::into_domain(o.extract_field("host")),
            host_key_fingerprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostKeyFingerprint"),
            ),
            integration_runtime_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("integrationRuntimeName"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            port: pulumi_wasm_rust::__private::into_domain(o.extract_field("port")),
            skip_host_key_validation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skipHostKeyValidation"),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("username"),
            ),
        }
    }
}
