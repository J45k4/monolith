use std::path::Path;
use std::sync::Arc;

use flexscript::ASTNode;
use flexscript::Parser;
use flexscript::Value;

use crate::codegen::Child;
use crate::codegen::CodeNode;
use crate::codegen::CssCode;
use crate::codegen::Head;
use crate::codegen::Html;
use crate::codegen::HtmlEl;
use crate::codegen::HtmlElType;
use crate::codegen::ToJS;
use crate::prog::Prog;
use crate::prog::ProgCtx;
use crate::prog::ToProg;


pub struct DynProg {
    js: String,
    css: String,
    html: String
}

impl Prog for DynProg {
    fn js(&self, ctx: ProgCtx) -> String {
        self.js.clone()
    }

    fn css(&self, ctx: ProgCtx) -> String {
        self.css.clone()
    }

    fn html(&self, ctx: ProgCtx) -> String {
        self.html.clone()
    }
}

impl ToProg for String {
    fn to_prog(&self) -> Arc<dyn Prog + Send + Sync> {
        let path = Path::new(self);

        let code_text = if path.exists() {
            std::fs::read_to_string(path).unwrap()
        } else {
            self.clone()
        };

        create_dynprog(&code_text)
    }
}

struct CompState {
    js: CodeNode,
    css: CssCode,
    html: Html
}

fn create_dynprog(code_text: &str) -> Arc<dyn Prog + Send + Sync> {
    let ast_nodes = Parser::new(code_text).parse();

    let mut state = CompState {
        js: CodeNode::None,
        css: CssCode::None,
        html: Html::default()
    };

    // for node in ast_nodes {
    //     compile_node(&mut state, &node);
    // }

    Arc::new(DynProg{
        html: gen_html(&ast_nodes).to_string(),
        css: state.css.to_string(),
        js: state.js.to_js()
    })
}

fn gen_head(node: &ASTNode) -> Head {
    println!("gen_head");
    let mut head = Head::default();

    match node {
        ASTNode::StructIns(ins) => {
            match ins.name.as_str() {
                "Head" => {
                    for prop in &ins.probs {
                        if prop.name == "title" {
                            if let ASTNode::Lit(value) = &*prop.value {
                                if let Value::Str(s) = value {
                                    head.title = s.to_string();
                                }
                            }
                        }
                    }
                },
                _ => todo!(),
            }
        },
        _ => todo!()
    }

    head
}

fn gen_el(node: &ASTNode) -> HtmlEl {
    println!("gen_el");
    match node {
        ASTNode::StructIns(ins) => {
            let typ = match ins.name.as_str() {
                "H1" => HtmlElType::H1,
                "H2" => HtmlElType::H2,
                "H3" => HtmlElType::H3,
                "H4" => HtmlElType::H4,
                "H5" => HtmlElType::H5,
                "H6" => HtmlElType::H6,
                "Button" => HtmlElType::Button,
                "Div" => HtmlElType::Div,
                "Input" => HtmlElType::Input,
                _ => todo!()
            };

            println!("typ: {:?}", typ);

            let mut el = HtmlEl {
                typ: typ.clone(),
                children: Vec::new()
            };

            for prop in &ins.probs {
                match (&typ, prop.name.as_str()) {
                    (_, "text") => {
                        if let ASTNode::Lit(value) = &*prop.value {
                            if let Value::Str(s) = value {
                                el.children.push(Child::Text(s.to_string()));
                            }
                        }
                    }
                    (HtmlElType::Div, "children") => {
                        if let ASTNode::Array(arr) = &*prop.value {
                            for item in &arr.items {
                                if let ASTNode::Lit(value) = item {
                                    if let Value::Str(s) = value {
                                        el.children.push(Child::Text(s.to_string()));
                                    }
                                } else {
                                    el.children.push(Child::HtmlEl(gen_el(&item)));
                                }
                            }

                            continue;   
                        }

                        el.children.push(Child::HtmlEl(gen_el(&prop.value)));
                    }
                    (_, _) => {}
                };
            }

            el
        },
        _ => todo!("{:?}", node)
    }
}

fn get_body(node: &ASTNode) -> HtmlEl {
    println!("gen_body");

    let mut h = HtmlEl {
        typ: HtmlElType::Body,
        children: Vec::new(),
    };

    match node {
        ASTNode::Array(arr) => {
            for item in &arr.items {
                if let ASTNode::Lit(v) = item {
                    match v {
                        Value::Str(s) => {
                            h.children.push(Child::Text(s.to_string()));
                        }
                        _ => {}
                    }
                } else {
                    h.children.push(Child::HtmlEl(gen_el(&item)));
                }
            }
        }
        ASTNode::StructIns(ins) => {
            match ins.name.as_str() {
                "Body" => {
                    for prop in &ins.probs {
                        if prop.name == "children" {
                            h.children.push(Child::HtmlEl(gen_el(&prop.value)));
                        }
                    }
                },
                _ => todo!()
            }
        }
        _ => todo!()
    }

    h
}

fn gen_html(nodes: &Vec<ASTNode>) -> Html {
    println!("gen_html");

    let mut html = Html::default();

    for node in nodes {
        match node {
            ASTNode::Ret(r) => {
                match &*r.value {
                    Some(ASTNode::StructIns(ins)) => {
                        if ins.name == "Html" {
                            for prop in &ins.probs {
                                if prop.name == "head" {
                                    html.head = gen_head(&prop.value)
                                }

                                if prop.name == "body" {
                                    html.body = get_body(&prop.value)
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => break
        }
    }

    html
}

fn compile_node(state: &mut CompState, node: &ASTNode) {
    match node {
        ASTNode::Ident(idt) => {
            if idt == "Html" {

            }
        },
        ASTNode::Assign(_) => todo!(),
        ASTNode::StructIns(ins) => {
            if ins.name == "Html" {
                for prop in &ins.probs {
                    if prop.name == "head" {
                        if let ASTNode::StructIns(ins) = &*prop.value {
                            for prop in &ins.probs {
                                if prop.name == "title" {
                                    if let ASTNode::Lit(value) = &*prop.value {
                                        if let Value::Str(s) = value {
                                            state.html.head.title = s.to_string();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if ins.name == "Body" {

            }
        },
        ASTNode::ForLoop(_) => todo!(),
        ASTNode::Array(arr) => {
            for node in &arr.items {
                compile_node(state, node);
            }
        },
        ASTNode::Call(_) => todo!(),
        ASTNode::Property(_, _) => todo!(),
        ASTNode::Lit(_) => todo!(),
        ASTNode::LiteralPercent(_) => todo!(),
        ASTNode::Fun(_) => todo!(),
        ASTNode::StructDef(_) => todo!(),
        ASTNode::TypeDef(_) => todo!(),
        ASTNode::Var(_) => todo!(),
        ASTNode::ProbAccess(_) => todo!(),
        ASTNode::Obj(_) => todo!(),
        ASTNode::Ret(r) => {

        },
        ASTNode::BinOp(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_some() {
        
    }
}