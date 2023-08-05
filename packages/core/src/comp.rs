use std::fs;
use std::path::Path;

use flexscript::ASTNode;
use flexscript::Assign;
use flexscript::Parser;
use flexscript::Value;

use crate::codegen::CodeNode;
use crate::codegen::Html;



pub struct Compiler {
    js: Vec<CodeNode>,
    html: Html
}

impl Compiler {
    pub fn compile(&mut self, ast: &Vec<ASTNode>) -> String {
        let mut buff = String::new();
        for node in ast {
            match node {
                ASTNode::Call(cal) => {
                    // match *cal.callee {
                    //     ASTNode::Ident(ident) => {
                    //         // match ident.as_str() {
                    //         //     "import" => {
                    //         //         if cal.args.len() != 1 {
                    //         //             panic!("import takes 1 argument");
                    //         //         }

                    //         //         let s = match &cal.args[0] {
                    //         //             ASTNode::Lit(lit) => {
                    //         //                 match *lit {
                    //         //                     Value::Str(s) => s,
                    //         //                     _ => panic!("import takes a string")
                    //         //                 }
                    //         //             },
                    //         //             _ => panic!("import takes a string")
                    //         //         };

                    //         //         let path = Path::new(&s);
                    //         //         let data = fs::read_to_string(path).unwrap();
                    //         //         let mut ast = Parser::new(&data).parse();
                    //         //         buff.push_str(&self.compile(&mut ast));

                    //         //         continue;
                    //         //     },
                    //         //     _ => {}
                    //         // }
                    //     },
                    //     _ => {}
                    // }
                },
                ASTNode::Assign(asg) => {
                    // let assign = Assign {
                    //     left: Box::new(self.compile(asg.left)),
                    //     right: Box::new(self.compile(asg.right))
                    // };
                },
                _ => {}
            }
            // buff.push_str(&node.to_js());
            // buff.push_str("\n");
        }
        buff
    }
}

