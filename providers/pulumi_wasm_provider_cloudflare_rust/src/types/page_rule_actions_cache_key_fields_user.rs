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
    /// ```yaml
    /// resources:
    ///   # Unrealistic example with all features used
    ///   foobar:
    ///     type: cloudflare:PageRule
    ///     properties:
    ///       zoneId: ${cloudflareZoneId}
    ///       target: ${cloudflareZone}/app/*
    ///       priority: 1
    ///       actions:
    ///         cacheKeyFields:
    ///           cookie:
    ///             checkPresences:
    ///               - wordpress_test_cookie
    ///           header:
    ///             checkPresences:
    ///               - header_present
    ///             excludes:
    ///               - origin
    ///             includes:
    ///               - api-key
    ///               - dnt
    ///           host:
    ///             resolved: true
    ///           queryString:
    ///             ignore: true
    ///           user:
    ///             deviceType: false
    ///             geo: true
    ///             lang: true
    /// ```
    /// <!--End PulumiCodeChooser -->
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "lang")]
    pub r#lang: Box<Option<bool>>,
}
