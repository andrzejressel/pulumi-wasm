/// Represents a deployment of a security posture on a resource. A posture contains user curated policy sets. A posture can
/// be deployed on a project or on a folder or on an organization. To deploy a posture we need to populate the posture's name
/// and its revision_id in the posture deployment configuration. Every update to a deployed posture generates a new revision_id.
/// Thus, the updated revision_id should be used in the respective posture deployment's configuration to deploy that posture
/// on a resource.
///
///
/// To get more information about PostureDeployment, see:
/// * How-to Guides
///     * [Create and deploy a posture](https://cloud.google.com/security-command-center/docs/how-to-use-security-posture)
///
/// ## Import
///
/// PostureDeployment can be imported using any of these accepted formats:
///
/// * `{{parent}}/locations/{{location}}/postureDeployments/{{posture_deployment_id}}`
///
/// When using the `pulumi import` command, PostureDeployment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securityposture/postureDeployment:PostureDeployment default {{parent}}/locations/{{location}}/postureDeployments/{{posture_deployment_id}}
/// ```
///
pub mod posture_deployment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PostureDeploymentArgs {
        /// Description of the posture deployment.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The location of the resource, eg. global`.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The parent of the resource, an organization. Format should be `organizations/{organization_id}`.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
        /// ID of the posture deployment.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub posture_deployment_id: pulumi_wasm_rust::Output<String>,
        /// Relative name of the posture which needs to be deployed. It should be in the format:
        /// organizations/{organization_id}/locations/{location}/postures/{posture_id}
        #[builder(into)]
        pub posture_id: pulumi_wasm_rust::Output<String>,
        /// Revision_id the posture which needs to be deployed.
        #[builder(into)]
        pub posture_revision_id: pulumi_wasm_rust::Output<String>,
        /// The resource on which the posture should be deployed. This can be in one of the following formats:
        /// projects/{project_number},
        /// folders/{folder_number},
        /// organizations/{organization_id}
        #[builder(into)]
        pub target_resource: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PostureDeploymentResult {
        /// Time the posture deployment was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Description of the posture deployment.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// This is an output only optional field which will be filled in case when
        /// PostureDeployment state is UPDATE_FAILED or CREATE_FAILED or DELETE_FAILED.
        /// It denotes the desired posture to be deployed.
        pub desired_posture_id: pulumi_wasm_rust::Output<String>,
        /// This is an output only optional field which will be filled in case when
        /// PostureDeployment state is UPDATE_FAILED or CREATE_FAILED or DELETE_FAILED.
        /// It denotes the desired posture revision_id to be deployed.
        pub desired_posture_revision_id: pulumi_wasm_rust::Output<String>,
        /// For Resource freshness validation (https://google.aip.dev/154)
        pub etag: pulumi_wasm_rust::Output<String>,
        /// This is a output only optional field which will be filled in case where
        /// PostureDeployment enters a failure state like UPDATE_FAILED or
        /// CREATE_FAILED or DELETE_FAILED. It will have the failure message for posture deployment's
        /// CREATE/UPDATE/DELETE methods.
        pub failure_message: pulumi_wasm_rust::Output<String>,
        /// The location of the resource, eg. global`.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Name of the posture deployment instance.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parent of the resource, an organization. Format should be `organizations/{organization_id}`.
        pub parent: pulumi_wasm_rust::Output<String>,
        /// ID of the posture deployment.
        ///
        ///
        /// - - -
        pub posture_deployment_id: pulumi_wasm_rust::Output<String>,
        /// Relative name of the posture which needs to be deployed. It should be in the format:
        /// organizations/{organization_id}/locations/{location}/postures/{posture_id}
        pub posture_id: pulumi_wasm_rust::Output<String>,
        /// Revision_id the posture which needs to be deployed.
        pub posture_revision_id: pulumi_wasm_rust::Output<String>,
        /// If set, there are currently changes in flight to the posture deployment.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// State of the posture deployment. A posture deployment can be in the following terminal states:
        /// ACTIVE, CREATE_FAILED, UPDATE_FAILED, DELETE_FAILED.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The resource on which the posture should be deployed. This can be in one of the following formats:
        /// projects/{project_number},
        /// folders/{folder_number},
        /// organizations/{organization_id}
        pub target_resource: pulumi_wasm_rust::Output<String>,
        /// Time the posture deployment was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PostureDeploymentArgs) -> PostureDeploymentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let location_binding = args.location.get_inner();
        let parent_binding = args.parent.get_inner();
        let posture_deployment_id_binding = args.posture_deployment_id.get_inner();
        let posture_id_binding = args.posture_id.get_inner();
        let posture_revision_id_binding = args.posture_revision_id.get_inner();
        let target_resource_binding = args.target_resource.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securityposture/postureDeployment:PostureDeployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "postureDeploymentId".into(),
                    value: &posture_deployment_id_binding,
                },
                register_interface::ObjectField {
                    name: "postureId".into(),
                    value: &posture_id_binding,
                },
                register_interface::ObjectField {
                    name: "postureRevisionId".into(),
                    value: &posture_revision_id_binding,
                },
                register_interface::ObjectField {
                    name: "targetResource".into(),
                    value: &target_resource_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "desiredPostureId".into(),
                },
                register_interface::ResultField {
                    name: "desiredPostureRevisionId".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "failureMessage".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "postureDeploymentId".into(),
                },
                register_interface::ResultField {
                    name: "postureId".into(),
                },
                register_interface::ResultField {
                    name: "postureRevisionId".into(),
                },
                register_interface::ResultField {
                    name: "reconciling".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "targetResource".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PostureDeploymentResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            desired_posture_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredPostureId").unwrap(),
            ),
            desired_posture_revision_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredPostureRevisionId").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            failure_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failureMessage").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            posture_deployment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("postureDeploymentId").unwrap(),
            ),
            posture_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("postureId").unwrap(),
            ),
            posture_revision_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("postureRevisionId").unwrap(),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconciling").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            target_resource: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetResource").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
