#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RemoteImageArgs {
    /// Configuration to build an image. Please see [docker build command reference](https://docs.docker.com/engine/reference/commandline/build/#options) too.
    #[builder(into, default)]
    pub build: pulumi_wasm_rust::Output<Option<super::types::RemoteImageBuild>>,
    /// Always remove intermediate containers
    #[builder(into, default)]
    pub force_remove: pulumi_wasm_rust::Output<Option<bool>>,
    /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker local storage on destroy operation.
    #[builder(into, default)]
    pub keep_locally: pulumi_wasm_rust::Output<Option<bool>>,
    /// type of ulimit, e.g. `nofile`
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Set platform if server is multi-platform capable
    #[builder(into, default)]
    pub platform: pulumi_wasm_rust::Output<Option<String>>,
    /// List of values which cause an image pull when changed. This is used to store the image digest from the registry when using the docker*registry*image.
    #[builder(into, default)]
    pub pull_triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// A map of arbitrary strings that, when changed, will force the `docker.RemoteImage` resource to be replaced. This can be used to rebuild an image when contents of source code folders change
    #[builder(into, default)]
    pub triggers: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
}
pub struct RemoteImageResult {
    /// Configuration to build an image. Please see [docker build command reference](https://docs.docker.com/engine/reference/commandline/build/#options) too.
    pub build: pulumi_wasm_rust::Output<Option<super::types::RemoteImageBuild>>,
    /// Always remove intermediate containers
    pub force_remove: pulumi_wasm_rust::Output<Option<bool>>,
    /// The ID of the image (as seen when executing `docker inspect` on the image). Can be used to reference the image via its ID in other resources.
    pub image_id: pulumi_wasm_rust::Output<String>,
    /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker local storage on destroy operation.
    pub keep_locally: pulumi_wasm_rust::Output<Option<bool>>,
    /// type of ulimit, e.g. `nofile`
    pub name: pulumi_wasm_rust::Output<String>,
    /// Set platform if server is multi-platform capable
    pub platform: pulumi_wasm_rust::Output<Option<String>>,
    /// List of values which cause an image pull when changed. This is used to store the image digest from the registry when using the docker*registry*image.
    pub pull_triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The image sha256 digest in the form of `repo[:tag]@sha256:<hash>`.
    pub repo_digest: pulumi_wasm_rust::Output<String>,
    /// A map of arbitrary strings that, when changed, will force the `docker.RemoteImage` resource to be replaced. This can be used to rebuild an image when contents of source code folders change
    pub triggers: pulumi_wasm_rust::Output<
        Option<std::collections::HashMap<String, String>>,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: RemoteImageArgs) -> RemoteImageResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let build_binding = args.build.get_inner();
    let force_remove_binding = args.force_remove.get_inner();
    let keep_locally_binding = args.keep_locally.get_inner();
    let name_binding = args.name.get_inner();
    let platform_binding = args.platform.get_inner();
    let pull_triggers_binding = args.pull_triggers.get_inner();
    let triggers_binding = args.triggers.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "docker:index/remoteImage:RemoteImage".into(),
        name: name.to_string(),
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
        results: vec![
            register_interface::ResultField { name : "build".into() },
            register_interface::ResultField { name : "forceRemove".into() },
            register_interface::ResultField { name : "imageId".into() },
            register_interface::ResultField { name : "keepLocally".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "platform".into() },
            register_interface::ResultField { name : "pullTriggers".into() },
            register_interface::ResultField { name : "repoDigest".into() },
            register_interface::ResultField { name : "triggers".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    RemoteImageResult {
        build: into_domain(hashmap.remove("build").unwrap()),
        force_remove: into_domain(hashmap.remove("forceRemove").unwrap()),
        image_id: into_domain(hashmap.remove("imageId").unwrap()),
        keep_locally: into_domain(hashmap.remove("keepLocally").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        platform: into_domain(hashmap.remove("platform").unwrap()),
        pull_triggers: into_domain(hashmap.remove("pullTriggers").unwrap()),
        repo_digest: into_domain(hashmap.remove("repoDigest").unwrap()),
        triggers: into_domain(hashmap.remove("triggers").unwrap()),
    }
}
