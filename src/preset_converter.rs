use crate::log;
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

pub fn convert_preset(s : &str) -> String {
    let mut reader = Reader::from_str(s);
    reader.trim_text(true);

    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event_unbuffered()`
        match reader.read_event(&mut buf) {
            Ok(Event::Start(e)) => {
                log!("found {}", std::str::from_utf8(e.name()).unwrap());
                match e.name() {
                    n if n.starts_with(b"crs:ToneCurve") => {
                        decode_tonecurve(&mut reader, String::from_utf8(e.name().to_vec()).unwrap());
                    },
                    b"rdf:Description" => {
                        for a in e.attributes() {
                            decode_description_attribute(a.unwrap());
                        }
                    },
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => log!("Text at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => {log!("eof"); break}, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => {log!("other event"); ()}, // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    String::from("new preset format")
}
