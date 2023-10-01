use std::collections::HashSet;

use flexscript::ASTNode;
use flexscript::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum JSNode {
    Assign {
        left: Box<JSNode>,
        right: Box<JSNode>
    },
    Let {
        name: String,
        value: Box<JSNode>
    },
    ForOf {
        name: String,
        value: Box<JSNode>,
        body: Box<JSNode>
    },
    If {
        condition: Box<JSNode>,
        body: Box<JSNode>
    },
    Ident(String),
    Number(f64),
    String(String),
    PropAccess {
        object: Box<JSNode>,
        property: Box<JSNode>
    },
    IndexAccess {
        object: Box<JSNode>,
        index: Box<JSNode>
    },
    Ret(Option<Box<JSNode>>),
    Fn {
        args: Vec<String>,
        body: Vec<JSNode>
    },
    Call {
        callee: Box<JSNode>,
        args: Vec<JSNode>
    },
    Many(Vec<JSNode>)
}

impl ToString for JSNode {
    fn to_string(&self) -> String {
        match self {
            JSNode::Assign { left, right } => {
                format!("{} = {}", left.to_string(), right.to_string())
            },
            JSNode::Let { name, value } => {
                format!("const {} = {}", name, value.to_string())
            },
            JSNode::ForOf { name, value, body } => {
                format!("for (const {} of {}) {{\n{}\n}}", name, value.to_string(), body.to_string())
            },
            JSNode::If { condition, body } => {
                format!("if ({}) {{\n{}\n}}", condition.to_string(), body.to_string())
            },
            JSNode::Ident(i) => i.clone(),
            JSNode::Number(n) => n.to_string(),
            JSNode::String(s) => format!("\"{}\"", s),
            JSNode::PropAccess { object, property } => {
                format!("{}.{}", object.to_string(), property.to_string())
            },
            JSNode::Ret(None) => "return".to_string(),
            JSNode::Ret(Some(v)) => format!("return {}", v.to_string()),
            JSNode::Fn { args, body } => {
                let args_str = args.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let body_str = body.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("\n");
                format!("({}) => {{\n{}\n}}", args_str, body_str)
            },
            JSNode::Call { callee, args } => {
                let mut args_str = String::new();
                for arg in args {
                    args_str.push_str(&arg.to_string());
                    args_str.push_str(", ");
                }
                args_str.pop();
                args_str.pop();
                format!("{}({})", callee.to_string(), args_str)
            },
            JSNode::Many(many) => {
                let mut many_str = String::new();
                for node in many {
                    many_str.push_str(&node.to_string());
                    many_str.push_str("\n");
                }
                many_str
            },
            JSNode::IndexAccess { object, index } => {
                format!("{}[{}]", object.to_string(), index.to_string())
            }
        }
    }
}

pub struct JSGen {
    log: usize,
    depth: usize,
    pub symbol_table: Vec<HashSet<String>>
}

impl JSGen {
    pub fn new() -> Self {
        Self {
            log: 0,
            depth: 0,
            symbol_table: Vec::new()
        }
    }

    fn set_log(mut self, log: usize) -> Self {
        self.log = log;
        self
    }

    fn does_var_exist(&self, name: &str) -> bool {
        for scope in &self.symbol_table {
            if scope.contains(name) {
                return true;
            }
        }
        false
    }
    fn insert_var(&mut self, name: &str) {
        if self.symbol_table.len() == 0 {
            self.symbol_table.push(HashSet::new());
        }
        self.symbol_table.last_mut().unwrap().insert(name.to_string());
    }

