extern crate serde;

use crate::log;
use quick_xml;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SequenceItem {
    #[serde(rename = "$value")]
    content: Option<String>,
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
struct Name {
    #[serde(rename = "rdf_Alt")]
    sequence: Sequence,
}

#[derive(Debug, Deserialize)]
struct Description {
    #[serde(rename = "crs_PresetType")]
    preset_type: String,

    #[serde(rename = "crs_Cluster")]
    cluster: String,

    #[serde(rename = "crs_UUID")]
    uuid: String,

    #[serde(rename = "crs_SupportsAmount")]
    supports_amount: bool,

    #[serde(rename = "crs_SupportsColor")]
    supports_color: bool,

    #[serde(rename = "crs_SupportsMonochrome")]
    supports_monochrome: bool,

    #[serde(rename = "crs_SupportsHighDynamicRange")]
    supports_high_dynamic_range: bool,

    #[serde(rename = "crs_SupportsNormalDynamicRange")]
    supports_normal_dynamic_range: bool,

    #[serde(rename = "crs_SupportsSceneReferred")]
    supports_scene_referred: bool,

    #[serde(rename = "crs_SupportsOutputReferred")]
    supports_output_referred: bool,

    #[serde(rename = "crs_CameraModelRestriction")]
    camera_model_restriction: String,

    #[serde(rename = "crs_Copyright")]
    copyright: String,

    #[serde(rename = "crs_ContactInfo")]
    contact_info: String,

    #[serde(rename = "crs_Version")]
    version: String,

    #[serde(rename = "crs_ProcessVersion")]
    process_version: String,

    #[serde(rename = "crs_Saturation")]
    saturation: i32,

    #[serde(rename = "crs_ShadowTint")]
    shadow_tint: i32,

    #[serde(rename = "crs_RedHue")]
    red_hue: i32,

    #[serde(rename = "crs_RedSaturation")]
    red_saturation: i32,

    #[serde(rename = "crs_GreenHue")]
    green_hue: i32,

    #[serde(rename = "crs_GreenSaturation")]
    green_saturation: i32,

    #[serde(rename = "crs_BlueHue")]
    blue_hue: i32,

    #[serde(rename = "crs_BlueSaturation")]
    blue_saturation: i32,

    #[serde(rename = "crs_Vibrance")]
    vibrance: Option<i32>,

    #[serde(rename = "crs_HueAdjustmentRed")]
    hue_adjustment_red: i32,
    #[serde(rename = "crs_HueAdjustmentOrange")]
    hue_adjustment_orange: i32,
    #[serde(rename = "crs_HueAdjustmentYellow")]
    hue_adjustment_yellow: i32,
    #[serde(rename = "crs_HueAdjustmentGreen")]
    hue_adjustment_green: i32,
    #[serde(rename = "crs_HueAdjustmentAqua")]
    hue_adjustment_aqua: i32,
    #[serde(rename = "crs_HueAdjustmentBlue")]
    hue_adjustment_blue: i32,
    #[serde(rename = "crs_HueAdjustmentPurple")]
    hue_adjustment_purple: i32,
    #[serde(rename = "crs_HueAdjustmentMagenta")]
    hue_adjustment_magenta: i32,

    #[serde(rename = "crs_SaturationAdjustmentRed")]
    saturation_adjustment_red: i32,
    #[serde(rename = "crs_SaturationAdjustmentOrange")]
    saturation_adjustment_orange: i32,
    #[serde(rename = "crs_SaturationAdjustmentYellow")]
    saturation_adjustment_yellow: i32,
    #[serde(rename = "crs_SaturationAdjustmentGreen")]
    saturation_adjustment_green: i32,
    #[serde(rename = "crs_SaturationAdjustmentAqua")]
    saturation_adjustment_aqua: i32,
    #[serde(rename = "crs_SaturationAdjustmentBlue")]
    saturation_adjustment_blue: i32,
    #[serde(rename = "crs_SaturationAdjustmentPurple")]
    saturation_adjustment_purple: i32,
    #[serde(rename = "crs_SaturationAdjustmentMagenta")]
    saturation_adjustment_magenta: i32,

    #[serde(rename = "crs_LuminanceAdjustmentRed")]
    luminance_adjustment_red: i32,
    #[serde(rename = "crs_LuminanceAdjustmentOrange")]
    luminance_adjustment_orange: i32,
    #[serde(rename = "crs_LuminanceAdjustmentYellow")]
    luminance_adjustment_yellow: i32,
    #[serde(rename = "crs_LuminanceAdjustmentGreen")]
    luminance_adjustment_green: i32,
    #[serde(rename = "crs_LuminanceAdjustmentAqua")]
    luminance_adjustment_aqua: i32,
    #[serde(rename = "crs_LuminanceAdjustmentBlue")]
    luminance_adjustment_blue: i32,
    #[serde(rename = "crs_LuminanceAdjustmentPurple")]
    luminance_adjustment_purple: i32,
    #[serde(rename = "crs_LuminanceAdjustmentMagenta")]
    luminance_adjustment_magenta: i32,

    #[serde(rename = "crs_SplitToningShadowHue")]
    split_toning_shadow_hue: i32,

    #[serde(rename = "crs_SplitToningShadowSaturation")]
    split_toning_shadow_saturation: i32,

    #[serde(rename = "crs_SplitToningHighlightHue")]
    split_toning_highlights_hue: i32,

    #[serde(rename = "crs_SplitToningHighlightSaturation")]
    split_toning_highlights_saturation: i32,

    #[serde(rename = "crs_SplitToningBalance")]
    split_toning_balance: i32,

    #[serde(rename = "crs_ParametricShadows")]
    parametric_shadows: i32,

    #[serde(rename = "crs_ParametricDarks")]
    parametric_darks: i32,

    #[serde(rename = "crs_ParametricLights")]
    parametric_lights: i32,

    #[serde(rename = "crs_ParametricHighlights")]
    parametric_highlights: i32,

    #[serde(rename = "crs_ParametricShadowSplit")]
    parametric_shadow_split: i32,

    #[serde(rename = "crs_ParametricMidtoneSplit")]
    parametric_midtone_split: i32,

    #[serde(rename = "crs_ParametricHighlightSplit")]
    parametric_highlight_split: i32,

    #[serde(rename = "crs_Contrast2012")]
    contrast_2012: i32,

    #[serde(rename = "crs_Highlights2012")]
    highlights_2012: i32,

    #[serde(rename = "crs_Shadows2012")]
    shadows_2012: i32,

    #[serde(rename = "crs_Whites2012")]
    whites_2012: i32,

    #[serde(rename = "crs_Blacks2012")]
    blacks_2012: i32,

    #[serde(rename = "crs_Clarity2012")]
    clarity_2012: i32,

    #[serde(rename = "crs_Dehaze")]
    dehaze: Option<i32>,

    #[serde(rename = "crs_Texture")]
    texture: Option<i32>,

    #[serde(rename = "crs_ConvertToGrayscale")]
    convert_to_grayscale: Option<bool>,

    #[serde(rename = "crs_ToneCurveName2012")]
    tone_curve_name: String,

    #[serde(rename = "crs_CameraProfile")]
    camera_profile: Option<String>,

    #[serde(rename = "crs_CameraProfileDigest")]
    camera_profile_digest: Option<String>,

    #[serde(rename = "crs_HasSettings")]
    has_settings: bool,

    #[serde(rename = "crs_ToneCurvePV2012")]
    tone_curve_2012: ToneCurve2012,

    #[serde(rename = "crs_ToneCurvePV2012Red")]
    tone_curve_red_2012: ToneCurve2012,

    #[serde(rename = "crs_ToneCurvePV2012Green")]
    tone_curve_green_2012: ToneCurve2012,

    #[serde(rename = "crs_ToneCurvePV2012Blue")]
    tone_curve_blue_2012: ToneCurve2012,

    #[serde(rename = "crs_Name")]
    name: Name,

    #[serde(rename = "crs_ShortName")]
    shortName: Name,

    #[serde(rename = "crs_SortName")]
    sortName: Name,

    #[serde(rename = "crs_Group")]
    groupName: Name,
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
