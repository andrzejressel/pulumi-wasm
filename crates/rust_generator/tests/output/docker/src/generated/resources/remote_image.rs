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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RemoteImageArgs,
    ) -> RemoteImageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let build_binding_1 = args.build.get_output(context);
        let build_binding = build_binding_1.get_inner();
        let force_remove_binding_1 = args.force_remove.get_output(context);
        let force_remove_binding = force_remove_binding_1.get_inner();
        let keep_locally_binding_1 = args.keep_locally.get_output(context);
        let keep_locally_binding = keep_locally_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let platform_binding_1 = args.platform.get_output(context);
        let platform_binding = platform_binding_1.get_inner();
        let pull_triggers_binding_1 = args.pull_triggers.get_output(context);
        let pull_triggers_binding = pull_triggers_binding_1.get_inner();
        let triggers_binding_1 = args.triggers.get_output(context);
        let triggers_binding = triggers_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/remoteImage:RemoteImage".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "build".into(),
                    value: &build_binding,
                },
                register_interface::ObjectField {
                    name: "forceRemove".into(),
                    value: &force_remove_binding,
                },
                register_interface::ObjectField {
                    name: "keepLocally".into(),
                    value: &keep_locally_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "platform".into(),
                    value: &platform_binding,
                },
                register_interface::ObjectField {
                    name: "pullTriggers".into(),
                    value: &pull_triggers_binding,
                },
                register_interface::ObjectField {
                    name: "triggers".into(),
                    value: &triggers_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RemoteImageResult {
            build: pulumi_gestalt_rust::__private::into_domain(o.extract_field("build")),
            force_remove: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceRemove"),
            ),
            image_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageId"),
            ),
            keep_locally: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keepLocally"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            platform: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platform"),
            ),
            pull_triggers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pullTriggers"),
            ),
            repo_digest: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repoDigest"),
            ),
            triggers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("triggers"),
            ),
        }
    }
}
