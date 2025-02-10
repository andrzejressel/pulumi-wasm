/// Provides a Glue User Defined Function Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = catalog_database::create(
///         "example",
///         CatalogDatabaseArgs::builder().name("my_database").build_struct(),
///     );
///     let exampleUserDefinedFunction = user_defined_function::create(
///         "exampleUserDefinedFunction",
///         UserDefinedFunctionArgs::builder()
///             .catalog_id("${example.catalogId}")
///             .class_name("class")
///             .database_name("${example.name}")
///             .name("my_func")
///             .owner_name("owner")
///             .owner_type("GROUP")
///             .resource_uris(
///                 vec![
///                     UserDefinedFunctionResourceUri::builder().resourceType("ARCHIVE")
///                     .uri("uri").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue User Defined Functions using the `catalog_id:database_name:function_name`. If you have not set a Catalog ID specify the AWS Account ID that the database is in. For example:
///
/// ```sh
/// $ pulumi import aws:glue/userDefinedFunction:UserDefinedFunction func 123456789012:my_database:my_func
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_defined_function {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserDefinedFunctionArgs {
        /// ID of the Glue Catalog to create the function in. If omitted, this defaults to the AWS Account ID.
        #[builder(into, default)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Java class that contains the function code.
        #[builder(into)]
        pub class_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Database to create the Function.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the function.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The owner of the function.
        #[builder(into)]
        pub owner_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The owner type. can be one of `USER`, `ROLE`, and `GROUP`.
        #[builder(into)]
        pub owner_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The configuration block for Resource URIs. See resource uris below for more details.
        #[builder(into, default)]
        pub resource_uris: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::glue::UserDefinedFunctionResourceUri>>,
        >,
    }
    #[allow(dead_code)]
    pub struct UserDefinedFunctionResult {
        /// The ARN of the Glue User Defined Function.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the Glue Catalog to create the function in. If omitted, this defaults to the AWS Account ID.
        pub catalog_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Java class that contains the function code.
        pub class_name: pulumi_gestalt_rust::Output<String>,
        /// The time at which the function was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The name of the Database to create the Function.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the function.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The owner of the function.
        pub owner_name: pulumi_gestalt_rust::Output<String>,
        /// The owner type. can be one of `USER`, `ROLE`, and `GROUP`.
        pub owner_type: pulumi_gestalt_rust::Output<String>,
        /// The configuration block for Resource URIs. See resource uris below for more details.
        pub resource_uris: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::glue::UserDefinedFunctionResourceUri>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserDefinedFunctionArgs,
    ) -> UserDefinedFunctionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let catalog_id_binding = args.catalog_id.get_output(context);
        let class_name_binding = args.class_name.get_output(context);
        let database_name_binding = args.database_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let owner_name_binding = args.owner_name.get_output(context);
        let owner_type_binding = args.owner_type.get_output(context);
        let resource_uris_binding = args.resource_uris.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/userDefinedFunction:UserDefinedFunction".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "catalogId".into(),
                    value: catalog_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "className".into(),
                    value: class_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: database_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ownerName".into(),
                    value: owner_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ownerType".into(),
                    value: owner_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceUris".into(),
                    value: resource_uris_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserDefinedFunctionResult {
            arn: o.get_field("arn"),
            catalog_id: o.get_field("catalogId"),
            class_name: o.get_field("className"),
            create_time: o.get_field("createTime"),
            database_name: o.get_field("databaseName"),
            name: o.get_field("name"),
            owner_name: o.get_field("ownerName"),
            owner_type: o.get_field("ownerType"),
            resource_uris: o.get_field("resourceUris"),
        }
    }
}
