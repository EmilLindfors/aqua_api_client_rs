/// Error types.
pub mod error {
    /// Error from a TryFrom or FromStr implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///Adresse.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Adresse.",
///  "type": "object",
///  "properties": {
///    "countryCode": {
///      "description": "Landskoden hvor addressen er.",
///      "type": "string"
///    },
///    "countryName": {
///      "description": "Navnet på landet addressen ligger i.",
///      "type": "string"
///    },
///    "countyCode": {
///      "description": "Fylkesnummeret som addressen hører til.",
///      "type": "string"
///    },
///    "countyName": {
///      "description": "Fylkesnavnet hvor addressen er.",
///      "type": "string"
///    },
///    "id": {
///      "description": "Unik identifikator for addressen.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "officialSourceType": {
///      "description": "Offisiell kildetype for addressen.",
///      "type": "string"
///    },
///    "type": {
///      "description": "Addressetypen.",
///      "type": "string"
///    },
///    "value": {
///      "description": "Verdi.",
///      "type": "string"
///    },
///    "zipCode": {
///      "description": "Postnummeret som addressen hører til.",
///      "type": "string"
///    },
///    "zipName": {
///      "description": "Poststedet som addressen hører til.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Address {
    ///Landskoden hvor addressen er.
    #[serde(
        rename = "countryCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub country_code: ::std::option::Option<::std::string::String>,
    ///Navnet på landet addressen ligger i.
    #[serde(
        rename = "countryName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub country_name: ::std::option::Option<::std::string::String>,
    ///Fylkesnummeret som addressen hører til.
    #[serde(
        rename = "countyCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub county_code: ::std::option::Option<::std::string::String>,
    ///Fylkesnavnet hvor addressen er.
    #[serde(
        rename = "countyName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub county_name: ::std::option::Option<::std::string::String>,
    ///Unik identifikator for addressen.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<i64>,
    ///Offisiell kildetype for addressen.
    #[serde(
        rename = "officialSourceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub official_source_type: ::std::option::Option<::std::string::String>,
    ///Addressetypen.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    ///Verdi.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
    ///Postnummeret som addressen hører til.
    #[serde(
        rename = "zipCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub zip_code: ::std::option::Option<::std::string::String>,
    ///Poststedet som addressen hører til.
    #[serde(
        rename = "zipName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub zip_name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Address> for Address {
    fn from(value: &Address) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Address {
    fn default() -> Self {
        Self {
            country_code: Default::default(),
            country_name: Default::default(),
            county_code: Default::default(),
            county_name: Default::default(),
            id: Default::default(),
            official_source_type: Default::default(),
            type_: Default::default(),
            value: Default::default(),
            zip_code: Default::default(),
            zip_name: Default::default(),
        }
    }
}
impl Address {
    pub fn builder() -> builder::Address {
        Default::default()
    }
}
///Geografisk informasjon om et spesifikt område.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Geografisk informasjon om et spesifikt område.",
///  "type": "object",
///  "properties": {
///    "defaultCode": {
///      "description": "Standardkode",
///      "type": "string"
///    },
///    "defaultName": {
///      "description": "Standardnavn",
///      "type": "string"
///    },
///    "id": {
///      "description": "Unik identifikator.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "name": {
///      "description": "Navnet på området lokaliteten ligger på.",
///      "type": "string"
///    },
///    "shortName": {
///      "description": "Forkortelse av navnet på området lokaliteten ligger på.",
///      "type": "string"
///    },
///    "type": {
///      "description": "Områdetypen",
///      "type": "string"
///    },
///    "validFrom": {
///      "description": "Tidsstempel for når lokaliteten ble godkjent til å benytte seg av området.",
///      "type": "string"
///    },
///    "validUntil": {
///      "description": "Sluttid for når lokaliteten er godkjent til å benytte seg av området",
///      "type": "string"
///    },
///    "versionId": {
///      "description": "Unik versjonsidentifikator",
///      "type": "integer",
///      "format": "int64"
///    },
///    "versionNextId": {
///      "description": "Unik identifikator for neste versjon.",
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AreaListItem {
    ///Standardkode
    #[serde(
        rename = "defaultCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_code: ::std::option::Option<::std::string::String>,
    ///Standardnavn
    #[serde(
        rename = "defaultName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub default_name: ::std::option::Option<::std::string::String>,
    ///Unik identifikator.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<i64>,
    ///Navnet på området lokaliteten ligger på.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Forkortelse av navnet på området lokaliteten ligger på.
    #[serde(
        rename = "shortName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub short_name: ::std::option::Option<::std::string::String>,
    ///Områdetypen
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når lokaliteten ble godkjent til å benytte seg av området.
    #[serde(
        rename = "validFrom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_from: ::std::option::Option<::std::string::String>,
    ///Sluttid for når lokaliteten er godkjent til å benytte seg av området
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_until: ::std::option::Option<::std::string::String>,
    ///Unik versjonsidentifikator
    #[serde(
        rename = "versionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_id: ::std::option::Option<i64>,
    ///Unik identifikator for neste versjon.
    #[serde(
        rename = "versionNextId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_next_id: ::std::option::Option<i64>,
}
impl ::std::convert::From<&AreaListItem> for AreaListItem {
    fn from(value: &AreaListItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AreaListItem {
    fn default() -> Self {
        Self {
            default_code: Default::default(),
            default_name: Default::default(),
            id: Default::default(),
            name: Default::default(),
            short_name: Default::default(),
            type_: Default::default(),
            valid_from: Default::default(),
            valid_until: Default::default(),
            version_id: Default::default(),
            version_next_id: Default::default(),
        }
    }
}
impl AreaListItem {
    pub fn builder() -> builder::AreaListItem {
        Default::default()
    }
}
///Geografisk beskrivelse av området.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Geografisk beskrivelse av området.",
///  "type": "object",
///  "properties": {
///    "countyCode": {
///      "description": "Fylkesnummeret som området er registrert til.",
///      "type": "string"
///    },
///    "countyName": {
///      "description": "Fylkesnavnet som området er registrert til.",
///      "type": "string"
///    },
///    "municipalityCode": {
///      "description": "Kommunenummeret som området er registrert til.",
///      "type": "string"
///    },
///    "municipalityName": {
///      "description": "Kommunenavnet som området er registrert til.",
///      "type": "string"
///    },
///    "prodAreaCode": {
///      "description": "Produksjonsområdet som området er registrert til.",
///      "type": "string"
///    },
///    "prodAreaName": {
///      "description": "Navnet på produksjonsområdet som området er registrert til.",
///      "type": "string"
///    },
///    "prodAreaStatus": {
///      "description": "Statusen til produksjonsområdet.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AreaPlacement {
    ///Fylkesnummeret som området er registrert til.
    #[serde(
        rename = "countyCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub county_code: ::std::option::Option<::std::string::String>,
    ///Fylkesnavnet som området er registrert til.
    #[serde(
        rename = "countyName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub county_name: ::std::option::Option<::std::string::String>,
    ///Kommunenummeret som området er registrert til.
    #[serde(
        rename = "municipalityCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub municipality_code: ::std::option::Option<::std::string::String>,
    ///Kommunenavnet som området er registrert til.
    #[serde(
        rename = "municipalityName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub municipality_name: ::std::option::Option<::std::string::String>,
    ///Produksjonsområdet som området er registrert til.
    #[serde(
        rename = "prodAreaCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub prod_area_code: ::std::option::Option<::std::string::String>,
    ///Navnet på produksjonsområdet som området er registrert til.
    #[serde(
        rename = "prodAreaName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub prod_area_name: ::std::option::Option<::std::string::String>,
    ///Statusen til produksjonsområdet.
    #[serde(
        rename = "prodAreaStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub prod_area_status: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&AreaPlacement> for AreaPlacement {
    fn from(value: &AreaPlacement) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AreaPlacement {
    fn default() -> Self {
        Self {
            county_code: Default::default(),
            county_name: Default::default(),
            municipality_code: Default::default(),
            municipality_name: Default::default(),
            prod_area_code: Default::default(),
            prod_area_name: Default::default(),
            prod_area_status: Default::default(),
        }
    }
}
impl AreaPlacement {
    pub fn builder() -> builder::AreaPlacement {
        Default::default()
    }
}
///Grensepunkt beskrevet i bredde- og lengdegrad i desimalgrader.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Grensepunkt beskrevet i bredde- og lengdegrad i desimalgrader.",
///  "type": "object",
///  "properties": {
///    "id": {
///      "description": "Unik identifikator",
///      "type": "integer",
///      "format": "int64"
///    },
///    "index": {
///      "description": "En indeks som representerer en unik identifikator for hvert grensepunkt.",
///      "type": "integer",
///      "format": "int32"
///    },
///    "latitude": {
///      "description": "Breddegrad",
///      "type": "number",
///      "format": "double"
///    },
///    "longitude": {
///      "description": "Lengdegrad",
///      "type": "number",
///      "format": "double"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BorderPoint {
    ///Unik identifikator
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<i64>,
    ///En indeks som representerer en unik identifikator for hvert grensepunkt.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub index: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub latitude: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub longitude: ::std::option::Option<f64>,
}
impl ::std::convert::From<&BorderPoint> for BorderPoint {
    fn from(value: &BorderPoint) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for BorderPoint {
    fn default() -> Self {
        Self {
            id: Default::default(),
            index: Default::default(),
            latitude: Default::default(),
            longitude: Default::default(),
        }
    }
}
impl BorderPoint {
    pub fn builder() -> builder::BorderPoint {
        Default::default()
    }
}
///Grensetype.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Grensetype.",
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BorderType {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&BorderType> for BorderType {
    fn from(value: &BorderType) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for BorderType {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl BorderType {
    pub fn builder() -> builder::BorderType {
        Default::default()
    }
}
///Virksomhetstype.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Virksomhetstype.",
///  "type": "object",
///  "properties": {
///    "name": {
///      "description": "Navnet på virksomhetstypen.",
///      "type": "string"
///    },
///    "value": {
///      "description": "Verdi",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BusinessType {
    ///Navnet på virksomhetstypen.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Verdi
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&BusinessType> for BusinessType {
    fn from(value: &BusinessType) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for BusinessType {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: Default::default(),
        }
    }
}
impl BusinessType {
    pub fn builder() -> builder::BusinessType {
        Default::default()
    }
}
///Informasjon om kapasitet.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Informasjon om kapasitet.",
///  "type": "object",
///  "properties": {
///    "accumulated": {
///      "description": "Akkumulert kapasitet.",
///      "type": "number",
///      "format": "double"
///    },
///    "current": {
///      "description": "Nåværende kapasitet.",
///      "type": "number",
///      "format": "double"
///    },
///    "type": {
///      "description": "Kapasistetstypen",
///      "type": "string"
///    },
///    "unit": {
///      "description": "Enhet, f.eks. Tonn.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CapacityInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub accumulated: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub current: ::std::option::Option<f64>,
    ///Kapasistetstypen
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    ///Enhet, f.eks. Tonn.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub unit: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&CapacityInfo> for CapacityInfo {
    fn from(value: &CapacityInfo) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for CapacityInfo {
    fn default() -> Self {
        Self {
            accumulated: Default::default(),
            current: Default::default(),
            type_: Default::default(),
            unit: Default::default(),
        }
    }
}
impl CapacityInfo {
    pub fn builder() -> builder::CapacityInfo {
        Default::default()
    }
}
///Enhet.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Enhet.",
///  "type": "object",
///  "properties": {
///    "addresses": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/Address"
///      }
///    },
///    "brregStatuses": {
///      "description": "Liste av enhetens statuser i brønnøysundsregisteret.",
///      "type": "array",
///      "items": {
///        "description": "Liste av enhetens statuser i brønnøysundsregisteret.",
///        "type": "string"
///      }
///    },
///    "businessTypes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/BusinessType"
///      }
///    },
///    "id": {
///      "description": "Unik enhets identifikator.",
///      "type": "string"
///    },
///    "industryCodes": {
///      "description": "Liste av industrikoder som enheten har.",
///      "type": "array",
///      "items": {
///        "description": "Liste av industrikoder som enheten har.",
///        "type": "string"
///      }
///    },
///    "name": {
///      "description": "Navnet til enheten.",
///      "type": "string"
///    },
///    "officialSourceType": {
///      "description": "Offisiell kildetype til enheten.",
///      "type": "string"
///    },
///    "openNr": {
///      "description": "Åpent nummer.",
///      "type": "string"
///    },
///    "status": {
///      "description": "Status.",
///      "type": "string"
///    },
///    "typeName": {
///      "description": "Navnetypen",
///      "type": "string"
///    },
///    "typeValue": {
///      "description": "Typeverdien.",
///      "type": "string"
///    },
///    "versionId": {
///      "description": "Unik versjons identifikator.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "versionRegisteredBy": {
///      "description": "Hvem som registrerte enhetsversjonen.",
///      "type": "string"
///    },
///    "versionRegisteredTime": {
///      "description": "Tidsstempel for når enhetsversjonen er registrert.",
///      "type": "string"
///    },
///    "versionValidFrom": {
///      "description": "Tidsstempel for når enhetsversjonen er gyldig fra.",
///      "type": "string"
///    },
///    "versionValidUntil": {
///      "description": "Tidsstempel for når enhetsversjonen er gyldig til.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Entity {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub addresses: ::std::vec::Vec<Address>,
    ///Liste av enhetens statuser i brønnøysundsregisteret.
    #[serde(
        rename = "brregStatuses",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub brreg_statuses: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "businessTypes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub business_types: ::std::vec::Vec<BusinessType>,
    ///Unik enhets identifikator.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    ///Liste av industrikoder som enheten har.
    #[serde(
        rename = "industryCodes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub industry_codes: ::std::vec::Vec<::std::string::String>,
    ///Navnet til enheten.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Offisiell kildetype til enheten.
    #[serde(
        rename = "officialSourceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub official_source_type: ::std::option::Option<::std::string::String>,
    ///Åpent nummer.
    #[serde(
        rename = "openNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_nr: ::std::option::Option<::std::string::String>,
    ///Status.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    ///Navnetypen
    #[serde(
        rename = "typeName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_name: ::std::option::Option<::std::string::String>,
    ///Typeverdien.
    #[serde(
        rename = "typeValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_value: ::std::option::Option<::std::string::String>,
    ///Unik versjons identifikator.
    #[serde(
        rename = "versionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_id: ::std::option::Option<i64>,
    ///Hvem som registrerte enhetsversjonen.
    #[serde(
        rename = "versionRegisteredBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_registered_by: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når enhetsversjonen er registrert.
    #[serde(
        rename = "versionRegisteredTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_registered_time: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når enhetsversjonen er gyldig fra.
    #[serde(
        rename = "versionValidFrom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_valid_from: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når enhetsversjonen er gyldig til.
    #[serde(
        rename = "versionValidUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_valid_until: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Entity> for Entity {
    fn from(value: &Entity) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Entity {
    fn default() -> Self {
        Self {
            addresses: Default::default(),
            brreg_statuses: Default::default(),
            business_types: Default::default(),
            id: Default::default(),
            industry_codes: Default::default(),
            name: Default::default(),
            official_source_type: Default::default(),
            open_nr: Default::default(),
            status: Default::default(),
            type_name: Default::default(),
            type_value: Default::default(),
            version_id: Default::default(),
            version_registered_by: Default::default(),
            version_registered_time: Default::default(),
            version_valid_from: Default::default(),
            version_valid_until: Default::default(),
        }
    }
}
impl Entity {
    pub fn builder() -> builder::Entity {
        Default::default()
    }
}
///Error respons.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Error respons.",
///  "type": "object",
///  "properties": {
///    "message": {
///      "description": "Error melding",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ErrorResponse {
    ///Error melding
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ErrorResponse> for ErrorResponse {
    fn from(value: &ErrorResponse) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ErrorResponse {
    fn default() -> Self {
        Self {
            message: Default::default(),
        }
    }
}
impl ErrorResponse {
    pub fn builder() -> builder::ErrorResponse {
        Default::default()
    }
}
///Fiskekode.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Fiskekode.",
///  "type": "object",
///  "properties": {
///    "code": {
///      "description": "Artskoden til gitt fisk, f.eks \"071101\".",
///      "type": "string"
///    },
///    "enGbName": {
///      "description": "Fiskens navn på engelsk, f.eks. \"Salmon\".",
///      "type": "string"
///    },
///    "latinName": {
///      "description": "Det latinske navnet på fisken, f.eks. \"Salmo salar\".",
///      "type": "string"
///    },
///    "nbNoName": {
///      "description": "Fiskens navn på bokmål, f.eks. \"Laks\".",
///      "type": "string"
///    },
///    "nnNoName": {
///      "description": "Fiskens navn på nynorsk.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FishCode {
    ///Artskoden til gitt fisk, f.eks "071101".
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<::std::string::String>,
    ///Fiskens navn på engelsk, f.eks. "Salmon".
    #[serde(
        rename = "enGbName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub en_gb_name: ::std::option::Option<::std::string::String>,
    ///Det latinske navnet på fisken, f.eks. "Salmo salar".
    #[serde(
        rename = "latinName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub latin_name: ::std::option::Option<::std::string::String>,
    ///Fiskens navn på bokmål, f.eks. "Laks".
    #[serde(
        rename = "nbNoName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub nb_no_name: ::std::option::Option<::std::string::String>,
    ///Fiskens navn på nynorsk.
    #[serde(
        rename = "nnNoName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub nn_no_name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FishCode> for FishCode {
    fn from(value: &FishCode) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FishCode {
    fn default() -> Self {
        Self {
            code: Default::default(),
            en_gb_name: Default::default(),
            latin_name: Default::default(),
            nb_no_name: Default::default(),
            nn_no_name: Default::default(),
        }
    }
}
impl FishCode {
    pub fn builder() -> builder::FishCode {
        Default::default()
    }
}
///Informasjon om tillatelser.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Informasjon om tillatelser.",
///  "type": "object",
///  "properties": {
///    "capacity": {
///      "description": "Tillatt kapasitet.",
///      "type": "number",
///      "format": "double"
///    },
///    "capacityType": {
///      "description": "Kapasitetstypen.",
///      "type": "string"
///    },
///    "capacityUnit": {
///      "description": "Enheten på kapasitetsmengden, f.eks. Tonn.",
///      "type": "string"
///    },
///    "grantedTime": {
///      "description": "Tidsstempel på når tillatelsen ble gyldig.",
///      "type": "string"
///    },
///    "legalEntityName": {
///      "description": "Navnet på den juridiske enheten",
///      "type": "string"
///    },
///    "legalEntityNrId": {
///      "description": "Juridisk-enhetsnummer-identifikator.",
///      "type": "string"
///    },
///    "openLegalEntityNr": {
///      "description": "Åpent juridisk enhetsnummer",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GrantInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub capacity: ::std::option::Option<f64>,
    ///Kapasitetstypen.
    #[serde(
        rename = "capacityType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub capacity_type: ::std::option::Option<::std::string::String>,
    ///Enheten på kapasitetsmengden, f.eks. Tonn.
    #[serde(
        rename = "capacityUnit",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub capacity_unit: ::std::option::Option<::std::string::String>,
    ///Tidsstempel på når tillatelsen ble gyldig.
    #[serde(
        rename = "grantedTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub granted_time: ::std::option::Option<::std::string::String>,
    ///Navnet på den juridiske enheten
    #[serde(
        rename = "legalEntityName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legal_entity_name: ::std::option::Option<::std::string::String>,
    ///Juridisk-enhetsnummer-identifikator.
    #[serde(
        rename = "legalEntityNrId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legal_entity_nr_id: ::std::option::Option<::std::string::String>,
    ///Åpent juridisk enhetsnummer
    #[serde(
        rename = "openLegalEntityNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_legal_entity_nr: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&GrantInfo> for GrantInfo {
    fn from(value: &GrantInfo) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for GrantInfo {
    fn default() -> Self {
        Self {
            capacity: Default::default(),
            capacity_type: Default::default(),
            capacity_unit: Default::default(),
            granted_time: Default::default(),
            legal_entity_name: Default::default(),
            legal_entity_nr_id: Default::default(),
            open_legal_entity_nr: Default::default(),
        }
    }
}
impl GrantInfo {
    pub fn builder() -> builder::GrantInfo {
        Default::default()
    }
}
///Informasjon om formålstypen.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Informasjon om formålstypen.",
///  "type": "object",
///  "properties": {
///    "id": {
///      "description": "Unik identifikator for formålstypen.",
///      "type": "integer",
///      "format": "int32"
///    },
///    "localizedValue": {
///      "description": "Lokalisert verdi.",
///      "type": "string"
///    },
///    "regulationName": {
///      "description": "Navnet på reguleringen for formålet.",
///      "type": "string"
///    },
///    "value": {
///      "description": "Verdi.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct IntentionType {
    ///Unik identifikator for formålstypen.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<i32>,
    ///Lokalisert verdi.
    #[serde(
        rename = "localizedValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub localized_value: ::std::option::Option<::std::string::String>,
    ///Navnet på reguleringen for formålet.
    #[serde(
        rename = "regulationName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub regulation_name: ::std::option::Option<::std::string::String>,
    ///Verdi.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&IntentionType> for IntentionType {
    fn from(value: &IntentionType) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for IntentionType {
    fn default() -> Self {
        Self {
            id: Default::default(),
            localized_value: Default::default(),
            regulation_name: Default::default(),
            value: Default::default(),
        }
    }
}
impl IntentionType {
    pub fn builder() -> builder::IntentionType {
        Default::default()
    }
}
///Detaljert informasjon om lokalitetstilkoblingen som senest ble registrert til gitt lokalitet.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Detaljert informasjon om lokalitetstilkoblingen som senest ble registrert til gitt lokalitet.",
///  "type": "object",
///  "properties": {
///    "active": {
///      "description": "Om lokalitetstilkoblingen til tillatelsen er aktiv.",
///      "type": "boolean"
///    },
///    "key": {
///      "description": "Nøkkel",
///      "type": "string"
///    },
///    "licenseId": {
///      "description": "Unik tillatelsesidentifikator.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "licenseNr": {
///      "description": "Tillatelsesnummer",
///      "type": "string"
///    },
///    "registeredTime": {
///      "description": "Tidsstempel for nå lokalitetstilkoblingen ble registrert til gitt lokalitet.",
///      "type": "string"
///    },
///    "siteId": {
///      "description": "Unik lokalitets-identifikator.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "siteName": {
///      "description": "Lokalitetsnavn",
///      "type": "string"
///    },
///    "siteNr": {
///      "description": "Lokalitetsnummer",
///      "type": "string"
///    },
///    "status": {
///      "description": "Status på lokalitetstilkoblingene til tillatelsen.",
///      "type": "string"
///    },
///    "statusValue": {
///      "description": "Statusverdi",
///      "type": "string"
///    },
///    "temporaryUntil": {
///      "description": "Tidsstempel for når lokalitetstilkoblingen er midlertidig gyldig til.",
///      "type": "string"
///    },
///    "validFrom": {
///      "description": "Tidsstempel for når lokalitetstilkoblingen ble gyldig.",
///      "type": "string"
///    },
///    "validUntil": {
///      "description": "Tidsstempel for når lokalitetstilkoblingen er gyldig til.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LatestLicenseSiteConnectionDetail {
    ///Om lokalitetstilkoblingen til tillatelsen er aktiv.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub active: ::std::option::Option<bool>,
    ///Nøkkel
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub key: ::std::option::Option<::std::string::String>,
    ///Unik tillatelsesidentifikator.
    #[serde(
        rename = "licenseId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_id: ::std::option::Option<i64>,
    ///Tillatelsesnummer
    #[serde(
        rename = "licenseNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_nr: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for nå lokalitetstilkoblingen ble registrert til gitt lokalitet.
    #[serde(
        rename = "registeredTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub registered_time: ::std::option::Option<::std::string::String>,
    ///Unik lokalitets-identifikator.
    #[serde(
        rename = "siteId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_id: ::std::option::Option<i64>,
    ///Lokalitetsnavn
    #[serde(
        rename = "siteName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_name: ::std::option::Option<::std::string::String>,
    ///Lokalitetsnummer
    #[serde(
        rename = "siteNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_nr: ::std::option::Option<::std::string::String>,
    ///Status på lokalitetstilkoblingene til tillatelsen.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    ///Statusverdi
    #[serde(
        rename = "statusValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_value: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når lokalitetstilkoblingen er midlertidig gyldig til.
    #[serde(
        rename = "temporaryUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporary_until: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når lokalitetstilkoblingen ble gyldig.
    #[serde(
        rename = "validFrom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_from: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når lokalitetstilkoblingen er gyldig til.
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_until: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&LatestLicenseSiteConnectionDetail>
for LatestLicenseSiteConnectionDetail {
    fn from(value: &LatestLicenseSiteConnectionDetail) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LatestLicenseSiteConnectionDetail {
    fn default() -> Self {
        Self {
            active: Default::default(),
            key: Default::default(),
            license_id: Default::default(),
            license_nr: Default::default(),
            registered_time: Default::default(),
            site_id: Default::default(),
            site_name: Default::default(),
            site_nr: Default::default(),
            status: Default::default(),
            status_value: Default::default(),
            temporary_until: Default::default(),
            valid_from: Default::default(),
            valid_until: Default::default(),
        }
    }
}
impl LatestLicenseSiteConnectionDetail {
    pub fn builder() -> builder::LatestLicenseSiteConnectionDetail {
        Default::default()
    }
}
///Oversikt over siste tilkoblingstillatelse til lokalitet.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Oversikt over siste tilkoblingstillatelse til lokalitet.",
///  "type": "object",
///  "properties": {
///    "licenseNr": {
///      "description": "Tillatelsesnummer",
///      "type": "string"
///    },
///    "siteNr": {
///      "description": "Lokalitetsnummer",
///      "type": "string"
///    },
///    "status": {
///      "description": "Status",
///      "type": "string"
///    },
///    "statusValue": {
///      "description": "Statusverdi.",
///      "type": "string"
///    },
///    "temporaryUntil": {
///      "description": "Tidsstempel for når den siste tilkoblingstillatelsen er midlertidig gyldig til.",
///      "type": "string"
///    },
///    "validFrom": {
///      "description": "Tidsstempel for når den siste tilkoblingstillatelsen er gyldig fra.",
///      "type": "string"
///    },
///    "validUntil": {
///      "description": "Tidsstempel for når den siste tilkoblingstillatelsen er gyldig til.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LatestLicenseSiteConnectionOverview {
    ///Tillatelsesnummer
    #[serde(
        rename = "licenseNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_nr: ::std::option::Option<::std::string::String>,
    ///Lokalitetsnummer
    #[serde(
        rename = "siteNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_nr: ::std::option::Option<::std::string::String>,
    ///Status
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    ///Statusverdi.
    #[serde(
        rename = "statusValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_value: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når den siste tilkoblingstillatelsen er midlertidig gyldig til.
    #[serde(
        rename = "temporaryUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporary_until: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når den siste tilkoblingstillatelsen er gyldig fra.
    #[serde(
        rename = "validFrom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_from: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når den siste tilkoblingstillatelsen er gyldig til.
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_until: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&LatestLicenseSiteConnectionOverview>
for LatestLicenseSiteConnectionOverview {
    fn from(value: &LatestLicenseSiteConnectionOverview) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LatestLicenseSiteConnectionOverview {
    fn default() -> Self {
        Self {
            license_nr: Default::default(),
            site_nr: Default::default(),
            status: Default::default(),
            status_value: Default::default(),
            temporary_until: Default::default(),
            valid_from: Default::default(),
            valid_until: Default::default(),
        }
    }
}
impl LatestLicenseSiteConnectionOverview {
    pub fn builder() -> builder::LatestLicenseSiteConnectionOverview {
        Default::default()
    }
}
///Kapasitetshistorikken til tillatelsen.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Kapasitetshistorikken til tillatelsen.",
///  "type": "object",
///  "properties": {
///    "accumulatedCapacity": {
///      "description": "Den akkumulerte kapasiteten som tillatelsen har oppnådd.",
///      "type": "number",
///      "format": "double"
///    },
///    "atLeastFrom": {
///      "type": "string"
///    },
///    "atLeastUntil": {
///      "type": "string"
///    },
///    "capacityUnitType": {
///      "description": "Enhetstypen til kapasiteten f.eks. Tonn.",
///      "type": "string"
///    },
///    "capacityValueType": {
///      "description": "Verditypen til kapasiteten.",
///      "type": "string"
///    },
///    "currentCapacity": {
///      "description": "Den nåværende kapasiteten til tillatelsen.",
///      "type": "number",
///      "format": "double"
///    },
///    "status": {
///      "description": "Status",
///      "type": "string"
///    },
///    "statusValue": {
///      "description": "Statusverdi",
///      "type": "string"
///    },
///    "type": {
///      "description": "Kapasitetstypen",
///      "type": "string"
///    },
///    "typeValue": {
///      "description": "Typeverdi.",
///      "type": "string"
///    },
///    "validFrom": {
///      "description": "Tidsstempel for når tillatelsen ble gyldig fra.",
///      "type": "string"
///    },
///    "validUntil": {
///      "description": "Tidsstempel for når tillatelsen er gyldig til.",
///      "type": "string"
///    },
///    "value": {
///      "description": "Verdi",
///      "type": "number",
///      "format": "double"
///    },
///    "valueType": {
///      "description": "Verditype",
///      "type": "string"
///    },
///    "valueTypeValue": {
///      "description": "Verdien til typeverdien.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LicenseCapacityHistory {
    #[serde(
        rename = "accumulatedCapacity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub accumulated_capacity: ::std::option::Option<f64>,
    #[serde(
        rename = "atLeastFrom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub at_least_from: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "atLeastUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub at_least_until: ::std::option::Option<::std::string::String>,
    ///Enhetstypen til kapasiteten f.eks. Tonn.
    #[serde(
        rename = "capacityUnitType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub capacity_unit_type: ::std::option::Option<::std::string::String>,
    ///Verditypen til kapasiteten.
    #[serde(
        rename = "capacityValueType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub capacity_value_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "currentCapacity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub current_capacity: ::std::option::Option<f64>,
    ///Status
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    ///Statusverdi
    #[serde(
        rename = "statusValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_value: ::std::option::Option<::std::string::String>,
    ///Kapasitetstypen
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    ///Typeverdi.
    #[serde(
        rename = "typeValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_value: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når tillatelsen ble gyldig fra.
    #[serde(
        rename = "validFrom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_from: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når tillatelsen er gyldig til.
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_until: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
    ///Verditype
    #[serde(
        rename = "valueType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_type: ::std::option::Option<::std::string::String>,
    ///Verdien til typeverdien.
    #[serde(
        rename = "valueTypeValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub value_type_value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&LicenseCapacityHistory> for LicenseCapacityHistory {
    fn from(value: &LicenseCapacityHistory) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LicenseCapacityHistory {
    fn default() -> Self {
        Self {
            accumulated_capacity: Default::default(),
            at_least_from: Default::default(),
            at_least_until: Default::default(),
            capacity_unit_type: Default::default(),
            capacity_value_type: Default::default(),
            current_capacity: Default::default(),
            status: Default::default(),
            status_value: Default::default(),
            type_: Default::default(),
            type_value: Default::default(),
            valid_from: Default::default(),
            valid_until: Default::default(),
            value: Default::default(),
            value_type: Default::default(),
            value_type_value: Default::default(),
        }
    }
}
impl LicenseCapacityHistory {
    pub fn builder() -> builder::LicenseCapacityHistory {
        Default::default()
    }
}
///Tillatelsesforbindelsene til en gitt lokalitet.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Tillatelsesforbindelsene til en gitt lokalitet.",
///  "type": "object",
///  "properties": {
///    "capacity": {
///      "$ref": "#/components/schemas/CapacityInfo"
///    },
///    "connectionDetail": {
///      "$ref": "#/components/schemas/LatestLicenseSiteConnectionDetail"
///    },
///    "intention": {
///      "description": "Formålet med benyttelsen av lokaliteten.",
///      "type": "string"
///    },
///    "intentionValue": {
///      "description": "Formålsverdi",
///      "type": "string"
///    },
///    "legacyLicenseNr": {
///      "description": "Arvet tillatelsesnummer.",
///      "type": "string"
///    },
///    "legalEntityName": {
///      "description": "Juridisk enhetsnavn",
///      "type": "string"
///    },
///    "legalEntityNrId": {
///      "description": "Juridisk-enhetsnummer-identifikator",
///      "type": "string"
///    },
///    "openLegalEntityNr": {
///      "description": "Åpent juridisk enhetsnummer",
///      "type": "string"
///    },
///    "productionStage": {
///      "description": "Produksjonsstadiet lokaliteten og dens forbindelser er på.",
///      "type": "string"
///    },
///    "productionStageValue": {
///      "description": "Produksjonsstadie-verdi",
///      "type": "string"
///    },
///    "status": {
///      "description": "Status på tillatelsesforbindelsene til lokaliteten.",
///      "type": "string"
///    },
///    "statusValue": {
///      "description": "Statusverdi",
///      "type": "string"
///    },
///    "tag": {
///      "description": "Etikett.",
///      "type": "string"
///    },
///    "temporaryUntil": {
///      "description": "Tidsstempel for når tillatelsesforbindelsen er midlertidig gyldig til.",
///      "type": "string"
///    },
///    "validFrom": {
///      "description": "Tidsstempel for når tillatelsesforbindelsen er gyldig fra.",
///      "type": "string"
///    },
///    "validUntil": {
///      "description": "Tidsstempel for når tillatelsesforbindelsen er gyldig til.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LicenseConnectionForSite {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub capacity: ::std::option::Option<CapacityInfo>,
    #[serde(
        rename = "connectionDetail",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub connection_detail: ::std::option::Option<LatestLicenseSiteConnectionDetail>,
    ///Formålet med benyttelsen av lokaliteten.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub intention: ::std::option::Option<::std::string::String>,
    ///Formålsverdi
    #[serde(
        rename = "intentionValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub intention_value: ::std::option::Option<::std::string::String>,
    ///Arvet tillatelsesnummer.
    #[serde(
        rename = "legacyLicenseNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legacy_license_nr: ::std::option::Option<::std::string::String>,
    ///Juridisk enhetsnavn
    #[serde(
        rename = "legalEntityName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legal_entity_name: ::std::option::Option<::std::string::String>,
    ///Juridisk-enhetsnummer-identifikator
    #[serde(
        rename = "legalEntityNrId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legal_entity_nr_id: ::std::option::Option<::std::string::String>,
    ///Åpent juridisk enhetsnummer
    #[serde(
        rename = "openLegalEntityNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_legal_entity_nr: ::std::option::Option<::std::string::String>,
    ///Produksjonsstadiet lokaliteten og dens forbindelser er på.
    #[serde(
        rename = "productionStage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub production_stage: ::std::option::Option<::std::string::String>,
    ///Produksjonsstadie-verdi
    #[serde(
        rename = "productionStageValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub production_stage_value: ::std::option::Option<::std::string::String>,
    ///Status på tillatelsesforbindelsene til lokaliteten.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    ///Statusverdi
    #[serde(
        rename = "statusValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_value: ::std::option::Option<::std::string::String>,
    ///Etikett.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tag: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når tillatelsesforbindelsen er midlertidig gyldig til.
    #[serde(
        rename = "temporaryUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporary_until: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når tillatelsesforbindelsen er gyldig fra.
    #[serde(
        rename = "validFrom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_from: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når tillatelsesforbindelsen er gyldig til.
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_until: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&LicenseConnectionForSite> for LicenseConnectionForSite {
    fn from(value: &LicenseConnectionForSite) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LicenseConnectionForSite {
    fn default() -> Self {
        Self {
            capacity: Default::default(),
            connection_detail: Default::default(),
            intention: Default::default(),
            intention_value: Default::default(),
            legacy_license_nr: Default::default(),
            legal_entity_name: Default::default(),
            legal_entity_nr_id: Default::default(),
            open_legal_entity_nr: Default::default(),
            production_stage: Default::default(),
            production_stage_value: Default::default(),
            status: Default::default(),
            status_value: Default::default(),
            tag: Default::default(),
            temporary_until: Default::default(),
            valid_from: Default::default(),
            valid_until: Default::default(),
        }
    }
}
impl LicenseConnectionForSite {
    pub fn builder() -> builder::LicenseConnectionForSite {
        Default::default()
    }
}
///Informasjon om vedtak for denne tillatelsen.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Informasjon om vedtak for denne tillatelsen.",
///  "type": "object",
///  "properties": {
///    "caseGrantedTime": {
///      "description": "Tidsstempel for den gitte fristen til saken.",
///      "type": "string"
///    },
///    "caseType": {
///      "description": "Sakstype.",
///      "type": "string"
///    },
///    "caseTypeValue": {
///      "description": "Sakstype verdi.",
///      "type": "string"
///    },
///    "decisionInRefToLicenseNr": {
///      "description": "Vedtak i referanse til tillatelsesnummer.",
///      "type": "string"
///    },
///    "id": {
///      "description": "Unik identifikator for vedtaket.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "licenseId": {
///      "description": "Unik tillatelses identifikator.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "licenseNr": {
///      "description": "Tillatelsesnummer for tillatelsen som vedtaket gjelder for.",
///      "type": "string"
///    },
///    "licenseVersionId": {
///      "description": "Unik tillatelsesversjons identifikator.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "registeredTime": {
///      "description": "Tidsstempel for når vedtaket ble registrert.",
///      "type": "string"
///    },
///    "status": {
///      "description": "Status",
///      "type": "string"
///    },
///    "statusValue": {
///      "description": "Statusverdi.",
///      "type": "string"
///    },
///    "type": {
///      "description": "Type vedtak.",
///      "type": "string"
///    },
///    "typeValue": {
///      "description": "Verdien til denne typen vedtak.",
///      "type": "string"
///    },
///    "validFrom": {
///      "description": "Tidsstempel for når vedtaket er gyldig fra.",
///      "type": "string"
///    },
///    "validUntil": {
///      "description": "Tidsstempel for når vedtaket er gyldig til.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LicenseDecision {
    ///Tidsstempel for den gitte fristen til saken.
    #[serde(
        rename = "caseGrantedTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub case_granted_time: ::std::option::Option<::std::string::String>,
    ///Sakstype.
    #[serde(
        rename = "caseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub case_type: ::std::option::Option<::std::string::String>,
    ///Sakstype verdi.
    #[serde(
        rename = "caseTypeValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub case_type_value: ::std::option::Option<::std::string::String>,
    ///Vedtak i referanse til tillatelsesnummer.
    #[serde(
        rename = "decisionInRefToLicenseNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub decision_in_ref_to_license_nr: ::std::option::Option<::std::string::String>,
    ///Unik identifikator for vedtaket.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<i64>,
    ///Unik tillatelses identifikator.
    #[serde(
        rename = "licenseId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_id: ::std::option::Option<i64>,
    ///Tillatelsesnummer for tillatelsen som vedtaket gjelder for.
    #[serde(
        rename = "licenseNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_nr: ::std::option::Option<::std::string::String>,
    ///Unik tillatelsesversjons identifikator.
    #[serde(
        rename = "licenseVersionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_version_id: ::std::option::Option<i64>,
    ///Tidsstempel for når vedtaket ble registrert.
    #[serde(
        rename = "registeredTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub registered_time: ::std::option::Option<::std::string::String>,
    ///Status
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    ///Statusverdi.
    #[serde(
        rename = "statusValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_value: ::std::option::Option<::std::string::String>,
    ///Type vedtak.
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    ///Verdien til denne typen vedtak.
    #[serde(
        rename = "typeValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_value: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når vedtaket er gyldig fra.
    #[serde(
        rename = "validFrom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_from: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når vedtaket er gyldig til.
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_until: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&LicenseDecision> for LicenseDecision {
    fn from(value: &LicenseDecision) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LicenseDecision {
    fn default() -> Self {
        Self {
            case_granted_time: Default::default(),
            case_type: Default::default(),
            case_type_value: Default::default(),
            decision_in_ref_to_license_nr: Default::default(),
            id: Default::default(),
            license_id: Default::default(),
            license_nr: Default::default(),
            license_version_id: Default::default(),
            registered_time: Default::default(),
            status: Default::default(),
            status_value: Default::default(),
            type_: Default::default(),
            type_value: Default::default(),
            valid_from: Default::default(),
            valid_until: Default::default(),
        }
    }
}
impl LicenseDecision {
    pub fn builder() -> builder::LicenseDecision {
        Default::default()
    }
}
///Detaljert informasjon om tillatelser gitt til en innehaver.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Detaljert informasjon om tillatelser gitt til en innehaver.",
///  "type": "object",
///  "properties": {
///    "capacity": {
///      "$ref": "#/components/schemas/CapacityInfo"
///    },
///    "connections": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/LatestLicenseSiteConnectionDetail"
///      }
///    },
///    "grantInformation": {
///      "$ref": "#/components/schemas/GrantInfo"
///    },
///    "legacyLicenseNr": {
///      "description": "Arvet tillatelsesnummer",
///      "type": "string"
///    },
///    "legalEntityName": {
///      "description": "Juridisk enhetsnavn.",
///      "type": "string"
///    },
///    "legalEntityNrId": {
///      "description": "Juridisk-enhetsnummer-identifikator.",
///      "type": "string"
///    },
///    "licenseId": {
///      "description": "Unik tillatelses-identifikator",
///      "type": "integer",
///      "format": "int64"
///    },
///    "licenseNr": {
///      "description": "Tillatelsesnummer",
///      "type": "string"
///    },
///    "openLegalEntityNr": {
///      "description": "Åpent juridisk enhetsnummer.",
///      "type": "string"
///    },
///    "originalLicenseNr": {
///      "description": "Tillatelsesnummer som ble brukt når tillatelsen ble registrert.",
///      "type": "string"
///    },
///    "placement": {
///      "$ref": "#/components/schemas/AreaPlacement"
///    },
///    "portfolioMasterLicenseId": {
///      "description": "Identifikatoren for hovedtillatelsen i tillatelsesporteføljen.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "portfolioType": {
///      "$ref": "#/components/schemas/LicensePortfolioType"
///    },
///    "productionModel": {
///      "description": "Informasjon om produksjonsmodellen til tillatelsen.",
///      "type": "string"
///    },
///    "productionRegime": {
///      "description": "Informasjon om produksjonsregimet til tillatelsen.",
///      "type": "string"
///    },
///    "species": {
///      "$ref": "#/components/schemas/SpeciesDetail"
///    },
///    "type": {
///      "$ref": "#/components/schemas/LicenseTypeDetail"
///    },
///    "version": {
///      "$ref": "#/components/schemas/VersionDetail"
///    },
///    "versionId": {
///      "description": "Versjons-ID",
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LicenseDetail {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub capacity: ::std::option::Option<CapacityInfo>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub connections: ::std::vec::Vec<LatestLicenseSiteConnectionDetail>,
    #[serde(
        rename = "grantInformation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub grant_information: ::std::option::Option<GrantInfo>,
    ///Arvet tillatelsesnummer
    #[serde(
        rename = "legacyLicenseNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legacy_license_nr: ::std::option::Option<::std::string::String>,
    ///Juridisk enhetsnavn.
    #[serde(
        rename = "legalEntityName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legal_entity_name: ::std::option::Option<::std::string::String>,
    ///Juridisk-enhetsnummer-identifikator.
    #[serde(
        rename = "legalEntityNrId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legal_entity_nr_id: ::std::option::Option<::std::string::String>,
    ///Unik tillatelses-identifikator
    #[serde(
        rename = "licenseId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_id: ::std::option::Option<i64>,
    ///Tillatelsesnummer
    #[serde(
        rename = "licenseNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_nr: ::std::option::Option<::std::string::String>,
    ///Åpent juridisk enhetsnummer.
    #[serde(
        rename = "openLegalEntityNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_legal_entity_nr: ::std::option::Option<::std::string::String>,
    ///Tillatelsesnummer som ble brukt når tillatelsen ble registrert.
    #[serde(
        rename = "originalLicenseNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub original_license_nr: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub placement: ::std::option::Option<AreaPlacement>,
    ///Identifikatoren for hovedtillatelsen i tillatelsesporteføljen.
    #[serde(
        rename = "portfolioMasterLicenseId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub portfolio_master_license_id: ::std::option::Option<i64>,
    #[serde(
        rename = "portfolioType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub portfolio_type: ::std::option::Option<LicensePortfolioType>,
    ///Informasjon om produksjonsmodellen til tillatelsen.
    #[serde(
        rename = "productionModel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub production_model: ::std::option::Option<::std::string::String>,
    ///Informasjon om produksjonsregimet til tillatelsen.
    #[serde(
        rename = "productionRegime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub production_regime: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub species: ::std::option::Option<SpeciesDetail>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<LicenseTypeDetail>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<VersionDetail>,
    ///Versjons-ID
    #[serde(
        rename = "versionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_id: ::std::option::Option<i64>,
}
impl ::std::convert::From<&LicenseDetail> for LicenseDetail {
    fn from(value: &LicenseDetail) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LicenseDetail {
    fn default() -> Self {
        Self {
            capacity: Default::default(),
            connections: Default::default(),
            grant_information: Default::default(),
            legacy_license_nr: Default::default(),
            legal_entity_name: Default::default(),
            legal_entity_nr_id: Default::default(),
            license_id: Default::default(),
            license_nr: Default::default(),
            open_legal_entity_nr: Default::default(),
            original_license_nr: Default::default(),
            placement: Default::default(),
            portfolio_master_license_id: Default::default(),
            portfolio_type: Default::default(),
            production_model: Default::default(),
            production_regime: Default::default(),
            species: Default::default(),
            type_: Default::default(),
            version: Default::default(),
            version_id: Default::default(),
        }
    }
}
impl LicenseDetail {
    pub fn builder() -> builder::LicenseDetail {
        Default::default()
    }
}
///Oversikt over tillatelser som hører til innehaver.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Oversikt over tillatelser som hører til innehaver.",
///  "type": "object",
///  "properties": {
///    "capacity": {
///      "$ref": "#/components/schemas/CapacityInfo"
///    },
///    "connections": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/LatestLicenseSiteConnectionOverview"
///      }
///    },
///    "legacyLicenseNr": {
///      "description": "Arvet tillatelsesnummer.",
///      "type": "string"
///    },
///    "legalEntityName": {
///      "description": "Juridisk enhetsnavn.",
///      "type": "string"
///    },
///    "legalEntityNrId": {
///      "description": "Juridisk-enhetsnummer-identifikator",
///      "type": "string"
///    },
///    "licenseNr": {
///      "description": "Tillatelsesnummer",
///      "type": "string"
///    },
///    "openLegalEntityNr": {
///      "description": "Åpent juridisk enhetsnummer.",
///      "type": "string"
///    },
///    "originalLicenseNr": {
///      "description": "Tillatelsesnummer slik det var da innehaveren fikk tillatelsen.",
///      "type": "string"
///    },
///    "placement": {
///      "$ref": "#/components/schemas/AreaPlacement"
///    },
///    "portfolioMasterLicenseId": {
///      "description": "Unik identifikator for hovedtillatelsen i innehaverens portefølje over tillatelser.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "portfolioType": {
///      "$ref": "#/components/schemas/LicensePortfolioType"
///    },
///    "productionModel": {
///      "description": "Modellen for hvordan produksjonsprosessen skal være.",
///      "type": "string"
///    },
///    "productionRegime": {
///      "description": "Regimet som produksjonsprosessen for den juridiske enheten opererer under.",
///      "type": "string"
///    },
///    "species": {
///      "$ref": "#/components/schemas/SpeciesOverview"
///    },
///    "type": {
///      "$ref": "#/components/schemas/LicenseTypeOverview"
///    },
///    "version": {
///      "$ref": "#/components/schemas/VersionOverview"
///    },
///    "versionId": {
///      "description": "Unik identifikator for tillatelsesversjonen.",
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LicenseOverview {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub capacity: ::std::option::Option<CapacityInfo>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub connections: ::std::vec::Vec<LatestLicenseSiteConnectionOverview>,
    ///Arvet tillatelsesnummer.
    #[serde(
        rename = "legacyLicenseNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legacy_license_nr: ::std::option::Option<::std::string::String>,
    ///Juridisk enhetsnavn.
    #[serde(
        rename = "legalEntityName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legal_entity_name: ::std::option::Option<::std::string::String>,
    ///Juridisk-enhetsnummer-identifikator
    #[serde(
        rename = "legalEntityNrId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub legal_entity_nr_id: ::std::option::Option<::std::string::String>,
    ///Tillatelsesnummer
    #[serde(
        rename = "licenseNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_nr: ::std::option::Option<::std::string::String>,
    ///Åpent juridisk enhetsnummer.
    #[serde(
        rename = "openLegalEntityNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_legal_entity_nr: ::std::option::Option<::std::string::String>,
    ///Tillatelsesnummer slik det var da innehaveren fikk tillatelsen.
    #[serde(
        rename = "originalLicenseNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub original_license_nr: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub placement: ::std::option::Option<AreaPlacement>,
    ///Unik identifikator for hovedtillatelsen i innehaverens portefølje over tillatelser.
    #[serde(
        rename = "portfolioMasterLicenseId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub portfolio_master_license_id: ::std::option::Option<i64>,
    #[serde(
        rename = "portfolioType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub portfolio_type: ::std::option::Option<LicensePortfolioType>,
    ///Modellen for hvordan produksjonsprosessen skal være.
    #[serde(
        rename = "productionModel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub production_model: ::std::option::Option<::std::string::String>,
    ///Regimet som produksjonsprosessen for den juridiske enheten opererer under.
    #[serde(
        rename = "productionRegime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub production_regime: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub species: ::std::option::Option<SpeciesOverview>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<LicenseTypeOverview>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<VersionOverview>,
    ///Unik identifikator for tillatelsesversjonen.
    #[serde(
        rename = "versionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_id: ::std::option::Option<i64>,
}
impl ::std::convert::From<&LicenseOverview> for LicenseOverview {
    fn from(value: &LicenseOverview) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LicenseOverview {
    fn default() -> Self {
        Self {
            capacity: Default::default(),
            connections: Default::default(),
            legacy_license_nr: Default::default(),
            legal_entity_name: Default::default(),
            legal_entity_nr_id: Default::default(),
            license_nr: Default::default(),
            open_legal_entity_nr: Default::default(),
            original_license_nr: Default::default(),
            placement: Default::default(),
            portfolio_master_license_id: Default::default(),
            portfolio_type: Default::default(),
            production_model: Default::default(),
            production_regime: Default::default(),
            species: Default::default(),
            type_: Default::default(),
            version: Default::default(),
            version_id: Default::default(),
        }
    }
}
impl LicenseOverview {
    pub fn builder() -> builder::LicenseOverview {
        Default::default()
    }
}
///Porteføljetypen til tillatelsen.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Porteføljetypen til tillatelsen.",
///  "type": "object",
///  "properties": {
///    "apiSearchValue": {
///      "description": "Verdi brukt til å spesifisere søknadskriterier ved forespørsel mot API.",
///      "type": "string"
///    },
///    "name": {
///      "description": "Navnet på porteføljetypen.",
///      "type": "string"
///    },
///    "value": {
///      "description": "Verdi",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LicensePortfolioType {
    ///Verdi brukt til å spesifisere søknadskriterier ved forespørsel mot API.
    #[serde(
        rename = "apiSearchValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub api_search_value: ::std::option::Option<::std::string::String>,
    ///Navnet på porteføljetypen.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Verdi
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&LicensePortfolioType> for LicensePortfolioType {
    fn from(value: &LicensePortfolioType) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LicensePortfolioType {
    fn default() -> Self {
        Self {
            api_search_value: Default::default(),
            name: Default::default(),
            value: Default::default(),
        }
    }
}
impl LicensePortfolioType {
    pub fn builder() -> builder::LicensePortfolioType {
        Default::default()
    }
}
///Informasjon om overføringer av tillatelser mellom juridiske enheter.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Informasjon om overføringer av tillatelser mellom juridiske enheter.",
///  "type": "object",
///  "properties": {
///    "ajourDate": {
///      "description": "Ajour-dato for når overføringen av tillatelsen ble gjennomført.",
///      "type": "string"
///    },
///    "transfers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/Transfer"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LicenseTransfers {
    ///Ajour-dato for når overføringen av tillatelsen ble gjennomført.
    #[serde(
        rename = "ajourDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ajour_date: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub transfers: ::std::vec::Vec<Transfer>,
}
impl ::std::convert::From<&LicenseTransfers> for LicenseTransfers {
    fn from(value: &LicenseTransfers) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LicenseTransfers {
    fn default() -> Self {
        Self {
            ajour_date: Default::default(),
            transfers: Default::default(),
        }
    }
}
impl LicenseTransfers {
    pub fn builder() -> builder::LicenseTransfers {
        Default::default()
    }
}
///Detaljert informasjon om tillatelsestypen.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Detaljert informasjon om tillatelsestypen.",
///  "type": "object",
///  "properties": {
///    "allocationInformation": {
///      "description": "Informasjon om fordelingen av ressurser eller kvoter på denne typen tillatelse.",
///      "type": "string"
///    },
///    "feeding": {
///      "description": "Informasjon om fôrforbruk på tillatelsen.",
///      "type": "string"
///    },
///    "feedingValue": {
///      "description": "Fôrforbruk-verdi.",
///      "type": "string"
///    },
///    "intention": {
///      "description": "Unik identifikator for tillatelsestypen.",
///      "type": "string"
///    },
///    "intentionValue": {
///      "description": "Formålsverdi",
///      "type": "string"
///    },
///    "produces": {
///      "description": "Informasjon om hva som blir produsert av denne typen tillatelse.",
///      "type": "string"
///    },
///    "producesValue": {
///      "description": "Produksjonsprosess-verdi.",
///      "type": "string"
///    },
///    "productionStage": {
///      "description": "Informasjon om den nåværende fasen av produksjonsprosessen.",
///      "type": "string"
///    },
///    "productionStageValue": {
///      "description": "Produksjonsprosess verdi.",
///      "type": "string"
///    },
///    "stocking": {
///      "description": "Informasjon om lagringsprosessen på denne typen tillatelse.",
///      "type": "string"
///    },
///    "stockingValue": {
///      "description": "Lagringsverdi",
///      "type": "string"
///    },
///    "tag": {
///      "description": "Etikett f.eks. KOMM-MATF.",
///      "type": "string"
///    },
///    "templateId": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "temporal": {
///      "description": "Informasjon om midlertidig tillatelsestype.",
///      "type": "string"
///    },
///    "temporalValue": {
///      "description": "Midlertidig verdi.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LicenseTypeDetail {
    ///Informasjon om fordelingen av ressurser eller kvoter på denne typen tillatelse.
    #[serde(
        rename = "allocationInformation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allocation_information: ::std::option::Option<::std::string::String>,
    ///Informasjon om fôrforbruk på tillatelsen.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub feeding: ::std::option::Option<::std::string::String>,
    ///Fôrforbruk-verdi.
    #[serde(
        rename = "feedingValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub feeding_value: ::std::option::Option<::std::string::String>,
    ///Unik identifikator for tillatelsestypen.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub intention: ::std::option::Option<::std::string::String>,
    ///Formålsverdi
    #[serde(
        rename = "intentionValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub intention_value: ::std::option::Option<::std::string::String>,
    ///Informasjon om hva som blir produsert av denne typen tillatelse.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub produces: ::std::option::Option<::std::string::String>,
    ///Produksjonsprosess-verdi.
    #[serde(
        rename = "producesValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub produces_value: ::std::option::Option<::std::string::String>,
    ///Informasjon om den nåværende fasen av produksjonsprosessen.
    #[serde(
        rename = "productionStage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub production_stage: ::std::option::Option<::std::string::String>,
    ///Produksjonsprosess verdi.
    #[serde(
        rename = "productionStageValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub production_stage_value: ::std::option::Option<::std::string::String>,
    ///Informasjon om lagringsprosessen på denne typen tillatelse.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stocking: ::std::option::Option<::std::string::String>,
    ///Lagringsverdi
    #[serde(
        rename = "stockingValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stocking_value: ::std::option::Option<::std::string::String>,
    ///Etikett f.eks. KOMM-MATF.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tag: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "templateId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub template_id: ::std::option::Option<i32>,
    ///Informasjon om midlertidig tillatelsestype.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub temporal: ::std::option::Option<::std::string::String>,
    ///Midlertidig verdi.
    #[serde(
        rename = "temporalValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporal_value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&LicenseTypeDetail> for LicenseTypeDetail {
    fn from(value: &LicenseTypeDetail) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LicenseTypeDetail {
    fn default() -> Self {
        Self {
            allocation_information: Default::default(),
            feeding: Default::default(),
            feeding_value: Default::default(),
            intention: Default::default(),
            intention_value: Default::default(),
            produces: Default::default(),
            produces_value: Default::default(),
            production_stage: Default::default(),
            production_stage_value: Default::default(),
            stocking: Default::default(),
            stocking_value: Default::default(),
            tag: Default::default(),
            template_id: Default::default(),
            temporal: Default::default(),
            temporal_value: Default::default(),
        }
    }
}
impl LicenseTypeDetail {
    pub fn builder() -> builder::LicenseTypeDetail {
        Default::default()
    }
}
///Oversikt over tillatelsestyper.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Oversikt over tillatelsestyper.",
///  "type": "object",
///  "properties": {
///    "allocationInformation": {
///      "description": "Informasjon om fordelingen av ressurser på tillatelsen.",
///      "type": "string"
///    },
///    "intention": {
///      "description": "Formålet med tillatelsestypen.",
///      "type": "string"
///    },
///    "intentionValue": {
///      "description": "Formålsverdi",
///      "type": "string"
///    },
///    "produces": {
///      "description": "Hva som blir produsert på tillatelsen.",
///      "type": "string"
///    },
///    "producesValue": {
///      "description": "Produksjonsverdi.",
///      "type": "string"
///    },
///    "productionStage": {
///      "description": "Den nåværende fasen av produksjonsprosessen til tillatelsen.",
///      "type": "string"
///    },
///    "productionStageValue": {
///      "description": "Verdien til produksjonsprosess-fasen.",
///      "type": "string"
///    },
///    "tag": {
///      "description": "Etikett som f.eks. KOMM-MATF.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LicenseTypeOverview {
    ///Informasjon om fordelingen av ressurser på tillatelsen.
    #[serde(
        rename = "allocationInformation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub allocation_information: ::std::option::Option<::std::string::String>,
    ///Formålet med tillatelsestypen.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub intention: ::std::option::Option<::std::string::String>,
    ///Formålsverdi
    #[serde(
        rename = "intentionValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub intention_value: ::std::option::Option<::std::string::String>,
    ///Hva som blir produsert på tillatelsen.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub produces: ::std::option::Option<::std::string::String>,
    ///Produksjonsverdi.
    #[serde(
        rename = "producesValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub produces_value: ::std::option::Option<::std::string::String>,
    ///Den nåværende fasen av produksjonsprosessen til tillatelsen.
    #[serde(
        rename = "productionStage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub production_stage: ::std::option::Option<::std::string::String>,
    ///Verdien til produksjonsprosess-fasen.
    #[serde(
        rename = "productionStageValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub production_stage_value: ::std::option::Option<::std::string::String>,
    ///Etikett som f.eks. KOMM-MATF.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tag: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&LicenseTypeOverview> for LicenseTypeOverview {
    fn from(value: &LicenseTypeOverview) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LicenseTypeOverview {
    fn default() -> Self {
        Self {
            allocation_information: Default::default(),
            intention: Default::default(),
            intention_value: Default::default(),
            produces: Default::default(),
            produces_value: Default::default(),
            production_stage: Default::default(),
            production_stage_value: Default::default(),
            tag: Default::default(),
        }
    }
}
impl LicenseTypeOverview {
    pub fn builder() -> builder::LicenseTypeOverview {
        Default::default()
    }
}
///Informasjon om panterett.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Informasjon om panterett.",
///  "type": "object",
///  "properties": {
///    "amount": {
///      "description": "Mengde som inngår i panteretten.",
///      "type": "number",
///      "format": "double"
///    },
///    "currency": {
///      "description": "Valutaen som gjelder for verdien som er sikret av panteretten.",
///      "type": "string"
///    },
///    "journalDate": {
///      "description": "Tidsstempel for journaldato for panteretten.",
///      "type": "string"
///    },
///    "journalNr": {
///      "description": "Journalnummer.",
///      "type": "string"
///    },
///    "lienholder": {
///      "$ref": "#/components/schemas/Lienholder"
///    },
///    "registeredOwner": {
///      "$ref": "#/components/schemas/RegisteredLicenseOwner"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Lien {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub amount: ::std::option::Option<f64>,
    ///Valutaen som gjelder for verdien som er sikret av panteretten.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub currency: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for journaldato for panteretten.
    #[serde(
        rename = "journalDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub journal_date: ::std::option::Option<::std::string::String>,
    ///Journalnummer.
    #[serde(
        rename = "journalNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub journal_nr: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lienholder: ::std::option::Option<Lienholder>,
    #[serde(
        rename = "registeredOwner",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub registered_owner: ::std::option::Option<RegisteredLicenseOwner>,
}
impl ::std::convert::From<&Lien> for Lien {
    fn from(value: &Lien) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Lien {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            currency: Default::default(),
            journal_date: Default::default(),
            journal_nr: Default::default(),
            lienholder: Default::default(),
            registered_owner: Default::default(),
        }
    }
}
impl Lien {
    pub fn builder() -> builder::Lien {
        Default::default()
    }
}
///Informasjon om panterettholdere.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Informasjon om panterettholdere.",
///  "type": "object",
///  "properties": {
///    "ajourDate": {
///      "description": "Ajour-dato for panteretten.",
///      "type": "string"
///    },
///    "liens": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/Lien"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LienHolders {
    ///Ajour-dato for panteretten.
    #[serde(
        rename = "ajourDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ajour_date: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub liens: ::std::vec::Vec<Lien>,
}
impl ::std::convert::From<&LienHolders> for LienHolders {
    fn from(value: &LienHolders) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LienHolders {
    fn default() -> Self {
        Self {
            ajour_date: Default::default(),
            liens: Default::default(),
        }
    }
}
impl LienHolders {
    pub fn builder() -> builder::LienHolders {
        Default::default()
    }
}
///Lienholder
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "city": {
///      "type": "string"
///    },
///    "country": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "orgNr": {
///      "type": "string"
///    },
///    "zipCode": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Lienholder {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub city: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "orgNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub org_nr: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "zipCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub zip_code: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Lienholder> for Lienholder {
    fn from(value: &Lienholder) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Lienholder {
    fn default() -> Self {
        Self {
            city: Default::default(),
            country: Default::default(),
            name: Default::default(),
            org_nr: Default::default(),
            zip_code: Default::default(),
        }
    }
}
impl Lienholder {
    pub fn builder() -> builder::Lienholder {
        Default::default()
    }
}
///RegisteredLicenseOwner
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "name": {
///      "type": "string"
///    },
///    "orgNr": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RegisteredLicenseOwner {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "orgNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub org_nr: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&RegisteredLicenseOwner> for RegisteredLicenseOwner {
    fn from(value: &RegisteredLicenseOwner) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for RegisteredLicenseOwner {
    fn default() -> Self {
        Self {
            name: Default::default(),
            org_nr: Default::default(),
        }
    }
}
impl RegisteredLicenseOwner {
    pub fn builder() -> builder::RegisteredLicenseOwner {
        Default::default()
    }
}
///Lokalitet
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Lokalitet",
///  "type": "object",
///  "properties": {
///    "capacity": {
///      "description": "Kapasiteten til lokaliteten",
///      "type": "number",
///      "format": "double"
///    },
///    "capacityUnitType": {
///      "description": "Kapasitets enhetstype",
///      "type": "string"
///    },
///    "connections": {
///      "description": "Forbindelsene til lokaliteten",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/LatestLicenseSiteConnectionDetail"
///      }
///    },
///    "firstClearanceTime": {
///      "description": "Første klareringstiden til lokaliteten",
///      "type": "string"
///    },
///    "firstClearanceType": {
///      "description": "Første klareringstypen til lokaliteten",
///      "type": "string"
///    },
///    "firstClearanceTypeValue": {
///      "description": "Første klareringstypeverdien til lokaliteten",
///      "type": "string"
///    },
///    "hasColocation": {
///      "description": "Om lokaliteten er en del av samlokalitet",
///      "type": "boolean"
///    },
///    "hasCommercialActivity": {
///      "description": "Om lokaliteten har kommersiell aktivitet eller ikke",
///      "type": "boolean"
///    },
///    "hasJointOperation": {
///      "description": "Om lokaliteten har felles drift med annen lokalitet",
///      "type": "boolean"
///    },
///    "isSlaughtery": {
///      "description": "Om lokalitet er slakteri eller ikke",
///      "type": "boolean"
///    },
///    "latitude": {
///      "description": "Breddegrad",
///      "type": "number",
///      "format": "double"
///    },
///    "longitude": {
///      "description": "Lengdegrad",
///      "type": "number",
///      "format": "double"
///    },
///    "name": {
///      "description": "Navnet på lokaliteten",
///      "type": "string"
///    },
///    "obsoleteConnections": {
///      "description": "Utdaterte forbindelser til lokaliteten",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/LatestLicenseSiteConnectionDetail"
///      }
///    },
///    "placement": {
///      "$ref": "#/components/schemas/AreaPlacement"
///    },
///    "placementType": {
///      "description": "Plasseringstypen til lokaliteten",
///      "type": "string"
///    },
///    "placementTypeValue": {
///      "description": "Plasseringstypeverdien til lokaliteten",
///      "type": "string"
///    },
///    "siteId": {
///      "description": "Unik lokalitets identifikator",
///      "type": "integer",
///      "format": "int64"
///    },
///    "siteNr": {
///      "description": "Lokalitetsnummer",
///      "type": "integer",
///      "format": "int32"
///    },
///    "speciesLimitations": {
///      "description": "Artsbegrensninger",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/FishCode"
///      }
///    },
///    "speciesType": {
///      "description": "Artstype",
///      "type": "string"
///    },
///    "speciesTypeValue": {
///      "description": "Artstypeverdi",
///      "type": "string"
///    },
///    "tempCapacity": {
///      "description": "Den midlertidige kapasiteten til lokaliteten",
///      "type": "number",
///      "format": "double"
///    },
///    "version": {
///      "$ref": "#/components/schemas/VersionDetail"
///    },
///    "versionId": {
///      "description": "Versjons ID",
///      "type": "integer",
///      "format": "int64"
///    },
///    "waterType": {
///      "description": "Vanntype som benyttes på lokaliteten",
///      "type": "string"
///    },
///    "waterTypeValue": {
///      "description": "Vanntypeverdien på lokaliteten",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Site {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub capacity: ::std::option::Option<f64>,
    ///Kapasitets enhetstype
    #[serde(
        rename = "capacityUnitType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub capacity_unit_type: ::std::option::Option<::std::string::String>,
    ///Forbindelsene til lokaliteten
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub connections: ::std::vec::Vec<LatestLicenseSiteConnectionDetail>,
    ///Første klareringstiden til lokaliteten
    #[serde(
        rename = "firstClearanceTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_clearance_time: ::std::option::Option<::std::string::String>,
    ///Første klareringstypen til lokaliteten
    #[serde(
        rename = "firstClearanceType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_clearance_type: ::std::option::Option<::std::string::String>,
    ///Første klareringstypeverdien til lokaliteten
    #[serde(
        rename = "firstClearanceTypeValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub first_clearance_type_value: ::std::option::Option<::std::string::String>,
    ///Om lokaliteten er en del av samlokalitet
    #[serde(
        rename = "hasColocation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_colocation: ::std::option::Option<bool>,
    ///Om lokaliteten har kommersiell aktivitet eller ikke
    #[serde(
        rename = "hasCommercialActivity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_commercial_activity: ::std::option::Option<bool>,
    ///Om lokaliteten har felles drift med annen lokalitet
    #[serde(
        rename = "hasJointOperation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_joint_operation: ::std::option::Option<bool>,
    ///Om lokalitet er slakteri eller ikke
    #[serde(
        rename = "isSlaughtery",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_slaughtery: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub latitude: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub longitude: ::std::option::Option<f64>,
    ///Navnet på lokaliteten
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Utdaterte forbindelser til lokaliteten
    #[serde(
        rename = "obsoleteConnections",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub obsolete_connections: ::std::vec::Vec<LatestLicenseSiteConnectionDetail>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub placement: ::std::option::Option<AreaPlacement>,
    ///Plasseringstypen til lokaliteten
    #[serde(
        rename = "placementType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub placement_type: ::std::option::Option<::std::string::String>,
    ///Plasseringstypeverdien til lokaliteten
    #[serde(
        rename = "placementTypeValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub placement_type_value: ::std::option::Option<::std::string::String>,
    ///Unik lokalitets identifikator
    #[serde(
        rename = "siteId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_id: ::std::option::Option<i64>,
    ///Lokalitetsnummer
    #[serde(
        rename = "siteNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_nr: ::std::option::Option<i32>,
    ///Artsbegrensninger
    #[serde(
        rename = "speciesLimitations",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub species_limitations: ::std::vec::Vec<FishCode>,
    ///Artstype
    #[serde(
        rename = "speciesType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub species_type: ::std::option::Option<::std::string::String>,
    ///Artstypeverdi
    #[serde(
        rename = "speciesTypeValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub species_type_value: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "tempCapacity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temp_capacity: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<VersionDetail>,
    ///Versjons ID
    #[serde(
        rename = "versionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_id: ::std::option::Option<i64>,
    ///Vanntype som benyttes på lokaliteten
    #[serde(
        rename = "waterType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub water_type: ::std::option::Option<::std::string::String>,
    ///Vanntypeverdien på lokaliteten
    #[serde(
        rename = "waterTypeValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub water_type_value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Site> for Site {
    fn from(value: &Site) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Site {
    fn default() -> Self {
        Self {
            capacity: Default::default(),
            capacity_unit_type: Default::default(),
            connections: Default::default(),
            first_clearance_time: Default::default(),
            first_clearance_type: Default::default(),
            first_clearance_type_value: Default::default(),
            has_colocation: Default::default(),
            has_commercial_activity: Default::default(),
            has_joint_operation: Default::default(),
            is_slaughtery: Default::default(),
            latitude: Default::default(),
            longitude: Default::default(),
            name: Default::default(),
            obsolete_connections: Default::default(),
            placement: Default::default(),
            placement_type: Default::default(),
            placement_type_value: Default::default(),
            site_id: Default::default(),
            site_nr: Default::default(),
            species_limitations: Default::default(),
            species_type: Default::default(),
            species_type_value: Default::default(),
            temp_capacity: Default::default(),
            version: Default::default(),
            version_id: Default::default(),
            water_type: Default::default(),
            water_type_value: Default::default(),
        }
    }
}
impl Site {
    pub fn builder() -> builder::Site {
        Default::default()
    }
}
///Grensen til en gitt lokalitet.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Grensen til en gitt lokalitet.",
///  "type": "object",
///  "properties": {
///    "areaM2": {
///      "description": "Størrelsen på lokaliteten målt i kvadratmeter.",
///      "type": "number",
///      "format": "double"
///    },
///    "id": {
///      "description": "Unik identifikator",
///      "type": "integer",
///      "format": "int64"
///    },
///    "name": {
///      "description": "Navnet på lokaliteten.",
///      "type": "string"
///    },
///    "points": {
///      "description": "Liste over grensepunkter for en gitt lokalitet.",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/BorderPoint"
///      }
///    },
///    "siteNr": {
///      "description": "Lokalitetsnummer",
///      "type": "integer",
///      "format": "int32"
///    },
///    "siteVersionId": {
///      "description": "Unik lokalitetsversjons-identifikator",
///      "type": "integer",
///      "format": "int64"
///    },
///    "type": {
///      "$ref": "#/components/schemas/BorderType"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SiteBorder {
    #[serde(
        rename = "areaM2",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub area_m2: ::std::option::Option<f64>,
    ///Unik identifikator
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<i64>,
    ///Navnet på lokaliteten.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Liste over grensepunkter for en gitt lokalitet.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub points: ::std::vec::Vec<BorderPoint>,
    ///Lokalitetsnummer
    #[serde(
        rename = "siteNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_nr: ::std::option::Option<i32>,
    ///Unik lokalitetsversjons-identifikator
    #[serde(
        rename = "siteVersionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_version_id: ::std::option::Option<i64>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<BorderType>,
}
impl ::std::convert::From<&SiteBorder> for SiteBorder {
    fn from(value: &SiteBorder) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SiteBorder {
    fn default() -> Self {
        Self {
            area_m2: Default::default(),
            id: Default::default(),
            name: Default::default(),
            points: Default::default(),
            site_nr: Default::default(),
            site_version_id: Default::default(),
            type_: Default::default(),
        }
    }
}
impl SiteBorder {
    pub fn builder() -> builder::SiteBorder {
        Default::default()
    }
}
///Informasjon om tillatelsens lokalitetstilknytninger.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Informasjon om tillatelsens lokalitetstilknytninger.",
///  "type": "object",
///  "properties": {
///    "capacity": {
///      "description": "Kapasistet",
///      "type": "number",
///      "format": "double"
///    },
///    "capacityUnitType": {
///      "description": "Kapasitetens enhetstype f.eks. Tonn.",
///      "type": "string"
///    },
///    "connectionDetail": {
///      "$ref": "#/components/schemas/LatestLicenseSiteConnectionDetail"
///    },
///    "name": {
///      "description": "Navn",
///      "type": "string"
///    },
///    "placement": {
///      "$ref": "#/components/schemas/AreaPlacement"
///    },
///    "tempCapacity": {
///      "description": "Midlertidig kapasitet.",
///      "type": "number",
///      "format": "double"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SiteConnectionForLicense {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub capacity: ::std::option::Option<f64>,
    ///Kapasitetens enhetstype f.eks. Tonn.
    #[serde(
        rename = "capacityUnitType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub capacity_unit_type: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "connectionDetail",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub connection_detail: ::std::option::Option<LatestLicenseSiteConnectionDetail>,
    ///Navn
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub placement: ::std::option::Option<AreaPlacement>,
    #[serde(
        rename = "tempCapacity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temp_capacity: ::std::option::Option<f64>,
}
impl ::std::convert::From<&SiteConnectionForLicense> for SiteConnectionForLicense {
    fn from(value: &SiteConnectionForLicense) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SiteConnectionForLicense {
    fn default() -> Self {
        Self {
            capacity: Default::default(),
            capacity_unit_type: Default::default(),
            connection_detail: Default::default(),
            name: Default::default(),
            placement: Default::default(),
            temp_capacity: Default::default(),
        }
    }
}
impl SiteConnectionForLicense {
    pub fn builder() -> builder::SiteConnectionForLicense {
        Default::default()
    }
}
///Vedtak som gjelder for lokaliteten.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Vedtak som gjelder for lokaliteten.",
///  "type": "object",
///  "properties": {
///    "caseGrantedTime": {
///      "description": "Tidsstempel for når vedtaket ble satt.",
///      "type": "string"
///    },
///    "caseType": {
///      "description": "Sakstype",
///      "type": "string"
///    },
///    "caseTypeValue": {
///      "description": "Sakstypeverdi",
///      "type": "string"
///    },
///    "id": {
///      "description": "Unik identifikator",
///      "type": "integer",
///      "format": "int64"
///    },
///    "registeredTime": {
///      "description": "Tidsstempel for når vedtaket ble registrert.",
///      "type": "string"
///    },
///    "siteId": {
///      "description": "Unik lokalitets-identifikator",
///      "type": "integer",
///      "format": "int64"
///    },
///    "siteNr": {
///      "description": "Lokalitetsnummer",
///      "type": "integer",
///      "format": "int32"
///    },
///    "siteVersionId": {
///      "description": "Lokalitets-versjons-identifikator.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "status": {
///      "description": "Status på vedtaket.",
///      "type": "string"
///    },
///    "statusValue": {
///      "description": "Statusverdi",
///      "type": "string"
///    },
///    "type": {
///      "description": "Vedtakstype",
///      "type": "string"
///    },
///    "typeValue": {
///      "description": "Vedtakstype-verdi",
///      "type": "string"
///    },
///    "validFrom": {
///      "description": "Tidsstempel for når vedtaket er gyldig fra.",
///      "type": "string"
///    },
///    "validUntil": {
///      "description": "Tidsstempel for når vedtaket er gyldig til.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SiteDecision {
    ///Tidsstempel for når vedtaket ble satt.
    #[serde(
        rename = "caseGrantedTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub case_granted_time: ::std::option::Option<::std::string::String>,
    ///Sakstype
    #[serde(
        rename = "caseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub case_type: ::std::option::Option<::std::string::String>,
    ///Sakstypeverdi
    #[serde(
        rename = "caseTypeValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub case_type_value: ::std::option::Option<::std::string::String>,
    ///Unik identifikator
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<i64>,
    ///Tidsstempel for når vedtaket ble registrert.
    #[serde(
        rename = "registeredTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub registered_time: ::std::option::Option<::std::string::String>,
    ///Unik lokalitets-identifikator
    #[serde(
        rename = "siteId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_id: ::std::option::Option<i64>,
    ///Lokalitetsnummer
    #[serde(
        rename = "siteNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_nr: ::std::option::Option<i32>,
    ///Lokalitets-versjons-identifikator.
    #[serde(
        rename = "siteVersionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_version_id: ::std::option::Option<i64>,
    ///Status på vedtaket.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    ///Statusverdi
    #[serde(
        rename = "statusValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_value: ::std::option::Option<::std::string::String>,
    ///Vedtakstype
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    ///Vedtakstype-verdi
    #[serde(
        rename = "typeValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_value: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når vedtaket er gyldig fra.
    #[serde(
        rename = "validFrom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_from: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når vedtaket er gyldig til.
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_until: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SiteDecision> for SiteDecision {
    fn from(value: &SiteDecision) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SiteDecision {
    fn default() -> Self {
        Self {
            case_granted_time: Default::default(),
            case_type: Default::default(),
            case_type_value: Default::default(),
            id: Default::default(),
            registered_time: Default::default(),
            site_id: Default::default(),
            site_nr: Default::default(),
            site_version_id: Default::default(),
            status: Default::default(),
            status_value: Default::default(),
            type_: Default::default(),
            type_value: Default::default(),
            valid_from: Default::default(),
            valid_until: Default::default(),
        }
    }
}
impl SiteDecision {
    pub fn builder() -> builder::SiteDecision {
        Default::default()
    }
}
///Lokalitet som blir benyttet av gitt juridisk enhet.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Lokalitet som blir benyttet av gitt juridisk enhet.",
///  "type": "object",
///  "properties": {
///    "connections": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/LicenseConnectionForSite"
///      }
///    },
///    "siteCapacity": {
///      "description": "Tillatt kapasitet for den gitte lokaliteten.",
///      "type": "number",
///      "format": "double"
///    },
///    "siteCapacityUnitType": {
///      "description": "Kapasitetsenheten som blir brukt for å beskrive kapasitetsmengden på lokaliteten. Oppgitt i f.eks. Tonn.",
///      "type": "string"
///    },
///    "siteName": {
///      "description": "Navnet på lokaliteten.",
///      "type": "string"
///    },
///    "siteNr": {
///      "description": "Lokalitetsnummer",
///      "type": "string"
///    },
///    "sitePlacement": {
///      "$ref": "#/components/schemas/AreaPlacement"
///    },
///    "siteTempCapacity": {
///      "description": "Den midlertidige kapasiteten til en lokalitet.",
///      "type": "number",
///      "format": "double"
///    },
///    "status": {
///      "description": "Status",
///      "type": "string"
///    },
///    "statusValue": {
///      "description": "Statusverdi",
///      "type": "string"
///    },
///    "temporaryUntil": {
///      "description": "Tidsstempel for når den gjeldende lokaliteten er midlertidig gyldig til for den juridiske enheten.",
///      "type": "string"
///    },
///    "validFrom": {
///      "description": "Tidsstempel for når lokaliteten begynte å bli benyttet av den juridiske enheten.",
///      "type": "string"
///    },
///    "validUntil": {
///      "description": "Tidsstempel for når lokaliteten blir benyttet til av den juridiske enheten.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SiteForLegalEntity {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub connections: ::std::vec::Vec<LicenseConnectionForSite>,
    #[serde(
        rename = "siteCapacity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_capacity: ::std::option::Option<f64>,
    ///Kapasitetsenheten som blir brukt for å beskrive kapasitetsmengden på lokaliteten. Oppgitt i f.eks. Tonn.
    #[serde(
        rename = "siteCapacityUnitType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_capacity_unit_type: ::std::option::Option<::std::string::String>,
    ///Navnet på lokaliteten.
    #[serde(
        rename = "siteName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_name: ::std::option::Option<::std::string::String>,
    ///Lokalitetsnummer
    #[serde(
        rename = "siteNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_nr: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "sitePlacement",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_placement: ::std::option::Option<AreaPlacement>,
    #[serde(
        rename = "siteTempCapacity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub site_temp_capacity: ::std::option::Option<f64>,
    ///Status
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    ///Statusverdi
    #[serde(
        rename = "statusValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_value: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når den gjeldende lokaliteten er midlertidig gyldig til for den juridiske enheten.
    #[serde(
        rename = "temporaryUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporary_until: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når lokaliteten begynte å bli benyttet av den juridiske enheten.
    #[serde(
        rename = "validFrom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_from: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når lokaliteten blir benyttet til av den juridiske enheten.
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_until: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SiteForLegalEntity> for SiteForLegalEntity {
    fn from(value: &SiteForLegalEntity) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SiteForLegalEntity {
    fn default() -> Self {
        Self {
            connections: Default::default(),
            site_capacity: Default::default(),
            site_capacity_unit_type: Default::default(),
            site_name: Default::default(),
            site_nr: Default::default(),
            site_placement: Default::default(),
            site_temp_capacity: Default::default(),
            status: Default::default(),
            status_value: Default::default(),
            temporary_until: Default::default(),
            valid_from: Default::default(),
            valid_until: Default::default(),
        }
    }
}
impl SiteForLegalEntity {
    pub fn builder() -> builder::SiteForLegalEntity {
        Default::default()
    }
}
///SpeciesDetail
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "compositionId": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "fishCodes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/FishCode"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SpeciesDetail {
    #[serde(
        rename = "compositionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub composition_id: ::std::option::Option<i64>,
    #[serde(
        rename = "fishCodes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub fish_codes: ::std::vec::Vec<FishCode>,
}
impl ::std::convert::From<&SpeciesDetail> for SpeciesDetail {
    fn from(value: &SpeciesDetail) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SpeciesDetail {
    fn default() -> Self {
        Self {
            composition_id: Default::default(),
            fish_codes: Default::default(),
        }
    }
}
impl SpeciesDetail {
    pub fn builder() -> builder::SpeciesDetail {
        Default::default()
    }
}
///Oversikt over arter.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Oversikt over arter.",
///  "type": "object",
///  "properties": {
///    "compositionId": {
///      "description": "Unik artskomposisjons-identifikator.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "fishCodes": {
///      "description": "Liste over fiskekoder.",
///      "type": "array",
///      "items": {
///        "description": "Liste over fiskekoder.",
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SpeciesOverview {
    ///Unik artskomposisjons-identifikator.
    #[serde(
        rename = "compositionId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub composition_id: ::std::option::Option<i64>,
    ///Liste over fiskekoder.
    #[serde(
        rename = "fishCodes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub fish_codes: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&SpeciesOverview> for SpeciesOverview {
    fn from(value: &SpeciesOverview) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SpeciesOverview {
    fn default() -> Self {
        Self {
            composition_id: Default::default(),
            fish_codes: Default::default(),
        }
    }
}
impl SpeciesOverview {
    pub fn builder() -> builder::SpeciesOverview {
        Default::default()
    }
}
///Overføring av tillatelser mellom juridiske enheter.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Overføring av tillatelser mellom juridiske enheter.",
///  "type": "object",
///  "properties": {
///    "identityNr": {
///      "description": "Unikt identitetsnummer.",
///      "type": "string"
///    },
///    "journalDate": {
///      "description": "Tidsstempel for når journalen ble gjort.",
///      "type": "string"
///    },
///    "journalNr": {
///      "description": "Journal nummer for overføringen.",
///      "type": "string"
///    },
///    "officialName": {
///      "description": "Offisielt navn.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Transfer {
    ///Unikt identitetsnummer.
    #[serde(
        rename = "identityNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub identity_nr: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når journalen ble gjort.
    #[serde(
        rename = "journalDate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub journal_date: ::std::option::Option<::std::string::String>,
    ///Journal nummer for overføringen.
    #[serde(
        rename = "journalNr",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub journal_nr: ::std::option::Option<::std::string::String>,
    ///Offisielt navn.
    #[serde(
        rename = "officialName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub official_name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Transfer> for Transfer {
    fn from(value: &Transfer) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Transfer {
    fn default() -> Self {
        Self {
            identity_nr: Default::default(),
            journal_date: Default::default(),
            journal_nr: Default::default(),
            official_name: Default::default(),
        }
    }
}
impl Transfer {
    pub fn builder() -> builder::Transfer {
        Default::default()
    }
}
///Detaljert informasjon om en bestemt versjon av forskjellige data.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Detaljert informasjon om en bestemt versjon av forskjellige data.",
///  "type": "object",
///  "properties": {
///    "registeredTime": {
///      "description": "Tidsstempel for disse versjonsdataene sin registrering.",
///      "type": "string"
///    },
///    "status": {
///      "description": "Status på versjonen av lokalitetsdataene.",
///      "type": "string"
///    },
///    "statusValue": {
///      "description": "Statusverdi",
///      "type": "string"
///    },
///    "temporaryUntil": {
///      "description": "Tidsstempel for når disse versjonsdataene er midlertidig gyldig til.",
///      "type": "string"
///    },
///    "validFrom": {
///      "description": "Tidsstempel for når disse versjonsdataene er gyldig fra.",
///      "type": "string"
///    },
///    "validUntil": {
///      "description": "Tidsstempel for når disse versjonsdataene er gyldig til.",
///      "type": "string"
///    },
///    "versionCauseType": {
///      "description": "Årsakstypen til versjonsdataene.",
///      "type": "string"
///    },
///    "versionCauseTypeValue": {
///      "description": "Årsakstypeverdien",
///      "type": "string"
///    },
///    "versionableStatus": {
///      "description": "Versjonsbar status",
///      "type": "string"
///    },
///    "versionableStatusValue": {
///      "description": "Versjonsbar statusverdi",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VersionDetail {
    ///Tidsstempel for disse versjonsdataene sin registrering.
    #[serde(
        rename = "registeredTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub registered_time: ::std::option::Option<::std::string::String>,
    ///Status på versjonen av lokalitetsdataene.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    ///Statusverdi
    #[serde(
        rename = "statusValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_value: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når disse versjonsdataene er midlertidig gyldig til.
    #[serde(
        rename = "temporaryUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporary_until: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når disse versjonsdataene er gyldig fra.
    #[serde(
        rename = "validFrom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_from: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når disse versjonsdataene er gyldig til.
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_until: ::std::option::Option<::std::string::String>,
    ///Årsakstypen til versjonsdataene.
    #[serde(
        rename = "versionCauseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_cause_type: ::std::option::Option<::std::string::String>,
    ///Årsakstypeverdien
    #[serde(
        rename = "versionCauseTypeValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_cause_type_value: ::std::option::Option<::std::string::String>,
    ///Versjonsbar status
    #[serde(
        rename = "versionableStatus",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub versionable_status: ::std::option::Option<::std::string::String>,
    ///Versjonsbar statusverdi
    #[serde(
        rename = "versionableStatusValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub versionable_status_value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&VersionDetail> for VersionDetail {
    fn from(value: &VersionDetail) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for VersionDetail {
    fn default() -> Self {
        Self {
            registered_time: Default::default(),
            status: Default::default(),
            status_value: Default::default(),
            temporary_until: Default::default(),
            valid_from: Default::default(),
            valid_until: Default::default(),
            version_cause_type: Default::default(),
            version_cause_type_value: Default::default(),
            versionable_status: Default::default(),
            versionable_status_value: Default::default(),
        }
    }
}
impl VersionDetail {
    pub fn builder() -> builder::VersionDetail {
        Default::default()
    }
}
///Oversikt over versjoner.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Oversikt over versjoner.",
///  "type": "object",
///  "properties": {
///    "status": {
///      "description": "Status.",
///      "type": "string"
///    },
///    "statusValue": {
///      "description": "Statusverdi.",
///      "type": "string"
///    },
///    "temporaryUntil": {
///      "description": "Tidsstempel for når versjonen er midlertidig gyldig til.",
///      "type": "string"
///    },
///    "validFrom": {
///      "description": "Tidsstempel for når versjonen er gyldig fra.",
///      "type": "string"
///    },
///    "validUntil": {
///      "description": "Tidsstempel for når versjonen er gyldig til.",
///      "type": "string"
///    },
///    "versionCauseType": {
///      "description": "Årsakstype for versjonen.",
///      "type": "string"
///    },
///    "versionCauseTypeValue": {
///      "description": "Årsakstype-verdi.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VersionOverview {
    ///Status.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    ///Statusverdi.
    #[serde(
        rename = "statusValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_value: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når versjonen er midlertidig gyldig til.
    #[serde(
        rename = "temporaryUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub temporary_until: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når versjonen er gyldig fra.
    #[serde(
        rename = "validFrom",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_from: ::std::option::Option<::std::string::String>,
    ///Tidsstempel for når versjonen er gyldig til.
    #[serde(
        rename = "validUntil",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub valid_until: ::std::option::Option<::std::string::String>,
    ///Årsakstype for versjonen.
    #[serde(
        rename = "versionCauseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_cause_type: ::std::option::Option<::std::string::String>,
    ///Årsakstype-verdi.
    #[serde(
        rename = "versionCauseTypeValue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_cause_type_value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&VersionOverview> for VersionOverview {
    fn from(value: &VersionOverview) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for VersionOverview {
    fn default() -> Self {
        Self {
            status: Default::default(),
            status_value: Default::default(),
            temporary_until: Default::default(),
            valid_from: Default::default(),
            valid_until: Default::default(),
            version_cause_type: Default::default(),
            version_cause_type_value: Default::default(),
        }
    }
}
impl VersionOverview {
    pub fn builder() -> builder::VersionOverview {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Address {
        country_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        country_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        county_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        county_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        official_source_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        zip_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        zip_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Address {
        fn default() -> Self {
            Self {
                country_code: Ok(Default::default()),
                country_name: Ok(Default::default()),
                county_code: Ok(Default::default()),
                county_name: Ok(Default::default()),
                id: Ok(Default::default()),
                official_source_type: Ok(Default::default()),
                type_: Ok(Default::default()),
                value: Ok(Default::default()),
                zip_code: Ok(Default::default()),
                zip_name: Ok(Default::default()),
            }
        }
    }
    impl Address {
        pub fn country_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.country_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for country_code: {}", e)
                });
            self
        }
        pub fn country_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.country_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for country_name: {}", e)
                });
            self
        }
        pub fn county_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.county_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for county_code: {}", e)
                });
            self
        }
        pub fn county_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.county_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for county_name: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn official_source_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.official_source_type = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for official_source_type: {}", e
                    )
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
        pub fn zip_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.zip_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for zip_code: {}", e)
                });
            self
        }
        pub fn zip_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.zip_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for zip_name: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Address> for super::Address {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Address,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                country_code: value.country_code?,
                country_name: value.country_name?,
                county_code: value.county_code?,
                county_name: value.county_name?,
                id: value.id?,
                official_source_type: value.official_source_type?,
                type_: value.type_?,
                value: value.value?,
                zip_code: value.zip_code?,
                zip_name: value.zip_name?,
            })
        }
    }
    impl ::std::convert::From<super::Address> for Address {
        fn from(value: super::Address) -> Self {
            Self {
                country_code: Ok(value.country_code),
                country_name: Ok(value.country_name),
                county_code: Ok(value.county_code),
                county_name: Ok(value.county_name),
                id: Ok(value.id),
                official_source_type: Ok(value.official_source_type),
                type_: Ok(value.type_),
                value: Ok(value.value),
                zip_code: Ok(value.zip_code),
                zip_name: Ok(value.zip_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AreaListItem {
        default_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        default_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        short_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_from: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        version_next_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AreaListItem {
        fn default() -> Self {
            Self {
                default_code: Ok(Default::default()),
                default_name: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Ok(Default::default()),
                short_name: Ok(Default::default()),
                type_: Ok(Default::default()),
                valid_from: Ok(Default::default()),
                valid_until: Ok(Default::default()),
                version_id: Ok(Default::default()),
                version_next_id: Ok(Default::default()),
            }
        }
    }
    impl AreaListItem {
        pub fn default_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for default_code: {}", e)
                });
            self
        }
        pub fn default_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for default_name: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn short_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.short_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for short_name: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_from: {}", e)
                });
            self
        }
        pub fn valid_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_until: {}", e)
                });
            self
        }
        pub fn version_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version_id: {}", e)
                });
            self
        }
        pub fn version_next_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_next_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version_next_id: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AreaListItem> for super::AreaListItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AreaListItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default_code: value.default_code?,
                default_name: value.default_name?,
                id: value.id?,
                name: value.name?,
                short_name: value.short_name?,
                type_: value.type_?,
                valid_from: value.valid_from?,
                valid_until: value.valid_until?,
                version_id: value.version_id?,
                version_next_id: value.version_next_id?,
            })
        }
    }
    impl ::std::convert::From<super::AreaListItem> for AreaListItem {
        fn from(value: super::AreaListItem) -> Self {
            Self {
                default_code: Ok(value.default_code),
                default_name: Ok(value.default_name),
                id: Ok(value.id),
                name: Ok(value.name),
                short_name: Ok(value.short_name),
                type_: Ok(value.type_),
                valid_from: Ok(value.valid_from),
                valid_until: Ok(value.valid_until),
                version_id: Ok(value.version_id),
                version_next_id: Ok(value.version_next_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AreaPlacement {
        county_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        county_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        municipality_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        municipality_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        prod_area_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        prod_area_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        prod_area_status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AreaPlacement {
        fn default() -> Self {
            Self {
                county_code: Ok(Default::default()),
                county_name: Ok(Default::default()),
                municipality_code: Ok(Default::default()),
                municipality_name: Ok(Default::default()),
                prod_area_code: Ok(Default::default()),
                prod_area_name: Ok(Default::default()),
                prod_area_status: Ok(Default::default()),
            }
        }
    }
    impl AreaPlacement {
        pub fn county_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.county_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for county_code: {}", e)
                });
            self
        }
        pub fn county_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.county_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for county_name: {}", e)
                });
            self
        }
        pub fn municipality_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.municipality_code = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for municipality_code: {}", e
                    )
                });
            self
        }
        pub fn municipality_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.municipality_name = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for municipality_name: {}", e
                    )
                });
            self
        }
        pub fn prod_area_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.prod_area_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for prod_area_code: {}", e)
                });
            self
        }
        pub fn prod_area_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.prod_area_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for prod_area_name: {}", e)
                });
            self
        }
        pub fn prod_area_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.prod_area_status = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for prod_area_status: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AreaPlacement> for super::AreaPlacement {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AreaPlacement,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                county_code: value.county_code?,
                county_name: value.county_name?,
                municipality_code: value.municipality_code?,
                municipality_name: value.municipality_name?,
                prod_area_code: value.prod_area_code?,
                prod_area_name: value.prod_area_name?,
                prod_area_status: value.prod_area_status?,
            })
        }
    }
    impl ::std::convert::From<super::AreaPlacement> for AreaPlacement {
        fn from(value: super::AreaPlacement) -> Self {
            Self {
                county_code: Ok(value.county_code),
                county_name: Ok(value.county_name),
                municipality_code: Ok(value.municipality_code),
                municipality_name: Ok(value.municipality_name),
                prod_area_code: Ok(value.prod_area_code),
                prod_area_name: Ok(value.prod_area_name),
                prod_area_status: Ok(value.prod_area_status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BorderPoint {
        id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        index: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        latitude: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        longitude: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BorderPoint {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                index: Ok(Default::default()),
                latitude: Ok(Default::default()),
                longitude: Ok(Default::default()),
            }
        }
    }
    impl BorderPoint {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn index<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.index = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for index: {}", e)
                });
            self
        }
        pub fn latitude<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.latitude = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for latitude: {}", e)
                });
            self
        }
        pub fn longitude<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.longitude = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for longitude: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<BorderPoint> for super::BorderPoint {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BorderPoint,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                index: value.index?,
                latitude: value.latitude?,
                longitude: value.longitude?,
            })
        }
    }
    impl ::std::convert::From<super::BorderPoint> for BorderPoint {
        fn from(value: super::BorderPoint) -> Self {
            Self {
                id: Ok(value.id),
                index: Ok(value.index),
                latitude: Ok(value.latitude),
                longitude: Ok(value.longitude),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BorderType {
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BorderType {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl BorderType {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<BorderType> for super::BorderType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BorderType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::BorderType> for BorderType {
        fn from(value: super::BorderType) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BusinessType {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BusinessType {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl BusinessType {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<BusinessType> for super::BusinessType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BusinessType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::BusinessType> for BusinessType {
        fn from(value: super::BusinessType) -> Self {
            Self {
                name: Ok(value.name),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CapacityInfo {
        accumulated: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        current: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        unit: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CapacityInfo {
        fn default() -> Self {
            Self {
                accumulated: Ok(Default::default()),
                current: Ok(Default::default()),
                type_: Ok(Default::default()),
                unit: Ok(Default::default()),
            }
        }
    }
    impl CapacityInfo {
        pub fn accumulated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.accumulated = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for accumulated: {}", e)
                });
            self
        }
        pub fn current<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.current = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for current: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn unit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.unit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unit: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CapacityInfo> for super::CapacityInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CapacityInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accumulated: value.accumulated?,
                current: value.current?,
                type_: value.type_?,
                unit: value.unit?,
            })
        }
    }
    impl ::std::convert::From<super::CapacityInfo> for CapacityInfo {
        fn from(value: super::CapacityInfo) -> Self {
            Self {
                accumulated: Ok(value.accumulated),
                current: Ok(value.current),
                type_: Ok(value.type_),
                unit: Ok(value.unit),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Entity {
        addresses: ::std::result::Result<
            ::std::vec::Vec<super::Address>,
            ::std::string::String,
        >,
        brreg_statuses: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        business_types: ::std::result::Result<
            ::std::vec::Vec<super::BusinessType>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        industry_codes: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        official_source_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        open_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        version_registered_by: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version_registered_time: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version_valid_from: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version_valid_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Entity {
        fn default() -> Self {
            Self {
                addresses: Ok(Default::default()),
                brreg_statuses: Ok(Default::default()),
                business_types: Ok(Default::default()),
                id: Ok(Default::default()),
                industry_codes: Ok(Default::default()),
                name: Ok(Default::default()),
                official_source_type: Ok(Default::default()),
                open_nr: Ok(Default::default()),
                status: Ok(Default::default()),
                type_name: Ok(Default::default()),
                type_value: Ok(Default::default()),
                version_id: Ok(Default::default()),
                version_registered_by: Ok(Default::default()),
                version_registered_time: Ok(Default::default()),
                version_valid_from: Ok(Default::default()),
                version_valid_until: Ok(Default::default()),
            }
        }
    }
    impl Entity {
        pub fn addresses<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Address>>,
            T::Error: ::std::fmt::Display,
        {
            self.addresses = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for addresses: {}", e)
                });
            self
        }
        pub fn brreg_statuses<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.brreg_statuses = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for brreg_statuses: {}", e)
                });
            self
        }
        pub fn business_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::BusinessType>>,
            T::Error: ::std::fmt::Display,
        {
            self.business_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for business_types: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn industry_codes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.industry_codes = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for industry_codes: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn official_source_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.official_source_type = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for official_source_type: {}", e
                    )
                });
            self
        }
        pub fn open_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for open_nr: {}", e)
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn type_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_name: {}", e)
                });
            self
        }
        pub fn type_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_value: {}", e)
                });
            self
        }
        pub fn version_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version_id: {}", e)
                });
            self
        }
        pub fn version_registered_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_registered_by = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for version_registered_by: {}",
                        e
                    )
                });
            self
        }
        pub fn version_registered_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_registered_time = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for version_registered_time: {}",
                        e
                    )
                });
            self
        }
        pub fn version_valid_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_valid_from = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for version_valid_from: {}", e
                    )
                });
            self
        }
        pub fn version_valid_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_valid_until = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for version_valid_until: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Entity> for super::Entity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Entity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                addresses: value.addresses?,
                brreg_statuses: value.brreg_statuses?,
                business_types: value.business_types?,
                id: value.id?,
                industry_codes: value.industry_codes?,
                name: value.name?,
                official_source_type: value.official_source_type?,
                open_nr: value.open_nr?,
                status: value.status?,
                type_name: value.type_name?,
                type_value: value.type_value?,
                version_id: value.version_id?,
                version_registered_by: value.version_registered_by?,
                version_registered_time: value.version_registered_time?,
                version_valid_from: value.version_valid_from?,
                version_valid_until: value.version_valid_until?,
            })
        }
    }
    impl ::std::convert::From<super::Entity> for Entity {
        fn from(value: super::Entity) -> Self {
            Self {
                addresses: Ok(value.addresses),
                brreg_statuses: Ok(value.brreg_statuses),
                business_types: Ok(value.business_types),
                id: Ok(value.id),
                industry_codes: Ok(value.industry_codes),
                name: Ok(value.name),
                official_source_type: Ok(value.official_source_type),
                open_nr: Ok(value.open_nr),
                status: Ok(value.status),
                type_name: Ok(value.type_name),
                type_value: Ok(value.type_value),
                version_id: Ok(value.version_id),
                version_registered_by: Ok(value.version_registered_by),
                version_registered_time: Ok(value.version_registered_time),
                version_valid_from: Ok(value.version_valid_from),
                version_valid_until: Ok(value.version_valid_until),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ErrorResponse {
        message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ErrorResponse {
        fn default() -> Self {
            Self {
                message: Ok(Default::default()),
            }
        }
    }
    impl ErrorResponse {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for message: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ErrorResponse> for super::ErrorResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ErrorResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { message: value.message? })
        }
    }
    impl ::std::convert::From<super::ErrorResponse> for ErrorResponse {
        fn from(value: super::ErrorResponse) -> Self {
            Self { message: Ok(value.message) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FishCode {
        code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        en_gb_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        latin_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nb_no_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nn_no_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FishCode {
        fn default() -> Self {
            Self {
                code: Ok(Default::default()),
                en_gb_name: Ok(Default::default()),
                latin_name: Ok(Default::default()),
                nb_no_name: Ok(Default::default()),
                nn_no_name: Ok(Default::default()),
            }
        }
    }
    impl FishCode {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn en_gb_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.en_gb_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for en_gb_name: {}", e)
                });
            self
        }
        pub fn latin_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.latin_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for latin_name: {}", e)
                });
            self
        }
        pub fn nb_no_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.nb_no_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for nb_no_name: {}", e)
                });
            self
        }
        pub fn nn_no_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.nn_no_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for nn_no_name: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FishCode> for super::FishCode {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FishCode,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                en_gb_name: value.en_gb_name?,
                latin_name: value.latin_name?,
                nb_no_name: value.nb_no_name?,
                nn_no_name: value.nn_no_name?,
            })
        }
    }
    impl ::std::convert::From<super::FishCode> for FishCode {
        fn from(value: super::FishCode) -> Self {
            Self {
                code: Ok(value.code),
                en_gb_name: Ok(value.en_gb_name),
                latin_name: Ok(value.latin_name),
                nb_no_name: Ok(value.nb_no_name),
                nn_no_name: Ok(value.nn_no_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GrantInfo {
        capacity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        capacity_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        capacity_unit: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        granted_time: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        legal_entity_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        legal_entity_nr_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        open_legal_entity_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GrantInfo {
        fn default() -> Self {
            Self {
                capacity: Ok(Default::default()),
                capacity_type: Ok(Default::default()),
                capacity_unit: Ok(Default::default()),
                granted_time: Ok(Default::default()),
                legal_entity_name: Ok(Default::default()),
                legal_entity_nr_id: Ok(Default::default()),
                open_legal_entity_nr: Ok(Default::default()),
            }
        }
    }
    impl GrantInfo {
        pub fn capacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for capacity: {}", e)
                });
            self
        }
        pub fn capacity_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for capacity_type: {}", e)
                });
            self
        }
        pub fn capacity_unit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity_unit = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for capacity_unit: {}", e)
                });
            self
        }
        pub fn granted_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.granted_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for granted_time: {}", e)
                });
            self
        }
        pub fn legal_entity_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.legal_entity_name = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for legal_entity_name: {}", e
                    )
                });
            self
        }
        pub fn legal_entity_nr_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.legal_entity_nr_id = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for legal_entity_nr_id: {}", e
                    )
                });
            self
        }
        pub fn open_legal_entity_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_legal_entity_nr = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for open_legal_entity_nr: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<GrantInfo> for super::GrantInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GrantInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                capacity: value.capacity?,
                capacity_type: value.capacity_type?,
                capacity_unit: value.capacity_unit?,
                granted_time: value.granted_time?,
                legal_entity_name: value.legal_entity_name?,
                legal_entity_nr_id: value.legal_entity_nr_id?,
                open_legal_entity_nr: value.open_legal_entity_nr?,
            })
        }
    }
    impl ::std::convert::From<super::GrantInfo> for GrantInfo {
        fn from(value: super::GrantInfo) -> Self {
            Self {
                capacity: Ok(value.capacity),
                capacity_type: Ok(value.capacity_type),
                capacity_unit: Ok(value.capacity_unit),
                granted_time: Ok(value.granted_time),
                legal_entity_name: Ok(value.legal_entity_name),
                legal_entity_nr_id: Ok(value.legal_entity_nr_id),
                open_legal_entity_nr: Ok(value.open_legal_entity_nr),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IntentionType {
        id: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        localized_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        regulation_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for IntentionType {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                localized_value: Ok(Default::default()),
                regulation_name: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl IntentionType {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn localized_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.localized_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for localized_value: {}", e)
                });
            self
        }
        pub fn regulation_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.regulation_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for regulation_name: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<IntentionType> for super::IntentionType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IntentionType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                localized_value: value.localized_value?,
                regulation_name: value.regulation_name?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::IntentionType> for IntentionType {
        fn from(value: super::IntentionType) -> Self {
            Self {
                id: Ok(value.id),
                localized_value: Ok(value.localized_value),
                regulation_name: Ok(value.regulation_name),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LatestLicenseSiteConnectionDetail {
        active: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        key: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        license_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        license_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        registered_time: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        site_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        site_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        site_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        temporary_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_from: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LatestLicenseSiteConnectionDetail {
        fn default() -> Self {
            Self {
                active: Ok(Default::default()),
                key: Ok(Default::default()),
                license_id: Ok(Default::default()),
                license_nr: Ok(Default::default()),
                registered_time: Ok(Default::default()),
                site_id: Ok(Default::default()),
                site_name: Ok(Default::default()),
                site_nr: Ok(Default::default()),
                status: Ok(Default::default()),
                status_value: Ok(Default::default()),
                temporary_until: Ok(Default::default()),
                valid_from: Ok(Default::default()),
                valid_until: Ok(Default::default()),
            }
        }
    }
    impl LatestLicenseSiteConnectionDetail {
        pub fn active<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.active = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for active: {}", e)
                });
            self
        }
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {}", e));
            self
        }
        pub fn license_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.license_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for license_id: {}", e)
                });
            self
        }
        pub fn license_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.license_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for license_nr: {}", e)
                });
            self
        }
        pub fn registered_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.registered_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for registered_time: {}", e)
                });
            self
        }
        pub fn site_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_id: {}", e)
                });
            self
        }
        pub fn site_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_name: {}", e)
                });
            self
        }
        pub fn site_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_nr: {}", e)
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn status_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_value: {}", e)
                });
            self
        }
        pub fn temporary_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporary_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for temporary_until: {}", e)
                });
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_from: {}", e)
                });
            self
        }
        pub fn valid_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_until: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LatestLicenseSiteConnectionDetail>
    for super::LatestLicenseSiteConnectionDetail {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LatestLicenseSiteConnectionDetail,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                active: value.active?,
                key: value.key?,
                license_id: value.license_id?,
                license_nr: value.license_nr?,
                registered_time: value.registered_time?,
                site_id: value.site_id?,
                site_name: value.site_name?,
                site_nr: value.site_nr?,
                status: value.status?,
                status_value: value.status_value?,
                temporary_until: value.temporary_until?,
                valid_from: value.valid_from?,
                valid_until: value.valid_until?,
            })
        }
    }
    impl ::std::convert::From<super::LatestLicenseSiteConnectionDetail>
    for LatestLicenseSiteConnectionDetail {
        fn from(value: super::LatestLicenseSiteConnectionDetail) -> Self {
            Self {
                active: Ok(value.active),
                key: Ok(value.key),
                license_id: Ok(value.license_id),
                license_nr: Ok(value.license_nr),
                registered_time: Ok(value.registered_time),
                site_id: Ok(value.site_id),
                site_name: Ok(value.site_name),
                site_nr: Ok(value.site_nr),
                status: Ok(value.status),
                status_value: Ok(value.status_value),
                temporary_until: Ok(value.temporary_until),
                valid_from: Ok(value.valid_from),
                valid_until: Ok(value.valid_until),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LatestLicenseSiteConnectionOverview {
        license_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        site_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        temporary_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_from: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LatestLicenseSiteConnectionOverview {
        fn default() -> Self {
            Self {
                license_nr: Ok(Default::default()),
                site_nr: Ok(Default::default()),
                status: Ok(Default::default()),
                status_value: Ok(Default::default()),
                temporary_until: Ok(Default::default()),
                valid_from: Ok(Default::default()),
                valid_until: Ok(Default::default()),
            }
        }
    }
    impl LatestLicenseSiteConnectionOverview {
        pub fn license_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.license_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for license_nr: {}", e)
                });
            self
        }
        pub fn site_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_nr: {}", e)
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn status_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_value: {}", e)
                });
            self
        }
        pub fn temporary_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporary_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for temporary_until: {}", e)
                });
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_from: {}", e)
                });
            self
        }
        pub fn valid_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_until: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LatestLicenseSiteConnectionOverview>
    for super::LatestLicenseSiteConnectionOverview {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LatestLicenseSiteConnectionOverview,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                license_nr: value.license_nr?,
                site_nr: value.site_nr?,
                status: value.status?,
                status_value: value.status_value?,
                temporary_until: value.temporary_until?,
                valid_from: value.valid_from?,
                valid_until: value.valid_until?,
            })
        }
    }
    impl ::std::convert::From<super::LatestLicenseSiteConnectionOverview>
    for LatestLicenseSiteConnectionOverview {
        fn from(value: super::LatestLicenseSiteConnectionOverview) -> Self {
            Self {
                license_nr: Ok(value.license_nr),
                site_nr: Ok(value.site_nr),
                status: Ok(value.status),
                status_value: Ok(value.status_value),
                temporary_until: Ok(value.temporary_until),
                valid_from: Ok(value.valid_from),
                valid_until: Ok(value.valid_until),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LicenseCapacityHistory {
        accumulated_capacity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        at_least_from: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        at_least_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        capacity_unit_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        capacity_value_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        current_capacity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_from: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value_type_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LicenseCapacityHistory {
        fn default() -> Self {
            Self {
                accumulated_capacity: Ok(Default::default()),
                at_least_from: Ok(Default::default()),
                at_least_until: Ok(Default::default()),
                capacity_unit_type: Ok(Default::default()),
                capacity_value_type: Ok(Default::default()),
                current_capacity: Ok(Default::default()),
                status: Ok(Default::default()),
                status_value: Ok(Default::default()),
                type_: Ok(Default::default()),
                type_value: Ok(Default::default()),
                valid_from: Ok(Default::default()),
                valid_until: Ok(Default::default()),
                value: Ok(Default::default()),
                value_type: Ok(Default::default()),
                value_type_value: Ok(Default::default()),
            }
        }
    }
    impl LicenseCapacityHistory {
        pub fn accumulated_capacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.accumulated_capacity = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for accumulated_capacity: {}", e
                    )
                });
            self
        }
        pub fn at_least_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.at_least_from = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for at_least_from: {}", e)
                });
            self
        }
        pub fn at_least_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.at_least_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for at_least_until: {}", e)
                });
            self
        }
        pub fn capacity_unit_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity_unit_type = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for capacity_unit_type: {}", e
                    )
                });
            self
        }
        pub fn capacity_value_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity_value_type = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for capacity_value_type: {}", e
                    )
                });
            self
        }
        pub fn current_capacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.current_capacity = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for current_capacity: {}", e
                    )
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn status_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_value: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn type_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_value: {}", e)
                });
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_from: {}", e)
                });
            self
        }
        pub fn valid_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_until: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
        pub fn value_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value_type: {}", e)
                });
            self
        }
        pub fn value_type_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value_type_value = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for value_type_value: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LicenseCapacityHistory>
    for super::LicenseCapacityHistory {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LicenseCapacityHistory,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accumulated_capacity: value.accumulated_capacity?,
                at_least_from: value.at_least_from?,
                at_least_until: value.at_least_until?,
                capacity_unit_type: value.capacity_unit_type?,
                capacity_value_type: value.capacity_value_type?,
                current_capacity: value.current_capacity?,
                status: value.status?,
                status_value: value.status_value?,
                type_: value.type_?,
                type_value: value.type_value?,
                valid_from: value.valid_from?,
                valid_until: value.valid_until?,
                value: value.value?,
                value_type: value.value_type?,
                value_type_value: value.value_type_value?,
            })
        }
    }
    impl ::std::convert::From<super::LicenseCapacityHistory> for LicenseCapacityHistory {
        fn from(value: super::LicenseCapacityHistory) -> Self {
            Self {
                accumulated_capacity: Ok(value.accumulated_capacity),
                at_least_from: Ok(value.at_least_from),
                at_least_until: Ok(value.at_least_until),
                capacity_unit_type: Ok(value.capacity_unit_type),
                capacity_value_type: Ok(value.capacity_value_type),
                current_capacity: Ok(value.current_capacity),
                status: Ok(value.status),
                status_value: Ok(value.status_value),
                type_: Ok(value.type_),
                type_value: Ok(value.type_value),
                valid_from: Ok(value.valid_from),
                valid_until: Ok(value.valid_until),
                value: Ok(value.value),
                value_type: Ok(value.value_type),
                value_type_value: Ok(value.value_type_value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LicenseConnectionForSite {
        capacity: ::std::result::Result<
            ::std::option::Option<super::CapacityInfo>,
            ::std::string::String,
        >,
        connection_detail: ::std::result::Result<
            ::std::option::Option<super::LatestLicenseSiteConnectionDetail>,
            ::std::string::String,
        >,
        intention: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        intention_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        legacy_license_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        legal_entity_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        legal_entity_nr_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        open_legal_entity_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        production_stage: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        production_stage_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tag: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        temporary_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_from: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LicenseConnectionForSite {
        fn default() -> Self {
            Self {
                capacity: Ok(Default::default()),
                connection_detail: Ok(Default::default()),
                intention: Ok(Default::default()),
                intention_value: Ok(Default::default()),
                legacy_license_nr: Ok(Default::default()),
                legal_entity_name: Ok(Default::default()),
                legal_entity_nr_id: Ok(Default::default()),
                open_legal_entity_nr: Ok(Default::default()),
                production_stage: Ok(Default::default()),
                production_stage_value: Ok(Default::default()),
                status: Ok(Default::default()),
                status_value: Ok(Default::default()),
                tag: Ok(Default::default()),
                temporary_until: Ok(Default::default()),
                valid_from: Ok(Default::default()),
                valid_until: Ok(Default::default()),
            }
        }
    }
    impl LicenseConnectionForSite {
        pub fn capacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CapacityInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for capacity: {}", e)
                });
            self
        }
        pub fn connection_detail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LatestLicenseSiteConnectionDetail>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.connection_detail = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for connection_detail: {}", e
                    )
                });
            self
        }
        pub fn intention<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.intention = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for intention: {}", e)
                });
            self
        }
        pub fn intention_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.intention_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for intention_value: {}", e)
                });
            self
        }
        pub fn legacy_license_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.legacy_license_nr = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for legacy_license_nr: {}", e
                    )
                });
            self
        }
        pub fn legal_entity_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.legal_entity_name = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for legal_entity_name: {}", e
                    )
                });
            self
        }
        pub fn legal_entity_nr_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.legal_entity_nr_id = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for legal_entity_nr_id: {}", e
                    )
                });
            self
        }
        pub fn open_legal_entity_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_legal_entity_nr = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for open_legal_entity_nr: {}", e
                    )
                });
            self
        }
        pub fn production_stage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.production_stage = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for production_stage: {}", e
                    )
                });
            self
        }
        pub fn production_stage_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.production_stage_value = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for production_stage_value: {}",
                        e
                    )
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn status_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_value: {}", e)
                });
            self
        }
        pub fn tag<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tag = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tag: {}", e));
            self
        }
        pub fn temporary_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporary_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for temporary_until: {}", e)
                });
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_from: {}", e)
                });
            self
        }
        pub fn valid_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_until: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LicenseConnectionForSite>
    for super::LicenseConnectionForSite {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LicenseConnectionForSite,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                capacity: value.capacity?,
                connection_detail: value.connection_detail?,
                intention: value.intention?,
                intention_value: value.intention_value?,
                legacy_license_nr: value.legacy_license_nr?,
                legal_entity_name: value.legal_entity_name?,
                legal_entity_nr_id: value.legal_entity_nr_id?,
                open_legal_entity_nr: value.open_legal_entity_nr?,
                production_stage: value.production_stage?,
                production_stage_value: value.production_stage_value?,
                status: value.status?,
                status_value: value.status_value?,
                tag: value.tag?,
                temporary_until: value.temporary_until?,
                valid_from: value.valid_from?,
                valid_until: value.valid_until?,
            })
        }
    }
    impl ::std::convert::From<super::LicenseConnectionForSite>
    for LicenseConnectionForSite {
        fn from(value: super::LicenseConnectionForSite) -> Self {
            Self {
                capacity: Ok(value.capacity),
                connection_detail: Ok(value.connection_detail),
                intention: Ok(value.intention),
                intention_value: Ok(value.intention_value),
                legacy_license_nr: Ok(value.legacy_license_nr),
                legal_entity_name: Ok(value.legal_entity_name),
                legal_entity_nr_id: Ok(value.legal_entity_nr_id),
                open_legal_entity_nr: Ok(value.open_legal_entity_nr),
                production_stage: Ok(value.production_stage),
                production_stage_value: Ok(value.production_stage_value),
                status: Ok(value.status),
                status_value: Ok(value.status_value),
                tag: Ok(value.tag),
                temporary_until: Ok(value.temporary_until),
                valid_from: Ok(value.valid_from),
                valid_until: Ok(value.valid_until),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LicenseDecision {
        case_granted_time: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        case_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        case_type_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        decision_in_ref_to_license_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        license_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        license_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        license_version_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        registered_time: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_from: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LicenseDecision {
        fn default() -> Self {
            Self {
                case_granted_time: Ok(Default::default()),
                case_type: Ok(Default::default()),
                case_type_value: Ok(Default::default()),
                decision_in_ref_to_license_nr: Ok(Default::default()),
                id: Ok(Default::default()),
                license_id: Ok(Default::default()),
                license_nr: Ok(Default::default()),
                license_version_id: Ok(Default::default()),
                registered_time: Ok(Default::default()),
                status: Ok(Default::default()),
                status_value: Ok(Default::default()),
                type_: Ok(Default::default()),
                type_value: Ok(Default::default()),
                valid_from: Ok(Default::default()),
                valid_until: Ok(Default::default()),
            }
        }
    }
    impl LicenseDecision {
        pub fn case_granted_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.case_granted_time = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for case_granted_time: {}", e
                    )
                });
            self
        }
        pub fn case_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.case_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for case_type: {}", e)
                });
            self
        }
        pub fn case_type_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.case_type_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for case_type_value: {}", e)
                });
            self
        }
        pub fn decision_in_ref_to_license_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.decision_in_ref_to_license_nr = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for decision_in_ref_to_license_nr: {}",
                        e
                    )
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn license_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.license_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for license_id: {}", e)
                });
            self
        }
        pub fn license_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.license_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for license_nr: {}", e)
                });
            self
        }
        pub fn license_version_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.license_version_id = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for license_version_id: {}", e
                    )
                });
            self
        }
        pub fn registered_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.registered_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for registered_time: {}", e)
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn status_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_value: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn type_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_value: {}", e)
                });
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_from: {}", e)
                });
            self
        }
        pub fn valid_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_until: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LicenseDecision> for super::LicenseDecision {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LicenseDecision,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                case_granted_time: value.case_granted_time?,
                case_type: value.case_type?,
                case_type_value: value.case_type_value?,
                decision_in_ref_to_license_nr: value.decision_in_ref_to_license_nr?,
                id: value.id?,
                license_id: value.license_id?,
                license_nr: value.license_nr?,
                license_version_id: value.license_version_id?,
                registered_time: value.registered_time?,
                status: value.status?,
                status_value: value.status_value?,
                type_: value.type_?,
                type_value: value.type_value?,
                valid_from: value.valid_from?,
                valid_until: value.valid_until?,
            })
        }
    }
    impl ::std::convert::From<super::LicenseDecision> for LicenseDecision {
        fn from(value: super::LicenseDecision) -> Self {
            Self {
                case_granted_time: Ok(value.case_granted_time),
                case_type: Ok(value.case_type),
                case_type_value: Ok(value.case_type_value),
                decision_in_ref_to_license_nr: Ok(value.decision_in_ref_to_license_nr),
                id: Ok(value.id),
                license_id: Ok(value.license_id),
                license_nr: Ok(value.license_nr),
                license_version_id: Ok(value.license_version_id),
                registered_time: Ok(value.registered_time),
                status: Ok(value.status),
                status_value: Ok(value.status_value),
                type_: Ok(value.type_),
                type_value: Ok(value.type_value),
                valid_from: Ok(value.valid_from),
                valid_until: Ok(value.valid_until),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LicenseDetail {
        capacity: ::std::result::Result<
            ::std::option::Option<super::CapacityInfo>,
            ::std::string::String,
        >,
        connections: ::std::result::Result<
            ::std::vec::Vec<super::LatestLicenseSiteConnectionDetail>,
            ::std::string::String,
        >,
        grant_information: ::std::result::Result<
            ::std::option::Option<super::GrantInfo>,
            ::std::string::String,
        >,
        legacy_license_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        legal_entity_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        legal_entity_nr_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        license_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        license_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        open_legal_entity_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        original_license_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        placement: ::std::result::Result<
            ::std::option::Option<super::AreaPlacement>,
            ::std::string::String,
        >,
        portfolio_master_license_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        portfolio_type: ::std::result::Result<
            ::std::option::Option<super::LicensePortfolioType>,
            ::std::string::String,
        >,
        production_model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        production_regime: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        species: ::std::result::Result<
            ::std::option::Option<super::SpeciesDetail>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::LicenseTypeDetail>,
            ::std::string::String,
        >,
        version: ::std::result::Result<
            ::std::option::Option<super::VersionDetail>,
            ::std::string::String,
        >,
        version_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LicenseDetail {
        fn default() -> Self {
            Self {
                capacity: Ok(Default::default()),
                connections: Ok(Default::default()),
                grant_information: Ok(Default::default()),
                legacy_license_nr: Ok(Default::default()),
                legal_entity_name: Ok(Default::default()),
                legal_entity_nr_id: Ok(Default::default()),
                license_id: Ok(Default::default()),
                license_nr: Ok(Default::default()),
                open_legal_entity_nr: Ok(Default::default()),
                original_license_nr: Ok(Default::default()),
                placement: Ok(Default::default()),
                portfolio_master_license_id: Ok(Default::default()),
                portfolio_type: Ok(Default::default()),
                production_model: Ok(Default::default()),
                production_regime: Ok(Default::default()),
                species: Ok(Default::default()),
                type_: Ok(Default::default()),
                version: Ok(Default::default()),
                version_id: Ok(Default::default()),
            }
        }
    }
    impl LicenseDetail {
        pub fn capacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CapacityInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for capacity: {}", e)
                });
            self
        }
        pub fn connections<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::LatestLicenseSiteConnectionDetail>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.connections = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for connections: {}", e)
                });
            self
        }
        pub fn grant_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::GrantInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.grant_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for grant_information: {}", e
                    )
                });
            self
        }
        pub fn legacy_license_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.legacy_license_nr = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for legacy_license_nr: {}", e
                    )
                });
            self
        }
        pub fn legal_entity_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.legal_entity_name = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for legal_entity_name: {}", e
                    )
                });
            self
        }
        pub fn legal_entity_nr_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.legal_entity_nr_id = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for legal_entity_nr_id: {}", e
                    )
                });
            self
        }
        pub fn license_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.license_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for license_id: {}", e)
                });
            self
        }
        pub fn license_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.license_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for license_nr: {}", e)
                });
            self
        }
        pub fn open_legal_entity_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_legal_entity_nr = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for open_legal_entity_nr: {}", e
                    )
                });
            self
        }
        pub fn original_license_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.original_license_nr = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for original_license_nr: {}", e
                    )
                });
            self
        }
        pub fn placement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AreaPlacement>>,
            T::Error: ::std::fmt::Display,
        {
            self.placement = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for placement: {}", e)
                });
            self
        }
        pub fn portfolio_master_license_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.portfolio_master_license_id = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for portfolio_master_license_id: {}",
                        e
                    )
                });
            self
        }
        pub fn portfolio_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LicensePortfolioType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.portfolio_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for portfolio_type: {}", e)
                });
            self
        }
        pub fn production_model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.production_model = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for production_model: {}", e
                    )
                });
            self
        }
        pub fn production_regime<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.production_regime = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for production_regime: {}", e
                    )
                });
            self
        }
        pub fn species<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SpeciesDetail>>,
            T::Error: ::std::fmt::Display,
        {
            self.species = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for species: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LicenseTypeDetail>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VersionDetail>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version: {}", e)
                });
            self
        }
        pub fn version_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version_id: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LicenseDetail> for super::LicenseDetail {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LicenseDetail,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                capacity: value.capacity?,
                connections: value.connections?,
                grant_information: value.grant_information?,
                legacy_license_nr: value.legacy_license_nr?,
                legal_entity_name: value.legal_entity_name?,
                legal_entity_nr_id: value.legal_entity_nr_id?,
                license_id: value.license_id?,
                license_nr: value.license_nr?,
                open_legal_entity_nr: value.open_legal_entity_nr?,
                original_license_nr: value.original_license_nr?,
                placement: value.placement?,
                portfolio_master_license_id: value.portfolio_master_license_id?,
                portfolio_type: value.portfolio_type?,
                production_model: value.production_model?,
                production_regime: value.production_regime?,
                species: value.species?,
                type_: value.type_?,
                version: value.version?,
                version_id: value.version_id?,
            })
        }
    }
    impl ::std::convert::From<super::LicenseDetail> for LicenseDetail {
        fn from(value: super::LicenseDetail) -> Self {
            Self {
                capacity: Ok(value.capacity),
                connections: Ok(value.connections),
                grant_information: Ok(value.grant_information),
                legacy_license_nr: Ok(value.legacy_license_nr),
                legal_entity_name: Ok(value.legal_entity_name),
                legal_entity_nr_id: Ok(value.legal_entity_nr_id),
                license_id: Ok(value.license_id),
                license_nr: Ok(value.license_nr),
                open_legal_entity_nr: Ok(value.open_legal_entity_nr),
                original_license_nr: Ok(value.original_license_nr),
                placement: Ok(value.placement),
                portfolio_master_license_id: Ok(value.portfolio_master_license_id),
                portfolio_type: Ok(value.portfolio_type),
                production_model: Ok(value.production_model),
                production_regime: Ok(value.production_regime),
                species: Ok(value.species),
                type_: Ok(value.type_),
                version: Ok(value.version),
                version_id: Ok(value.version_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LicenseOverview {
        capacity: ::std::result::Result<
            ::std::option::Option<super::CapacityInfo>,
            ::std::string::String,
        >,
        connections: ::std::result::Result<
            ::std::vec::Vec<super::LatestLicenseSiteConnectionOverview>,
            ::std::string::String,
        >,
        legacy_license_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        legal_entity_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        legal_entity_nr_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        license_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        open_legal_entity_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        original_license_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        placement: ::std::result::Result<
            ::std::option::Option<super::AreaPlacement>,
            ::std::string::String,
        >,
        portfolio_master_license_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        portfolio_type: ::std::result::Result<
            ::std::option::Option<super::LicensePortfolioType>,
            ::std::string::String,
        >,
        production_model: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        production_regime: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        species: ::std::result::Result<
            ::std::option::Option<super::SpeciesOverview>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::LicenseTypeOverview>,
            ::std::string::String,
        >,
        version: ::std::result::Result<
            ::std::option::Option<super::VersionOverview>,
            ::std::string::String,
        >,
        version_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LicenseOverview {
        fn default() -> Self {
            Self {
                capacity: Ok(Default::default()),
                connections: Ok(Default::default()),
                legacy_license_nr: Ok(Default::default()),
                legal_entity_name: Ok(Default::default()),
                legal_entity_nr_id: Ok(Default::default()),
                license_nr: Ok(Default::default()),
                open_legal_entity_nr: Ok(Default::default()),
                original_license_nr: Ok(Default::default()),
                placement: Ok(Default::default()),
                portfolio_master_license_id: Ok(Default::default()),
                portfolio_type: Ok(Default::default()),
                production_model: Ok(Default::default()),
                production_regime: Ok(Default::default()),
                species: Ok(Default::default()),
                type_: Ok(Default::default()),
                version: Ok(Default::default()),
                version_id: Ok(Default::default()),
            }
        }
    }
    impl LicenseOverview {
        pub fn capacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CapacityInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for capacity: {}", e)
                });
            self
        }
        pub fn connections<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::LatestLicenseSiteConnectionOverview>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.connections = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for connections: {}", e)
                });
            self
        }
        pub fn legacy_license_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.legacy_license_nr = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for legacy_license_nr: {}", e
                    )
                });
            self
        }
        pub fn legal_entity_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.legal_entity_name = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for legal_entity_name: {}", e
                    )
                });
            self
        }
        pub fn legal_entity_nr_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.legal_entity_nr_id = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for legal_entity_nr_id: {}", e
                    )
                });
            self
        }
        pub fn license_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.license_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for license_nr: {}", e)
                });
            self
        }
        pub fn open_legal_entity_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_legal_entity_nr = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for open_legal_entity_nr: {}", e
                    )
                });
            self
        }
        pub fn original_license_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.original_license_nr = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for original_license_nr: {}", e
                    )
                });
            self
        }
        pub fn placement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AreaPlacement>>,
            T::Error: ::std::fmt::Display,
        {
            self.placement = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for placement: {}", e)
                });
            self
        }
        pub fn portfolio_master_license_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.portfolio_master_license_id = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for portfolio_master_license_id: {}",
                        e
                    )
                });
            self
        }
        pub fn portfolio_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LicensePortfolioType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.portfolio_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for portfolio_type: {}", e)
                });
            self
        }
        pub fn production_model<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.production_model = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for production_model: {}", e
                    )
                });
            self
        }
        pub fn production_regime<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.production_regime = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for production_regime: {}", e
                    )
                });
            self
        }
        pub fn species<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SpeciesOverview>>,
            T::Error: ::std::fmt::Display,
        {
            self.species = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for species: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LicenseTypeOverview>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VersionOverview>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version: {}", e)
                });
            self
        }
        pub fn version_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version_id: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LicenseOverview> for super::LicenseOverview {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LicenseOverview,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                capacity: value.capacity?,
                connections: value.connections?,
                legacy_license_nr: value.legacy_license_nr?,
                legal_entity_name: value.legal_entity_name?,
                legal_entity_nr_id: value.legal_entity_nr_id?,
                license_nr: value.license_nr?,
                open_legal_entity_nr: value.open_legal_entity_nr?,
                original_license_nr: value.original_license_nr?,
                placement: value.placement?,
                portfolio_master_license_id: value.portfolio_master_license_id?,
                portfolio_type: value.portfolio_type?,
                production_model: value.production_model?,
                production_regime: value.production_regime?,
                species: value.species?,
                type_: value.type_?,
                version: value.version?,
                version_id: value.version_id?,
            })
        }
    }
    impl ::std::convert::From<super::LicenseOverview> for LicenseOverview {
        fn from(value: super::LicenseOverview) -> Self {
            Self {
                capacity: Ok(value.capacity),
                connections: Ok(value.connections),
                legacy_license_nr: Ok(value.legacy_license_nr),
                legal_entity_name: Ok(value.legal_entity_name),
                legal_entity_nr_id: Ok(value.legal_entity_nr_id),
                license_nr: Ok(value.license_nr),
                open_legal_entity_nr: Ok(value.open_legal_entity_nr),
                original_license_nr: Ok(value.original_license_nr),
                placement: Ok(value.placement),
                portfolio_master_license_id: Ok(value.portfolio_master_license_id),
                portfolio_type: Ok(value.portfolio_type),
                production_model: Ok(value.production_model),
                production_regime: Ok(value.production_regime),
                species: Ok(value.species),
                type_: Ok(value.type_),
                version: Ok(value.version),
                version_id: Ok(value.version_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LicensePortfolioType {
        api_search_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LicensePortfolioType {
        fn default() -> Self {
            Self {
                api_search_value: Ok(Default::default()),
                name: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl LicensePortfolioType {
        pub fn api_search_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_search_value = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for api_search_value: {}", e
                    )
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LicensePortfolioType> for super::LicensePortfolioType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LicensePortfolioType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_search_value: value.api_search_value?,
                name: value.name?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::LicensePortfolioType> for LicensePortfolioType {
        fn from(value: super::LicensePortfolioType) -> Self {
            Self {
                api_search_value: Ok(value.api_search_value),
                name: Ok(value.name),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LicenseTransfers {
        ajour_date: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        transfers: ::std::result::Result<
            ::std::vec::Vec<super::Transfer>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LicenseTransfers {
        fn default() -> Self {
            Self {
                ajour_date: Ok(Default::default()),
                transfers: Ok(Default::default()),
            }
        }
    }
    impl LicenseTransfers {
        pub fn ajour_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ajour_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for ajour_date: {}", e)
                });
            self
        }
        pub fn transfers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Transfer>>,
            T::Error: ::std::fmt::Display,
        {
            self.transfers = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for transfers: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LicenseTransfers> for super::LicenseTransfers {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LicenseTransfers,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ajour_date: value.ajour_date?,
                transfers: value.transfers?,
            })
        }
    }
    impl ::std::convert::From<super::LicenseTransfers> for LicenseTransfers {
        fn from(value: super::LicenseTransfers) -> Self {
            Self {
                ajour_date: Ok(value.ajour_date),
                transfers: Ok(value.transfers),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LicenseTypeDetail {
        allocation_information: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        feeding: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        feeding_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        intention: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        intention_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        produces: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        produces_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        production_stage: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        production_stage_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        stocking: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        stocking_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tag: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        template_id: ::std::result::Result<
            ::std::option::Option<i32>,
            ::std::string::String,
        >,
        temporal: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        temporal_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LicenseTypeDetail {
        fn default() -> Self {
            Self {
                allocation_information: Ok(Default::default()),
                feeding: Ok(Default::default()),
                feeding_value: Ok(Default::default()),
                intention: Ok(Default::default()),
                intention_value: Ok(Default::default()),
                produces: Ok(Default::default()),
                produces_value: Ok(Default::default()),
                production_stage: Ok(Default::default()),
                production_stage_value: Ok(Default::default()),
                stocking: Ok(Default::default()),
                stocking_value: Ok(Default::default()),
                tag: Ok(Default::default()),
                template_id: Ok(Default::default()),
                temporal: Ok(Default::default()),
                temporal_value: Ok(Default::default()),
            }
        }
    }
    impl LicenseTypeDetail {
        pub fn allocation_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.allocation_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for allocation_information: {}",
                        e
                    )
                });
            self
        }
        pub fn feeding<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.feeding = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for feeding: {}", e)
                });
            self
        }
        pub fn feeding_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.feeding_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for feeding_value: {}", e)
                });
            self
        }
        pub fn intention<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.intention = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for intention: {}", e)
                });
            self
        }
        pub fn intention_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.intention_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for intention_value: {}", e)
                });
            self
        }
        pub fn produces<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.produces = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for produces: {}", e)
                });
            self
        }
        pub fn produces_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.produces_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for produces_value: {}", e)
                });
            self
        }
        pub fn production_stage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.production_stage = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for production_stage: {}", e
                    )
                });
            self
        }
        pub fn production_stage_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.production_stage_value = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for production_stage_value: {}",
                        e
                    )
                });
            self
        }
        pub fn stocking<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.stocking = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for stocking: {}", e)
                });
            self
        }
        pub fn stocking_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.stocking_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for stocking_value: {}", e)
                });
            self
        }
        pub fn tag<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tag = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tag: {}", e));
            self
        }
        pub fn template_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.template_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for template_id: {}", e)
                });
            self
        }
        pub fn temporal<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporal = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for temporal: {}", e)
                });
            self
        }
        pub fn temporal_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporal_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for temporal_value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LicenseTypeDetail> for super::LicenseTypeDetail {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LicenseTypeDetail,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allocation_information: value.allocation_information?,
                feeding: value.feeding?,
                feeding_value: value.feeding_value?,
                intention: value.intention?,
                intention_value: value.intention_value?,
                produces: value.produces?,
                produces_value: value.produces_value?,
                production_stage: value.production_stage?,
                production_stage_value: value.production_stage_value?,
                stocking: value.stocking?,
                stocking_value: value.stocking_value?,
                tag: value.tag?,
                template_id: value.template_id?,
                temporal: value.temporal?,
                temporal_value: value.temporal_value?,
            })
        }
    }
    impl ::std::convert::From<super::LicenseTypeDetail> for LicenseTypeDetail {
        fn from(value: super::LicenseTypeDetail) -> Self {
            Self {
                allocation_information: Ok(value.allocation_information),
                feeding: Ok(value.feeding),
                feeding_value: Ok(value.feeding_value),
                intention: Ok(value.intention),
                intention_value: Ok(value.intention_value),
                produces: Ok(value.produces),
                produces_value: Ok(value.produces_value),
                production_stage: Ok(value.production_stage),
                production_stage_value: Ok(value.production_stage_value),
                stocking: Ok(value.stocking),
                stocking_value: Ok(value.stocking_value),
                tag: Ok(value.tag),
                template_id: Ok(value.template_id),
                temporal: Ok(value.temporal),
                temporal_value: Ok(value.temporal_value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LicenseTypeOverview {
        allocation_information: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        intention: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        intention_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        produces: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        produces_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        production_stage: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        production_stage_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tag: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LicenseTypeOverview {
        fn default() -> Self {
            Self {
                allocation_information: Ok(Default::default()),
                intention: Ok(Default::default()),
                intention_value: Ok(Default::default()),
                produces: Ok(Default::default()),
                produces_value: Ok(Default::default()),
                production_stage: Ok(Default::default()),
                production_stage_value: Ok(Default::default()),
                tag: Ok(Default::default()),
            }
        }
    }
    impl LicenseTypeOverview {
        pub fn allocation_information<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.allocation_information = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for allocation_information: {}",
                        e
                    )
                });
            self
        }
        pub fn intention<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.intention = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for intention: {}", e)
                });
            self
        }
        pub fn intention_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.intention_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for intention_value: {}", e)
                });
            self
        }
        pub fn produces<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.produces = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for produces: {}", e)
                });
            self
        }
        pub fn produces_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.produces_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for produces_value: {}", e)
                });
            self
        }
        pub fn production_stage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.production_stage = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for production_stage: {}", e
                    )
                });
            self
        }
        pub fn production_stage_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.production_stage_value = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for production_stage_value: {}",
                        e
                    )
                });
            self
        }
        pub fn tag<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tag = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tag: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LicenseTypeOverview> for super::LicenseTypeOverview {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LicenseTypeOverview,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allocation_information: value.allocation_information?,
                intention: value.intention?,
                intention_value: value.intention_value?,
                produces: value.produces?,
                produces_value: value.produces_value?,
                production_stage: value.production_stage?,
                production_stage_value: value.production_stage_value?,
                tag: value.tag?,
            })
        }
    }
    impl ::std::convert::From<super::LicenseTypeOverview> for LicenseTypeOverview {
        fn from(value: super::LicenseTypeOverview) -> Self {
            Self {
                allocation_information: Ok(value.allocation_information),
                intention: Ok(value.intention),
                intention_value: Ok(value.intention_value),
                produces: Ok(value.produces),
                produces_value: Ok(value.produces_value),
                production_stage: Ok(value.production_stage),
                production_stage_value: Ok(value.production_stage_value),
                tag: Ok(value.tag),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Lien {
        amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        currency: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        journal_date: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        journal_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        lienholder: ::std::result::Result<
            ::std::option::Option<super::Lienholder>,
            ::std::string::String,
        >,
        registered_owner: ::std::result::Result<
            ::std::option::Option<super::RegisteredLicenseOwner>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Lien {
        fn default() -> Self {
            Self {
                amount: Ok(Default::default()),
                currency: Ok(Default::default()),
                journal_date: Ok(Default::default()),
                journal_nr: Ok(Default::default()),
                lienholder: Ok(Default::default()),
                registered_owner: Ok(Default::default()),
            }
        }
    }
    impl Lien {
        pub fn amount<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.amount = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for amount: {}", e)
                });
            self
        }
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for currency: {}", e)
                });
            self
        }
        pub fn journal_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.journal_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for journal_date: {}", e)
                });
            self
        }
        pub fn journal_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.journal_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for journal_nr: {}", e)
                });
            self
        }
        pub fn lienholder<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Lienholder>>,
            T::Error: ::std::fmt::Display,
        {
            self.lienholder = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for lienholder: {}", e)
                });
            self
        }
        pub fn registered_owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::RegisteredLicenseOwner>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.registered_owner = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for registered_owner: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Lien> for super::Lien {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Lien,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                amount: value.amount?,
                currency: value.currency?,
                journal_date: value.journal_date?,
                journal_nr: value.journal_nr?,
                lienholder: value.lienholder?,
                registered_owner: value.registered_owner?,
            })
        }
    }
    impl ::std::convert::From<super::Lien> for Lien {
        fn from(value: super::Lien) -> Self {
            Self {
                amount: Ok(value.amount),
                currency: Ok(value.currency),
                journal_date: Ok(value.journal_date),
                journal_nr: Ok(value.journal_nr),
                lienholder: Ok(value.lienholder),
                registered_owner: Ok(value.registered_owner),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LienHolders {
        ajour_date: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        liens: ::std::result::Result<
            ::std::vec::Vec<super::Lien>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LienHolders {
        fn default() -> Self {
            Self {
                ajour_date: Ok(Default::default()),
                liens: Ok(Default::default()),
            }
        }
    }
    impl LienHolders {
        pub fn ajour_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ajour_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for ajour_date: {}", e)
                });
            self
        }
        pub fn liens<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Lien>>,
            T::Error: ::std::fmt::Display,
        {
            self.liens = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for liens: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LienHolders> for super::LienHolders {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LienHolders,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ajour_date: value.ajour_date?,
                liens: value.liens?,
            })
        }
    }
    impl ::std::convert::From<super::LienHolders> for LienHolders {
        fn from(value: super::LienHolders) -> Self {
            Self {
                ajour_date: Ok(value.ajour_date),
                liens: Ok(value.liens),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Lienholder {
        city: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        country: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        org_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        zip_code: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Lienholder {
        fn default() -> Self {
            Self {
                city: Ok(Default::default()),
                country: Ok(Default::default()),
                name: Ok(Default::default()),
                org_nr: Ok(Default::default()),
                zip_code: Ok(Default::default()),
            }
        }
    }
    impl Lienholder {
        pub fn city<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.city = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for city: {}", e));
            self
        }
        pub fn country<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.country = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for country: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn org_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.org_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for org_nr: {}", e)
                });
            self
        }
        pub fn zip_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.zip_code = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for zip_code: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Lienholder> for super::Lienholder {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Lienholder,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                city: value.city?,
                country: value.country?,
                name: value.name?,
                org_nr: value.org_nr?,
                zip_code: value.zip_code?,
            })
        }
    }
    impl ::std::convert::From<super::Lienholder> for Lienholder {
        fn from(value: super::Lienholder) -> Self {
            Self {
                city: Ok(value.city),
                country: Ok(value.country),
                name: Ok(value.name),
                org_nr: Ok(value.org_nr),
                zip_code: Ok(value.zip_code),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RegisteredLicenseOwner {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        org_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for RegisteredLicenseOwner {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
                org_nr: Ok(Default::default()),
            }
        }
    }
    impl RegisteredLicenseOwner {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn org_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.org_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for org_nr: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<RegisteredLicenseOwner>
    for super::RegisteredLicenseOwner {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RegisteredLicenseOwner,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                org_nr: value.org_nr?,
            })
        }
    }
    impl ::std::convert::From<super::RegisteredLicenseOwner> for RegisteredLicenseOwner {
        fn from(value: super::RegisteredLicenseOwner) -> Self {
            Self {
                name: Ok(value.name),
                org_nr: Ok(value.org_nr),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Site {
        capacity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        capacity_unit_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        connections: ::std::result::Result<
            ::std::vec::Vec<super::LatestLicenseSiteConnectionDetail>,
            ::std::string::String,
        >,
        first_clearance_time: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        first_clearance_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        first_clearance_type_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        has_colocation: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        has_commercial_activity: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        has_joint_operation: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        is_slaughtery: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        latitude: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        longitude: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        obsolete_connections: ::std::result::Result<
            ::std::vec::Vec<super::LatestLicenseSiteConnectionDetail>,
            ::std::string::String,
        >,
        placement: ::std::result::Result<
            ::std::option::Option<super::AreaPlacement>,
            ::std::string::String,
        >,
        placement_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        placement_type_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        site_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        site_nr: ::std::result::Result<
            ::std::option::Option<i32>,
            ::std::string::String,
        >,
        species_limitations: ::std::result::Result<
            ::std::vec::Vec<super::FishCode>,
            ::std::string::String,
        >,
        species_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        species_type_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        temp_capacity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        version: ::std::result::Result<
            ::std::option::Option<super::VersionDetail>,
            ::std::string::String,
        >,
        version_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        water_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        water_type_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Site {
        fn default() -> Self {
            Self {
                capacity: Ok(Default::default()),
                capacity_unit_type: Ok(Default::default()),
                connections: Ok(Default::default()),
                first_clearance_time: Ok(Default::default()),
                first_clearance_type: Ok(Default::default()),
                first_clearance_type_value: Ok(Default::default()),
                has_colocation: Ok(Default::default()),
                has_commercial_activity: Ok(Default::default()),
                has_joint_operation: Ok(Default::default()),
                is_slaughtery: Ok(Default::default()),
                latitude: Ok(Default::default()),
                longitude: Ok(Default::default()),
                name: Ok(Default::default()),
                obsolete_connections: Ok(Default::default()),
                placement: Ok(Default::default()),
                placement_type: Ok(Default::default()),
                placement_type_value: Ok(Default::default()),
                site_id: Ok(Default::default()),
                site_nr: Ok(Default::default()),
                species_limitations: Ok(Default::default()),
                species_type: Ok(Default::default()),
                species_type_value: Ok(Default::default()),
                temp_capacity: Ok(Default::default()),
                version: Ok(Default::default()),
                version_id: Ok(Default::default()),
                water_type: Ok(Default::default()),
                water_type_value: Ok(Default::default()),
            }
        }
    }
    impl Site {
        pub fn capacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for capacity: {}", e)
                });
            self
        }
        pub fn capacity_unit_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity_unit_type = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for capacity_unit_type: {}", e
                    )
                });
            self
        }
        pub fn connections<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::LatestLicenseSiteConnectionDetail>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.connections = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for connections: {}", e)
                });
            self
        }
        pub fn first_clearance_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.first_clearance_time = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for first_clearance_time: {}", e
                    )
                });
            self
        }
        pub fn first_clearance_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.first_clearance_type = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for first_clearance_type: {}", e
                    )
                });
            self
        }
        pub fn first_clearance_type_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.first_clearance_type_value = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for first_clearance_type_value: {}",
                        e
                    )
                });
            self
        }
        pub fn has_colocation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.has_colocation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for has_colocation: {}", e)
                });
            self
        }
        pub fn has_commercial_activity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.has_commercial_activity = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for has_commercial_activity: {}",
                        e
                    )
                });
            self
        }
        pub fn has_joint_operation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.has_joint_operation = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for has_joint_operation: {}", e
                    )
                });
            self
        }
        pub fn is_slaughtery<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.is_slaughtery = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for is_slaughtery: {}", e)
                });
            self
        }
        pub fn latitude<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.latitude = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for latitude: {}", e)
                });
            self
        }
        pub fn longitude<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.longitude = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for longitude: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn obsolete_connections<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::LatestLicenseSiteConnectionDetail>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.obsolete_connections = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for obsolete_connections: {}", e
                    )
                });
            self
        }
        pub fn placement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AreaPlacement>>,
            T::Error: ::std::fmt::Display,
        {
            self.placement = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for placement: {}", e)
                });
            self
        }
        pub fn placement_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.placement_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for placement_type: {}", e)
                });
            self
        }
        pub fn placement_type_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.placement_type_value = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for placement_type_value: {}", e
                    )
                });
            self
        }
        pub fn site_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_id: {}", e)
                });
            self
        }
        pub fn site_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_nr: {}", e)
                });
            self
        }
        pub fn species_limitations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FishCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.species_limitations = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for species_limitations: {}", e
                    )
                });
            self
        }
        pub fn species_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.species_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for species_type: {}", e)
                });
            self
        }
        pub fn species_type_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.species_type_value = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for species_type_value: {}", e
                    )
                });
            self
        }
        pub fn temp_capacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temp_capacity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for temp_capacity: {}", e)
                });
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VersionDetail>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version: {}", e)
                });
            self
        }
        pub fn version_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version_id: {}", e)
                });
            self
        }
        pub fn water_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.water_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for water_type: {}", e)
                });
            self
        }
        pub fn water_type_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.water_type_value = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for water_type_value: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Site> for super::Site {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Site,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                capacity: value.capacity?,
                capacity_unit_type: value.capacity_unit_type?,
                connections: value.connections?,
                first_clearance_time: value.first_clearance_time?,
                first_clearance_type: value.first_clearance_type?,
                first_clearance_type_value: value.first_clearance_type_value?,
                has_colocation: value.has_colocation?,
                has_commercial_activity: value.has_commercial_activity?,
                has_joint_operation: value.has_joint_operation?,
                is_slaughtery: value.is_slaughtery?,
                latitude: value.latitude?,
                longitude: value.longitude?,
                name: value.name?,
                obsolete_connections: value.obsolete_connections?,
                placement: value.placement?,
                placement_type: value.placement_type?,
                placement_type_value: value.placement_type_value?,
                site_id: value.site_id?,
                site_nr: value.site_nr?,
                species_limitations: value.species_limitations?,
                species_type: value.species_type?,
                species_type_value: value.species_type_value?,
                temp_capacity: value.temp_capacity?,
                version: value.version?,
                version_id: value.version_id?,
                water_type: value.water_type?,
                water_type_value: value.water_type_value?,
            })
        }
    }
    impl ::std::convert::From<super::Site> for Site {
        fn from(value: super::Site) -> Self {
            Self {
                capacity: Ok(value.capacity),
                capacity_unit_type: Ok(value.capacity_unit_type),
                connections: Ok(value.connections),
                first_clearance_time: Ok(value.first_clearance_time),
                first_clearance_type: Ok(value.first_clearance_type),
                first_clearance_type_value: Ok(value.first_clearance_type_value),
                has_colocation: Ok(value.has_colocation),
                has_commercial_activity: Ok(value.has_commercial_activity),
                has_joint_operation: Ok(value.has_joint_operation),
                is_slaughtery: Ok(value.is_slaughtery),
                latitude: Ok(value.latitude),
                longitude: Ok(value.longitude),
                name: Ok(value.name),
                obsolete_connections: Ok(value.obsolete_connections),
                placement: Ok(value.placement),
                placement_type: Ok(value.placement_type),
                placement_type_value: Ok(value.placement_type_value),
                site_id: Ok(value.site_id),
                site_nr: Ok(value.site_nr),
                species_limitations: Ok(value.species_limitations),
                species_type: Ok(value.species_type),
                species_type_value: Ok(value.species_type_value),
                temp_capacity: Ok(value.temp_capacity),
                version: Ok(value.version),
                version_id: Ok(value.version_id),
                water_type: Ok(value.water_type),
                water_type_value: Ok(value.water_type_value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SiteBorder {
        area_m2: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        points: ::std::result::Result<
            ::std::vec::Vec<super::BorderPoint>,
            ::std::string::String,
        >,
        site_nr: ::std::result::Result<
            ::std::option::Option<i32>,
            ::std::string::String,
        >,
        site_version_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::BorderType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SiteBorder {
        fn default() -> Self {
            Self {
                area_m2: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Ok(Default::default()),
                points: Ok(Default::default()),
                site_nr: Ok(Default::default()),
                site_version_id: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl SiteBorder {
        pub fn area_m2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.area_m2 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for area_m2: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn points<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::BorderPoint>>,
            T::Error: ::std::fmt::Display,
        {
            self.points = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for points: {}", e)
                });
            self
        }
        pub fn site_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_nr: {}", e)
                });
            self
        }
        pub fn site_version_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_version_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_version_id: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BorderType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SiteBorder> for super::SiteBorder {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SiteBorder,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                area_m2: value.area_m2?,
                id: value.id?,
                name: value.name?,
                points: value.points?,
                site_nr: value.site_nr?,
                site_version_id: value.site_version_id?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::SiteBorder> for SiteBorder {
        fn from(value: super::SiteBorder) -> Self {
            Self {
                area_m2: Ok(value.area_m2),
                id: Ok(value.id),
                name: Ok(value.name),
                points: Ok(value.points),
                site_nr: Ok(value.site_nr),
                site_version_id: Ok(value.site_version_id),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SiteConnectionForLicense {
        capacity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        capacity_unit_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        connection_detail: ::std::result::Result<
            ::std::option::Option<super::LatestLicenseSiteConnectionDetail>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        placement: ::std::result::Result<
            ::std::option::Option<super::AreaPlacement>,
            ::std::string::String,
        >,
        temp_capacity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SiteConnectionForLicense {
        fn default() -> Self {
            Self {
                capacity: Ok(Default::default()),
                capacity_unit_type: Ok(Default::default()),
                connection_detail: Ok(Default::default()),
                name: Ok(Default::default()),
                placement: Ok(Default::default()),
                temp_capacity: Ok(Default::default()),
            }
        }
    }
    impl SiteConnectionForLicense {
        pub fn capacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for capacity: {}", e)
                });
            self
        }
        pub fn capacity_unit_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.capacity_unit_type = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for capacity_unit_type: {}", e
                    )
                });
            self
        }
        pub fn connection_detail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LatestLicenseSiteConnectionDetail>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.connection_detail = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for connection_detail: {}", e
                    )
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn placement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AreaPlacement>>,
            T::Error: ::std::fmt::Display,
        {
            self.placement = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for placement: {}", e)
                });
            self
        }
        pub fn temp_capacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temp_capacity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for temp_capacity: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SiteConnectionForLicense>
    for super::SiteConnectionForLicense {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SiteConnectionForLicense,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                capacity: value.capacity?,
                capacity_unit_type: value.capacity_unit_type?,
                connection_detail: value.connection_detail?,
                name: value.name?,
                placement: value.placement?,
                temp_capacity: value.temp_capacity?,
            })
        }
    }
    impl ::std::convert::From<super::SiteConnectionForLicense>
    for SiteConnectionForLicense {
        fn from(value: super::SiteConnectionForLicense) -> Self {
            Self {
                capacity: Ok(value.capacity),
                capacity_unit_type: Ok(value.capacity_unit_type),
                connection_detail: Ok(value.connection_detail),
                name: Ok(value.name),
                placement: Ok(value.placement),
                temp_capacity: Ok(value.temp_capacity),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SiteDecision {
        case_granted_time: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        case_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        case_type_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        registered_time: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        site_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        site_nr: ::std::result::Result<
            ::std::option::Option<i32>,
            ::std::string::String,
        >,
        site_version_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_from: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SiteDecision {
        fn default() -> Self {
            Self {
                case_granted_time: Ok(Default::default()),
                case_type: Ok(Default::default()),
                case_type_value: Ok(Default::default()),
                id: Ok(Default::default()),
                registered_time: Ok(Default::default()),
                site_id: Ok(Default::default()),
                site_nr: Ok(Default::default()),
                site_version_id: Ok(Default::default()),
                status: Ok(Default::default()),
                status_value: Ok(Default::default()),
                type_: Ok(Default::default()),
                type_value: Ok(Default::default()),
                valid_from: Ok(Default::default()),
                valid_until: Ok(Default::default()),
            }
        }
    }
    impl SiteDecision {
        pub fn case_granted_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.case_granted_time = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for case_granted_time: {}", e
                    )
                });
            self
        }
        pub fn case_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.case_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for case_type: {}", e)
                });
            self
        }
        pub fn case_type_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.case_type_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for case_type_value: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn registered_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.registered_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for registered_time: {}", e)
                });
            self
        }
        pub fn site_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_id: {}", e)
                });
            self
        }
        pub fn site_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_nr: {}", e)
                });
            self
        }
        pub fn site_version_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_version_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_version_id: {}", e)
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn status_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_value: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn type_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_value: {}", e)
                });
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_from: {}", e)
                });
            self
        }
        pub fn valid_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_until: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SiteDecision> for super::SiteDecision {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SiteDecision,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                case_granted_time: value.case_granted_time?,
                case_type: value.case_type?,
                case_type_value: value.case_type_value?,
                id: value.id?,
                registered_time: value.registered_time?,
                site_id: value.site_id?,
                site_nr: value.site_nr?,
                site_version_id: value.site_version_id?,
                status: value.status?,
                status_value: value.status_value?,
                type_: value.type_?,
                type_value: value.type_value?,
                valid_from: value.valid_from?,
                valid_until: value.valid_until?,
            })
        }
    }
    impl ::std::convert::From<super::SiteDecision> for SiteDecision {
        fn from(value: super::SiteDecision) -> Self {
            Self {
                case_granted_time: Ok(value.case_granted_time),
                case_type: Ok(value.case_type),
                case_type_value: Ok(value.case_type_value),
                id: Ok(value.id),
                registered_time: Ok(value.registered_time),
                site_id: Ok(value.site_id),
                site_nr: Ok(value.site_nr),
                site_version_id: Ok(value.site_version_id),
                status: Ok(value.status),
                status_value: Ok(value.status_value),
                type_: Ok(value.type_),
                type_value: Ok(value.type_value),
                valid_from: Ok(value.valid_from),
                valid_until: Ok(value.valid_until),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SiteForLegalEntity {
        connections: ::std::result::Result<
            ::std::vec::Vec<super::LicenseConnectionForSite>,
            ::std::string::String,
        >,
        site_capacity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        site_capacity_unit_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        site_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        site_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        site_placement: ::std::result::Result<
            ::std::option::Option<super::AreaPlacement>,
            ::std::string::String,
        >,
        site_temp_capacity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        temporary_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_from: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SiteForLegalEntity {
        fn default() -> Self {
            Self {
                connections: Ok(Default::default()),
                site_capacity: Ok(Default::default()),
                site_capacity_unit_type: Ok(Default::default()),
                site_name: Ok(Default::default()),
                site_nr: Ok(Default::default()),
                site_placement: Ok(Default::default()),
                site_temp_capacity: Ok(Default::default()),
                status: Ok(Default::default()),
                status_value: Ok(Default::default()),
                temporary_until: Ok(Default::default()),
                valid_from: Ok(Default::default()),
                valid_until: Ok(Default::default()),
            }
        }
    }
    impl SiteForLegalEntity {
        pub fn connections<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::LicenseConnectionForSite>>,
            T::Error: ::std::fmt::Display,
        {
            self.connections = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for connections: {}", e)
                });
            self
        }
        pub fn site_capacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_capacity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_capacity: {}", e)
                });
            self
        }
        pub fn site_capacity_unit_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_capacity_unit_type = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for site_capacity_unit_type: {}",
                        e
                    )
                });
            self
        }
        pub fn site_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_name: {}", e)
                });
            self
        }
        pub fn site_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_nr: {}", e)
                });
            self
        }
        pub fn site_placement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AreaPlacement>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_placement = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for site_placement: {}", e)
                });
            self
        }
        pub fn site_temp_capacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.site_temp_capacity = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for site_temp_capacity: {}", e
                    )
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn status_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_value: {}", e)
                });
            self
        }
        pub fn temporary_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporary_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for temporary_until: {}", e)
                });
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_from: {}", e)
                });
            self
        }
        pub fn valid_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_until: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SiteForLegalEntity> for super::SiteForLegalEntity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SiteForLegalEntity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                connections: value.connections?,
                site_capacity: value.site_capacity?,
                site_capacity_unit_type: value.site_capacity_unit_type?,
                site_name: value.site_name?,
                site_nr: value.site_nr?,
                site_placement: value.site_placement?,
                site_temp_capacity: value.site_temp_capacity?,
                status: value.status?,
                status_value: value.status_value?,
                temporary_until: value.temporary_until?,
                valid_from: value.valid_from?,
                valid_until: value.valid_until?,
            })
        }
    }
    impl ::std::convert::From<super::SiteForLegalEntity> for SiteForLegalEntity {
        fn from(value: super::SiteForLegalEntity) -> Self {
            Self {
                connections: Ok(value.connections),
                site_capacity: Ok(value.site_capacity),
                site_capacity_unit_type: Ok(value.site_capacity_unit_type),
                site_name: Ok(value.site_name),
                site_nr: Ok(value.site_nr),
                site_placement: Ok(value.site_placement),
                site_temp_capacity: Ok(value.site_temp_capacity),
                status: Ok(value.status),
                status_value: Ok(value.status_value),
                temporary_until: Ok(value.temporary_until),
                valid_from: Ok(value.valid_from),
                valid_until: Ok(value.valid_until),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpeciesDetail {
        composition_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        fish_codes: ::std::result::Result<
            ::std::vec::Vec<super::FishCode>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SpeciesDetail {
        fn default() -> Self {
            Self {
                composition_id: Ok(Default::default()),
                fish_codes: Ok(Default::default()),
            }
        }
    }
    impl SpeciesDetail {
        pub fn composition_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.composition_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for composition_id: {}", e)
                });
            self
        }
        pub fn fish_codes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FishCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.fish_codes = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for fish_codes: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SpeciesDetail> for super::SpeciesDetail {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SpeciesDetail,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                composition_id: value.composition_id?,
                fish_codes: value.fish_codes?,
            })
        }
    }
    impl ::std::convert::From<super::SpeciesDetail> for SpeciesDetail {
        fn from(value: super::SpeciesDetail) -> Self {
            Self {
                composition_id: Ok(value.composition_id),
                fish_codes: Ok(value.fish_codes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpeciesOverview {
        composition_id: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        fish_codes: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SpeciesOverview {
        fn default() -> Self {
            Self {
                composition_id: Ok(Default::default()),
                fish_codes: Ok(Default::default()),
            }
        }
    }
    impl SpeciesOverview {
        pub fn composition_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.composition_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for composition_id: {}", e)
                });
            self
        }
        pub fn fish_codes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.fish_codes = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for fish_codes: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SpeciesOverview> for super::SpeciesOverview {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SpeciesOverview,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                composition_id: value.composition_id?,
                fish_codes: value.fish_codes?,
            })
        }
    }
    impl ::std::convert::From<super::SpeciesOverview> for SpeciesOverview {
        fn from(value: super::SpeciesOverview) -> Self {
            Self {
                composition_id: Ok(value.composition_id),
                fish_codes: Ok(value.fish_codes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Transfer {
        identity_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        journal_date: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        journal_nr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        official_name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Transfer {
        fn default() -> Self {
            Self {
                identity_nr: Ok(Default::default()),
                journal_date: Ok(Default::default()),
                journal_nr: Ok(Default::default()),
                official_name: Ok(Default::default()),
            }
        }
    }
    impl Transfer {
        pub fn identity_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.identity_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for identity_nr: {}", e)
                });
            self
        }
        pub fn journal_date<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.journal_date = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for journal_date: {}", e)
                });
            self
        }
        pub fn journal_nr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.journal_nr = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for journal_nr: {}", e)
                });
            self
        }
        pub fn official_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.official_name = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for official_name: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Transfer> for super::Transfer {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Transfer,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                identity_nr: value.identity_nr?,
                journal_date: value.journal_date?,
                journal_nr: value.journal_nr?,
                official_name: value.official_name?,
            })
        }
    }
    impl ::std::convert::From<super::Transfer> for Transfer {
        fn from(value: super::Transfer) -> Self {
            Self {
                identity_nr: Ok(value.identity_nr),
                journal_date: Ok(value.journal_date),
                journal_nr: Ok(value.journal_nr),
                official_name: Ok(value.official_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VersionDetail {
        registered_time: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        temporary_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_from: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version_cause_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version_cause_type_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        versionable_status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        versionable_status_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VersionDetail {
        fn default() -> Self {
            Self {
                registered_time: Ok(Default::default()),
                status: Ok(Default::default()),
                status_value: Ok(Default::default()),
                temporary_until: Ok(Default::default()),
                valid_from: Ok(Default::default()),
                valid_until: Ok(Default::default()),
                version_cause_type: Ok(Default::default()),
                version_cause_type_value: Ok(Default::default()),
                versionable_status: Ok(Default::default()),
                versionable_status_value: Ok(Default::default()),
            }
        }
    }
    impl VersionDetail {
        pub fn registered_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.registered_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for registered_time: {}", e)
                });
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn status_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_value: {}", e)
                });
            self
        }
        pub fn temporary_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporary_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for temporary_until: {}", e)
                });
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_from: {}", e)
                });
            self
        }
        pub fn valid_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_until: {}", e)
                });
            self
        }
        pub fn version_cause_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_cause_type = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for version_cause_type: {}", e
                    )
                });
            self
        }
        pub fn version_cause_type_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_cause_type_value = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for version_cause_type_value: {}",
                        e
                    )
                });
            self
        }
        pub fn versionable_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.versionable_status = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for versionable_status: {}", e
                    )
                });
            self
        }
        pub fn versionable_status_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.versionable_status_value = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for versionable_status_value: {}",
                        e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<VersionDetail> for super::VersionDetail {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VersionDetail,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                registered_time: value.registered_time?,
                status: value.status?,
                status_value: value.status_value?,
                temporary_until: value.temporary_until?,
                valid_from: value.valid_from?,
                valid_until: value.valid_until?,
                version_cause_type: value.version_cause_type?,
                version_cause_type_value: value.version_cause_type_value?,
                versionable_status: value.versionable_status?,
                versionable_status_value: value.versionable_status_value?,
            })
        }
    }
    impl ::std::convert::From<super::VersionDetail> for VersionDetail {
        fn from(value: super::VersionDetail) -> Self {
            Self {
                registered_time: Ok(value.registered_time),
                status: Ok(value.status),
                status_value: Ok(value.status_value),
                temporary_until: Ok(value.temporary_until),
                valid_from: Ok(value.valid_from),
                valid_until: Ok(value.valid_until),
                version_cause_type: Ok(value.version_cause_type),
                version_cause_type_value: Ok(value.version_cause_type_value),
                versionable_status: Ok(value.versionable_status),
                versionable_status_value: Ok(value.versionable_status_value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VersionOverview {
        status: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        temporary_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_from: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        valid_until: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version_cause_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version_cause_type_value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VersionOverview {
        fn default() -> Self {
            Self {
                status: Ok(Default::default()),
                status_value: Ok(Default::default()),
                temporary_until: Ok(Default::default()),
                valid_from: Ok(Default::default()),
                valid_until: Ok(Default::default()),
                version_cause_type: Ok(Default::default()),
                version_cause_type_value: Ok(Default::default()),
            }
        }
    }
    impl VersionOverview {
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status: {}", e)
                });
            self
        }
        pub fn status_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for status_value: {}", e)
                });
            self
        }
        pub fn temporary_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.temporary_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for temporary_until: {}", e)
                });
            self
        }
        pub fn valid_from<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_from = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_from: {}", e)
                });
            self
        }
        pub fn valid_until<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.valid_until = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for valid_until: {}", e)
                });
            self
        }
        pub fn version_cause_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_cause_type = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for version_cause_type: {}", e
                    )
                });
            self
        }
        pub fn version_cause_type_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_cause_type_value = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for version_cause_type_value: {}",
                        e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<VersionOverview> for super::VersionOverview {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VersionOverview,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                status: value.status?,
                status_value: value.status_value?,
                temporary_until: value.temporary_until?,
                valid_from: value.valid_from?,
                valid_until: value.valid_until?,
                version_cause_type: value.version_cause_type?,
                version_cause_type_value: value.version_cause_type_value?,
            })
        }
    }
    impl ::std::convert::From<super::VersionOverview> for VersionOverview {
        fn from(value: super::VersionOverview) -> Self {
            Self {
                status: Ok(value.status),
                status_value: Ok(value.status_value),
                temporary_until: Ok(value.temporary_until),
                valid_from: Ok(value.valid_from),
                valid_until: Ok(value.valid_until),
                version_cause_type: Ok(value.version_cause_type),
                version_cause_type_value: Ok(value.version_cause_type_value),
            }
        }
    }
}
