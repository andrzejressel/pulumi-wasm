pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The location of the instance. eg us-central1
        ///
        /// - - -
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Cloud Run v2 Service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        pub annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub binary_authorizations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceBinaryAuthorization>,
        >,
        pub client: pulumi_wasm_rust::Output<String>,
        pub client_version: pulumi_wasm_rust::Output<String>,
        pub conditions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceCondition>,
        >,
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub creator: pulumi_wasm_rust::Output<String>,
        pub custom_audiences: pulumi_wasm_rust::Output<Vec<String>>,
        pub default_uri_disabled: pulumi_wasm_rust::Output<bool>,
        pub delete_time: pulumi_wasm_rust::Output<String>,
        pub deletion_protection: pulumi_wasm_rust::Output<bool>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub etag: pulumi_wasm_rust::Output<String>,
        pub expire_time: pulumi_wasm_rust::Output<String>,
        pub generation: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ingress: pulumi_wasm_rust::Output<String>,
        pub invoker_iam_disabled: pulumi_wasm_rust::Output<bool>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub last_modifier: pulumi_wasm_rust::Output<String>,
        pub latest_created_revision: pulumi_wasm_rust::Output<String>,
        pub latest_ready_revision: pulumi_wasm_rust::Output<String>,
        pub launch_stage: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub observed_generation: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        pub scalings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceScaling>,
        >,
        pub templates: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceTemplate>,
        >,
        pub terminal_conditions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceTerminalCondition>,
        >,
        pub traffic_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceTrafficStatus>,
        >,
        pub traffics: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceTraffic>,
        >,
        pub uid: pulumi_wasm_rust::Output<String>,
        pub update_time: pulumi_wasm_rust::Output<String>,
        pub uri: pulumi_wasm_rust::Output<String>,
        pub urls: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:cloudrunv2/getService:getService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServiceResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            binary_authorizations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("binaryAuthorizations"),
            ),
            client: pulumi_wasm_rust::__private::into_domain(o.extract_field("client")),
            client_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientVersion"),
            ),
            conditions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("conditions"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            creator: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creator"),
            ),
            custom_audiences: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customAudiences"),
            ),
            default_uri_disabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultUriDisabled"),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deleteTime"),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            expire_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expireTime"),
            ),
            generation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("generation"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ingress: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ingress"),
            ),
            invoker_iam_disabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("invokerIamDisabled"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            last_modifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastModifier"),
            ),
            latest_created_revision: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("latestCreatedRevision"),
            ),
            latest_ready_revision: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("latestReadyRevision"),
            ),
            launch_stage: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("launchStage"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            observed_generation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("observedGeneration"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reconciling"),
            ),
            scalings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scalings"),
            ),
            templates: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("templates"),
            ),
            terminal_conditions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("terminalConditions"),
            ),
            traffic_statuses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trafficStatuses"),
            ),
            traffics: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("traffics"),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            uri: pulumi_wasm_rust::__private::into_domain(o.extract_field("uri")),
            urls: pulumi_wasm_rust::__private::into_domain(o.extract_field("urls")),
        }
    }
}
