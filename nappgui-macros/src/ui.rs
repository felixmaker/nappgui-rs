use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use roxmltree::Node;
use syn::{parse2, Ident, LitFloat, LitInt, LitStr};

pub fn process_ui_macro(input: TokenStream) -> TokenStream {
    let xml_literal = match parse2::<LitStr>(input) {
        Ok(lit) => lit.value(),
        Err(e) => return e.to_compile_error(),
    };

    let doc = match roxmltree::Document::parse(&xml_literal) {
        Ok(parsed_doc) => parsed_doc,
        Err(err) => {
            // Fallback: provide a compile error highlighting the issue
            return syn::Error::new(
                Span::call_site(),
                format!(
                    "Implicit XML Syntax Error: {}. Raw content processed: '{}'",
                    err, xml_literal
                ),
            )
            .to_compile_error();
        }
    };

    let generator = match StructGenerator::from_xml_doc(&doc) {
        Ok(generator) => generator,
        Err(err) => return err.to_compile_error(),
    };
    generator.generate()
}

struct StructGenerator {
    id: usize,
    root: Option<usize>,
    nodes: HashMap<usize, StructNode>,
}

impl StructGenerator {
    fn new() -> Self {
        Self {
            root: None,
            id: 0,
            nodes: HashMap::new(),
        }
    }

    /// Get the next unique ident for a node. And increment the id for the next node.
    fn next_id(&mut self) -> usize {
        self.id += 1;
        self.id
    }

    /// Insert a node into the generator. Return the unique ident for the node.
    fn insert_node_from_xml_node(&mut self, node: Node) -> Result<usize, syn::Error> {
        let id = self.next_id();
        self.nodes.insert(id, StructNode::from_xml_node(node, id)?);
        Ok(id)
    }

    /// Set the parent of a node.
    fn set_parent(&mut self, id: usize, parent: usize) {
        self.nodes.get_mut(&id).unwrap().set_parent(parent);
        self.nodes.get_mut(&parent).unwrap().add_child(id);
    }

    /// Create a new struct generator from an XML document.
    fn from_xml_doc(doc: &roxmltree::Document) -> Result<Self, syn::Error> {
        let mut generator = Self::new();
        let mut nodes_left = Vec::new();
        nodes_left.push((doc.root_element(), None)); // Add the root node to the queue.
        while let Some((node, parent)) = nodes_left.pop() {
            let node_id = generator.insert_node_from_xml_node(node)?;
            if let Some(parent) = parent {
                generator.set_parent(node_id, parent);
            } else {
                generator.root = Some(node_id);
            }
            for child in node.children() {
                if child.is_element() {
                    nodes_left.push((child, Some(node_id)));
                }
            }
        }
        Ok(generator)
    }

    /// Generate the struct code from the generator.
    fn generate(&self) -> TokenStream {
        let root_node = self.root_node();
        let struct_ident = root_node.ident();
        let struct_type = root_node.ty.to_type().expect("Root node type is not supported");
        let struct_name = root_node.name();
        let define_fields = self.nodes.iter().filter_map(|(_, node)| node.generate_self_type());
        let init_fields = self.nodes.iter().filter_map(|(_, node)| node.generate_self_field());
        let apply_layouts = self.nodes.iter().filter_map(|(_, node)| node.apply_layouts(self));
        let apply_attrs = self.nodes.iter().filter_map(|(_, node)| node.apply_attrs(self));
        let define_setters = self.nodes.iter().filter_map(|(_, node)| node.generate_setter());
        quote! {
            pub struct #struct_ident {
                #(#define_fields,)*
            }

            impl #struct_ident {
                pub fn new() -> Self {
                    let obj = Self {
                        #(#init_fields,)*
                    };
                    #(#apply_layouts)*
                    #(#apply_attrs)*
                    obj
                }

                #(#define_setters)*
            }

            impl Into<#struct_type> for #struct_ident {
                fn into(self: Self) -> #struct_type {
                    self.#struct_name
                }
            }
        }
    }

    /// Get the root node from the generator.
    fn root_node(&self) -> &StructNode {
        self.nodes.get(&self.root.unwrap()).unwrap()
    }

    /// Get the node from the generator.
    fn get(&self, id: usize) -> Option<&StructNode> {
        self.nodes.get(&id)
    }
}

struct StructNode {
    /// Unique ident for the node, used as field name in the struct and object name in the code.
    id: usize,
    /// The type of the node, used to determine the field type in the struct.
    ty: StructFieldType,
    /// Attributes of the node, used to apply properties to the object.
    attrs: Option<HashMap<String, String>>,
    /// The unique ident of the parent node, used to build the hierarchy of objects.
    parent: Option<usize>,
    /// The unique ident of the children nodes, used to build the hierarchy of objects.
    children: Option<Vec<usize>>,
}

