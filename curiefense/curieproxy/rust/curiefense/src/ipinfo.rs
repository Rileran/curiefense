//! This file originated from ipinfo rust client api description. It was
//! modify to corespond to the datastructure from mmdb files instead of the web
//! api.
//!
//! see https://github.com/ipinfo/rust/blob/master/src/api.rs
//!
//! All data in the database are stored as str and must be parsed afterward.
//! This is why all field are represented as str.

use serde::{Deserialize, Serialize};

/// IP address lookup details.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocationDetails<'a> {
    /// The city for the IP address.
    pub city: &'a str,

    /// The country for the IP address.
    pub country: &'a str,

    /// The latitude for the IP address. (f64)
    pub lat: &'a str,

    /// The longitude for the IP address. (f64)
    pub lng: &'a str,

    /// The region for the IP address.
    pub region: &'a str,

    /// The region for the IP address.
    pub region_code: &'a str,

    /// The postal code for the IP address.
    pub postal_code: Option<&'a str>,

    /// The timezone for the IP address.
    pub timezone: Option<&'a str>,
}

/// Privacy details.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PrivacyDetails<'a> {
    /// Whether this IP address belongs to a VPN. (bool)
    pub vpn: &'a str,

    /// Whether this IP address belongs to a proxy. (bool)
    pub proxy: &'a str,

    /// Whether this IP address is using Tor. (bool)
    pub tor: &'a str,

    /// Whether this IP address is a relay. (bool)
    pub relay: &'a str,

    /// Whether this IP address is from a hosting provider. (bool)
    pub hosting: &'a str,

    /// The service offering the privacy service(s) listed here.
    pub service: &'a str,
}

/// Company details.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CompanyDetails<'a> {
    // COMPANY
    /// The name of the entity that owns the IP address.
    pub name: &'a str,

    /// The domain for the entity that owns this IP address.
    pub domain: &'a str,

    /// The type of entity that owns this IP address. (i.e., business, education, hosting, isp)
    #[serde(rename = "type")]
    pub company_type: &'a str,

    // AS
    /// The AS number. (format "AS{u32}")
    pub asn: &'a str,

    /// The name of the entity that owns this AS.
    pub as_name: &'a str,

    /// The domain for the entity that owns this AS.
    pub as_domain: &'a str,

    /// The entity type that owns this AS. (i.e., business, education, hosting, isp)
    pub as_type: &'a str,
}

/// Mobile carrier details.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CarrierDetails<'a> {
    /// The name of the carrier ISP that owns that mobile IP address.
    pub carrier: &'a str,

    /// The country code of the carrier ISP that owns that mobile IP address.
    #[serde(rename = "cc")]
    pub country_code: &'a str,

    /// MCC GSM network code of this carrier.
    pub mcc: &'a str,

    /// MNC GSM network code of this carrier.
    pub mnc: &'a str,

    /// The network of the carrier ISP that owns that mobile IP address.
    pub network: &'a str,
}
