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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRuntimeVersionsArgs,
    ) -> GetRuntimeVersionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let runtime_versions_binding = args
            .runtime_versions
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:synthetics/getRuntimeVersions:getRuntimeVersions".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "runtimeVersions".into(),
                    value: &runtime_versions_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRuntimeVersionsResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            runtime_versions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runtimeVersions"),
            ),
        }
    }
}
