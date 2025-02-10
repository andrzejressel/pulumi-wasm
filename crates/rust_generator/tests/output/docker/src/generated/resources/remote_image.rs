/// <!-- Bug: Type and Name are switched -->
/// Pulls a Docker image to a given Docker host from a Docker Registry.
///  This resource will *not* pull new layers of the image automatically unless used in conjunction with docker.RegistryImage data source to update the `pull_triggers` field.
///
/// ## Example Usage
///
/// ### Basic
///
/// Finds and downloads the latest `ubuntu:precise` image but does not check
/// for further updates of the image
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ubuntu = remote_image::create(
///         "ubuntu",
///         RemoteImageArgs::builder().name("ubuntu:precise").build_struct(),
///     );
/// }
/// ```
///
/// ### Dynamic updates
///
/// To be able to update an image dynamically when the `sha256` sum changes,
/// you need to use it in combination with `docker.RegistryImage` as follows:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ubuntuRegistryImage = get_registry_image::invoke(
///         GetRegistryImageArgs::builder().name("ubuntu:precise").build_struct(),
///     );
///     let ubuntuRemoteImage = remote_image::create(
///         "ubuntuRemoteImage",
///         RemoteImageArgs::builder()
///             .name("${ubuntuRegistryImage.name}")
///             .pull_triggers(vec!["${ubuntuRegistryImage.sha256Digest}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Build
///
/// You can also use the resource to build an image.
/// In this case the image "zoo" and "zoo:develop" are built.
///
/// ```yaml
/// resources:
///   zoo:
///     type: docker:RemoteImage
///     properties:
///       name: zoo
///       build:
///         context: .
///         tags:
///           - zoo:develop
///         buildArg:
///           foo: zoo
///         label:
///           author: zoo
/// ```
///
/// You can use the `triggers` argument to specify when the image should be rebuild. This is for example helpful when you want to rebuild the docker image whenever the source code changes.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod remote_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RemoteImageArgs {
        /// Configuration to build an image. Please see [docker build command reference](https://docs.docker.com/engine/reference/commandline/build/#options) too.
        #[builder(into, default)]
        pub build: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::RemoteImageBuild>,
        >,
        /// Always remove intermediate containers
        #[builder(into, default)]
        pub force_remove: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker local storage on destroy operation.
        #[builder(into, default)]
        pub keep_locally: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// type of ulimit, e.g. `nofile`
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Set platform if server is multi-platform capable
        #[builder(into, default)]
        pub platform: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of values which cause an image pull when changed. This is used to store the image digest from the registry when using the docker*registry*image.
        #[builder(into, default)]
        pub pull_triggers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of arbitrary strings that, when changed, will force the `docker.RemoteImage` resource to be replaced. This can be used to rebuild an image when contents of source code folders change
        #[builder(into, default)]
        pub triggers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RemoteImageResult {
        /// Configuration to build an image. Please see [docker build command reference](https://docs.docker.com/engine/reference/commandline/build/#options) too.
        pub build: pulumi_gestalt_rust::Output<Option<super::types::RemoteImageBuild>>,
        /// Always remove intermediate containers
        pub force_remove: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the image (as seen when executing `docker inspect` on the image). Can be used to reference the image via its ID in other resources.
        pub image_id: pulumi_gestalt_rust::Output<String>,
        /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker local storage on destroy operation.
        pub keep_locally: pulumi_gestalt_rust::Output<Option<bool>>,
        /// type of ulimit, e.g. `nofile`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Set platform if server is multi-platform capable
        pub platform: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of values which cause an image pull when changed. This is used to store the image digest from the registry when using the docker*registry*image.
        pub pull_triggers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The image sha256 digest in the form of `repo[:tag]@sha256:<hash>`.
        pub repo_digest: pulumi_gestalt_rust::Output<String>,
        /// A map of arbitrary strings that, when changed, will force the `docker.RemoteImage` resource to be replaced. This can be used to rebuild an image when contents of source code folders change
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
        args: RemoteImageArgs,
    ) -> RemoteImageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let build_binding = args.build.get_output(context);
        let force_remove_binding = args.force_remove.get_output(context);
        let keep_locally_binding = args.keep_locally.get_output(context);
        let name_binding = args.name.get_output(context);
        let platform_binding = args.platform.get_output(context);
        let pull_triggers_binding = args.pull_triggers.get_output(context);
        let triggers_binding = args.triggers.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "docker:index/remoteImage:RemoteImage".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "build".into(),
                    value: build_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceRemove".into(),
                    value: force_remove_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keepLocally".into(),
                    value: keep_locally_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "platform".into(),
                    value: platform_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pullTriggers".into(),
                    value: pull_triggers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggers".into(),
                    value: triggers_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RemoteImageResult {
            build: o.get_field("build"),
            force_remove: o.get_field("forceRemove"),
            image_id: o.get_field("imageId"),
            keep_locally: o.get_field("keepLocally"),
            name: o.get_field("name"),
            platform: o.get_field("platform"),
            pull_triggers: o.get_field("pullTriggers"),
            repo_digest: o.get_field("repoDigest"),
            triggers: o.get_field("triggers"),
        }
    }
}
