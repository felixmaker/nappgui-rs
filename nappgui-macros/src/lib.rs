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

    code.parse().expect("To parse")
}

#[derive(Debug)]
enum ResourceData {
    Message(String),
    Bytes(PathBuf),
    File(PathBuf),
}

#[derive(Debug)]
struct Resource {
    pub rid: String,
    pub uids_ordered: Vec<String>,
    pub uids: HashMap<String, usize>,
    pub locates: HashSet<String>,
    pub resource: HashMap<String, HashMap<String, ResourceData>>,
}

impl Resource {
    fn new<T>(rid: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            rid: rid.as_ref().to_owned(),
            uids_ordered: Vec::new(),
            uids: HashMap::new(),
            locates: HashSet::new(),
            resource: HashMap::new(),
        }
    }

    fn get_uid<T>(&self, uid: T) -> String
    where
        T: AsRef<str>,
    {
        format!("N23R3C75::{}::{}", self.rid, self.uids[uid.as_ref()])
    }

    fn push<T>(&mut self, uid: T, locale: T, data: ResourceData)
    where
        T: AsRef<str>,
    {
        let uid = uid.as_ref().to_owned();

        if !self.uids.contains_key(&uid) {
            self.uids.insert(uid.clone(), self.uids.len());
        }
        let iuid = self.get_uid(&uid);
        if !self.resource.contains_key(iuid.as_str()) {
            self.uids_ordered.push(uid.clone());
            self.resource.insert(iuid.clone(), HashMap::new());
        }
        let uid_data = self.resource.get_mut(&iuid).unwrap();

        if !uid_data.contains_key(locale.as_ref()) {
            self.locates.insert(locale.as_ref().to_string());
            uid_data.insert(locale.as_ref().to_string(), data);
        }
    }
}

fn read_data<P>(dir: P) -> Resource
where
    P: AsRef<Path>,
{
    let dir = Path::new(dir.as_ref());
    let rid = dir.file_name().unwrap();
    let mut resources = Resource::new(rid.to_string_lossy());

    let items = std::fs::read_dir(dir).unwrap();
    for item in items {
        let path = item.unwrap().path();
        if path.is_dir() {
            // Dir name is locale
            let locale = path.file_name().unwrap();
            let inner_items = std::fs::read_dir(&path).unwrap();
            for inner_item in inner_items {
                let inner_path = inner_item.unwrap().path();
                if inner_path.is_dir() {
                    continue;
                }
                if let Some(extension) = inner_path.extension() {
                    if let Some(file_name) = inner_path.file_name() {
                        let file_name = file_name.to_string_lossy().to_string();
                        if extension.to_string_lossy() == "msg" {
                            let text = std::fs::read_to_string(inner_path).unwrap();

                            let mut comment = false;
                            for line in text.lines() {
                                // support comment start with /* end with */
                                if comment {
                                    if line.trim().ends_with("*/") {
                                        comment = false;
                                    }
                                    continue;
                                }
                                if line.trim().starts_with("/*") {
                                    comment = true;
                                    if line.trim().ends_with("*/") {
                                        comment = false;
                                    }
                                    continue;
                                }
                                // support comment start with //
                                if line.trim().starts_with("//") {
                                    continue;
                                }
                                // suppor empty line
                                if line.trim().len() == 0 {
                                    continue;
                                }
                                let line = line.trim().split_once(char::is_whitespace).unwrap();
                                let message_id = line.0.to_uppercase();
                                let message = line.1.trim().trim_matches('"').to_string();
                                resources.push(
                                    message_id,
                                    locale.to_string_lossy().to_string(),
                                    ResourceData::Message(message),
                                );
                            }
                        }
                        else if extension.to_string_lossy() == "png"
                            || extension.to_string_lossy() == "jpg"
                            || extension.to_string_lossy() == "gif"
                            || extension.to_string_lossy() == "bmp"
                        {
                            let uid = file_name.replace(".", "_").to_uppercase();

                            resources.push(
                                uid,
                                locale.to_string_lossy().to_string(),
                                ResourceData::Bytes(inner_path),
                            );
                        } else {
                            let uid = file_name.replace(".", "_").to_uppercase();

                            resources.push(
                                uid,
                                locale.to_string_lossy().to_string(),
                                ResourceData::File(inner_path),
                            );
                        }
                    }
                }
            }
        } else {
            if let Some(extension) = path.extension() {
                if let Some(file_name) = path.file_name() {
                    let file_name = file_name.to_string_lossy().to_string();
                    if extension.to_string_lossy() == "msg" {
                        let text = std::fs::read_to_string(path).unwrap();

                        let mut comment = false;
                        for line in text.lines() {
                            // support comment start with /* end with */
                            if comment {
                                if line.trim().ends_with("*/") {
                                    comment = false;
                                }
                                continue;
                            }
                            if line.trim().starts_with("/*") {
                                comment = true;
                                if line.trim().ends_with("*/") {
                                    comment = false;
                                }
                                continue;
                            }
                            // support comment start with //
                            if line.trim().starts_with("//") {
                                continue;
                            }
                            // suppor empty line
                            if line.trim().len() == 0 {
                                continue;
                            }
                            let line = line.trim().split_once(char::is_whitespace).unwrap();
                            let message_id = line.0.to_uppercase();
                            let message = line.1.trim().trim_matches('"').to_string();
                            resources.push(
                                message_id,
                                "default".to_owned(),
                                ResourceData::Message(message),
                            );
                        }
                    } else {
                        let uid = file_name.replace(".", "_").to_uppercase();
                        resources.push(uid, "default".to_owned(), ResourceData::Bytes(path));
                    }
                }
            }
        }
    }
    resources
}

