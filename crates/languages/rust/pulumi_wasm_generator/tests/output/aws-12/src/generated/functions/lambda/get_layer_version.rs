pub mod get_layer_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLayerVersionArgs {
        /// Specific architecture the layer version could support. Conflicts with `version`. If specified, the latest available layer version supporting the provided architecture will be used.
        #[builder(into, default)]
        pub compatible_architecture: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specific runtime the layer version must support. Conflicts with `version`. If specified, the latest available layer version supporting the provided runtime will be used.
        #[builder(into, default)]
        pub compatible_runtime: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the lambda layer.
        #[builder(into)]
        pub layer_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specific layer version. Conflicts with `compatible_runtime` and `compatible_architecture`. If omitted, the latest available layer version will be used.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct GetLayerVersionResult {
        /// ARN of the Lambda Layer with version.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Base64-encoded representation of raw SHA-256 sum of the zip file.
        pub code_sha256: pulumi_wasm_rust::Output<String>,
        pub compatible_architecture: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of [Architectures](https://docs.aws.amazon.com/lambda/latest/dg/API_GetLayerVersion.html#SSS-GetLayerVersion-response-CompatibleArchitectures) the specific Lambda Layer version is compatible with.
        pub compatible_architectures: pulumi_wasm_rust::Output<Vec<String>>,
        pub compatible_runtime: pulumi_wasm_rust::Output<Option<String>>,
        /// List of [Runtimes](https://docs.aws.amazon.com/lambda/latest/dg/API_GetLayerVersion.html#SSS-GetLayerVersion-response-CompatibleRuntimes) the specific Lambda Layer version is compatible with.
        pub compatible_runtimes: pulumi_wasm_rust::Output<Vec<String>>,
        /// Date this resource was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Description of the specific Lambda Layer version.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN of the Lambda Layer without version.
        pub layer_arn: pulumi_wasm_rust::Output<String>,
        pub layer_name: pulumi_wasm_rust::Output<String>,
        /// License info associated with the specific Lambda Layer version.
        pub license_info: pulumi_wasm_rust::Output<String>,
        /// ARN of a signing job.
        pub signing_job_arn: pulumi_wasm_rust::Output<String>,
        /// The ARN for a signing profile version.
        pub signing_profile_version_arn: pulumi_wasm_rust::Output<String>,
        /// (**Deprecated** use `code_sha256` instead) Base64-encoded representation of raw SHA-256 sum of the zip file.
        pub source_code_hash: pulumi_wasm_rust::Output<String>,
        /// Size in bytes of the function .zip file.
        pub source_code_size: pulumi_wasm_rust::Output<i32>,
        /// This Lambda Layer version.
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetLayerVersionArgs,
    ) -> GetLayerVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let compatible_architecture_binding = args
            .compatible_architecture
            .get_output(context)
            .get_inner();
        let compatible_runtime_binding = args
            .compatible_runtime
            .get_output(context)
            .get_inner();
        let layer_name_binding = args.layer_name.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lambda/getLayerVersion:getLayerVersion".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "compatibleArchitecture".into(),
                    value: &compatible_architecture_binding,
                },
                register_interface::ObjectField {
                    name: "compatibleRuntime".into(),
                    value: &compatible_runtime_binding,
                },
                register_interface::ObjectField {
                    name: "layerName".into(),
                    value: &layer_name_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetLayerVersionResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            code_sha256: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("codeSha256"),
            ),
            compatible_architecture: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("compatibleArchitecture"),
            ),
            compatible_architectures: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("compatibleArchitectures"),
            ),
            compatible_runtime: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("compatibleRuntime"),
            ),
            compatible_runtimes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("compatibleRuntimes"),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            layer_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("layerArn"),
            ),
            layer_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("layerName"),
            ),
            license_info: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("licenseInfo"),
            ),
            signing_job_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("signingJobArn"),
            ),
            signing_profile_version_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("signingProfileVersionArn"),
            ),
            source_code_hash: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceCodeHash"),
            ),
            source_code_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceCodeSize"),
            ),
            version: pulumi_wasm_rust::__private::into_domain(o.extract_field("version")),
        }
    }
}
