use crate::uniform_name_conversion::Rename;
use regex::Match;
use rspirv_reflect::rspirv::spirv::Decoration::BuiltIn;
use std::collections::HashSet;

pub struct BuiltinRename<'a> {
    exclude_list: HashSet<&'a str>,
}
impl<'a> BuiltinRename<'a> {
    pub fn init() -> Self {
        let mut exclude_list = HashSet::new();
        exclude_list.insert("gl_Position");
        exclude_list.insert("gl_PointSize");
        exclude_list.insert("gl_ClipVertex");

        Self { exclude_list }
    }
    pub fn rename<Source: AsRef<str>>(&self, source: Source) -> String {
        let mut source = source.as_ref().to_string();
        let regex = regex::Regex::new(r"gl_[a-zA-Z]+").unwrap();

        let mut allow_list = HashSet::new();
        let clone_src = source.clone();
        for captures in regex.captures_iter(&clone_src) {
            for capture in captures.iter() {
                match capture {
                    None => {}
                    Some(mach) => {
                        println!("{}", mach.as_str());
                        if !self.exclude_list.contains(mach.as_str()) {
                            println!("we replace {}", mach.as_str());
                            allow_list.insert(mach.as_str());
                        }
                    }
                }
            }
        }
        for target in allow_list {
            let replacer = format!("_{}", target);
            source = source.replace(target, &replacer);
        }
        source
    }
}
#[test]
fn test_rename() {
    let source = include_str!("../test_shader/shadow.vsh");
    println!("input source:\n{}", source);
    let rename = BuiltinRename::init();
    println!("processed source :\n{}", rename.rename(source));
}
