/// Provides a CodeStar Connection.
///
/// > **NOTE:** The `aws.codestarconnections.Connection` resource is created in the state `PENDING`. Authentication with the connection provider must be completed in the AWS Console. See the [AWS documentation](https://docs.aws.amazon.com/dtconsole/latest/userguide/connections-update.html) for details.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:codestarconnections:Connection
///     properties:
///       name: example-connection
///       providerType: Bitbucket
///   examplePipeline:
///     type: aws:codepipeline:Pipeline
///     name: example
///     properties:
///       artifactStores:
///         - {}
///       stages:
///         - name: Source
///           actions:
///             - name: Source
///               category: Source
///               owner: AWS
///               provider: CodeStarSourceConnection
///               version: '1'
///               outputArtifacts:
///                 - source_output
///               configuration:
///                 ConnectionArn: ${example.arn}
///                 FullRepositoryId: my-organization/test
///                 BranchName: main
///         - actions:
///             - {}
///           name: Build
///         - actions:
///             - {}
///           name: Deploy
///       name: tf-test-pipeline
///       roleArn: ${codepipelineRole.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeStar connections using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:codestarconnections/connection:Connection test-connection arn:aws:codestar-connections:us-west-1:0123456789:connection/79d4d357-a2ee-41e4-b350-2fe39ae59448
/// ```
pub mod connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// The Amazon Resource Name (ARN) of the host associated with the connection. Conflicts with `provider_type`
        #[builder(into, default)]
        pub host_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the connection to be created. The name must be unique in the calling AWS account. Changing `name` will create a new resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the external provider where your third-party code repository is configured. Valid values are `Bitbucket`, `GitHub`, `GitHubEnterpriseServer`, `GitLab` or `GitLabSelfManaged`. Changing `provider_type` will create a new resource. Conflicts with `host_arn`
        #[builder(into, default)]
        pub provider_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of key-value resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// The codestar connection ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The codestar connection status. Possible values are `PENDING`, `AVAILABLE` and `ERROR`.
        pub connection_status: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the host associated with the connection. Conflicts with `provider_type`
        pub host_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the connection to be created. The name must be unique in the calling AWS account. Changing `name` will create a new resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the external provider where your third-party code repository is configured. Valid values are `Bitbucket`, `GitHub`, `GitHubEnterpriseServer`, `GitLab` or `GitLabSelfManaged`. Changing `provider_type` will create a new resource. Conflicts with `host_arn`
        pub provider_type: pulumi_wasm_rust::Output<String>,
        /// Map of key-value resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectionArgs) -> ConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let host_arn_binding = args.host_arn.get_inner();
        let name_binding = args.name.get_inner();
        let provider_type_binding = args.provider_type.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codestarconnections/connection:Connection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hostArn".into(),
                    value: &host_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "providerType".into(),
                    value: &provider_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "connectionStatus".into(),
                },
                register_interface::ResultField {
                    name: "hostArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "providerType".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            connection_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionStatus").unwrap(),
            ),
            host_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            provider_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerType").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
