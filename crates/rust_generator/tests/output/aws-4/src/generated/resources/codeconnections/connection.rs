/// Resource for managing an AWS CodeConnections Connection.
///
/// > **NOTE:** The `aws.codeconnections.Connection` resource is created in the state `PENDING`. Authentication with the connection provider must be completed in the AWS Console. See the [AWS documentation](https://docs.aws.amazon.com/dtconsole/latest/userguide/connections-update.html) for details.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = connection::create(
///         "example",
///         ConnectionArgs::builder()
///             .name("example-connection")
///             .provider_type("Bitbucket")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeConnections connection using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:codeconnections/connection:Connection test-connection arn:aws:codeconnections:us-west-1:0123456789:connection/79d4d357-a2ee-41e4-b350-2fe39ae59448
/// ```
pub mod connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// The Amazon Resource Name (ARN) of the host associated with the connection. Conflicts with `provider_type`
        #[builder(into, default)]
        pub host_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the connection to be created. The name must be unique in the calling AWS account. Changing `name` will create a new resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the external provider where your third-party code repository is configured. Changing `provider_type` will create a new resource. Conflicts with `host_arn`.
        #[builder(into, default)]
        pub provider_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of key-value resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::codeconnections::ConnectionTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// The codeconnections connection ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The codeconnections connection status. Possible values are `PENDING`, `AVAILABLE` and `ERROR`.
        pub connection_status: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the host associated with the connection. Conflicts with `provider_type`
        pub host_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the connection to be created. The name must be unique in the calling AWS account. Changing `name` will create a new resource.
        pub name: pulumi_wasm_rust::Output<String>,
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
        /// The name of the external provider where your third-party code repository is configured. Changing `provider_type` will create a new resource. Conflicts with `host_arn`.
        pub provider_type: pulumi_wasm_rust::Output<String>,
        /// Map of key-value resource tags to associate with the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::codeconnections::ConnectionTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ConnectionArgs,
    ) -> ConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let host_arn_binding = args.host_arn.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let provider_type_binding = args.provider_type.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codeconnections/connection:Connection".into(),
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
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ConnectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            connection_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionStatus"),
            ),
            host_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostArn"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            owner_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerAccountId"),
            ),
            provider_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("providerType"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
