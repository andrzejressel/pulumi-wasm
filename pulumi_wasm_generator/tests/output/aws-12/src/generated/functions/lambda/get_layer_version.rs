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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "codeSha256".into(),
                },
                register_interface::ResultField {
                    name: "compatibleArchitecture".into(),
                },
                register_interface::ResultField {
                    name: "compatibleArchitectures".into(),
                },
                register_interface::ResultField {
                    name: "compatibleRuntime".into(),
                },
                register_interface::ResultField {
                    name: "compatibleRuntimes".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "layerArn".into(),
                },
                register_interface::ResultField {
                    name: "layerName".into(),
                },
                register_interface::ResultField {
                    name: "licenseInfo".into(),
                },
                register_interface::ResultField {
                    name: "signingJobArn".into(),
                },
                register_interface::ResultField {
                    name: "signingProfileVersionArn".into(),
                },
                register_interface::ResultField {
                    name: "sourceCodeHash".into(),
                },
                register_interface::ResultField {
                    name: "sourceCodeSize".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetLayerVersionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            code_sha256: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("codeSha256").unwrap(),
            ),
            compatible_architecture: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compatibleArchitecture").unwrap(),
            ),
            compatible_architectures: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compatibleArchitectures").unwrap(),
            ),
            compatible_runtime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compatibleRuntime").unwrap(),
            ),
            compatible_runtimes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("compatibleRuntimes").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            layer_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("layerArn").unwrap(),
            ),
            layer_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("layerName").unwrap(),
            ),
            license_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseInfo").unwrap(),
            ),
            signing_job_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingJobArn").unwrap(),
            ),
            signing_profile_version_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingProfileVersionArn").unwrap(),
            ),
            source_code_hash: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceCodeHash").unwrap(),
            ),
            source_code_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceCodeSize").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
