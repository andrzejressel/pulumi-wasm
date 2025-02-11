#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_layer_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLayerVersionArgs {
        /// Specific architecture the layer version could support. Conflicts with `version`. If specified, the latest available layer version supporting the provided architecture will be used.
        #[builder(into, default)]
        pub compatible_architecture: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specific runtime the layer version must support. Conflicts with `version`. If specified, the latest available layer version supporting the provided runtime will be used.
        #[builder(into, default)]
        pub compatible_runtime: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the lambda layer.
        #[builder(into)]
        pub layer_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specific layer version. Conflicts with `compatible_runtime` and `compatible_architecture`. If omitted, the latest available layer version will be used.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GetLayerVersionResult {
        /// ARN of the Lambda Layer with version.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Base64-encoded representation of raw SHA-256 sum of the zip file.
        pub code_sha256: pulumi_gestalt_rust::Output<String>,
        pub compatible_architecture: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of [Architectures](https://docs.aws.amazon.com/lambda/latest/dg/API_GetLayerVersion.html#SSS-GetLayerVersion-response-CompatibleArchitectures) the specific Lambda Layer version is compatible with.
        pub compatible_architectures: pulumi_gestalt_rust::Output<Vec<String>>,
        pub compatible_runtime: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of [Runtimes](https://docs.aws.amazon.com/lambda/latest/dg/API_GetLayerVersion.html#SSS-GetLayerVersion-response-CompatibleRuntimes) the specific Lambda Layer version is compatible with.
        pub compatible_runtimes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Date this resource was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Description of the specific Lambda Layer version.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Lambda Layer without version.
        pub layer_arn: pulumi_gestalt_rust::Output<String>,
        pub layer_name: pulumi_gestalt_rust::Output<String>,
        /// License info associated with the specific Lambda Layer version.
        pub license_info: pulumi_gestalt_rust::Output<String>,
        /// ARN of a signing job.
        pub signing_job_arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN for a signing profile version.
        pub signing_profile_version_arn: pulumi_gestalt_rust::Output<String>,
        /// (**Deprecated** use `code_sha256` instead) Base64-encoded representation of raw SHA-256 sum of the zip file.
        pub source_code_hash: pulumi_gestalt_rust::Output<String>,
        /// Size in bytes of the function .zip file.
        pub source_code_size: pulumi_gestalt_rust::Output<i32>,
        /// This Lambda Layer version.
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLayerVersionArgs,
    ) -> GetLayerVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let compatible_architecture_binding = args
            .compatible_architecture
            .get_output(context);
        let compatible_runtime_binding = args.compatible_runtime.get_output(context);
        let layer_name_binding = args.layer_name.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lambda/getLayerVersion:getLayerVersion".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "compatibleArchitecture".into(),
                    value: &compatible_architecture_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "compatibleRuntime".into(),
                    value: &compatible_runtime_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "layerName".into(),
                    value: &layer_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLayerVersionResult {
            arn: o.get_field("arn"),
            code_sha256: o.get_field("codeSha256"),
            compatible_architecture: o.get_field("compatibleArchitecture"),
            compatible_architectures: o.get_field("compatibleArchitectures"),
            compatible_runtime: o.get_field("compatibleRuntime"),
            compatible_runtimes: o.get_field("compatibleRuntimes"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            layer_arn: o.get_field("layerArn"),
            layer_name: o.get_field("layerName"),
            license_info: o.get_field("licenseInfo"),
            signing_job_arn: o.get_field("signingJobArn"),
            signing_profile_version_arn: o.get_field("signingProfileVersionArn"),
            source_code_hash: o.get_field("sourceCodeHash"),
            source_code_size: o.get_field("sourceCodeSize"),
            version: o.get_field("version"),
        }
    }
}
