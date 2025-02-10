#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_runtime_versions {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRuntimeVersionsArgs {
        /// List of runtime versions. See `runtime_versions` attribute reference.
        #[builder(into, default)]
        pub runtime_versions: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::synthetics::GetRuntimeVersionsRuntimeVersion,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRuntimeVersionsResult {
        /// Name of the AWS region from which runtime versions are fetched.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of runtime versions. See `runtime_versions` attribute reference.
        pub runtime_versions: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::synthetics::GetRuntimeVersionsRuntimeVersion,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRuntimeVersionsArgs,
    ) -> GetRuntimeVersionsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let runtime_versions_binding = args.runtime_versions.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:synthetics/getRuntimeVersions:getRuntimeVersions".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtimeVersions".into(),
                    value: runtime_versions_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRuntimeVersionsResult {
            id: o.get_field("id"),
            runtime_versions: o.get_field("runtimeVersions"),
        }
    }
}
