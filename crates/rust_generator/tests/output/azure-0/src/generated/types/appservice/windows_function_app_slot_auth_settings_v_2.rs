#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WindowsFunctionAppSlotAuthSettingsV2 {
    /// An `active_directory_v2` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "activeDirectoryV2")]
    pub r#active_directory_v_2: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsV2ActiveDirectoryV2>>,
    /// An `apple_v2` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "appleV2")]
    pub r#apple_v_2: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsV2AppleV2>>,
    /// Should the AuthV2 Settings be enabled. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "authEnabled")]
    pub r#auth_enabled: Box<Option<bool>>,
    /// An `azure_static_web_app_v2` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "azureStaticWebAppV2")]
    pub r#azure_static_web_app_v_2: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsV2AzureStaticWebAppV2>>,
    /// The path to the App Auth settings.
    /// 
    /// > **Note:** Relative Paths are evaluated from the Site Root directory.
    #[builder(into, default)]
    #[serde(rename = "configFilePath")]
    pub r#config_file_path: Box<Option<String>>,
    /// Zero or more `custom_oidc_v2` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "customOidcV2s")]
    pub r#custom_oidc_v_2_s: Box<Option<Vec<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsV2CustomOidcV2>>>,
    /// The Default Authentication Provider to use when the `unauthenticated_action` is set to `RedirectToLoginPage`. Possible values include: `apple`, `azureactivedirectory`, `facebook`, `github`, `google`, `twitter` and the `name` of your `custom_oidc_v2` provider.
    /// 
    /// > **NOTE:** Whilst any value will be accepted by the API for `default_provider`, it can leave the app in an unusable state if this value does not correspond to the name of a known provider (either built-in value, or custom_oidc name) as it is used to build the auth endpoint URI.
    #[builder(into, default)]
    #[serde(rename = "defaultProvider")]
    pub r#default_provider: Box<Option<String>>,
    /// The paths which should be excluded from the `unauthenticated_action` when it is set to `RedirectToLoginPage`.
    /// 
    /// > **NOTE:** This list should be used instead of setting `WEBSITE_WARMUP_PATH` in `app_settings` as it takes priority.
    #[builder(into, default)]
    #[serde(rename = "excludedPaths")]
    pub r#excluded_paths: Box<Option<Vec<String>>>,
    /// A `facebook_v2` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "facebookV2")]
    pub r#facebook_v_2: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsV2FacebookV2>>,
    /// The convention used to determine the url of the request made. Possible values include `NoProxy`, `Standard`, `Custom`. Defaults to `NoProxy`.
    #[builder(into, default)]
    #[serde(rename = "forwardProxyConvention")]
    pub r#forward_proxy_convention: Box<Option<String>>,
    /// The name of the custom header containing the host of the request.
    #[builder(into, default)]
    #[serde(rename = "forwardProxyCustomHostHeaderName")]
    pub r#forward_proxy_custom_host_header_name: Box<Option<String>>,
    /// The name of the custom header containing the scheme of the request.
    #[builder(into, default)]
    #[serde(rename = "forwardProxyCustomSchemeHeaderName")]
    pub r#forward_proxy_custom_scheme_header_name: Box<Option<String>>,
    /// A `github_v2` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "githubV2")]
    pub r#github_v_2: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsV2GithubV2>>,
    /// A `google_v2` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "googleV2")]
    pub r#google_v_2: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsV2GoogleV2>>,
    /// The prefix that should precede all the authentication and authorisation paths. Defaults to `/.auth`.
    #[builder(into, default)]
    #[serde(rename = "httpRouteApiPrefix")]
    pub r#http_route_api_prefix: Box<Option<String>>,
    /// A `login` block as defined below.
    #[builder(into)]
    #[serde(rename = "login")]
    pub r#login: Box<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsV2Login>,
    /// A `microsoft_v2` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "microsoftV2")]
    pub r#microsoft_v_2: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsV2MicrosoftV2>>,
    /// Should the authentication flow be used for all requests.
    #[builder(into, default)]
    #[serde(rename = "requireAuthentication")]
    pub r#require_authentication: Box<Option<bool>>,
    /// Should HTTPS be required on connections? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "requireHttps")]
    pub r#require_https: Box<Option<bool>>,
    /// The Runtime Version of the Authentication and Authorisation feature of this App. Defaults to `~1`.
    #[builder(into, default)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: Box<Option<String>>,
    /// A `twitter_v2` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "twitterV2")]
    pub r#twitter_v_2: Box<Option<super::super::types::appservice::WindowsFunctionAppSlotAuthSettingsV2TwitterV2>>,
    /// The action to take for requests made without authentication. Possible values include `RedirectToLoginPage`, `AllowAnonymous`, `Return401`, and `Return403`. Defaults to `RedirectToLoginPage`.
    #[builder(into, default)]
    #[serde(rename = "unauthenticatedAction")]
    pub r#unauthenticated_action: Box<Option<String>>,
}
