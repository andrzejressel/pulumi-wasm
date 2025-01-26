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
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "binaryAuthorizations".into(),
                },
                register_interface::ResultField {
                    name: "client".into(),
                },
                register_interface::ResultField {
                    name: "clientVersion".into(),
                },
                register_interface::ResultField {
                    name: "conditions".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "creator".into(),
                },
                register_interface::ResultField {
                    name: "customAudiences".into(),
                },
                register_interface::ResultField {
                    name: "defaultUriDisabled".into(),
                },
                register_interface::ResultField {
                    name: "deleteTime".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "expireTime".into(),
                },
                register_interface::ResultField {
                    name: "generation".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ingress".into(),
                },
                register_interface::ResultField {
                    name: "invokerIamDisabled".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "lastModifier".into(),
                },
                register_interface::ResultField {
                    name: "latestCreatedRevision".into(),
                },
                register_interface::ResultField {
                    name: "latestReadyRevision".into(),
                },
                register_interface::ResultField {
                    name: "launchStage".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "observedGeneration".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "reconciling".into(),
                },
                register_interface::ResultField {
                    name: "scalings".into(),
                },
                register_interface::ResultField {
                    name: "templates".into(),
                },
                register_interface::ResultField {
                    name: "terminalConditions".into(),
                },
                register_interface::ResultField {
                    name: "trafficStatuses".into(),
                },
                register_interface::ResultField {
                    name: "traffics".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "uri".into(),
                },
                register_interface::ResultField {
                    name: "urls".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServiceResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            binary_authorizations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("binaryAuthorizations").unwrap(),
            ),
            client: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("client").unwrap(),
            ),
            client_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientVersion").unwrap(),
            ),
            conditions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("conditions").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            creator: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creator").unwrap(),
            ),
            custom_audiences: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customAudiences").unwrap(),
            ),
            default_uri_disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultUriDisabled").unwrap(),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteTime").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            expire_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expireTime").unwrap(),
            ),
            generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("generation").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ingress: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingress").unwrap(),
            ),
            invoker_iam_disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invokerIamDisabled").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            last_modifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifier").unwrap(),
            ),
            latest_created_revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestCreatedRevision").unwrap(),
            ),
            latest_ready_revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestReadyRevision").unwrap(),
            ),
            launch_stage: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launchStage").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            observed_generation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("observedGeneration").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconciling").unwrap(),
            ),
            scalings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scalings").unwrap(),
            ),
            templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templates").unwrap(),
            ),
            terminal_conditions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terminalConditions").unwrap(),
            ),
            traffic_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficStatuses").unwrap(),
            ),
            traffics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("traffics").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uri").unwrap(),
            ),
            urls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("urls").unwrap(),
            ),
        }
    }
}