impl StructNode {
    /// Create a new node from an XML node.
    fn from_xml_node(node: Node, id: usize) -> Result<Self, syn::Error> {
        let attrs: HashMap<String, String> = node
            .attributes()
            .map(|attr| (attr.name().to_string(), attr.value().to_string()))
            .collect();
        let ty = StructFieldType::from_str(node.tag_name().name())?;
        Ok(Self {
            id,
            ty,
            attrs: Some(attrs),
            parent: None,
            children: None,
        })
    }

    /// Set the parent of the node.
    fn set_parent(&mut self, parent: usize) {
        self.parent = Some(parent);
    }

    /// Add a child node to the node.
    fn add_child(&mut self, child: usize) {
        if self.children.is_none() {
            self.children = Some(Vec::new());
        }
        self.children.as_mut().unwrap().push(child);
    }

    fn name(&self) -> Ident {
        if let Some(attrs) = &self.attrs {
            if let Some(name) = attrs.get("name") {
                return Ident::new(name, Span::call_site());
            }
        }
        Ident::new(&format!("__obj_{}", self.id), Span::call_site())
    }

    fn ident(&self) -> Ident {
        if let Some(attrs) = &self.attrs {
            if let Some(ident) = attrs.get("ident") {
                return Ident::new(ident, Span::call_site());
            }
        }
        Ident::new(&format!("__Obj_{}", self.id), Span::call_site())
    }

    fn attr_or(&self, name: &str, default: &'static str) -> &str {
        let names: Vec<&str> = name.split("|").collect();
        if let Some(attrs) = &self.attrs {
            for name in names {
                if let Some(value) = attrs.get(name) {
                    return value;
                }
            }
        }
        default
    }

    fn attr(&self, name: &str) -> Option<&str> {
        let names: Vec<&str> = name.split("|").collect();
        if let Some(attrs) = &self.attrs {
            for name in names {
                if let Some(value) = attrs.get(name) {
                    return Some(value);
                }
            }
        }
        None
    }

    fn generate_self_type(&self) -> Option<TokenStream> {
        let field_ident = self.name();
        let field_type = self.ty.to_type()?;
        Some(quote! {
            #field_ident: #field_type
        })
    }

