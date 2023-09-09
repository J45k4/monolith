pub trait ToJS {
    fn to_js(&self) -> String;
}



// pub fn js_code_for_node(buff: &mut String, ast: &ASTNode) {
//     match ast {
//         ASTNode::Assign(assign) => {
//             js_code_for_node(buff, &assign.left);
//             buff.push_str(" = ");
//             js_code_for_node(buff, &assign.right);
//         },
//         ASTNode::Ident(ident) => {
//             buff.push_str(&ident);
//         },
//         ASTNode::Lit(lit) => {
//             match lit {
//                 Value::Int(i) => {
//                     buff.push_str(&i.to_string());
//                 },
//                 _ => {
//                     todo!("unexpected value: {:?}", lit);
//                 }
//             }
//         },
//         _ => {
//             todo!("unexpected node: {:?}", ast);
//         }
//     }
// }

// pub fn js_code_for_nodes(buff: &mut String, ast: &Vec<ASTNode>) {
//     for node in ast {
//         js_code_for_node(buff, node);
//     }
// }



// impl ToHTML for HtmlNode {
//     fn to_html(&self) -> String {
//         match self {
//             HtmlNode::H1(s) => format!("<h1>{}</h1>", s),
//             HtmlNode::H2(s) => format!("<h2>{}</h2>", s),
//             HtmlNode::H3(s) => format!("<h3>{}</h3>", s),
//             HtmlNode::H4(s) => format!("<h4>{}</h4>", s),
//             HtmlNode::H5(s) => {
//                 format!("<h5>{}</h5>", s)
//             },
//             HtmlNode::H6(s) => {
//                 format!("<h6>{}</h6>", s)
//             },
//         }
//     }
// }


pub struct Fn {
    pub asyn: bool,
    pub args: Vec<String>,
    pub body: Vec<CodeNode>
}

impl ToJS for Fn {
    fn to_js(&self) -> String {
        let mut buff = String::new();
        if self.asyn {
            buff.push_str("async ");
        }
        buff.push_str("(");
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                buff.push_str(", ");
            }
            buff.push_str(arg);
        }
        buff.push_str(") => {\n");
        for node in &self.body {
            buff.push_str(&node.to_js());
            buff.push_str("\n");
        }
        buff.push_str("}");
        buff
    }
}

pub struct Call {
    pub calle: Box<CodeNode>,
    pub args: Vec<CodeNode>
}

impl ToJS for Call {
    fn to_js(&self) -> String {
        let mut buff = String::new();
        buff.push_str(&self.calle.to_js());
        buff.push_str("(");
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                buff.push_str(", ");
            }
            buff.push_str(&arg.to_js());
        }
        buff.push_str(")");
        buff
    }
}

pub struct ConstVar {
    pub name: String,
    pub value: Box<CodeNode>
}

impl ToJS for ConstVar {
    fn to_js(&self) -> String {
        format!("const {} = {}", self.name, self.value.to_js())
    }
}

pub enum ForCond {
    In {
        name: String,
        value: Box<CodeNode>
    },
    Of {
        name: String,
        value: Box<CodeNode>
    }
}

pub struct For {
    pub cond: ForCond,
    pub body: Vec<CodeNode>
}

impl ToJS for For {
    fn to_js(&self) -> String {
        let mut buff = String::new();
        buff.push_str("for (const ");
        match &self.cond {
            ForCond::In { name, value } => {
                buff.push_str(&name);
                buff.push_str(" in ");
                buff.push_str(&value.to_js());
            },
            ForCond::Of { name, value } => {
                buff.push_str(&name);
                buff.push_str(" of ");
                buff.push_str(&value.to_js());
            }
        }
        buff.push_str(") {\n");
        for node in &self.body {
            buff.push_str(&node.to_js());
            buff.push_str("\n");
        }
        buff.push_str("}");
        buff
    }
}

pub struct PropAccess {
    pub name: Box<CodeNode>,
    pub value: Box<CodeNode>
}

pub struct Assign {
    pub name: Box<CodeNode>,
    pub value: Box<CodeNode>
}

pub enum CodeNode {
    None,
    Fn(Fn),
    Call(Call),
    ConstVar(ConstVar),
    Num(i64),
    Str(String),
    Ident(String),
    PropAccess(PropAccess),
}

impl ToJS for CodeNode {
    fn to_js(&self) -> String {
        match self {
            CodeNode::None => "".to_string(),
            CodeNode::Fn(f) => f.to_js(),
            CodeNode::Call(c) => c.to_js(),
            CodeNode::ConstVar(c) => c.to_js(),
            CodeNode::Num(n) => n.to_string(),
            CodeNode::Str(s) => format!("\"{}\"", s),
            CodeNode::Ident(i) => i.to_string(),
            CodeNode::PropAccess(p) => format!("{}.{}", p.name.to_js(), p.value.to_js())
        }
    }
}

pub enum CssCode {
    None
}

impl ToString for CssCode {
    fn to_string(&self) -> String {
        match self {
            CssCode::None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_codegen() {
        let code_desc = ConstVar {
            name: "a".to_string(),
            value: Box::new(CodeNode::Num(5))
        };

        assert_eq!(code_desc.to_js(), "const a = 5");
    }

    #[test]
    fn test_js_async_fn() {
        let code_desc = Fn {
            asyn: true,
            args: vec!["a".to_string(), "b".to_string()],
            body: vec![
                CodeNode::Call(Call {
                    calle: Box::new(CodeNode::PropAccess(PropAccess {
                        name: Box::new(CodeNode::Ident("console".to_string())),
                        value: Box::new(CodeNode::Ident("log".to_string()))
                    })),
                    args: vec![
                        CodeNode::Str("hello".to_string())
                    ]
                })
            ]
        };

        assert_eq!(code_desc.to_js(), "async (a, b) => {\nconsole.log(\"hello\")\n}");
    }

    #[test]
    fn test_js_for() {
        let code_desc = For {
            cond: ForCond::In {
                name: "a".to_string(),
                value: Box::new(CodeNode::Ident("b".to_string()))
            },
            body: vec![
                CodeNode::Call(Call {
                    calle: Box::new(CodeNode::Ident("foo".to_string())),
                    args: vec![]
                })
            ]
        };

        assert_eq!(code_desc.to_js(), "for (const a in b) {\nfoo()\n}");
    }

    #[test]
    fn test_js_prop_access() {
        let code_desc = Call {
            calle: Box::new(CodeNode::PropAccess(PropAccess { 
                name: Box::new(CodeNode::Ident("document".to_string())), 
                value: Box::new(CodeNode::Ident("createElement".to_string())),
            })), 
            args: vec![CodeNode::Str("div".to_string())]
        };

        assert_eq!(code_desc.to_js(), r#"document.createElement("div")"#);
    }

    #[test]
    fn test_html_test_div() {
        let html_desc = HtmlEl {
            typ: HtmlElType::Div,
            children: vec![
                Child::Text("hello".to_string()),
            ]
        };

        assert_eq!(html_desc.to_html(), "<div>\nhello\n</div>");
    }
}