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

    let generator = Generator::from_xml_doc(&doc);
    generator.generate()
}

struct Generator {
    id: usize,
    root: Option<usize>,
    nodes: HashMap<usize, GeneratorNode>,
}

impl Generator {
    fn new() -> Self {
        Self {
            id: 0,
            root: None,
            nodes: HashMap::new(),
        }
    }

    /// Get the next unique ident for a node. And increment the id for the next node.
    fn next_id(&mut self) -> usize {
        self.id += 1;
        self.id
    }

    /// Insert a node into the generator. Return the unique ident for the node.
    fn insert_node<F>(&mut self, f: F) -> usize
    where
        F: FnOnce(usize) -> GeneratorNode,
    {
        let id = self.next_id();
        self.nodes.insert(id, f(id));
        id
    }

    /// Set the parent of a node.
    fn set_parent(&mut self, id: usize, parent_id: usize) {
        let child = self.nodes.get_mut(&id).unwrap();
        child.parent = Some(parent_id);
        let parent = self.nodes.get_mut(&parent_id).unwrap();
        parent.children.push(id);
    }

    /// Create a new struct generator from an XML document.
    fn from_xml_doc(doc: &roxmltree::Document) -> Self {
        let mut generator = Self::new();
        let mut nodes_stack = Vec::new();
        nodes_stack.push((doc.root_element(), None)); // Add the root node to the queue.
        while let Some((node, parent)) = nodes_stack.pop() {
            let node_id = generator.insert_node(|id| GeneratorNode::from_xml_node(node, id));
            if let Some(parent) = parent {
                generator.set_parent(node_id, parent);
            } else {
                generator.root = Some(node_id);
            }
            for child in node.children() {
                if child.is_element() {
                    nodes_stack.push((child, Some(node_id)));
                }
            }
        }
        generator
    }

    fn define_field(&self, id: usize) -> Option<TokenStream> {
        let node = self.node(id);
        let field_name = node.name_ident();
        let field_type = node.type_ident()?;
        Some(quote! {
            #field_name: #field_type
        })
    }