    pub fn process_node(&mut self, node: &ASTNode) -> JSNode {
        self.depth += 1;

        if self.log > 0 {
            match node {
                ASTNode::Ident(_) => println!("Ident"),
                ASTNode::Assign(_) => println!("Assign"),
                ASTNode::ObjIns(o) => println!("ObjIns {:?}", o.name),
                ASTNode::Array(_) => println!("Array"),
                ASTNode::Call(_) => println!("Call"),
                ASTNode::Property(_, _) => println!("Property"),
                ASTNode::Lit(_) => println!("Lit"),
                ASTNode::LiteralPercent(_) => println!("LiteralPercent"),
                ASTNode::Fun(_) => println!("Fun"),
                ASTNode::StructDef(_) => println!("StructDef"),
                ASTNode::TypeDef(_) => println!("TypeDef"),
                ASTNode::Var(_) => println!("Var"),
                ASTNode::ProbAccess(_) => println!("ProbAccess"),
                ASTNode::Ret(_) => println!("Ret"),
                ASTNode::BinOp(_) => println!("BinOp"),
                ASTNode::If(_) => println!("If"),
                ASTNode::For(_) => println!("For"),
            }
        }

        let r = match node {
            ASTNode::Assign(a) => {
                let left = self.process_node(&a.left);
                let right = self.process_node(&a.right);

                if let JSNode::Ident(i) = &left {
                    if !self.does_var_exist(&i) {
                        self.insert_var(&i);
                        return JSNode::Let { 
                            name: i.clone(), 
                            value: Box::new(self.process_node(&a.right)) 
                        }
                    }
                }
                
                return JSNode::Assign { 
                    left: Box::new(left), 
                    right: Box::new(right)
                }
            },
            ASTNode::Ident(i) => JSNode::Ident(i.clone()),
            ASTNode::Lit(lit) => {
                match lit {
                    Value::Int(i) => JSNode::Number(*i as f64),
                    Value::Float(f) => JSNode::Number(*f),
                    Value::Str(s) => JSNode::String(s.clone()),
                    _ => todo!()
                }
            },
            ASTNode::Ret(r) => {
                if self.depth == 1 {
                    let mut body = Vec::new();

                    if let Some(ref v) = *r.value {
                        // body.push(self.process_node(&v));

                        match v {
                            ASTNode::ObjIns(o) => {
                                for prop in &o.props {
                                    if prop.name == "head" {
                                        let mut title = None;

                                        match &*prop.value {
                                            ASTNode::ObjIns(o) => {
                                                for prop in &o.props {
                                                    if prop.name == "title" {
                                                        match &*prop.value {
                                                            ASTNode::Lit(Value::Str(s)) => {
                                                                title = Some(s.clone());
                                                            },
                                                            _ => todo!()
                                                        }
                                                    }
                                                }
                                            },
                                            _ => todo!()
                                        }

                                        body.push(
                                            JSNode::Let { 
                                                name: "head".to_string(), 
                                                value: Box::new(JSNode::Call { 
                                                    callee: Box::new(JSNode::PropAccess { 
                                                        object: Box::new(JSNode::Ident("document".to_string())), 
                                                        property: Box::new(JSNode::Ident("querySelector".to_string()))
                                                    }), 
                                                    args: vec![JSNode::String("head".to_string())] 
                                                })
                                            }
                                        );

                                        if let Some(t) = title {
                                            body.push(
                                                JSNode::Assign { 
                                                    left: Box::new(JSNode::PropAccess { 
                                                        object: Box::new(JSNode::Ident("head".to_string())), 
                                                        property: Box::new(JSNode::Ident("title".to_string()))
                                                    }), 
                                                    right: Box::new(JSNode::String(t))
                                                }
                                            );
                                        }
                                    }

                                    if prop.name == "body" {
                                        body.push(
                                            JSNode::Let { 
                                                name: "body".to_string(), 
                                                value: Box::new(JSNode::Call { 
                                                    callee: Box::new(JSNode::PropAccess { 
                                                        object: Box::new(JSNode::Ident("document".to_string())), 
                                                        property: Box::new(JSNode::Ident("querySelector".to_string()))
                                                    }), 
                                                    args: vec![JSNode::String("body".to_string())] 
                                                })
                                            }
                                        );

                                        body.push(
                                            JSNode::Let { 
                                                name: "children".to_string(), 
                                                value: Box::new(JSNode::PropAccess { 
                                                    object: Box::new(JSNode::Ident("body".to_string())), 
                                                    property: Box::new(JSNode::Ident("children".to_string()))
                                                })
                                            }
                                        );

                                        match &*prop.value {
                                            ASTNode::Array(a) => {
                                                for item in &a.items {
                                                    body.push(self.process_node(&item)); 
                                                }
                                            },
                                            _ => todo!()
                                        }
                                    }
                                }
                            },
                            _ => todo!()
                        }
                    }

                    // self.process_node(&r.value);
                    JSNode::Assign { 
                        left: Box::new(JSNode::PropAccess { 
                            object: Box::new(JSNode::Ident("window".to_string())), 
                            property: Box::new(JSNode::Ident("onload".to_string()))
                        }), 
                        right: Box::new(JSNode::Fn { 
                            args: vec![], 
                            body
                        })
                    }
                } else {
                    JSNode::Ret(None)
                }
            },
            ASTNode::ObjIns(o) => {
                let mut many = vec![];

                match &o.name {
                    Some(n) => {
                        many.push(
                            JSNode::Let { 
                                name: "e".to_string(), 
                                value: Box::new(JSNode::IndexAccess { 
                                    object:  Box::new(JSNode::Ident("children".to_string())),
                                    index: Box::new(JSNode::Number(0.0))
                                })
                            }
                        );

                        // many.push(
                        //     JSNode::Let { 
                        //         name: "e".to_string(), 
                        //         value: Box::new(JSNode::Call { 
                        //             callee: Box::new(JSNode::PropAccess { 
                        //                 object: Box::new(JSNode::Ident("document".to_string())), 
                        //                 property: Box::new(JSNode::Ident("createElement".to_string()))
                        //             }), 
                        //             args: vec![JSNode::String(n.to_string())] 
                        //         })
                        //     }
                        // );

                        // match n.as_str() {
                        //     "Html" => {}
                        //     "Head" => {},
                        //     "Body" => {},
                        //     _ => {}
                        // };
                        // JSNode::Call {
                        //     callee: Box::new(JSNode::PropAccess { 
                        //         object: Box::new(JSNode::Ident("document".to_string())), 
                        //         property: Box::new(JSNode::Ident("querySelector".to_string()))
                        //     }),
                        //     args: vec![JSNode::String(n.to_string())]
                        // }
                    },
                    None => todo!(),
                }

                for prop in &o.props {
                    match prop.name.as_str() {
                        "text" => {
                            many.push(
                                JSNode::Assign { 
                                    left: Box::new(JSNode::PropAccess { 
                                        object: Box::new(JSNode::Ident("e".to_string())), 
                                        property: Box::new(JSNode::Ident("innerText".to_string()))
                                    }), 
                                    right: match &*prop.value {
                                        ASTNode::Lit(Value::Str(s)) => Box::new(JSNode::String(s.clone())),
                                        _ => todo!()
                                    }
                                }
                            );
                        },
                        "on_click" => {
                            many.push(
                                JSNode::Assign { 
                                    left: Box::new(JSNode::PropAccess { 
                                        object: Box::new(JSNode::Ident("e".to_string())), 
                                        property: Box::new(JSNode::Ident("onclick".to_string()))
                                    }), 
                                    right: Box::new(self.process_node(&prop.value))
                                }
                            );
                        },
                        _ => {}
                    }
                }

                JSNode::Many(many)
            },
            ASTNode::Call(c) => {
                let callee = self.process_node(&c.callee);
                let mut args = Vec::new();

                for arg in &c.args {
                    args.push(self.process_node(&arg));
                }

                JSNode::Call { callee: Box::new(callee), args }
            },
            ASTNode::Fun(f) => {
                let mut args = Vec::new();
                let mut body = Vec::new();

                for arg in &f.params {
                    args.push(arg.name.clone());
                }

                for node in &f.body {
                    body.push(self.process_node(&node));
                }

                JSNode::Fn { args, body }
            },
            ASTNode::ProbAccess(p) => {
                let object = self.process_node(&p.object);
                let property = JSNode::Ident(p.property.clone());

                JSNode::PropAccess { object: Box::new(object), property: Box::new(property) }
            },
            _ => todo!("{:?}", node)
        };

        self.depth -= 1;

        r
    }

