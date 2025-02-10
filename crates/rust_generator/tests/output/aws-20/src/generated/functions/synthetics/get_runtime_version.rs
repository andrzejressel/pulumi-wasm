#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_runtime_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRuntimeVersionArgs {
        /// Whether the latest version of the runtime should be fetched. Conflicts with `version`. Valid values: `true`.
        #[builder(into, default)]
        pub latest: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name prefix of the runtime version (for example, `syn-nodejs-puppeteer`).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub prefix: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the runtime to be fetched (for example, `9.0`). Conflicts with `latest`.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRuntimeVersionResult {
        /// Date of deprecation if the runtme version is deprecated.
        pub deprecation_date: pulumi_gestalt_rust::Output<String>,
        /// Description of the runtime version, created by Amazon.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Name of the runtime version. For a list of valid runtime versions, see [Canary Runtime Versions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html).
        pub id: pulumi_gestalt_rust::Output<String>,
        pub latest: pulumi_gestalt_rust::Output<Option<bool>>,
        pub prefix: pulumi_gestalt_rust::Output<String>,
        /// Date that the runtime version was released.
        pub release_date: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the runtime version. For a list of valid runtime versions, see [Canary Runtime Versions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html).
        pub version_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRuntimeVersionArgs,
    ) -> GetRuntimeVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let latest_binding = args.latest.get_output(context);
        let prefix_binding = args.prefix.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:synthetics/getRuntimeVersion:getRuntimeVersion".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "latest".into(),
                    value: latest_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefix".into(),
                    value: prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRuntimeVersionResult {
            deprecation_date: o.get_field("deprecationDate"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            latest: o.get_field("latest"),
            prefix: o.get_field("prefix"),
            release_date: o.get_field("releaseDate"),
            version: o.get_field("version"),
            version_name: o.get_field("versionName"),
        }
    }
}
