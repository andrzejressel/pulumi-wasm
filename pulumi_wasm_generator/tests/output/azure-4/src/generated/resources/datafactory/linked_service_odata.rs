/// Manages a Linked Service (connection) between a Database and Azure Data Factory through OData protocol.
///
/// > **Note:** All arguments including the connection_string will be stored in the raw state as plain-text. [Read more about sensitive data in state](https://www.terraform.io/docs/state/sensitive-data.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let anonymous = linked_service_odata::create(
///         "anonymous",
///         LinkedServiceOdataArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .name("anonymous")
///             .url("https://services.odata.org/v4/TripPinServiceRW/People")
///             .build_struct(),
///     );
///     let basicAuth = linked_service_odata::create(
///         "basicAuth",
///         LinkedServiceOdataArgs::builder()
///             .basic_authentication(
///                 LinkedServiceOdataBasicAuthentication::builder()
///                     .password("Ch4ngeM3!")
///                     .username("emma")
///                     .build_struct(),
///             )
///             .data_factory_id("${exampleFactory.id}")
///             .name("basic_auth")
///             .url("https://services.odata.org/v4/TripPinServiceRW/People")
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
/// Data Factory OData Linked Service's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/linkedServiceOdata:LinkedServiceOdata example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/linkedservices/example
/// ```
///
pub mod linked_service_odata {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceOdataArgs {
        /// A map of additional properties to associate with the Data Factory Linked Service OData.
        #[builder(into, default)]
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service OData.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `basic_authentication` block as defined below.
        #[builder(into, default)]
        pub basic_authentication: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceOdataBasicAuthentication,
            >,
        >,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Linked Service OData.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service OData.
        #[builder(into, default)]
        pub integration_runtime_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service OData. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of parameters to associate with the Data Factory Linked Service OData.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The URL of the OData service endpoint.
        #[builder(into)]
        pub url: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceOdataResult {
        /// A map of additional properties to associate with the Data Factory Linked Service OData.
        pub additional_properties: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// List of tags that can be used for describing the Data Factory Linked Service OData.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A `basic_authentication` block as defined below.
        pub basic_authentication: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datafactory::LinkedServiceOdataBasicAuthentication,
            >,
        >,
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// The description for the Data Factory Linked Service OData.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The integration runtime reference to associate with the Data Factory Linked Service OData.
        pub integration_runtime_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Data Factory Linked Service OData. Changing this forces a new resource to be created. Must be unique within a data factory. See the [Microsoft documentation](https://docs.microsoft.com/azure/data-factory/naming-rules) for all restrictions.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of parameters to associate with the Data Factory Linked Service OData.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The URL of the OData service endpoint.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LinkedServiceOdataArgs) -> LinkedServiceOdataResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_properties_binding = args.additional_properties.get_inner();
        let annotations_binding = args.annotations.get_inner();
        let basic_authentication_binding = args.basic_authentication.get_inner();
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let description_binding = args.description.get_inner();
        let integration_runtime_name_binding = args.integration_runtime_name.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let url_binding = args.url.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/linkedServiceOdata:LinkedServiceOdata".into(),
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
                register_interface::ObjectField {
                    name: "url".into(),
                    value: &url_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalProperties".into(),
                },
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "basicAuthentication".into(),
                },
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "integrationRuntimeName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LinkedServiceOdataResult {
            additional_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalProperties").unwrap(),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            basic_authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("basicAuthentication").unwrap(),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            integration_runtime_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("integrationRuntimeName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
