use std::path::Path;
use std::sync::Arc;
use flexscript::Parser;

pub struct ProgCtx {}

pub trait Prog {
    fn js(&self, ctx: ProgCtx) -> String;
    fn css(&self, ctx: ProgCtx) -> String;
    fn html(&self, ctx: ProgCtx) -> String;
}

pub trait ToProg {
    fn to_prog(&self) -> Arc<dyn Prog + Send + Sync>;
}