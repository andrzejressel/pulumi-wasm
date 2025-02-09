/// Provides an EventBridge connection resource.
///
/// > **Note:** EventBridge was formerly known as CloudWatch Events. The functionality is identical.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = event_connection::create(
///         "test",
///         EventConnectionArgs::builder()
///             .auth_parameters(
///                 EventConnectionAuthParameters::builder()
///                     .apiKey(
///                         EventConnectionAuthParametersApiKey::builder()
///                             .key("x-signature")
///                             .value("1234")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .authorization_type("API_KEY")
///             .description("A connection description")
///             .name("ngrok-connection")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### Basic Authorization
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = event_connection::create(
///         "test",
///         EventConnectionArgs::builder()
///             .auth_parameters(
///                 EventConnectionAuthParameters::builder()
///                     .basic(
///                         EventConnectionAuthParametersBasic::builder()
///                             .password("Pass1234!")
///                             .username("user")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .authorization_type("BASIC")
///             .description("A connection description")
///             .name("ngrok-connection")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### OAuth Authorization
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = event_connection::create(
///         "test",
///         EventConnectionArgs::builder()
///             .auth_parameters(
///                 EventConnectionAuthParameters::builder()
///                     .oauth(
///                         EventConnectionAuthParametersOauth::builder()
///                             .authorizationEndpoint("https://auth.url.com/endpoint")
///                             .clientParameters(
///                                 EventConnectionAuthParametersOauthClientParameters::builder()
///                                     .clientId("1234567890")
///                                     .clientSecret("Pass1234!")
///                                     .build_struct(),
///                             )
///                             .httpMethod("GET")
///                             .oauthHttpParameters(
///                                 EventConnectionAuthParametersOauthOauthHttpParameters::builder()
///                                     .bodies(
///                                         vec![
///                                             EventConnectionAuthParametersOauthOauthHttpParametersBody::builder()
///                                             .isValueSecret(false).key("body-parameter-key")
///                                             .value("body-parameter-value").build_struct(),
///                                         ],
///                                     )
///                                     .headers(
///                                         vec![
///                                             EventConnectionAuthParametersOauthOauthHttpParametersHeader::builder()
///                                             .isValueSecret(false).key("header-parameter-key")
///                                             .value("header-parameter-value").build_struct(),
///                                         ],
///                                     )
///                                     .queryStrings(
///                                         vec![
///                                             EventConnectionAuthParametersOauthOauthHttpParametersQueryString::builder()
///                                             .isValueSecret(false).key("query-string-parameter-key")
///                                             .value("query-string-parameter-value").build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .authorization_type("OAUTH_CLIENT_CREDENTIALS")
///             .description("A connection description")
///             .name("ngrok-connection")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### Invocation Http Parameters
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = event_connection::create(
///         "test",
///         EventConnectionArgs::builder()
///             .auth_parameters(
///                 EventConnectionAuthParameters::builder()
///                     .basic(
///                         EventConnectionAuthParametersBasic::builder()
///                             .password("Pass1234!")
///                             .username("user")
///                             .build_struct(),
///                     )
///                     .invocationHttpParameters(
///                         EventConnectionAuthParametersInvocationHttpParameters::builder()
///                             .bodies(
///                                 vec![
///                                     EventConnectionAuthParametersInvocationHttpParametersBody::builder()
///                                     .isValueSecret(false).key("body-parameter-key")
///                                     .value("body-parameter-value").build_struct(),
///                                     EventConnectionAuthParametersInvocationHttpParametersBody::builder()
///                                     .isValueSecret(true).key("body-parameter-key2")
///                                     .value("body-parameter-value2").build_struct(),
///                                 ],
///                             )
///                             .headers(
///                                 vec![
///                                     EventConnectionAuthParametersInvocationHttpParametersHeader::builder()
///                                     .isValueSecret(false).key("header-parameter-key")
///                                     .value("header-parameter-value").build_struct(),
///                                 ],
///                             )
///                             .queryStrings(
///                                 vec![
///                                     EventConnectionAuthParametersInvocationHttpParametersQueryString::builder()
///                                     .isValueSecret(false).key("query-string-parameter-key")
///                                     .value("query-string-parameter-value").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .authorization_type("BASIC")
///             .description("A connection description")
///             .name("ngrok-connection")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EventBridge EventBridge connection using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/eventConnection:EventConnection test ngrok-connection
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventConnectionArgs {
        /// Parameters used for authorization. A maximum of 1 are allowed. Documented below.
        #[builder(into)]
        pub auth_parameters: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudwatch::EventConnectionAuthParameters,
        >,
        /// Choose the type of authorization to use for the connection. One of `API_KEY`,`BASIC`,`OAUTH_CLIENT_CREDENTIALS`.
        #[builder(into)]
        pub authorization_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Enter a description for the connection. Maximum of 512 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the new connection. Maximum of 64 characters consisting of numbers, lower/upper case letters, .,-,_.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EventConnectionResult {
        /// The Amazon Resource Name (ARN) of the connection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Parameters used for authorization. A maximum of 1 are allowed. Documented below.
        pub auth_parameters: pulumi_gestalt_rust::Output<
            super::super::types::cloudwatch::EventConnectionAuthParameters,
        >,
        /// Choose the type of authorization to use for the connection. One of `API_KEY`,`BASIC`,`OAUTH_CLIENT_CREDENTIALS`.
        pub authorization_type: pulumi_gestalt_rust::Output<String>,
        /// Enter a description for the connection. Maximum of 512 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the new connection. Maximum of 64 characters consisting of numbers, lower/upper case letters, .,-,_.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the secret created from the authorization parameters specified for the connection.
        pub secret_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EventConnectionArgs,
    ) -> EventConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auth_parameters_binding_1 = args.auth_parameters.get_output(context);
        let auth_parameters_binding = auth_parameters_binding_1.get_inner();
        let authorization_type_binding_1 = args.authorization_type.get_output(context);
        let authorization_type_binding = authorization_type_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/eventConnection:EventConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authParameters".into(),
                    value: &auth_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "authorizationType".into(),
                    value: &authorization_type_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EventConnectionResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auth_parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authParameters"),
            ),
            authorization_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizationType"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            secret_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretArn"),
            ),
        }
    }
}
