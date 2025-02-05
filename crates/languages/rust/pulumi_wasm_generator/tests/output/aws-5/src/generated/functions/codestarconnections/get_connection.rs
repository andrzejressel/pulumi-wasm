pub mod get_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectionArgs {
        /// CodeStar Connection ARN.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// CodeStar Connection name.
        ///
        /// > **NOTE:** When both `arn` and `name` are specified, `arn` takes precedence.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of key-value resource tags to associate with the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetConnectionResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// CodeStar Connection status. Possible values are `PENDING`, `AVAILABLE` and `ERROR`.
        pub connection_status: pulumi_wasm_rust::Output<String>,
        /// ARN of the host associated with the connection.
        pub host_arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the CodeStar Connection. The name is unique in the calling AWS account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Name of the external provider where your third-party code repository is configured. Possible values are `Bitbucket`, `GitHub` and `GitLab`. For connections to GitHub Enterprise Server or GitLab Self-Managed instances, you must create an aws.codestarconnections.Host resource and use `host_arn` instead.
        pub provider_type: pulumi_wasm_rust::Output<String>,
        /// Map of key-value resource tags to associate with the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetConnectionArgs,
    ) -> GetConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:codestarconnections/getConnection:getConnection".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetConnectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            connection_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionStatus"),
            ),
            host_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostArn"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            provider_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("providerType"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