fn generate_code(resource: &Resource) -> String {
    let mut code: Vec<String> = Vec::new();

    // Init Pub
    for uid in resource.uids_ordered.iter() {
        code.push(format!(
            "pub static {}: &str = \"{}\";",
            uid,
            resource.get_uid(uid)
        ));
    }

    // Init Private
    for uid in resource.uids_ordered.iter() {
        let inner_uid = resource.get_uid(uid);
        for (locate, data) in resource.resource[&inner_uid].iter() {
            match data {
                ResourceData::Message(message) => {
                    if locate == "default" {
                        code.push(format!("static {}_TEXT: &str = \"{}\";", uid, message));
                    } else {
                        code.push(format!(
                            "static {}_{}_TEXT: &str = \"{}\";",
                            uid,
                            locate.to_uppercase(),
                            message
                        ));
                    }
                }
                ResourceData::Bytes(path) => {
                    if locate == "default" {
                        code.push(format!(
                            "static {}_DATA: &'static [u8] = include_bytes!(\"{}\");",
                            uid,
                            std::path::absolute(path)
                                .unwrap()
                                .as_os_str()
                                .to_string_lossy()
                                .replace("\\", "/")
                        ));
                    } else {
                        code.push(format!(
                            "static {}_{}_DATA: &'static [u8] = include_bytes!(\"{}\");",
                            uid,
                            locate.to_uppercase(),
                            std::path::absolute(path)
                                .unwrap()
                                .as_os_str()
                                .to_string_lossy()
                                .replace("\\", "/")
                        ));
                    }
                }
                ResourceData::File(path) => {
                    code.push(format!(
                        "static {}_{}_DATA: &'static [u8] = include_bytes!(\"{}\");",
                        uid,
                        locate.to_uppercase(),
                        std::path::absolute(path)
                            .unwrap()
                            .as_os_str()
                            .to_string_lossy()
                            .replace("\\", "/")
                    ));
                }
            }
        }
    }

    code.push(format!("pub unsafe extern \"C\" fn {}_respack(locale: *const std::ffi::c_char) -> nappgui::core::ResPackPtr {{", resource.rid));
    code.push("#[allow(unused)]".to_owned());
    code.push(
        "let locale = unsafe { std::ffi::CStr::from_ptr(locale).to_str().unwrap() };".to_owned(),
    );
    code.push(format!(
        "let mut respack = nappgui::core::ResPack::embedded(\"{}\");",
        resource.rid
    ));
    for locale in resource.locates.iter() {
        if locale == "default" {
            continue;
        }
        code.push(format!("if locale == \"{}\" {{", locale));
        for uid in resource.uids_ordered.iter() {
            let rid = resource.get_uid(uid);
            let take_default = !resource.resource[&rid].contains_key(locale);
            let resource = resource.resource[&rid]
                .get(locale)
                .unwrap_or(resource.resource[&rid].get("default").unwrap());
            match resource {
                ResourceData::Message(_) => {
                    if take_default {
                        code.push(format!("respack.add_message({}_TEXT);", uid));
                    } else {
                        code.push(format!(
                            "respack.add_message({}_{}_TEXT);",
                            uid,
                            locale.to_uppercase()
                        ));
                    }
                }
                ResourceData::Bytes(_) => {
                    if take_default {
                        code.push(format!("respack.add_bytes({}_DATA);", uid));
                    } else {
                        code.push(format!(
                            "respack.add_bytes({}_{}_DATA);",
                            uid,
                            locale.to_uppercase()
                        ));
                    }
                }
                ResourceData::File(_) => {
                    if take_default {
                        code.push(format!("respack.add_file({}_DATA);", uid));
                    } else {
                        code.push(format!(
                            "respack.add_file({}_{}_DATA);",
                            uid,
                            locale.to_uppercase()
                        ));
                    }
                }
            }
        }
        code.push("return respack.as_ptr()".to_owned());
        code.push("}".to_owned());
    }
    for uid in resource.uids_ordered.iter() {
        let rid = resource.get_uid(uid);
        let resource = resource.resource[&rid].get("default").unwrap();
        match resource {
            ResourceData::Message(_) => {
                code.push(format!("respack.add_message({}_TEXT);", uid));
            }
            ResourceData::Bytes(_) => {
                code.push(format!("respack.add_bytes({}_DATA);", uid));
            }
            ResourceData::File(_) => {
                code.push(format!("respack.add_file({}_DATA);", uid));
            }
        }
    }
    code.push("respack.as_ptr()".to_owned());
    code.push("}".to_owned());
    code.join("\n")
}