    pub fn gen(mut self, ast: Vec<ASTNode>) -> JSNode {
        let mut js_nodes = Vec::new();

        for node in ast {
            js_nodes.push(self.process_node(&node));
        }

        JSNode::Many(js_nodes)
    }
}

#[cfg(test)]
mod tests {
    use flexscript::Parser;

    use super::*;

    #[test]
    fn const_assign() {
        let code = r#"
        a = 5
        "#;
        let ast = Parser::new(code).parse();
        let mut js_code = JSGen::new().gen(ast);

        println!("{:#?}", js_code);

        let expected = vec![
            JSNode::Let {
                name: "a".to_string(),
                value: Box::new(JSNode::Number(5.0))
            }
        ];
    }

    #[test]
    fn let_reassign() {
        let code = r#"
        a = 5
        a = 6
        "#;
        let ast = Parser::new(code).parse();
        let mut js_code = JSGen::new().gen(ast);

        println!("{:#?}", js_code);

        

        let expected = JSNode::Many(
            vec![
                JSNode::Let {
                    name: "a".to_string(),
                    value: Box::new(JSNode::Number(5.0))
                },
                JSNode::Assign { 
                    left: Box::new(JSNode::Ident("a".to_string())), 
                    right: Box::new(JSNode::Number(6.0))
                }
            ]
        );

        assert_eq!(js_code, expected);
    }

