#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct PageRuleActionsCacheKeyFieldsUser {
    /// `true` - classifies a request as “mobile”, “desktop”, or “tablet” based on the User Agent; defaults to `false`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<bool>>,
    /// `true` - includes the client’s country, derived from the IP address; defaults to `false`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "geo")]
    pub r#geo: Box<Option<bool>>,
    /// `true` - includes the first language code contained in the `Accept-Language` header sent by the client; defaults to `false`.
    /// 
    /// Example:
    /// 
    /// <!--Start PulumiCodeChooser -->
    /// ```ignore
    /// use pulumi_wasm_rust::Output;
    /// use pulumi_wasm_rust::{add_export, pulumi_main};
    /// #[pulumi_main]
    /// fn test_main() -> Result<(), Error> {
    ///     let foobar = page_rule::create(
    ///         "foobar",
    ///         PageRuleArgs::builder()
    ///             .actions(
    ///                 PageRuleActions::builder()
    ///                     .cacheKeyFields(
    ///                         PageRuleActionsCacheKeyFields::builder()
    ///                             .cookie(
    ///                                 PageRuleActionsCacheKeyFieldsCookie::builder()
    ///                                     .checkPresences(vec!["wordpress_test_cookie",])
    ///                                     .build_struct(),
    ///                             )
    ///                             .header(
    ///                                 PageRuleActionsCacheKeyFieldsHeader::builder()
    ///                                     .checkPresences(vec!["header_present",])
    ///                                     .excludes(vec!["origin",])
    ///                                     .includes(vec!["api-key", "dnt",])
    ///                                     .build_struct(),
    ///                             )
    ///                             .host(
    ///                                 PageRuleActionsCacheKeyFieldsHost::builder()
    ///                                     .resolved(true)
    ///                                     .build_struct(),
    ///                             )
    ///                             .queryString(
    ///                                 PageRuleActionsCacheKeyFieldsQueryString::builder()
    ///                                     .ignore(true)
    ///                                     .build_struct(),
    ///                             )
    ///                             .user(
    ///                                 PageRuleActionsCacheKeyFieldsUser::builder()
    ///                                     .deviceType(false)
    ///                                     .geo(true)
    ///                                     .lang(true)
    ///                                     .build_struct(),
    ///                             )
    ///                             .build_struct(),
    ///                     )
    ///                     .build_struct(),
    ///             )
    ///             .priority(1)
    ///             .target("${cloudflareZone}/app/*")
    ///             .zone_id("${cloudflareZoneId}")
    ///             .build_struct(),
    ///     );
    /// }
    /// ```
    /// <!--End PulumiCodeChooser -->
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "lang")]
    pub r#lang: Box<Option<bool>>,
}
