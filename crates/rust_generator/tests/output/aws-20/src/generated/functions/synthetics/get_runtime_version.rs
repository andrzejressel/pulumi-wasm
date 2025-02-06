pub mod get_runtime_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRuntimeVersionArgs {
        /// Whether the latest version of the runtime should be fetched. Conflicts with `version`. Valid values: `true`.
        #[builder(into, default)]
        pub latest: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Name prefix of the runtime version (for example, `syn-nodejs-puppeteer`).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub prefix: pulumi_wasm_rust::InputOrOutput<String>,
        /// Version of the runtime to be fetched (for example, `9.0`). Conflicts with `latest`.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRuntimeVersionResult {
        /// Date of deprecation if the runtme version is deprecated.
        pub deprecation_date: pulumi_wasm_rust::Output<String>,
        /// Description of the runtime version, created by Amazon.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Name of the runtime version. For a list of valid runtime versions, see [Canary Runtime Versions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html).
        pub id: pulumi_wasm_rust::Output<String>,
        pub latest: pulumi_wasm_rust::Output<Option<bool>>,
        pub prefix: pulumi_wasm_rust::Output<String>,
        /// Date that the runtime version was released.
        pub release_date: pulumi_wasm_rust::Output<String>,
        pub version: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the runtime version. For a list of valid runtime versions, see [Canary Runtime Versions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html).
        pub version_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRuntimeVersionArgs,
    ) -> GetRuntimeVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let latest_binding = args.latest.get_output(context).get_inner();
        let prefix_binding = args.prefix.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:synthetics/getRuntimeVersion:getRuntimeVersion".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "latest".into(),
                    value: &latest_binding,
                },
                register_interface::ObjectField {
                    name: "prefix".into(),
                    value: &prefix_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRuntimeVersionResult {
            deprecation_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deprecationDate"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            latest: pulumi_wasm_rust::__private::into_domain(o.extract_field("latest")),
            prefix: pulumi_wasm_rust::__private::into_domain(o.extract_field("prefix")),
            release_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("releaseDate"),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            version_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionName"),
            ),
        }
    }
}
