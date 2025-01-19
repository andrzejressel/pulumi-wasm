pub mod get_runtime_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRuntimeVersionArgs {
        /// Whether the latest version of the runtime should be fetched. Conflicts with `version`. Valid values: `true`.
        #[builder(into, default)]
        pub latest: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name prefix of the runtime version (for example, `syn-nodejs-puppeteer`).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub prefix: pulumi_wasm_rust::Output<String>,
        /// Version of the runtime to be fetched (for example, `9.0`). Conflicts with `latest`.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
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
    pub fn invoke(args: GetRuntimeVersionArgs) -> GetRuntimeVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let latest_binding = args.latest.get_inner();
        let prefix_binding = args.prefix.get_inner();
        let version_binding = args.version.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "deprecationDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "latest".into(),
                },
                register_interface::ResultField {
                    name: "prefix".into(),
                },
                register_interface::ResultField {
                    name: "releaseDate".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "versionName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRuntimeVersionResult {
            deprecation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deprecationDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            latest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latest").unwrap(),
            ),
            prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefix").unwrap(),
            ),
            release_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseDate").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            version_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionName").unwrap(),
            ),
        }
    }
}
