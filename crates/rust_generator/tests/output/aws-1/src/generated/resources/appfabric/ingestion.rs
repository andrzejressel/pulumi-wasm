/// Resource for managing an AWS AppFabric Ingestion.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appfabric:Ingestion
///     properties:
///       app: OKTA
///       appBundleArn: ${exampleAwsAppfabricAppBundle.arn}
///       tenantId: example.okta.com
///       ingestionType: auditLog
///       tags:
///         Environment: test
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppFabric Ingestion using the `app_bundle_identifier` and `arn` separated by `,`. For example:
///
/// ```sh
/// $ pulumi import aws:appfabric/ingestion:Ingestion example arn:aws:appfabric:[region]:[account]:appbundle/a9b91477-8831-43c0-970c-xxxxxxxxxx,arn:aws:appfabric:[region]:[account]:appbundle/a9b91477-8831-43c0-970c-xxxxxxxxxx/ingestion/32251416-710b-4425-96ca-xxxxxxxxxx
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ingestion {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IngestionArgs {
        /// Name of the application.
        /// Refer to the AWS Documentation for the [list of valid values](https://docs.aws.amazon.com/appfabric/latest/api/API_CreateIngestion.html#appfabric-CreateIngestion-request-app)
        #[builder(into)]
        pub app: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Amazon Resource Name (ARN) of the app bundle to use for the request.
        #[builder(into)]
        pub app_bundle_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Ingestion type. Valid values are `auditLog`.
        #[builder(into)]
        pub ingestion_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the application tenant.
        #[builder(into)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IngestionResult {
        /// Name of the application.
        /// Refer to the AWS Documentation for the [list of valid values](https://docs.aws.amazon.com/appfabric/latest/api/API_CreateIngestion.html#appfabric-CreateIngestion-request-app)
        pub app: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the app bundle to use for the request.
        pub app_bundle_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Ingestion.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Ingestion type. Valid values are `auditLog`.
        pub ingestion_type: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ID of the application tenant.
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IngestionArgs,
    ) -> IngestionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_binding = args.app.get_output(context);
        let app_bundle_arn_binding = args.app_bundle_arn.get_output(context);
        let ingestion_type_binding = args.ingestion_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tenant_id_binding = args.tenant_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appfabric/ingestion:Ingestion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "app".into(),
                    value: app_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appBundleArn".into(),
                    value: app_bundle_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ingestionType".into(),
                    value: ingestion_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantId".into(),
                    value: tenant_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IngestionResult {
            app: o.get_field("app"),
            app_bundle_arn: o.get_field("appBundleArn"),
            arn: o.get_field("arn"),
            ingestion_type: o.get_field("ingestionType"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tenant_id: o.get_field("tenantId"),
        }
    }
}
