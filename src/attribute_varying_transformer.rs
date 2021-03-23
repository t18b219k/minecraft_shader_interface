use std::ops::{Add, Range};
use std::str::FromStr;

/// convert attribute <type> <identifier>;->
/// location() in <type> identifier;
pub struct AttributeTransformer {
    source: String,
    current_location: usize,
    regex: regex::Regex,
    regex_array: regex::Regex,
}
/// used in vulkan or webgpu
pub struct Location<'a> {
    pub name: &'a str,
    pub location_range: Range<usize>,
    pub ty: GLSLType,
}
impl<'a> Location<'a> {
    fn from_ty_name_location(name: &'a str, range: Range<usize>, ty: &'a str) -> Self {
        let ty = match ty {
            "vec4" => GLSLType::Vec4,
            "vec3" => GLSLType::Vec3,
            "vec2" => GLSLType::Vec2,
            "float" => GLSLType::Float,
            "ivec4" => GLSLType::IVec4,
            "ivec3" => GLSLType::IVec3,
            "ivec2" => GLSLType::IVec2,
            "int" => GLSLType::Int,
            "uvec4" => GLSLType::UVec4,
            "uvec3" => GLSLType::UVec3,
            "uvec2" => GLSLType::UVec2,
            "uint" => GLSLType::Uint,
            "dvec4" => GLSLType::DVec4,
            "dvec3" => GLSLType::DVec3,
            "dvec2" => GLSLType::DVec2,
            "double" => GLSLType::Double,
            "bvec4" => GLSLType::BVec4,
            "bvec3" => GLSLType::BVec3,
            "bvec2" => GLSLType::BVec2,
            "bool" => GLSLType::Bool,
            _ => {
                panic!("Invalid attribute type")
            }
        };
        Self {
            name,
            location_range: range,
            ty,
        }
    }
}
pub enum GLSLType {
    Uint,
    Int,
    Float,
    Double,
    Bool,

    IVec2,
    Vec2,
    BVec2,
    DVec2,
    UVec2,

    IVec3,
    Vec3,
    BVec3,
    DVec3,
    UVec3,

    IVec4,
    Vec4,
    BVec4,
    DVec4,
    UVec4,
}
impl AttributeTransformer {
    pub fn new<Source: AsRef<str>>(source: Source) -> Self {
        let regex=regex::Regex::new(r"attribute\s+(?P<type>[a-zA-Z_][a-zA-Z0-9_]*?)\s+(?P<var_name>[a-z-A-Z_][a-zA-Z0-9_]*?)\s*;").unwrap();
        let regex_array=regex::Regex::new(r"attribute\s+(?P<type>[a-zA-Z_][a-zA-Z0-9_]*?)\s+(?P<var_name>[a-z-A-Z_][a-zA-Z0-9_]*?)\s*\[\s*(?P<length>[0-9]+?)\s*\];").unwrap();
        Self {
            source: source.as_ref().to_string(),
            current_location: 0,
            regex,
            regex_array,
        }
    }
    pub fn convert(&mut self) -> (String, Vec<Location>) {
        let mut locations = vec![];
        let lines = self.source.lines();
        let mut line_vec = vec![];
        for line in lines {
            if let Some(captures) = self.regex.captures(line) {
                let ty = captures.name("type");
                let var_name = captures.name("var_name");
                // validate declaration
                if ty.is_some() && var_name.is_some() {
                    let ty = ty.unwrap();
                    let var_name = var_name.unwrap();

                    println!("{} {}", ty.as_str(), var_name.as_str());
                    //
                    let replacer = format!(
                        "location({}) in {} {};\n",
                        self.current_location,
                        ty.as_str(),
                        var_name.as_str()
                    );
                    println!("{}", replacer);
                    // emit line
                    line_vec.push(replacer);
                    // emit location
                    let location = Location::from_ty_name_location(
                        var_name.as_str(),
                        self.current_location..self.current_location + 1,
                        ty.as_str(),
                    );
                    locations.push(location);
                    self.current_location += 1;
                }
            } else if let Some(captures) = self.regex_array.captures(line) {
                let ty = captures.name("type");
                let var_name = captures.name("var_name");
                let length = captures.name("length");
                // validate declaration
                if ty.is_some() && var_name.is_some() && length.is_some() {
                    let ty = ty.unwrap();
                    let var_name = var_name.unwrap();
                    let length = length.unwrap();
                    println!("{} {}", ty.as_str(), var_name.as_str());
                    //
                    let replacer = format!(
                        "location({}) in {} {}[{}];\n",
                        self.current_location,
                        ty.as_str(),
                        var_name.as_str(),
                        length.as_str()
                    );
                    println!("{}", replacer);
                    // emit line
                    line_vec.push(replacer);
                    // emit location
                    let len = usize::from_str(length.as_str()).unwrap();
                    let range = self.current_location..self.current_location + len;
                    let location =
                        Location::from_ty_name_location(var_name.as_str(), range, ty.as_str());
                    locations.push(location);
                    self.current_location += len;
                }
            } else {
                line_vec.push(line.to_string().add("\n"))
            }
        }
        let source = {
            let mut source = String::new();
            for line in line_vec.iter() {
                source.push_str(&line)
            }
            source
        };
        (source, locations)
    }
}
#[test]
fn test() {
    let test_source = include_str!("test_attribute.glsl");
    let mut transformer = AttributeTransformer::new(test_source);
    let x = transformer.convert();
    println!("output source:\n{}", x.0);
}
