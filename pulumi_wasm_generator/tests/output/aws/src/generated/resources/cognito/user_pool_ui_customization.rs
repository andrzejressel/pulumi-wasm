/// Provides a Cognito User Pool UI Customization resource.
///
/// > **Note:** To use this resource, the user pool must have a domain associated with it. For more information, see the Amazon Cognito Developer Guide on [Customizing the Built-in Sign-In and Sign-up Webpages](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-app-ui-customization.html).
///
/// ## Example Usage
///
/// ### UI customization settings for a single client
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cognito:UserPool
///     properties:
///       name: example
///   exampleUserPoolDomain:
///     type: aws:cognito:UserPoolDomain
///     name: example
///     properties:
///       domain: example
///       userPoolId: ${example.id}
///   exampleUserPoolClient:
///     type: aws:cognito:UserPoolClient
///     name: example
///     properties:
///       name: example
///       userPoolId: ${example.id}
///   exampleUserPoolUICustomization:
///     type: aws:cognito:UserPoolUICustomization
///     name: example
///     properties:
///       clientId: ${exampleUserPoolClient.id}
///       css: '.label-customizable {font-weight: 400;}'
///       imageFile:
///         fn::invoke:
///           Function: std:filebase64
///           Arguments:
///             input: logo.png
///           Return: result
///       userPoolId: ${exampleUserPoolDomain.userPoolId}
/// ```
///
/// ### UI customization settings for all clients
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cognito:UserPool
///     properties:
///       name: example
///   exampleUserPoolDomain:
///     type: aws:cognito:UserPoolDomain
///     name: example
///     properties:
///       domain: example
///       userPoolId: ${example.id}
///   exampleUserPoolUICustomization:
///     type: aws:cognito:UserPoolUICustomization
///     name: example
///     properties:
///       css: '.label-customizable {font-weight: 400;}'
///       imageFile:
///         fn::invoke:
///           Function: std:filebase64
///           Arguments:
///             input: logo.png
///           Return: result
///       userPoolId: ${exampleUserPoolDomain.userPoolId}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cognito User Pool UI Customizations using the `user_pool_id` and `client_id` separated by `,`. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/userPoolUICustomization:UserPoolUICustomization example us-west-2_ZCTarbt5C,12bu4fuk3mlgqa2rtrujgp6egq
/// ```
pub mod user_pool_ui_customization {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserPoolUICustomizationArgs {
        /// The client ID for the client app. Defaults to `ALL`. If `ALL` is specified, the `css` and/or `image_file` settings will be used for every client that has no UI customization set previously.
        #[builder(into, default)]
        pub client_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The CSS values in the UI customization, provided as a String. At least one of `css` or `image_file` is required.
        #[builder(into, default)]
        pub css: pulumi_wasm_rust::Output<Option<String>>,
        /// The uploaded logo image for the UI customization, provided as a base64-encoded String. Drift detection is not possible for this argument. At least one of `css` or `image_file` is required.
        #[builder(into, default)]
        pub image_file: pulumi_wasm_rust::Output<Option<String>>,
        /// The user pool ID for the user pool.
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct UserPoolUICustomizationResult {
        /// The client ID for the client app. Defaults to `ALL`. If `ALL` is specified, the `css` and/or `image_file` settings will be used for every client that has no UI customization set previously.
        pub client_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The creation date in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) for the UI customization.
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// The CSS values in the UI customization, provided as a String. At least one of `css` or `image_file` is required.
        pub css: pulumi_wasm_rust::Output<Option<String>>,
        /// The CSS version number.
        pub css_version: pulumi_wasm_rust::Output<String>,
        /// The uploaded logo image for the UI customization, provided as a base64-encoded String. Drift detection is not possible for this argument. At least one of `css` or `image_file` is required.
        pub image_file: pulumi_wasm_rust::Output<Option<String>>,
        /// The logo image URL for the UI customization.
        pub image_url: pulumi_wasm_rust::Output<String>,
        /// The last-modified date in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) for the UI customization.
        pub last_modified_date: pulumi_wasm_rust::Output<String>,
        /// The user pool ID for the user pool.
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: UserPoolUICustomizationArgs,
    ) -> UserPoolUICustomizationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let client_id_binding = args.client_id.get_inner();
        let css_binding = args.css.get_inner();
        let image_file_binding = args.image_file.get_inner();
        let user_pool_id_binding = args.user_pool_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/userPoolUICustomization:UserPoolUICustomization".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding,
                },
                register_interface::ObjectField {
                    name: "css".into(),
                    value: &css_binding,
                },
                register_interface::ObjectField {
                    name: "imageFile".into(),
                    value: &image_file_binding,
                },
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clientId".into(),
                },
                register_interface::ResultField {
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "css".into(),
                },
                register_interface::ResultField {
                    name: "cssVersion".into(),
                },
                register_interface::ResultField {
                    name: "imageFile".into(),
                },
                register_interface::ResultField {
                    name: "imageUrl".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedDate".into(),
                },
                register_interface::ResultField {
                    name: "userPoolId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserPoolUICustomizationResult {
            client_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientId").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            css: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("css").unwrap(),
            ),
            css_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cssVersion").unwrap(),
            ),
            image_file: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageFile").unwrap(),
            ),
            image_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageUrl").unwrap(),
            ),
            last_modified_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedDate").unwrap(),
            ),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPoolId").unwrap(),
            ),
        }
    }
}