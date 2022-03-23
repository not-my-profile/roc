use roc_module::symbol::ModuleId;

#[inline(always)]
pub fn module_source(module_id: ModuleId) -> &'static str {
    match module_id {
        ModuleId::RESULT => RESULT,
        ModuleId::NUM => NUM,
        ModuleId::STR => STR,
        ModuleId::LIST => LIST,
        ModuleId::DICT => DICT,
        ModuleId::SET => SET,
        ModuleId::BOX => BOX,
        ModuleId::BOOL => BOOL,
        _ => panic!(
            "ModuleId {:?} is not part of the standard library",
            module_id
        ),
    }
}

const RESULT: &str = include_str!("../roc/Result.roc");
const NUM: &str = include_str!("../roc/Num.roc");
const STR: &str = include_str!("../roc/Str.roc");
const LIST: &str = include_str!("../roc/List.roc");
const DICT: &str = include_str!("../roc/Dict.roc");
const SET: &str = include_str!("../roc/Set.roc");
const BOX: &str = include_str!("../roc/Box.roc");
const BOOL: &str = include_str!("../roc/Bool.roc");
