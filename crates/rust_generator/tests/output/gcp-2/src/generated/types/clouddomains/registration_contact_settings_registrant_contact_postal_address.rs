#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RegistrationContactSettingsRegistrantContactPostalAddress {
    /// Unstructured address lines describing the lower levels of an address.
    /// Because values in addressLines do not have type information and may sometimes contain multiple values in a single
    /// field (e.g. "Austin, TX"), it is important that the line order is clear. The order of address lines should be
    /// "envelope order" for the country/region of the address. In places where this can vary (e.g. Japan), address_language
    /// is used to make it explicit (e.g. "ja" for large-to-small ordering and "ja-Latn" or "en" for small-to-large). This way,
    /// the most specific line of an address can be selected based on the language.
    #[builder(into, default)]
    #[serde(rename = "addressLines")]
    pub r#address_lines: Box<Option<Vec<String>>>,
    /// Highest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state,
    /// a province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community
    /// (e.g. "Barcelona" and not "Catalonia"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland
    /// this should be left unpopulated.
    #[builder(into, default)]
    #[serde(rename = "administrativeArea")]
    pub r#administrative_area: Box<Option<String>>,
    /// Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world
    /// where localities are not well defined or do not fit into this structure well, leave locality empty and use addressLines.
    #[builder(into, default)]
    #[serde(rename = "locality")]
    pub r#locality: Box<Option<String>>,
    /// The name of the organization at the address.
    #[builder(into, default)]
    #[serde(rename = "organization")]
    pub r#organization: Box<Option<String>>,
    /// Postal code of the address. Not all countries use or require postal codes to be present, but where they are used,
    /// they may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.).
    #[builder(into, default)]
    #[serde(rename = "postalCode")]
    pub r#postal_code: Box<Option<String>>,
    /// The recipient at the address. This field may, under certain circumstances, contain multiline information. For example,
    /// it might contain "care of" information.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "recipients")]
    pub r#recipients: Box<Option<Vec<String>>>,
    /// Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to
    /// ensure the value is correct. See https://cldr.unicode.org/ and
    /// https://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: "CH" for Switzerland.
    #[builder(into)]
    #[serde(rename = "regionCode")]
    pub r#region_code: Box<String>,
}
