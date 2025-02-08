#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretArgs {
        /// The ID of the project in which the resource belongs.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the secret.
        #[builder(into)]
        pub secret_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSecretResult {
        pub annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub replications: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::secretmanager::GetSecretReplication>,
        >,
        pub rotations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::secretmanager::GetSecretRotation>,
        >,
        pub secret_id: pulumi_gestalt_rust::Output<String>,
        pub topics: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::secretmanager::GetSecretTopic>,
        >,
        pub ttl: pulumi_gestalt_rust::Output<String>,
        pub version_aliases: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub version_destroy_ttl: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSecretArgs,
    ) -> GetSecretResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_output(context).get_inner();
        let secret_id_binding = args.secret_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:secretmanager/getSecret:getSecret".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
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
        GetSecretResult {
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            effective_annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            expire_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expireTime"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            replications: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replications"),
            ),
            rotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rotations"),
            ),
            secret_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretId"),
            ),
            topics: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("topics"),
            ),
            ttl: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ttl")),
            version_aliases: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionAliases"),
            ),
            version_destroy_ttl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionDestroyTtl"),
            ),
        }
    }
}
