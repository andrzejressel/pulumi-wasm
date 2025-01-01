/// Generate service identity for a service.
///
/// > **Note:** Once created, this resource cannot be updated or destroyed. These
/// actions are a no-op.
///
/// > **Note:** This resource can be used to retrieve the emails of the [Google-managed service accounts](https://cloud.google.com/iam/docs/service-agents)
/// of the APIs that Google has configured with a Service Identity. You can run `gcloud beta services identity create --service SERVICE_NAME.googleapis.com` to
/// verify if an API supports this.
///
/// To get more information about Service Identity, see:
///
/// * [API documentation](https://cloud.google.com/service-usage/docs/reference/rest/v1beta1/services/generateServiceIdentity)
///
/// ## Example Usage
///
/// ### Service Identity Basic
///
/// ```yaml
/// resources:
///   hcSa:
///     type: gcp:projects:ServiceIdentity
///     name: hc_sa
///     properties:
///       project: ${project.projectId}
///       service: healthcare.googleapis.com
///   hcSaBqJobuser:
///     type: gcp:projects:IAMMember
///     name: hc_sa_bq_jobuser
///     properties:
///       project: ${project.projectId}
///       role: roles/bigquery.jobUser
///       member: ${hcSa.member}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
pub mod service_identity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceIdentityArgs {
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The service to generate identity for.
        ///
        /// - - -
        #[builder(into)]
        pub service: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceIdentityResult {
        /// The email address of the Google managed service account.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The Identity of the Google managed service account in the form 'serviceAccount:{email}'. This value is often used to refer to the service account in order to grant IAM permissions.
        pub member: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The service to generate identity for.
        ///
        /// - - -
        pub service: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceIdentityArgs) -> ServiceIdentityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_inner();
        let service_binding = args.service.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:projects/serviceIdentity:ServiceIdentity".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "member".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "service".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceIdentityResult {
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            member: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("member").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("service").unwrap(),
            ),
        }
    }
}