    #[test]
    fn return_html_with_button() {
        let code = r#"
        return Html {
            head: Head {
                title: "hello world"
            }
            body: [
                Button {
                    text: "click me"
                    on_click: () => {
                        print("hello world")
                    }
                }
            ]
        }
        "#;

        let ast = Parser::new(code).parse();
        println!("{:#?}", ast);
        let mut js_code = JSGen::new().set_log(1).gen(ast);

        println!("{:#?}", js_code);
        println!("{}", js_code.to_string());
    }

//     #[test]
//     fn button_onclick() {
//         let code = r#"
//         return Html {
//             head: Head {
//                 title: "hello world"
//             }
//             body: [
//                 Button {
//                     text: "click me",
//                     on_click: () => {
//                         print("hello world")
//                     }
//                 }
//             ]
//         }
//         "#;
//         let ast = Parser::new(code).parse();
//         let mut js_code = JsGenerator::new(ast).gen();

//         println!("{:#?}", js_code);

// //         let expected = r#"const head = document.querySelector("head")
// // const body = document.querySelector("body")
// // const el_1 = document.createElement("button")
// // el_1.innerText = "click me"
// // el_1.onclick = () => {
// //     console.log("hello world")
// // }"#;

// //         assert_eq!(js_code, expected);
//     }

//     #[test]
//     fn change_text() {
//         let code = r#"
//         state = 1
//         return Html {
//             head: Head {
//                 title: "hello world"
//             }
//             body: [
//                 Button {
//                     text: "click me"
//                     on_click: () => {
//                         state = state + 1
//                     }
//                 }
//                 H1 {
//                     text: state
//                 }
//             ]
//         }
//         "#;
//         let ast = Parser::new(code).parse();
//         println!("{:#?}", ast);
//         let mut js_code = JsGenerator::new(ast).gen();

// //         let expected = r#"const h_1 = document.createElement("h1")[0]
// // const setState = (state) => {
// //     c.innerText = state
// // }
// // const b_1 = document.createElement("button")[0]
// // b_1.onclick = () => {
// //     setState(state + 1)
// // }"#;

// //         assert_eq!(js_code, expected);
//     }
}