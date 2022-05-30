extern crate serde;

use crate::log;
use quick_xml;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SequenceItem {
    #[serde(rename = "$value")]
    content: String,
}

#[derive(Debug, Deserialize)]
struct Sequence {
    #[serde(rename = "rdf_li")]
    items: Vec<SequenceItem>
}

#[derive(Debug, Deserialize)]
struct ToneCurve2012 {
    #[serde(rename = "rdf_Seq")]
    sequence: Sequence,
}

#[derive(Debug, Deserialize)]
struct Description {
    #[serde(rename = "crs_PresetType")]
    preset_type: String,

    #[serde(rename = "crs_SupportsAmount")]
    supports_amount: bool,

    #[serde(rename = "crs_ToneCurvePV2012")]
    tone_curve_2012: ToneCurve2012,
}

#[derive(Debug, Deserialize)]
struct Rdf {
    #[serde(rename = "rdf_Description")]
    description: Description,
}

#[derive(Debug, Deserialize)]
struct xmpmeta {
    #[serde(rename = "rdf_RDF")]
    pub rdf: Rdf,
}

pub fn convert_preset(s : &str) -> String {
    // The xmp format is not obeying the xml specification completely and has some invalid
    // characters. We pre-process the given string slightly to make it easily digestible by
    // quick_xml.
    let result = str::replace(s, ":", "_");
    let slice: &str = &result[..];
    log!("{}", result);
    let doc_res: Result<xmpmeta, quick_xml::DeError> = quick_xml::de::from_str(slice);
    match doc_res {
        Ok(doc) => log!("Success parsing preset: {:#?}", doc),
        Err(x) => log!("Error parsing preset: {}", x),
    }

    String::from("new preset format")
}
