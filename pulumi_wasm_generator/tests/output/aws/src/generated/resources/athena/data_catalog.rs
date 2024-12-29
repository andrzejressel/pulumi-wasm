/// Provides an Athena data catalog.
///
/// More information about Athena and Athena data catalogs can be found in the [Athena User Guide](https://docs.aws.amazon.com/athena/latest/ug/what-is.html).
///
/// > **Tip:** for a more detailed explanation on the usage of `parameters`, see the [DataCatalog API documentation](https://docs.aws.amazon.com/athena/latest/APIReference/API_DataCatalog.html)
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:athena:DataCatalog
///     properties:
///       name: athena-data-catalog
///       description: Example Athena data catalog
///       type: LAMBDA
///       parameters:
///         function: arn:aws:lambda:eu-central-1:123456789012:function:not-important-lambda-function
///       tags:
///         Name: example-athena-data-catalog
/// ```
///
/// ### Hive based Data Catalog
///
/// ```yaml
/// resources:
///   example:
///     type: aws:athena:DataCatalog
///     properties:
///       name: hive-data-catalog
///       description: Hive based Data Catalog
///       type: HIVE
///       parameters:
///         metadata-function: arn:aws:lambda:eu-central-1:123456789012:function:not-important-lambda-function
/// ```
///
/// ### Glue based Data Catalog
///
/// ```yaml
/// resources:
///   example:
///     type: aws:athena:DataCatalog
///     properties:
///       name: glue-data-catalog
///       description: Glue based Data Catalog
///       type: GLUE
///       parameters:
///         catalog-id: '123456789012'
/// ```
///
/// ### Lambda based Data Catalog
///
/// ```yaml
/// resources:
///   example:
///     type: aws:athena:DataCatalog
///     properties:
///       name: lambda-data-catalog
///       description: Lambda based Data Catalog
///       type: LAMBDA
///       parameters:
///         metadata-function: arn:aws:lambda:eu-central-1:123456789012:function:not-important-lambda-function-1
///         record-function: arn:aws:lambda:eu-central-1:123456789012:function:not-important-lambda-function-2
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import data catalogs using their `name`. For example:
///
/// ```sh
/// $ pulumi import aws:athena/dataCatalog:DataCatalog example example-data-catalog
/// ```
pub mod data_catalog {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataCatalogArgs {
        /// Description of the data catalog.
        #[builder(into)]
        pub description: pulumi_wasm_rust::Output<String>,
        /// Name of the data catalog. The catalog name must be unique for the AWS account and can use a maximum of 128 alphanumeric, underscore, at sign, or hyphen characters.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key value pairs that specifies the Lambda function or functions to use for the data catalog. The mapping used depends on the catalog type.
        #[builder(into)]
        pub parameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of data catalog: `LAMBDA` for a federated catalog, `GLUE` for AWS Glue Catalog, or `HIVE` for an external hive metastore.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DataCatalogResult {
        /// ARN of the data catalog.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the data catalog.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Name of the data catalog. The catalog name must be unique for the AWS account and can use a maximum of 128 alphanumeric, underscore, at sign, or hyphen characters.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key value pairs that specifies the Lambda function or functions to use for the data catalog. The mapping used depends on the catalog type.
        pub parameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of data catalog: `LAMBDA` for a federated catalog, `GLUE` for AWS Glue Catalog, or `HIVE` for an external hive metastore.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DataCatalogArgs) -> DataCatalogResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:athena/dataCatalog:DataCatalog".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
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
        DataCatalogResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