    fn init_field(&self, id: usize) -> Option<TokenStream> {
        let node = self.node(id);
        let name = node.name_ident();
        let obj = match &node.ty {
            FieldType::Button => match node.attr_or("type", "push") {
                "check" => quote! { Button::new_check() },
                "check3" => quote! { Button::new_check3() },
                "radio" => quote! { Button::new_radio() },
                "flat" => quote! { Button::new_flat() },
                "flatgle" => quote! { Button::new_flatgle() },
                _ => quote! { Button::new() },
            },
            FieldType::Combo => quote! { Combo::new() },
            FieldType::Edit => {
                if bool::from_str(node.attr_or("multiline|multi-line", "false")) {
                    quote! { Edit::new_multiline() }
                } else {
                    quote! { Edsit::new() }
                }
            }
            FieldType::ImageView => quote! { ImageView::new() },
            FieldType::Label => quote! { Label::new() },
            FieldType::Panel => {
                let horizontal_scroll =
                    bool::from_str(node.attr_or("horizontal-scroll|horizontal_scroll|hscroll", "false"));
                let vertical_scroll = bool::from_str(node.attr_or("vertical-scroll|vertical_scroll|vscroll", "false"));
                let border = bool::from_str(node.attr_or("border|has-border", "false"));
                match (horizontal_scroll, vertical_scroll, border) {
                    (false, false, false) => quote! { Panel::new() },
                    (_, _, false) => quote! { Panel::new_scroll(#horizontal_scroll, #vertical_scroll) },
                    (_, _, true) => quote! { Panel::new_custom(#horizontal_scroll, #vertical_scroll, #border) },
                }
            }
            FieldType::ListBox => quote! { ListBox::new() },
            FieldType::PopUp => quote! { PopUp::new() },
            FieldType::Progress => quote! { Progress::new() },
            FieldType::Slider => {
                if bool::from_str(node.attr_or("vertical", "false")) {
                    quote! { Slider::new_vertical() }
                } else {
                    quote! { Slider::new() }
                }
            }
            FieldType::SplitView => {
                if bool::from_str(node.attr_or("vertical", "false")) {
                    quote! { SplitView::new_vertical() }
                } else {
                    quote! { SplitView::new() }
                }
            }
            FieldType::TableView => quote! { TableView::new() },
            FieldType::TextView => quote! { TextView::new() },
            FieldType::UpDown => quote! { UpDown::new() },
            FieldType::View => quote! { View::new() },
            FieldType::WebView => quote! { WebView::new() },
            FieldType::Line => {
                if bool::from_str(node.attr_or("vertical", "false")) {
                    quote! { Line::new_vertical() }
                } else {
                    quote! { Line::new() }
                }
            }
            FieldType::Layout => {
                let columns = LitInt::new(node.attr_or("cols|columns", "1"), Span::call_site());
                let rows = LitInt::new(node.attr_or("rows", "1"), Span::call_site());
                quote! { Layout::new(#columns, #rows) }
            }
            FieldType::Window => quote! { Window::new(WindowFlags::default()) },
            FieldType::Cell => return None,
            FieldType::Custom(_) => {
                let custom = node.type_ident()?;
                quote! { #custom::new() }
            }
        };
        Some(quote! {
            #name: #obj
        })
    }

    fn apply_attr(&self, id: usize) -> Option<TokenStream> {
        let node = self.node(id);
        let name = node.name_ident();
        match node.ty {
            FieldType::Button => {
                let text_setter = node.attr("text").map(|text| {
                    let text = LitStr::new(text, Span::call_site());
                    quote! { obj.#name.set_text(#text); }
                });
                let width_setter = node.attr("width").map(|width| {
                    let width = LitFloat::new(width, Span::call_site());
                    quote! { obj.#name.set_width(#width); }
                });
                let set_text_alt_setter = node.attr("set-text-alt").map(|set_text_alt| {
                    let set_text_alt = LitStr::new(set_text_alt, Span::call_site());
                    quote! { obj.#name.set_text_alt(#set_text_alt); }
                });
                Some(quote! {
                    #text_setter
                    #width_setter
                    #set_text_alt_setter
                })
            }
            FieldType::Label => {
                let text_setter = node.attr("text").map(|text| {
                    let text = LitStr::new(text, Span::call_site());
                    quote! { obj.#name.set_text(#text); }
                });
                Some(quote! {
                    #text_setter
                })
            }
            FieldType::Panel => {
                let panel_name = node.name_ident();
                let window = self.node(node.parent?);
                let window_name = window.name_ident();
                Some(quote! {
                    obj.#window_name.set_panel(obj.#panel_name);
                })
            }
            FieldType::Window => {
                let title_setter = node.attr("title").map(|title| {
                    let title = LitStr::new(title, Span::call_site());
                    quote! { obj.#name.set_title(#title); }
                });
                let client_size_setter = node.attr("client-size|size").and_then(|client_size| {
                    let client_size = client_size.split(',').collect::<Vec<_>>();
                    if client_size.len() != 2 {
                        return None;
                    }
                    let width = LitFloat::new(client_size[0], Span::call_site());
                    let height = LitFloat::new(client_size[1], Span::call_site());
                    Some(quote! { obj.#name.set_client_size(#width, #height); })
                });
                let origin_setter = node.attr("origin").and_then(|origin| {
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

    fn apply_layout(&self, id: usize) -> Option<TokenStream> {
        let node = self.node(id);
        match node.ty {
            FieldType::Layout => {
                let panel = self.node(node.parent?);
                let panel_name = panel.name_ident();
                let layout_name = node.name_ident();
                Some(quote! {
                    obj.#panel_name.add_layout(obj.#layout_name);
                })
            }
            FieldType::Cell => {
                let layout = self.node(node.parent?);
                let layout_name = layout.name_ident();
                let col = LitInt::new(node.attr("column|col")?, Span::call_site());
                let row = LitInt::new(node.attr("row")?, Span::call_site());
                let control_name = Ident::new(node.attr("for|control")?, Span::call_site());
                Some(quote! {
                    obj.#layout_name.set_control(#col, #row, obj.#control_name);
                })
            }
            _ => None,
        }
    }

    fn generate_setter(&self, id: usize) -> Option<TokenStream> {
        let node = self.node(id);
        let name = node.name_ident();
        match node.ty {
            FieldType::Button => {
                let on_click_setter = node.attr("on-click").map(|on_click| {
                    let on_click = Ident::new(&format!("setter_{}", on_click), Span::call_site());
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
            FieldType::Window => {
                let on_close_setter = node.attr("on-close").map(|on_close| {
                    let on_close = Ident::new(&format!("setter_{}", on_close), Span::call_site());
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
            FieldType::TextView => {
                let text_setter = node.attr("write").map(|text| {
                    let text_setter = Ident::new(&format!("setter_{}", text), Span::call_site());
                    quote! {
                        pub fn #text_setter(&self, text: &str) {
                            self.#name.write(text)
                        }
                    }
                });
                Some(quote! {
                    #text_setter
                })
            }
            _ => None,
        }
    }

    /// Generate all user defined struct.
    fn generate(&self) -> TokenStream {
        let root = self.node(self.root.unwrap());
        let mod_name = root.attr_or("mod|module|namespacing", "ui");
        let mod_name = Ident::new(mod_name, Span::call_site());
        let nodes = self.defined_nodes().map(|x| self.generate_node(*x));
        quote! {
            pub mod #mod_name {
                use nappgui::prelude::*;
                #(#nodes)*
            }

            use #mod_name::*;
        }
    }

    /// Generate the struct code from the generator.
    fn generate_node(&self, id: usize) -> TokenStream {
        let node = self.nodes.get(&id).unwrap();
        let child_nodes = self.child_nodes(id);

        let FieldType::Custom(struct_ident) = &node.ty else {
            panic!("Node under <UI> should be taged as custom type.")
        };

        let inner_name = node.name_ident();
        let inner_type = Ident::new(
            node.attr("inherits|extends")
                .expect("Root node should `inherits` from a existing type."),
            Span::call_site(),
        );
        let struct_ident = Ident::new(struct_ident, Span::call_site());
        let define_child_fields = child_nodes.iter().filter_map(|node| self.define_field(*node));
        let init_child_fields = child_nodes.iter().filter_map(|node| self.init_field(*node));
        let apply_layouts = child_nodes.iter().filter_map(|node| self.apply_layout(*node));
        let apply_attrs = child_nodes.iter().filter_map(|node| self.apply_attr(*node));
        let define_setters = child_nodes.iter().filter_map(|node| self.generate_setter(*node));
        quote! {
            #[derive(Debug, Clone, Copy)]
            pub struct #struct_ident {
                #inner_name: #inner_type,
                #(#define_child_fields,)*
            }

            impl #struct_ident {
                pub fn new() -> Self {
                    let obj = Self {
                        #inner_name: #inner_type::new(),
                        #(#init_child_fields,)*
                    };
                    #(#apply_layouts)*
                    #(#apply_attrs)*
                    obj
                }

                #(#define_setters)*
            }

            impl std::ops::Deref for #struct_ident {
                type Target = #inner_type;
                fn deref(&self) -> &Self::Target {
                    &self.#inner_name
                }
            }

            impl nappgui::gui::AsObject<#inner_type> for #struct_ident {
                fn as_object(self) -> #inner_type {
                    self.#inner_name
                }
            }
        }
    }

    /// Get the node from the generator. If not found, panic.
    fn node(&self, id: usize) -> &GeneratorNode {
        self.nodes.get(&id).unwrap()
    }

    /// Get all nodes based on id. If the id not found, panic.
    fn child_nodes(&self, id: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let mut node_stack = Vec::new();
        let node = self.node(id);
        for child in &node.children {
            node_stack.push(*child);
        }

        while let Some(node) = node_stack.pop() {
            result.push(node);
            for child in &self.node(node).children {
                node_stack.push(*child);
            }
        }
        result
    }

    /// Get the root nodes from the generator. If the id not found, panic.
    fn defined_nodes(&self) -> impl Iterator<Item = &usize> {
        let root = self.node(self.root.unwrap());
        root.children.iter()
    }
}

struct GeneratorNode {
    /// Unique ident for the node, used as field name in the struct and object name in the code.
    id: usize,
    /// The type of the node, used to determine the field type in the struct.
    ty: FieldType,
    /// Attributes of the node, used to apply properties to the object.
    attrs: HashMap<String, String>,
    /// The unique ident of the parent node, used to build the hierarchy of objects.
    parent: Option<usize>,
    /// The unique ident of the children nodes, used to build the hierarchy of objects.
    children: Vec<usize>,
}

impl GeneratorNode {
    /// Create a new node from an XML node.
    fn from_xml_node(node: Node, id: usize) -> Self {
        let attrs: HashMap<String, String> = node
            .attributes()
            .map(|attr| (attr.name().to_string(), attr.value().to_string()))
            .collect();
        let ty = FieldType::from_str(node.tag_name().name());
        Self {
            id,
            ty,
            attrs,
            parent: None,
            children: Vec::new(),
        }
    }

    /// Return the name of the node, which is the field name in the struct.
    fn name_ident(&self) -> Ident {
        if let Some(name) = self.attrs.get("name") {
            return Ident::new(name, Span::call_site());
        }
        Ident::new(&format!("__obj_{}", self.id), Span::call_site())
    }

    /// Return the type of the node, which is the field type in the struct.
    fn type_ident(&self) -> Option<Ident> {
        match &self.ty {
            FieldType::Button => Some(Ident::new("Button", Span::call_site())),
            FieldType::Combo => Some(Ident::new("Combo", Span::call_site())),
            FieldType::Edit => Some(Ident::new("Edit", Span::call_site())),
            FieldType::ImageView => Some(Ident::new("ImageView", Span::call_site())),
            FieldType::Label => Some(Ident::new("Label", Span::call_site())),
            FieldType::Panel => Some(Ident::new("Panel", Span::call_site())),
            FieldType::ListBox => Some(Ident::new("ListBox", Span::call_site())),
            FieldType::PopUp => Some(Ident::new("PopUp", Span::call_site())),
            FieldType::Progress => Some(Ident::new("Progress", Span::call_site())),
            FieldType::Slider => Some(Ident::new("Slider", Span::call_site())),
            FieldType::SplitView => Some(Ident::new("SplitView", Span::call_site())),
            FieldType::TableView => Some(Ident::new("TableView", Span::call_site())),
            FieldType::TextView => Some(Ident::new("TextView", Span::call_site())),
            FieldType::UpDown => Some(Ident::new("UpDown", Span::call_site())),
            FieldType::View => Some(Ident::new("View", Span::call_site())),
            FieldType::WebView => Some(Ident::new("WebView", Span::call_site())),
            FieldType::Line => Some(Ident::new("Line", Span::call_site())),
            FieldType::Layout => Some(Ident::new("Layout", Span::call_site())),
            FieldType::Window => Some(Ident::new("Window", Span::call_site())),
            FieldType::Custom(custom) => Some(Ident::new(custom, Span::call_site())),
            _ => None,
        }
    }

    /// Returns the real type of the node.
    /// If the node is a root node, it returns the type specified in the `inherits` attribute.
    fn attr_or(&self, name: &str, default: &'static str) -> &str {
        let names: Vec<&str> = name.split("|").collect();
        for name in names {
            if let Some(value) = self.attrs.get(name) {
                return value;
            }
        }
        default
    }

    fn attr(&self, name: &str) -> Option<&str> {
        let names: Vec<&str> = name.split("|").collect();
        for name in names {
            if let Some(value) = &self.attrs.get(name) {
                return Some(value);
            }
        }
        None
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum FieldType {
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
    Custom(String),
}

impl FieldType {
    fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "button" => Self::Button,
            "combo" => Self::Combo,
            "edit" => Self::Edit,
            "imageview" | "image-view" | "image_view" => Self::ImageView,
            "label" => Self::Label,
            "panel" => Self::Panel,
            "listbox" | "list-box" | "list_box" => Self::ListBox,
            "popup" => Self::PopUp,
            "progress" => Self::Progress,
            "slider" => Self::Slider,
            "splitview" | "split-view" | "split_view" => Self::SplitView,
            "tableview" | "table-view" | "table_view" => Self::TableView,
            "textview" | "text-view" | "text_view" => Self::TextView,
            "updown" | "up-down" | "up_down" => Self::UpDown,
            "view" => Self::View,
            "webview" | "web-view" | "web_view" => Self::WebView,
            "line" => Self::Line,
            "layout" => Self::Layout,
            "cell" => Self::Cell,
            "window" | "win" => Self::Window,
            _ => Self::Custom(s.trim().to_owned()),
        }
    }
}

trait FromStr {
    fn from_str(s: &str) -> Self;
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
