#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LocalRulestackRuleCategory {
    /// Specifies a list of URL categories to match. Possible values include `abortion`, `abused-drugs`, `adult`, `alcohol-and-tobacco`, `auctions`, `business-and-economy`, `command-and-control`, `computer-and-internet-info`, `content-delivery-networks`, `copyright-infringement`, `cryptocurrency`, `dating`, `dynamic-dns`, `educational-institutions`, `entertainment-and-arts`, `extremism`, `financial-services`, `gambling`, `games`, `government`, `grayware`, `hacking`, `health-and-medicine`, `high-risk`, `home-and-garden`, `hunting-and-fishing`, `insufficient-content`, `internet-communications-and-telephony`, `internet-portals`, `job-search`, `legal`, `low-risk`, `malware`, `medium-risk`, `military`, `motor-vehicles`, `music`, `newly-registered-domain`, `news`, `not-resolved`, `nudity`, `online-storage-and-backup`, `parked`, `peer-to-peer`, `personal-sites-and-blogs`, `philosophy-and-political-advocacy`, `phishing`, `private-ip-addresses`, `proxy-avoidance-and-anonymizers`, `questionable`, `real-estate`, `real-time-detection`, `recreation-and-hobbies`, `reference-and-research`, `religion`, `search-engines`, `sex-education`, `shareware-and-freeware`, `shopping`, `social-networking`, `society`, `sports`, `stock-advice-and-tools`, `streaming-media`, `swimsuits-and-intimate-apparel`, `training-and-tools`, `translation`, `travel`, `unknown`, `weapons`, `web-advertisements`, `web-based-email`, and `web-hosting`.
    #[builder(into)]
    #[serde(rename = "customUrls")]
    pub r#custom_urls: Box<Vec<String>>,
    /// Specifies a list of feeds to match.
    #[builder(into, default)]
    #[serde(rename = "feeds")]
    pub r#feeds: Box<Option<Vec<String>>>,
}
