use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use proc_macro::TokenStream;

#[proc_macro]
pub fn include_resource(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input = input.trim();

    let mut path = Path::new(&input).to_owned();
    if !path.is_absolute() {
        let local_file = proc_macro::Span::call_site()
            .local_file()
            .expect("Unable to get local file!");
        let local_file_parent = local_file
            .parent()
            .expect("Unable to get the parent local file!");
        path = local_file_parent.join(input);
    }

    let resource = read_data(path);
    let code = generate_code(&resource);

    code.parse().expect("Unable to parse resources!")
}

#[derive(Debug)]
enum ResourceData {
    Message(String),
    Bytes(PathBuf),
    File(PathBuf),
}

#[derive(Debug)]
struct Resource {
    rid: String,
    size: usize,
    uids_ordered: Vec<String>,
    uids: HashMap<String, usize>, // UID -> ID
    locates: HashSet<String>,
    resource: Vec<HashMap<String, ResourceData>>, // Locale -> ResourceData
}

impl Resource {
    fn new(rid: &str) -> Self {
        Self {
            rid: rid.to_owned(),
            size: 0,
            uids_ordered: Vec::new(),
            uids: HashMap::new(),
            locates: HashSet::new(),
            resource: Vec::new(),
        }
    }

    fn get_sid(&self, uid: &str) -> Option<String> {
        self.uids
            .get(uid)
            .map(|id| format!("N23R3C75::{}::{}", self.rid, id))
    }

    fn get_id(&self, uid: &str) -> Option<usize> {
        self.uids.get(uid).copied()
    }

    fn push(&mut self, uid: &str, locale: &str, data: ResourceData) {
        let id = if !self.uids.contains_key(uid) {
            let id = self.size;
            self.uids.insert(uid.to_owned(), id);
            self.uids_ordered.push(uid.to_owned());
            self.resource.push(HashMap::new());
            self.size += 1;
            id
        } else {
            *self
                .uids
                .get(uid)
                .expect("Unable to get uid when add data to Resource!")
        };

        self.locates.insert(locale.to_uppercase());
        self.resource
            .get_mut(id)
            .expect("Unable to get resource!")
            .insert(locale.to_uppercase(), data);
    }

    fn push_message(&mut self, locale: &str, message: &str) {
        let mut comment = false;
        for line in message.lines() {
            let line = line.trim();
            // support comment start with /* end with */
            if comment {
                if line.ends_with("*/") {
                    comment = false;
                }
                continue;
            }
            if line.starts_with("/*") {
                comment = true;
                if line.trim().ends_with("*/") {
                    comment = false;
                }
                continue;
            }
            // support comment start with //
            if line.starts_with("//") {
                continue;
            }
            // support empty line
            if line.len() == 0 {
                continue;
            }
            let line = line
                .split_once(char::is_whitespace)
                .expect(&format!("Unable to get id and message in line {}!", line));
            let uid = line.0.to_uppercase();
            // support message with ""
            let message = line.1.trim().trim_matches('"').to_string();
            self.push(&uid, locale, ResourceData::Message(message));
        }
    }

    fn push_file(&mut self, locale: &str, file: &Path) {
        assert!(file.is_file());

        let name = file.file_name().unwrap().to_string_lossy().to_string();
        let extension = file.extension().unwrap().to_string_lossy().to_string();
        match extension.as_ref() {
            "msg" => {
                let text = std::fs::read_to_string(file).expect("Unable to read messages");
                self.push_message(&locale, &text);
            }
            "png" | "jpg" | "gif" | "bmp" => {
                let uid = name.replace(".", "_").to_uppercase();
                self.push(&uid, &locale, ResourceData::Bytes(file.to_owned()));
            }
            _ => {
                let uid = name.replace(".", "_").to_uppercase();
                self.push(&uid, &locale, ResourceData::File(file.to_owned()));
            }
        }
    }
}

const DEFAULT: &str = "DEFAULT";

