/// Manages a Microsoft Azure Blob Storage Location within AWS DataSync.
///
/// > **NOTE:** The DataSync Agents must be available before creating this resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = location_azure_blob::create(
///         "example",
///         LocationAzureBlobArgs::builder()
///             .agent_arns(vec!["${exampleAwsDatasyncAgent.arn}",])
///             .authentication_type("SAS")
///             .container_url("https://myaccount.blob.core.windows.net/mycontainer")
///             .sas_configuration(
///                 LocationAzureBlobSasConfiguration::builder()
///                     .token(
///                         "sp=r&st=2023-12-20T14:54:52Z&se=2023-12-20T22:54:52Z&spr=https&sv=2021-06-08&sr=c&sig=aBBKDWQvyuVcTPH9EBp%2FXTI9E%2F%2Fmq171%2BZU178wcwqU%3D",
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_datasync_location_azure_blob` using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:datasync/locationAzureBlob:LocationAzureBlob example arn:aws:datasync:us-east-1:123456789012:location/loc-12345678901234567
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod location_azure_blob {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocationAzureBlobArgs {
        /// The access tier that you want your objects or files transferred into. Valid values: `HOT`, `COOL` and `ARCHIVE`. Default: `HOT`.
        #[builder(into, default)]
        pub access_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of DataSync Agent ARNs with which this location will be associated.
        #[builder(into)]
        pub agent_arns: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The authentication method DataSync uses to access your Azure Blob Storage. Valid values: `SAS`.
        #[builder(into)]
        pub authentication_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of blob that you want your objects or files to be when transferring them into Azure Blob Storage. Valid values: `BLOB`. Default: `BLOB`.
        #[builder(into, default)]
        pub blob_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL of the Azure Blob Storage container involved in your transfer.
        #[builder(into)]
        pub container_url: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SAS configuration that allows DataSync to access your Azure Blob Storage. See configuration below.
        #[builder(into, default)]
        pub sas_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datasync::LocationAzureBlobSasConfiguration>,
        >,
        /// Path segments if you want to limit your transfer to a virtual directory in the container.
        #[builder(into, default)]
        pub subdirectory: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LocationAzureBlobResult {
        /// The access tier that you want your objects or files transferred into. Valid values: `HOT`, `COOL` and `ARCHIVE`. Default: `HOT`.
        pub access_tier: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of DataSync Agent ARNs with which this location will be associated.
        pub agent_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Amazon Resource Name (ARN) of the DataSync Location.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The authentication method DataSync uses to access your Azure Blob Storage. Valid values: `SAS`.
        pub authentication_type: pulumi_gestalt_rust::Output<String>,
        /// The type of blob that you want your objects or files to be when transferring them into Azure Blob Storage. Valid values: `BLOB`. Default: `BLOB`.
        pub blob_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URL of the Azure Blob Storage container involved in your transfer.
        pub container_url: pulumi_gestalt_rust::Output<String>,
        /// The SAS configuration that allows DataSync to access your Azure Blob Storage. See configuration below.
        pub sas_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::datasync::LocationAzureBlobSasConfiguration>,
        >,
        /// Path segments if you want to limit your transfer to a virtual directory in the container.
        pub subdirectory: pulumi_gestalt_rust::Output<String>,
        /// Key-value pairs of resource tags to assign to the DataSync Location. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub uri: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LocationAzureBlobArgs,
    ) -> LocationAzureBlobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_tier_binding = args.access_tier.get_output(context);
        let agent_arns_binding = args.agent_arns.get_output(context);
        let authentication_type_binding = args.authentication_type.get_output(context);
        let blob_type_binding = args.blob_type.get_output(context);
        let container_url_binding = args.container_url.get_output(context);
        let sas_configuration_binding = args.sas_configuration.get_output(context);
        let subdirectory_binding = args.subdirectory.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datasync/locationAzureBlob:LocationAzureBlob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessTier".into(),
                    value: access_tier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agentArns".into(),
                    value: agent_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationType".into(),
                    value: authentication_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blobType".into(),
                    value: blob_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerUrl".into(),
                    value: container_url_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sasConfiguration".into(),
                    value: sas_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subdirectory".into(),
                    value: subdirectory_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LocationAzureBlobResult {
            access_tier: o.get_field("accessTier"),
            agent_arns: o.get_field("agentArns"),
            arn: o.get_field("arn"),
            authentication_type: o.get_field("authenticationType"),
            blob_type: o.get_field("blobType"),
            container_url: o.get_field("containerUrl"),
            sas_configuration: o.get_field("sasConfiguration"),
            subdirectory: o.get_field("subdirectory"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            uri: o.get_field("uri"),
        }
    }
}
