pub mod get_listener {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetListenerArgs {
        /// ID or Amazon Resource Name (ARN) of the listener
        #[builder(into)]
        pub listener_identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID or Amazon Resource Name (ARN) of the service network
        #[builder(into)]
        pub service_identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// List of tags associated with the listener.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetListenerResult {
        /// ARN of the listener.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The date and time that the listener was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The actions for the default listener rule.
        pub default_actions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vpclattice::GetListenerDefaultAction>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The date and time the listener was last updated.
        pub last_updated_at: pulumi_wasm_rust::Output<String>,
        /// The ID of the listener.
        pub listener_id: pulumi_wasm_rust::Output<String>,
        pub listener_identifier: pulumi_wasm_rust::Output<String>,
        /// The name of the listener.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The listener port.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The listener protocol. Either `HTTPS` or `HTTP`.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// The ARN of the service.
        pub service_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the service.
        pub service_id: pulumi_wasm_rust::Output<String>,
        pub service_identifier: pulumi_wasm_rust::Output<String>,
        /// List of tags associated with the listener.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetListenerArgs,
    ) -> GetListenerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let listener_identifier_binding = args
            .listener_identifier
            .get_output(context)
            .get_inner();
        let service_identifier_binding = args
            .service_identifier
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:vpclattice/getListener:getListener".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "listenerIdentifier".into(),
                    value: &listener_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "serviceIdentifier".into(),
                    value: &service_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetListenerResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            created_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            default_actions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultActions"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            last_updated_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastUpdatedAt"),
            ),
            listener_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("listenerId"),
            ),
            listener_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("listenerIdentifier"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            port: pulumi_wasm_rust::__private::into_domain(o.extract_field("port")),
            protocol: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protocol"),
            ),
            service_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceArn"),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceId"),
            ),
            service_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceIdentifier"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