fn read_data<P>(dir: P) -> Resource
where
    P: AsRef<Path>,
{
    let dir = Path::new(dir.as_ref());
    let rid = dir
        .file_name()
        .expect("Unable to get resource id from folder!");
    let mut resources = Resource::new(rid.to_string_lossy().as_ref());

    let items = std::fs::read_dir(dir).expect("Unable to read items in resource folder!");
    for item in items {
        let path = item.unwrap().path();
        if path.is_dir() {
            // folder name <-> locale
            let locale = path
                .file_name()
                .expect("Unable to get locale from folder!")
                .to_string_lossy();
            let inner_items = std::fs::read_dir(&path).unwrap();
            for inner_item in inner_items {
                let inner_path = inner_item.unwrap().path();
                if inner_path.is_dir() {
                    continue;
                }
                resources.push_file(&locale, &inner_path);
            }
        } else {
            resources.push_file(DEFAULT, &path);
        }
    }
    resources
}

fn generate_static_object(uid: &str, locale: &str, data: &ResourceData) -> String {
    let locale = if locale == DEFAULT {
        "".to_owned()
    } else {
        format!("{}_", locale)
    };
    match data {
        ResourceData::Message(message) => {
            format!(
                "static {}_{}TEXT: &'static str = \"{}\";",
                uid, locale, message
            )
        }
        ResourceData::Bytes(path) | ResourceData::File(path) => {
            format!(
                "static {}_{}DATA: &'static [u8] = include_bytes!(\"{}\");",
                uid,
                locale,
                std::path::absolute(path)
                    .unwrap()
                    .as_os_str()
                    .to_string_lossy()
                    .replace("\\", "/")
            )
        }
    }
}

fn generate_add_resource(uid: &str, locale: &str, data: &ResourceData) -> String {
    let locale = if locale == DEFAULT {
        "".to_owned()
    } else {
        format!("{}_", locale)
    };
    match data {
        ResourceData::Message(_) => {
            format!("respack.add_message({}_{}TEXT);", uid, locale)
        }
        ResourceData::Bytes(_) => {
            format!("respack.add_bytes({}_{}DATA);", uid, locale)
        }
        ResourceData::File(_) => {
            format!("respack.add_file({}_{}DATA);", uid, locale)
        }
    }
}

fn generate_code(resource: &Resource) -> String {
    let mut code: Vec<String> = Vec::new();

    // init public uid definition
    for uid in resource.uids_ordered.iter() {
        code.push(format!(
            "pub static {}: &str = \"{}\";",
            uid,
            resource.get_sid(uid).unwrap(),
        ));

        let id = resource.get_id(uid).unwrap();
        for (local, data) in resource.resource[id].iter() {
            code.push(generate_static_object(uid, local, data));
        }
    }

    code.push(format!("pub unsafe extern \"C\" fn {}_respack(locale: *const std::ffi::c_char) -> nappgui::core::ResPackPtr {{", resource.rid));
    code.push("#[allow(unused)]".to_owned());
    code.push(
        "let locale = unsafe { std::ffi::CStr::from_ptr(locale).to_str().unwrap() };".to_owned(),
    );
    code.push(format!(
        "let mut respack = nappgui::core::ResPack::new_embedded(\"{}\");",
        resource.rid
    ));

    for locale in resource.locates.iter() {
        if locale == DEFAULT {
            continue;
        }
        code.push(format!("if locale == \"{}\" {{", locale));
        for uid in resource.uids_ordered.iter() {
            let id = resource.get_id(uid).unwrap();
            let locale = if resource.resource[id].contains_key(locale) {
                locale
            } else {
                DEFAULT
            };
            let data = &resource.resource[id][locale];
            code.push(generate_add_resource(uid, locale, data))
        }
        code.push("return respack.as_ptr()".to_owned());
        code.push("}".to_owned());
    }

    for uid in resource.uids_ordered.iter() {
        let id = resource.get_id(uid).unwrap();
        let data = &resource.resource[id][DEFAULT];
        code.push(generate_add_resource(uid, DEFAULT, data));
    }

    code.push("respack.as_ptr()".to_owned());
    code.push("}".to_owned());

    let code = code.join("\n");
    code
}
