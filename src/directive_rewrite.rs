

pub struct DirectiveRewriter{
    source:String,
    regex_version:regex::Regex,
    regex_extension:regex::Regex
}
impl DirectiveRewriter{
    pub fn new<Source:AsRef<str>>(source:Source) -> Self{
        let regex_version=regex::Regex::new(r"#version \s+ (P?<version>.+?)").unwrap();
        let regex_extension =regex::Regex::new(r"#extension\s+(P?<ext>[a-zA-Z_][a-zA-Z0-9_]*?)\s*:\s*(P?<enable>.+?)").unwrap();
        Self{ source: source.as_ref().to_string(), regex_version, regex_extension }
    }
    pub fn convert(&self)->String{
        let mut lines =vec![];
        for line in self.source.lines(){
            if self.regex_version.captures(&self.source).is_none() && self.regex_extension.captures(&self.source).is_none() {
                lines.push(line);
            }
        }
        {let mut source =String::new();
        source.push_str("#version 450\n");
        source.push_str("#extension GL_GOOGLE_include_directive : enable\n");
            for line in lines.iter(){
                source.push_str(&format!("{}\n",line))
            }
            source
        }
    }
}
#[test]
fn test(){
    let source=include_str!("test_attribute.glsl");
    let transformer=DirectiveRewriter::new(source);
    let source=transformer.convert();
    println!("output source:\n{}",source);
}