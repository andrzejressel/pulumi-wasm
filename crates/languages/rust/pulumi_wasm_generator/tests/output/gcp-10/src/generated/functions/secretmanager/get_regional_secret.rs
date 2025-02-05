pub mod get_regional_secret {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionalSecretArgs {
        /// The location of the regional secret. eg us-central1
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the regional secret.
        #[builder(into)]
        pub secret_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRegionalSecretResult {
        pub annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub customer_managed_encryptions: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::secretmanager::GetRegionalSecretCustomerManagedEncryption,
            >,
        >,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub expire_time: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub rotations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::secretmanager::GetRegionalSecretRotation>,
        >,
        pub secret_id: pulumi_wasm_rust::Output<String>,
        pub topics: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::secretmanager::GetRegionalSecretTopic>,
        >,
        pub ttl: pulumi_wasm_rust::Output<String>,
        pub version_aliases: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub version_destroy_ttl: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRegionalSecretArgs,
    ) -> GetRegionalSecretResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let secret_id_binding = args.secret_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:secretmanager/getRegionalSecret:getRegionalSecret".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "secretId".into(),
                    value: &secret_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRegionalSecretResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            customer_managed_encryptions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerManagedEncryptions"),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            expire_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expireTime"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            rotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rotations"),
            ),
            secret_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secretId"),
            ),
            topics: pulumi_wasm_rust::__private::into_domain(o.extract_field("topics")),
            ttl: pulumi_wasm_rust::__private::into_domain(o.extract_field("ttl")),
            version_aliases: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionAliases"),
            ),
            version_destroy_ttl: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionDestroyTtl"),
            ),
        }
    }
}
