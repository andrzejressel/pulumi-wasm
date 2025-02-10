/// Resource for managing an AWS FinSpace Kx Database.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = key::create(
///         "example",
///         KeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("Example KMS Key")
///             .build_struct(),
///     );
///     let exampleKxDatabase = kx_database::create(
///         "exampleKxDatabase",
///         KxDatabaseArgs::builder()
///             .description("Example database description")
///             .environment_id("${exampleKxEnvironment.id}")
///             .name("my-tf-kx-database")
///             .build_struct(),
///     );
///     let exampleKxEnvironment = kx_environment::create(
///         "exampleKxEnvironment",
///         KxEnvironmentArgs::builder()
///             .kms_key_id("${example.arn}")
///             .name("my-tf-kx-environment")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an AWS FinSpace Kx Database using the `id` (environment ID and database name, comma-delimited). For example:
///
/// ```sh
/// $ pulumi import aws:finspace/kxDatabase:KxDatabase example n3ceo7wqxoxcti5tujqwzs,my-tf-kx-database
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod kx_database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KxDatabaseArgs {
        /// Description of the KX database.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique identifier for the KX environment.
        #[builder(into)]
        pub environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the KX database.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct KxDatabaseResult {
        /// Amazon Resource Name (ARN) identifier of the KX database.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Timestamp at which the databse is created in FinSpace. Value determined as epoch time in seconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000.
        pub created_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Description of the KX database.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Unique identifier for the KX environment.
        pub environment_id: pulumi_gestalt_rust::Output<String>,
        /// Last timestamp at which the database was updated in FinSpace. Value determined as epoch time in seconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000.
        pub last_modified_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Name of the KX database.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KxDatabaseArgs,
    ) -> KxDatabaseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let environment_id_binding = args.environment_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:finspace/kxDatabase:KxDatabase".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentId".into(),
                    value: environment_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        KxDatabaseResult {
            arn: o.get_field("arn"),
            created_timestamp: o.get_field("createdTimestamp"),
            description: o.get_field("description"),
            environment_id: o.get_field("environmentId"),
            last_modified_timestamp: o.get_field("lastModifiedTimestamp"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
