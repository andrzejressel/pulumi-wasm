#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_registry_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegistryImageArgs {
        /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
        #[builder(into, default)]
        pub insecure_skip_verify: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Docker image, including any tags. e.g. `alpine:latest`
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRegistryImageResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
        pub insecure_skip_verify: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Docker image, including any tags. e.g. `alpine:latest`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The content digest of the image, as stored in the registry.
        pub sha256_digest: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRegistryImageArgs,
    ) -> GetRegistryImageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let insecure_skip_verify_binding = args.insecure_skip_verify.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "docker:index/getRegistryImage:getRegistryImage".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "insecureSkipVerify".into(),
                    value: insecure_skip_verify_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRegistryImageResult {
            id: o.get_field("id"),
            insecure_skip_verify: o.get_field("insecureSkipVerify"),
            name: o.get_field("name"),
            sha256_digest: o.get_field("sha256Digest"),
        }
    }
}
