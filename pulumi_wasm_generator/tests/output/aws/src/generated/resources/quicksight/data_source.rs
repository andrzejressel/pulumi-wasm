/// Resource for managing QuickSight Data Source
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = data_source::create(
///         "default",
///         DataSourceArgs::builder()
///             .data_source_id("example-id")
///             .name("My Cool Data in S3")
///             .parameters(
///                 DataSourceParameters::builder()
///                     .s3(
///                         DataSourceParametersS3::builder()
///                             .manifestFileLocation(
///                                 DataSourceParametersS3ManifestFileLocation::builder()
///                                     .bucket("my-bucket")
///                                     .key("path/to/manifest.json")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .type_("S3")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a QuickSight data source using the AWS account ID, and data source ID separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/dataSource:DataSource example 123456789123/my-data-source-id
/// ```
pub mod data_source {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataSourceArgs {
        /// The ID for the AWS account that the data source is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The credentials Amazon QuickSight uses to connect to your underlying source. See Credentials below for more details.
        #[builder(into, default)]
        pub credentials: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::DataSourceCredentials>,
        >,
        /// An identifier for the data source.
        #[builder(into)]
        pub data_source_id: pulumi_wasm_rust::Output<String>,
        /// A name for the data source, maximum of 128 characters.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The parameters used to connect to this data source (exactly one).
        #[builder(into)]
        pub parameters: pulumi_wasm_rust::Output<
            super::super::types::quicksight::DataSourceParameters,
        >,
        /// A set of resource permissions on the data source. Maximum of 64 items. See Permission below for more details.
        #[builder(into, default)]
        pub permissions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::quicksight::DataSourcePermission>>,
        >,
        /// Secure Socket Layer (SSL) properties that apply when Amazon QuickSight connects to your underlying source. See SSL Properties below for more details.
        #[builder(into, default)]
        pub ssl_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::DataSourceSslProperties>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the data source. See the [AWS Documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_CreateDataSource.html#QS-CreateDataSource-request-Type) for the complete list of valid values.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Use this parameter only when you want Amazon QuickSight to use a VPC connection when connecting to your underlying source. See VPC Connection Properties below for more details.
        #[builder(into, default)]
        pub vpc_connection_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::DataSourceVpcConnectionProperties>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataSourceResult {
        /// Amazon Resource Name (ARN) of the data source
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID for the AWS account that the data source is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// The credentials Amazon QuickSight uses to connect to your underlying source. See Credentials below for more details.
        pub credentials: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::DataSourceCredentials>,
        >,
        /// An identifier for the data source.
        pub data_source_id: pulumi_wasm_rust::Output<String>,
        /// A name for the data source, maximum of 128 characters.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parameters used to connect to this data source (exactly one).
        pub parameters: pulumi_wasm_rust::Output<
            super::super::types::quicksight::DataSourceParameters,
        >,
        /// A set of resource permissions on the data source. Maximum of 64 items. See Permission below for more details.
        pub permissions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::quicksight::DataSourcePermission>>,
        >,
        /// Secure Socket Layer (SSL) properties that apply when Amazon QuickSight connects to your underlying source. See SSL Properties below for more details.
        pub ssl_properties: pulumi_wasm_rust::Output<
            super::super::types::quicksight::DataSourceSslProperties,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of the data source. See the [AWS Documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_CreateDataSource.html#QS-CreateDataSource-request-Type) for the complete list of valid values.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Use this parameter only when you want Amazon QuickSight to use a VPC connection when connecting to your underlying source. See VPC Connection Properties below for more details.
        pub vpc_connection_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::DataSourceVpcConnectionProperties>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DataSourceArgs) -> DataSourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_inner();
        let credentials_binding = args.credentials.get_inner();
        let data_source_id_binding = args.data_source_id.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let permissions_binding = args.permissions.get_inner();
        let ssl_properties_binding = args.ssl_properties.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let vpc_connection_properties_binding = args
            .vpc_connection_properties
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/dataSource:DataSource".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "credentials".into(),
                    value: &credentials_binding,
                },
                register_interface::ObjectField {
                    name: "dataSourceId".into(),
                    value: &data_source_id_binding,
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
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "sslProperties".into(),
                    value: &ssl_properties_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "vpcConnectionProperties".into(),
                    value: &vpc_connection_properties_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "credentials".into(),
                },
                register_interface::ResultField {
                    name: "dataSourceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "permissions".into(),
                },
                register_interface::ResultField {
                    name: "sslProperties".into(),
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
                register_interface::ResultField {
                    name: "vpcConnectionProperties".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataSourceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            credentials: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentials").unwrap(),
            ),
            data_source_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSourceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissions").unwrap(),
            ),
            ssl_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslProperties").unwrap(),
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
            vpc_connection_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConnectionProperties").unwrap(),
            ),
        }
    }
}