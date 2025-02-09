/// Manages a Linked Service (connection) between a Database and Azure Data Factory through ODBC protocol.
///
/// > **Note:** All arguments including the connection_string will be stored in the raw state as plain-text. [Read more about sensitive data in state](https://www.terraform.io/docs/state/sensitive-data.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let anonymous = linked_service_odbc::create(
///         "anonymous",
///         LinkedServiceOdbcArgs::builder()
///             .connection_string(
///                 "Driver={SQL Server};Server=test;Database=test;Uid=test;Pwd=test;",
///             )
///             .data_factory_id("${exampleFactory.id}")
///             .name("anonymous")
///             .build_struct(),
///     );
///     let basicAuth = linked_service_odbc::create(
///         "basicAuth",
///         LinkedServiceOdbcArgs::builder()
///             .basic_authentication(
///                 LinkedServiceOdbcBasicAuthentication::builder()
///                     .password("Ch4ngeM3!")
///                     .username("onrylmz")
///                     .build_struct(),
///             )
///             .connection_string(
///                 "Driver={SQL Server};Server=test;Database=test;Uid=test;Pwd=test;",
///             )
///             .data_factory_id("${exampleFactory.id}")
///             .name("basic_auth")
///             .build_struct(),
///     );
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
/// }
/// ```
///
/// ## Import
///
/// Data Factory ODBC Linked Service's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServiceOdbc:LinkedServiceOdbc example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linked_service_odbc {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceOdbcArgs {
        /// A map of additional properties to associate with the Data Factory Linked Service ODBC.
        #[builder(into, default)]
        pub additional_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service ODBC.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A `basic_authentication` block as defined below.
        #[builder(into, default)]
        pub basic_authentication: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datafactory::LinkedServiceOdbcBasicAuthentication,
            >,
        >,
        /// The connection string in which to authenticate with ODBC.
        #[builder(into)]
        pub connection_string: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the Data Factory Linked Service ODBC.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service ODBC.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service ODBC. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Linked Service ODBC.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceOdbcResult {
        /// A map of additional properties to associate with the Data Factory Linked Service ODBC.
        pub additional_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service ODBC.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A `basic_authentication` block as defined below.
        pub basic_authentication: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceOdbcBasicAuthentication,
            >,
        >,
        /// The connection string in which to authenticate with ODBC.
        pub connection_string: pulumi_gestalt_rust::Output<String>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// The description for the Data Factory Linked Service ODBC.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service ODBC.
        pub integration_runtime_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service ODBC. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Linked Service ODBC.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LinkedServiceOdbcArgs,
    ) -> LinkedServiceOdbcResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding_1 = args
            .additional_properties
            .get_output(context);
        let additional_properties_binding = additional_properties_binding_1.get_inner();
        let annotations_binding_1 = args.annotations.get_output(context);
        let annotations_binding = annotations_binding_1.get_inner();
        let basic_authentication_binding_1 = args
            .basic_authentication
            .get_output(context);
        let basic_authentication_binding = basic_authentication_binding_1.get_inner();
        let connection_string_binding_1 = args.connection_string.get_output(context);
        let connection_string_binding = connection_string_binding_1.get_inner();
        let data_factory_id_binding_1 = args.data_factory_id.get_output(context);
        let data_factory_id_binding = data_factory_id_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let integration_runtime_name_binding_1 = args
            .integration_runtime_name
            .get_output(context);
        let integration_runtime_name_binding = integration_runtime_name_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let parameters_binding_1 = args.parameters.get_output(context);
        let parameters_binding = parameters_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceOdbc:LinkedServiceOdbc".into(),
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
                    name: "basicAuthentication".into(),
                    value: &basic_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "connectionString".into(),
                    value: &connection_string_binding,
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
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LinkedServiceOdbcResult {
            additional_properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalProperties"),
            ),
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            basic_authentication: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("basicAuthentication"),
            ),
            connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionString"),
            ),
            data_factory_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataFactoryId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            integration_runtime_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("integrationRuntimeName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
        }
    }
}
