/// Manages an App Runner Connection.
///
/// > **NOTE:** After creation, you must complete the authentication handshake using the App Runner console.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:apprunner:Connection
///     properties:
///       connectionName: example
///       providerType: GITHUB
///       tags:
///         Name: example-apprunner-connection
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Runner Connections using the `connection_name`. For example:
///
/// ```sh
/// $ pulumi import aws:apprunner/connection:Connection example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionArgs {
        /// Name of the connection.
        #[builder(into)]
        pub connection_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Source repository provider. Valid values: `GITHUB`.
        #[builder(into)]
        pub provider_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectionResult {
        /// ARN of the connection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the connection.
        pub connection_name: pulumi_gestalt_rust::Output<String>,
        /// Source repository provider. Valid values: `GITHUB`.
        pub provider_type: pulumi_gestalt_rust::Output<String>,
        /// Current state of the App Runner connection. When the state is `AVAILABLE`, you can use the connection to create an `aws.apprunner.Service` resource.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ConnectionArgs,
    ) -> ConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let connection_name_binding = args
            .connection_name
            .get_output(context)
            .get_inner();
        let provider_type_binding = args.provider_type.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apprunner/connection:Connection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionName".into(),
                    value: &connection_name_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ConnectionResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            connection_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionName"),
            ),
            provider_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("providerType"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
