/// Resource for managing QuickSight Data Source
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_source {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataSourceArgs {
        /// The ID for the AWS account that the data source is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The credentials Amazon QuickSight uses to connect to your underlying source. See Credentials below for more details.
        #[builder(into, default)]
        pub credentials: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::quicksight::DataSourceCredentials>,
        >,
        /// An identifier for the data source.
        #[builder(into)]
        pub data_source_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A name for the data source, maximum of 128 characters.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parameters used to connect to this data source (exactly one).
        #[builder(into)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::quicksight::DataSourceParameters,
        >,
        /// A set of resource permissions on the data source. Maximum of 64 items. See Permission below for more details.
        #[builder(into, default)]
        pub permissions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::quicksight::DataSourcePermission>>,
        >,
        /// Secure Socket Layer (SSL) properties that apply when Amazon QuickSight connects to your underlying source. See SSL Properties below for more details.
        #[builder(into, default)]
        pub ssl_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::quicksight::DataSourceSslProperties>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the data source. See the [AWS Documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_CreateDataSource.html#QS-CreateDataSource-request-Type) for the complete list of valid values.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Use this parameter only when you want Amazon QuickSight to use a VPC connection when connecting to your underlying source. See VPC Connection Properties below for more details.
        #[builder(into, default)]
        pub vpc_connection_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::quicksight::DataSourceVpcConnectionProperties>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataSourceResult {
        /// Amazon Resource Name (ARN) of the data source
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID for the AWS account that the data source is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The credentials Amazon QuickSight uses to connect to your underlying source. See Credentials below for more details.
        pub credentials: pulumi_gestalt_rust::Output<
            Option<super::super::types::quicksight::DataSourceCredentials>,
        >,
        /// An identifier for the data source.
        pub data_source_id: pulumi_gestalt_rust::Output<String>,
        /// A name for the data source, maximum of 128 characters.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parameters used to connect to this data source (exactly one).
        pub parameters: pulumi_gestalt_rust::Output<
            super::super::types::quicksight::DataSourceParameters,
        >,
        /// A set of resource permissions on the data source. Maximum of 64 items. See Permission below for more details.
        pub permissions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::quicksight::DataSourcePermission>>,
        >,
        /// Secure Socket Layer (SSL) properties that apply when Amazon QuickSight connects to your underlying source. See SSL Properties below for more details.
        pub ssl_properties: pulumi_gestalt_rust::Output<
            super::super::types::quicksight::DataSourceSslProperties,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of the data source. See the [AWS Documentation](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_CreateDataSource.html#QS-CreateDataSource-request-Type) for the complete list of valid values.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Use this parameter only when you want Amazon QuickSight to use a VPC connection when connecting to your underlying source. See VPC Connection Properties below for more details.
        pub vpc_connection_properties: pulumi_gestalt_rust::Output<
            Option<super::super::types::quicksight::DataSourceVpcConnectionProperties>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataSourceArgs,
    ) -> DataSourceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aws_account_id_binding = args.aws_account_id.get_output(context);
        let credentials_binding = args.credentials.get_output(context);
        let data_source_id_binding = args.data_source_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let permissions_binding = args.permissions.get_output(context);
        let ssl_properties_binding = args.ssl_properties.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let vpc_connection_properties_binding = args
            .vpc_connection_properties
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:quicksight/dataSource:DataSource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: aws_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "credentials".into(),
                    value: credentials_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSourceId".into(),
                    value: data_source_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissions".into(),
                    value: permissions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sslProperties".into(),
                    value: ssl_properties_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConnectionProperties".into(),
                    value: vpc_connection_properties_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataSourceResult {
            arn: o.get_field("arn"),
            aws_account_id: o.get_field("awsAccountId"),
            credentials: o.get_field("credentials"),
            data_source_id: o.get_field("dataSourceId"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            permissions: o.get_field("permissions"),
            ssl_properties: o.get_field("sslProperties"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
            vpc_connection_properties: o.get_field("vpcConnectionProperties"),
        }
    }
}
