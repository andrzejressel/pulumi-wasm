/// Provides an IAM OpenID Connect provider.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = open_id_connect_provider::create(
///         "default",
///         OpenIdConnectProviderArgs::builder()
///             .client_id_lists(
///                 vec!["266362248691-342342xasdasdasda-apps.googleusercontent.com",],
///             )
///             .thumbprint_lists(vec!["cf23df2207d99a74fbe169e3eba035e633b65d94",])
///             .url("https://accounts.google.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM OpenID Connect Providers using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/openIdConnectProvider:OpenIdConnectProvider default arn:aws:iam::123456789012:oidc-provider/accounts.google.com
/// ```
pub mod open_id_connect_provider {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OpenIdConnectProviderArgs {
        /// A list of client IDs (also known as audiences). When a mobile or web app registers with an OpenID Connect provider, they establish a value that identifies the application. (This is the value that's sent as the client_id parameter on OAuth requests.)
        #[builder(into)]
        pub client_id_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// Map of resource tags for the IAM OIDC provider. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of server certificate thumbprints for the OpenID Connect (OIDC) identity provider's server certificate(s).
        #[builder(into)]
        pub thumbprint_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// The URL of the identity provider. Corresponds to the _iss_ claim.
        #[builder(into)]
        pub url: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct OpenIdConnectProviderResult {
        /// The ARN assigned by AWS for this provider.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A list of client IDs (also known as audiences). When a mobile or web app registers with an OpenID Connect provider, they establish a value that identifies the application. (This is the value that's sent as the client_id parameter on OAuth requests.)
        pub client_id_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// Map of resource tags for the IAM OIDC provider. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of server certificate thumbprints for the OpenID Connect (OIDC) identity provider's server certificate(s).
        pub thumbprint_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// The URL of the identity provider. Corresponds to the _iss_ claim.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OpenIdConnectProviderArgs,
    ) -> OpenIdConnectProviderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let client_id_lists_binding = args.client_id_lists.get_inner();
        let tags_binding = args.tags.get_inner();
        let thumbprint_lists_binding = args.thumbprint_lists.get_inner();
        let url_binding = args.url.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/openIdConnectProvider:OpenIdConnectProvider".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clientIdLists".into(),
                    value: &client_id_lists_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "thumbprintLists".into(),
                    value: &thumbprint_lists_binding,
                },
                register_interface::ObjectField {
                    name: "url".into(),
                    value: &url_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "clientIdLists".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "thumbprintLists".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OpenIdConnectProviderResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            client_id_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientIdLists").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            thumbprint_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thumbprintLists").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
