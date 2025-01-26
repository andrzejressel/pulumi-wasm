/// Manages a Linked Service (connection) between PostgreSQL and Azure Data Factory.
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
///     let exampleLinkedServicePostgresql = linked_service_postgresql::create(
///         "exampleLinkedServicePostgresql",
///         LinkedServicePostgresqlArgs::builder()
///             .connection_string(
///                 "Host=example;Port=5432;Database=example;UID=example;EncryptionMethod=0;Password=example",
///             )
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factory PostgreSQL Linked Service's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServicePostgresql:LinkedServicePostgresql example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
pub mod linked_service_postgresql {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServicePostgresqlArgs {
        /// A map of additional properties to associate with the Data Factory Linked Service PostgreSQL.
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service PostgreSQL.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The connection string in which to authenticate with PostgreSQL.
        #[builder(into)]
        pub connection_string: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The description for the Data Factory Linked Service PostgreSQL.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service PostgreSQL.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service PostgreSQL. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of parameters to associate with the Data Factory Linked Service PostgreSQL.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LinkedServicePostgresqlResult {
        /// A map of additional properties to associate with the Data Factory Linked Service PostgreSQL.
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service PostgreSQL.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The connection string in which to authenticate with PostgreSQL.
        pub connection_string: pulumi_wasm_rust::Output<String>,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Linked Service PostgreSQL.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service PostgreSQL.
        pub integration_runtime_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service PostgreSQL. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Linked Service PostgreSQL.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LinkedServicePostgresqlArgs,
    ) -> LinkedServicePostgresqlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding = args
            .additional_properties
            .get_output(context)
            .get_inner();
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let connection_string_binding = args
            .connection_string
            .get_output(context)
            .get_inner();
        let data_factory_id_binding = args
            .data_factory_id
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let integration_runtime_name_binding = args
            .integration_runtime_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServicePostgresql:LinkedServicePostgresql"
                .into(),
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
        LinkedServicePostgresqlResult {
            additional_properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("additionalProperties"),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionString"),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataFactoryId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            integration_runtime_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("integrationRuntimeName"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
        }
    }
}
