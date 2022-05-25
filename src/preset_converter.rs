extern crate serde;

use crate::log;
use quick_xml;
use serde::Deserialize;
use quick_xml::Reader;
use quick_xml::events::Event;

pub fn decode_description_attribute(attr: quick_xml::events::attributes::Attribute) {
    let key = String::from_utf8(attr.key.to_vec()).unwrap();
    let value = String::from_utf8(attr.value.into()).unwrap();
    log!("in decode {}: {}", key, value);
    //log!("in decode {}", std::str::from_utf8(attr.key).unwrap() + std::str::from_utf8_mut(attr.value.to_mut()).unwrap());
}

pub fn decode_li<T: std::io::BufRead>(reader: &mut Reader<T>, li_name: String) {
    let mut skip_buf = Vec::new();
    loop {
        match reader.read_event(&mut skip_buf) {
            Ok(Event::Start(element)) if String::from_utf8(element.name().to_vec()).unwrap() == li_name => {},
            Ok(Event::End(element)) if String::from_utf8(element.name().to_vec()).unwrap() == li_name => {
                break;
            },
            Ok(Event::Text(e)) => log!("Text at position {}: {:?}", reader.buffer_position(), e),
            _ => { panic!("Should not get here"); }
        }
    }
}

pub fn decode_tonecurve<T: std::io::BufRead>(reader: &mut Reader<T>, tonecurve_name: String) {
    log!("parsing tonecurve {}", tonecurve_name);

    let mut skip_buf = Vec::new();
    loop {
        match reader.read_event(&mut skip_buf) {
            Ok(Event::Start(element)) if element.name() == b"rdf:li" => {
                decode_li(reader, String::from_utf8(element.name().to_vec()).unwrap());
            },
            Ok(Event::Start(element)) => {
                //log!("nested: {}", String::from_utf8(element.name().to_vec()).unwrap());
            },
            Ok(Event::End(element)) if String::from_utf8(element.name().to_vec()).unwrap() == tonecurve_name => {
                log!("tonecurve {} closed", tonecurve_name);
                break;
            },
            Ok(Event::End(_element)) => {
            }
            _ => {}
        }
    }
}

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
        Ok(doc) => log!("Success parsing preset: {}", doc.rdf.description.tone_curve_2012.sequence.items[0].content),
        Err(x) => log!("Error parsing preset: {}", x),
    }

    String::from("new preset format")
}
