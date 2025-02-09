/// <!-- Bug: Type and Name are switched -->
/// Manages the lifecycle of docker image in a registry. You can upload images to a registry (= `docker push`) and also delete them again
///
/// ## Example Usage
///
/// Build an image with the `docker.RemoteImage` resource and then push it to a registry:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let helloworld = registry_image::create(
///         "helloworld",
///         RegistryImageArgs::builder().keep_remotely(true).build_struct(),
///     );
///     let image = remote_image::create(
///         "image",
///         RemoteImageArgs::builder()
///             .build(
///                 RemoteImageBuild::builder()
///                     .context("${path.cwd}/absolutePathToContextFolder")
///                     .build_struct(),
///             )
///             .name("registry.com/somename:1.0")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod registry_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryImageArgs {
        /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
        #[builder(into, default)]
        pub insecure_skip_verify: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`
        #[builder(into, default)]
        pub keep_remotely: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Docker image.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of arbitrary strings that, when changed, will force the `docker.RegistryImage` resource to be replaced. This can be used to repush a local image
        #[builder(into, default)]
        pub triggers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegistryImageResult {
        /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
        pub insecure_skip_verify: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`
        pub keep_remotely: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Docker image.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The sha256 digest of the image.
        pub sha256_digest: pulumi_gestalt_rust::Output<String>,
        /// A map of arbitrary strings that, when changed, will force the `docker.RegistryImage` resource to be replaced. This can be used to repush a local image
        pub triggers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegistryImageArgs,
    ) -> RegistryImageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let insecure_skip_verify_binding = args.insecure_skip_verify.get_output(context);
        let keep_remotely_binding = args.keep_remotely.get_output(context);
        let name_binding = args.name.get_output(context);
        let triggers_binding = args.triggers.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "docker:index/registryImage:RegistryImage".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "insecureSkipVerify".into(),
                    value: insecure_skip_verify_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keepRemotely".into(),
                    value: keep_remotely_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggers".into(),
                    value: triggers_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegistryImageResult {
            insecure_skip_verify: o.get_field("insecureSkipVerify"),
            keep_remotely: o.get_field("keepRemotely"),
            name: o.get_field("name"),
            sha256_digest: o.get_field("sha256Digest"),
            triggers: o.get_field("triggers"),
        }
    }
}
