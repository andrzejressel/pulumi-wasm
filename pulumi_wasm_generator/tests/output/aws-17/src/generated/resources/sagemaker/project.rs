/// Provides a SageMaker Project resource.
///
///  > Note: If you are trying to use SageMaker projects with SageMaker studio you will need to add a tag with the key `sagemaker:studio-visibility` with value `true`. For more on requirements to use projects and permission needed see [AWS Docs](https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-projects-templates-custom.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = project::create(
///         "example",
///         ProjectArgs::builder()
///             .project_name("example")
///             .service_catalog_provisioning_details(
///                 ProjectServiceCatalogProvisioningDetails::builder()
///                     .productId("${exampleAwsServicecatalogProduct.id}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Projects using the `project_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/project:Project example example
/// ```
pub mod project {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectArgs {
        /// A description for the project.
        #[builder(into, default)]
        pub project_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Project.
        #[builder(into)]
        pub project_name: pulumi_wasm_rust::Output<String>,
        /// The product ID and provisioning artifact ID to provision a service catalog. See Service Catalog Provisioning Details below.
        #[builder(into)]
        pub service_catalog_provisioning_details: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::ProjectServiceCatalogProvisioningDetails,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProjectResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Project.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A description for the project.
        pub project_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project.
        pub project_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Project.
        pub project_name: pulumi_wasm_rust::Output<String>,
        /// The product ID and provisioning artifact ID to provision a service catalog. See Service Catalog Provisioning Details below.
        pub service_catalog_provisioning_details: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::ProjectServiceCatalogProvisioningDetails,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProjectArgs) -> ProjectResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let project_description_binding = args.project_description.get_inner();
        let project_name_binding = args.project_name.get_inner();
        let service_catalog_provisioning_details_binding = args
            .service_catalog_provisioning_details
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/project:Project".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "projectDescription".into(),
                    value: &project_description_binding,
                },
                register_interface::ObjectField {
                    name: "projectName".into(),
                    value: &project_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceCatalogProvisioningDetails".into(),
                    value: &service_catalog_provisioning_details_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "projectDescription".into(),
                },
                register_interface::ResultField {
                    name: "projectId".into(),
                },
                register_interface::ResultField {
                    name: "projectName".into(),
                },
                register_interface::ResultField {
                    name: "serviceCatalogProvisioningDetails".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProjectResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            project_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectDescription").unwrap(),
            ),
            project_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectId").unwrap(),
            ),
            project_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectName").unwrap(),
            ),
            service_catalog_provisioning_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceCatalogProvisioningDetails").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
