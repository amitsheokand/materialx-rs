use crate::materialx::core::{
    export,
    util
};
static DEFAULT_TYPE_STRING: &'static str = "color3";
static FILENAME_TYPE_STRING: &'static str = "filename";
static GEOMNAME_TYPE_STRING: &'static str = "geomname";
static STRING_TYPE_STRING: &'static str = "string";
static BSDF_TYPE_STRING: &'static str = "BSDF";
static EDF_TYPE_STRING: &'static str = "EDF";
static VDF_TYPE_STRING: &'static str = "VDF";
static SURFACE_SHADER_TYPE_STRING: &'static str = "surfaceshader";
static DISPLACEMENT_SHADER_TYPE_STRING: &'static str = "displacementshader";
static VOLUME_SHADER_TYPE_STRING: &'static str = "volumeshader";
static LIGHT_SHADER_TYPE_STRING: &'static str = "lightshader";
static MATERIAL_TYPE_STRING: &'static str = "material";
static SURFACE_MATERIAL_NODE_STRING: &'static str = "surfacematerial";
static VOLUME_MATERIAL_NODE_STRING: &'static str = "volumematerial";
static MULTI_OUTPUT_TYPE_STRING: &'static str = "multioutput";
static NONE_TYPE_STRING: &'static str = "none";
static VALUE_STRING_TRUE: &'static str = "true";
static VALUE_STRING_FALSE: &'static str = "false";
static NAME_PREFIX_SEPARATOR: &'static str = ":";
static NAME_PATH_SEPARATOR: &'static str = "/";
static ARRAY_VALID_SEPARATORS: &'static str = ", ";
static ARRAY_PREFERRED_SEPARATOR: &'static str = ", ";