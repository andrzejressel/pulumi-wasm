/// Resource for managing an AWS Service Catalog AppRegistry Application.
///
/// > An AWS Service Catalog AppRegistry Application is displayed in the AWS Console under "MyApplications".
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
///     let example = appregistry_application::create(
///         "example",
///         AppregistryApplicationArgs::builder().name("example-app").build_struct(),
///     );
/// }
/// ```
///
/// ### Connecting Resources
///
/// ```yaml
/// resources:
///   example:
///     type: aws:servicecatalog:AppregistryApplication
///     properties:
///       name: example-app
///   bucket:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example-bucket
///       tags: ${example.applicationTag}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Service Catalog AppRegistry Application using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/appregistryApplication:AppregistryApplication example application-id-12345678
/// ```
pub mod appregistry_application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppregistryApplicationArgs {
        /// Description of the application.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the application. The name must be unique within an AWS region.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AppregistryApplicationResult {
        /// A map with a single tag key-value pair used to associate resources with the application. This attribute can be passed directly into the `tags` argument of another resource, or merged into a map of existing tags.
        pub application_tag: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ARN (Amazon Resource Name) of the application.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the application.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the application. The name must be unique within an AWS region.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AppregistryApplicationArgs,
    ) -> AppregistryApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/appregistryApplication:AppregistryApplication"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationTag".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AppregistryApplicationResult {
            application_tag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationTag").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}