    fn generate_self_field(&self) -> Option<TokenStream> {
        let name = self.name();
        let obj = match self.ty {
            StructFieldType::Button => match ButtonType::from_str(self.attr_or("type", "push")) {
                ButtonType::Push => quote! { Button::new() },
                ButtonType::Flat => quote! { Button::new_flat() },
                ButtonType::Check => quote! { Button::new_check() },
                ButtonType::Check3 => quote! { Button::new_check3() },
                ButtonType::Radio => quote! { Button::new_radio() },
                ButtonType::FlatGle => quote! { Button::new_flatgle() },
            },
            StructFieldType::Combo => quote! { Combo::new() },
            StructFieldType::Edit => {
                if bool::from_str(self.attr_or("multiline|multi-line", "false")) {
                    quote! { Edit::new_multiline() }
                } else {
                    quote! { Edit::new() }
                }
            }
            StructFieldType::ImageView => quote! { ImageView::new() },
            StructFieldType::Label => quote! { Label::new() },
            StructFieldType::Panel => {
                let horizontal_scroll =
                    bool::from_str(self.attr_or("horizontal-scroll|horizontal_scroll|hscroll", "false"));
                let vertical_scroll = bool::from_str(self.attr_or("vertical-scroll|vertical_scroll|vscroll", "false"));
                let border = bool::from_str(self.attr_or("border|has-border", "false"));
                match (horizontal_scroll, vertical_scroll, border) {
                    (false, false, false) => quote! { Panel::new() },
                    (_, _, false) => quote! { Panel::new_scroll(#horizontal_scroll, #vertical_scroll) },
                    (_, _, true) => quote! { Panel::new_custom(#horizontal_scroll, #vertical_scroll, #border) },
                }
            }
            StructFieldType::ListBox => quote! { ListBox::new() },
            StructFieldType::PopUp => quote! { PopUp::new() },
            StructFieldType::Progress => quote! { Progress::new() },
            StructFieldType::Slider => {
                if bool::from_str(self.attr_or("vertical", "false")) {
                    quote! { Slider::new_vertical() }
                } else {
                    quote! { Slider::new() }
                }
            }
            StructFieldType::SplitView => {
                if bool::from_str(self.attr_or("vertical", "false")) {
                    quote! { SplitView::new_vertical() }
                } else {
                    quote! { SplitView::new() }
                }
            }
            StructFieldType::TableView => quote! { TableView::new() },
            StructFieldType::TextView => quote! { TextView::new() },
            StructFieldType::UpDown => quote! { UpDown::new() },
            StructFieldType::View => quote! { View::new() },
            StructFieldType::WebView => quote! { WebView::new() },
            StructFieldType::Line => {
                if bool::from_str(self.attr_or("vertical", "false")) {
                    quote! { Line::new_vertical() }
                } else {
                    quote! { Line::new() }
                }
            }
            StructFieldType::Layout => {
                let columns = LitInt::new(self.attr_or("cols|columns", "1"), Span::call_site());
                let rows = LitInt::new(self.attr_or("rows", "1"), Span::call_site());
                quote! { Layout::new(#columns, #rows) }
            }
            StructFieldType::Window => quote! { Window::new(WindowFlags::default()) },
            StructFieldType::Cell => return None,
        };
        Some(quote! {
            #name: #obj
        })
    }

    fn apply_attrs(&self, generator: &StructGenerator) -> Option<TokenStream> {
        let name = self.name();
        match self.ty {
            StructFieldType::Button => {
                let text_setter = self.attr("text").map(|text| {
                    let text = LitStr::new(text, Span::call_site());
                    quote! { obj.#name.set_text(#text); }
                });
                let width_setter = self.attr("width").map(|width| {
                    let width = LitFloat::new(width, Span::call_site());
                    quote! { obj.#name.set_width(#width); }
                });
                let set_text_alt_setter = self.attr("set-text-alt").map(|set_text_alt| {
                    let set_text_alt = LitStr::new(set_text_alt, Span::call_site());
                    quote! { obj.#name.set_text_alt(#set_text_alt); }
                });
                Some(quote! {
                    #text_setter
                    #width_setter
                    #set_text_alt_setter
                })
            }
            StructFieldType::Label => {
                let text_setter = self.attr("text").map(|text| {
                    let text = LitStr::new(text, Span::call_site());
                    quote! { obj.#name.set_text(#text); }
                });
                Some(quote! {
                    #text_setter
                })
            }
            StructFieldType::Panel => {
                let panel_name = self.name();
                let window_name = generator.get(self.parent?)?;
                if window_name.ty != StructFieldType::Window {
                    return None;
                }
                let window_name = window_name.name();
                Some(quote! {
                    obj.#window_name.set_panel(obj.#panel_name);
                })
            }
            StructFieldType::Window => {
                let title_setter = self.attr("title").map(|title| {
                    let title = LitStr::new(title, Span::call_site());
                    quote! { obj.#name.set_title(#title); }
                });
                let client_size_setter = self.attr("client-size|size").and_then(|client_size| {
                    let client_size = client_size.split(',').collect::<Vec<_>>();
                    if client_size.len() != 2 {
                        return None;
                    }
                    let width = LitFloat::new(client_size[0], Span::call_site());
                    let height = LitFloat::new(client_size[1], Span::call_site());
                    Some(quote! { obj.#name.set_client_size(#width, #height); })
                });
                let origin_setter = self.attr("origin").and_then(|origin| {
                    let origin = origin.split(',').collect::<Vec<_>>();
                    if origin.len() != 2 {
                        return None;
                    }
                    let x = LitFloat::new(origin[0], Span::call_site());
                    let y = LitFloat::new(origin[1], Span::call_site());
                    Some(quote! { obj.#name.set_origin(#x, #y); })
                });
                Some(quote! {
                    #title_setter
                    #client_size_setter
                    #origin_setter
                })
            }
            _ => None,
        }
    }

    fn apply_layouts(&self, generator: &StructGenerator) -> Option<TokenStream> {
        match self.ty {
            StructFieldType::Layout => {
                let panel = generator.get(self.parent?)?;
                let panel_name = panel.name();
                let layout_name = self.name();
                Some(quote! {
                    obj.#panel_name.add_layout(obj.#layout_name);
                })
            }
            StructFieldType::Cell => {
                let layout = generator.get(self.parent?)?;
                let layout_name = layout.name();
                let col = LitInt::new(self.attr("column|col")?, Span::call_site());
                let row = LitInt::new(self.attr("row")?, Span::call_site());
                let control_name = Ident::new(self.attr("for|control")?, Span::call_site());
                Some(quote! {
                    obj.#layout_name.set_control(#col, #row, obj.#control_name);
                })
            }
            _ => None,
        }
    }

    fn generate_setter(&self) -> Option<TokenStream> {
        let name = self.name();
        match self.ty {
            StructFieldType::Button => {
                let on_click_setter = self.attr("on-click").map(|on_click| {
                    let on_click = Ident::new(&format!("set_{}_handler", on_click), Span::call_site());
                    quote! {
                        pub fn #on_click<F>(&self, callback: F)
                        where
                            F: Fn(&ButtonEvent) + 'static
                        {
                            self.#name.set_on_click_handler(callback);
                        }
                    }
                });
                Some(quote! {
                    #on_click_setter
                })
            }
            StructFieldType::Window => {
                let on_close_setter = self.attr("on-close").map(|on_close| {
                    let on_close = Ident::new(&format!("set_{}_handler", on_close), Span::call_site());
                    quote! {
                        pub fn #on_close<F>(&self, callback: F)
                        where
                            F: Fn(&WindowCloseEvent) -> bool + 'static,
                        {
                            self.#name.set_on_close_handler(callback);
                        }
                    }
                });
                Some(quote! {
                    #on_close_setter
                })
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum StructFieldType {
    Button,
    Combo,
    Edit,
    ImageView,
    Label,
    Panel,
    ListBox,
    PopUp,
    Progress,
    Slider,
    SplitView,
    TableView,
    TextView,
    UpDown,
    View,
    WebView,
    Line,
    Layout,
    Cell,
    Window,
}

impl StructFieldType {
    fn from_str(s: &str) -> Result<Self, syn::Error> {
        match s.trim().to_lowercase().as_str() {
            "button" => Ok(Self::Button),
            "combo" => Ok(Self::Combo),
            "edit" => Ok(Self::Edit),
            "imageview" | "image-view" | "image_view" => Ok(Self::ImageView),
            "label" => Ok(Self::Label),
            "panel" => Ok(Self::Panel),
            "listbox" | "list-box" | "list_box" => Ok(Self::ListBox),
            "popup" => Ok(Self::PopUp),
            "progress" => Ok(Self::Progress),
            "slider" => Ok(Self::Slider),
            "splitview" | "split-view" | "split_view" => Ok(Self::SplitView),
            "tableview" | "table-view" | "table_view" => Ok(Self::TableView),
            "textview" | "text-view" | "text_view" => Ok(Self::TextView),
            "updown" | "up-down" | "up_down" => Ok(Self::UpDown),
            "view" => Ok(Self::View),
            "webview" | "web-view" | "web_view" => Ok(Self::WebView),
            "line" => Ok(Self::Line),
            "layout" => Ok(Self::Layout),
            "cell" => Ok(Self::Cell),
            "window" | "win" => Ok(Self::Window),
            _ => Err(syn::Error::new(
                Span::call_site(),
                &format!("Unknown tag type: <{}>", s),
            )),
        }
    }

    fn to_type(&self) -> Option<Ident> {
        match self {
            Self::Button => Some(Ident::new("Button", Span::call_site())),
            Self::Combo => Some(Ident::new("Combo", Span::call_site())),
            Self::Edit => Some(Ident::new("Edit", Span::call_site())),
            Self::ImageView => Some(Ident::new("ImageView", Span::call_site())),
            Self::Label => Some(Ident::new("Label", Span::call_site())),
            Self::Panel => Some(Ident::new("Panel", Span::call_site())),
            Self::ListBox => Some(Ident::new("ListBox", Span::call_site())),
            Self::PopUp => Some(Ident::new("PopUp", Span::call_site())),
            Self::Progress => Some(Ident::new("Progress", Span::call_site())),
            Self::Slider => Some(Ident::new("Slider", Span::call_site())),
            Self::SplitView => Some(Ident::new("SplitView", Span::call_site())),
            Self::TableView => Some(Ident::new("TableView", Span::call_site())),
            Self::TextView => Some(Ident::new("TextView", Span::call_site())),
            Self::UpDown => Some(Ident::new("UpDown", Span::call_site())),
            Self::View => Some(Ident::new("View", Span::call_site())),
            Self::WebView => Some(Ident::new("WebView", Span::call_site())),
            Self::Line => Some(Ident::new("Line", Span::call_site())),
            Self::Layout => Some(Ident::new("Layout", Span::call_site())),
            Self::Window => Some(Ident::new("Window", Span::call_site())),
            _ => None,
        }
    }
}

trait FromStr {
    fn from_str(s: &str) -> Self;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum ButtonType {
    Push,
    Check,
    Check3,
    Radio,
    Flat,
    FlatGle,
}

impl FromStr for ButtonType {
    fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "push" => Self::Push,
            "check" => Self::Check,
            "check3" => Self::Check3,
            "radio" => Self::Radio,
            "flat" => Self::Flat,
            "flatgle" => Self::FlatGle,
            _ => Self::Push,
        }
    }
}

impl FromStr for bool {
    fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "true" | "1" => true,
            "false" | "0" => false,
            _ => false,
        }
    }
}

impl FromStr for u32 {
    fn from_str(s: &str) -> Self {
        s.parse().unwrap_or(0)
    }
}

impl FromStr for f32 {
    fn from_str(s: &str) -> Self {
        s.parse().unwrap_or(0.0)
    }
}
