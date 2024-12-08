
use std::{error::Error, fs};

use super::{Realizable, Resolvable, Env};
use crate::{load_document, parser::ast};

impl ast::IncludePath
{
    fn get_path(&self) -> Box<str>
    {
        match self {
            Self::LocalInclude(inc) => inc.content.clone(),
            Self::SystemInclude(inc) => {
                // TODO: scan system paths
                inc.content.clone()
            }
        }
    }
}

macro_rules! is_with_macros {
    ($dec:ident) => {
        Box::leak($dec.ident.clone()).eq(&"with_macros")
    }
}

impl<'pest> ast::Decorator<'pest>
{
    fn import_with_macros(&self) -> Result<Box<ast::Document<'pest>>, Box<dyn Error>>
    {
        let inc: &ast::CInclude;
        let src: &'pest str = "";

        if ! is_with_macros!(self) {
            return Err("Can only be used with the with_macros decorator.")?
        }
        if self.args.len() > 0 {
            return Err("with_macros does not take arguments.")?
        }
        match &self.target {
            ast::CChunk::CInclude(ci) => { inc = ci; },
            _ => { return Err("with_macros must decorate an #include statement")?; }
        }

        fs::read_to_string(Box::leak(inc.path.get_path()))?.clone_into(&mut src.to_owned());

        Ok(load_document(src))
    }
}

// TODO: do we need to be Realizable?
impl<'pest> Realizable<'pest> for ast::Decorator<'pest>
{
    fn realize(&self, env: Vec<Env>) -> &str
    {
        ""
    }
}

impl<'pest> Resolvable<'pest> for ast::Decorator<'pest>
{
    fn resolve(&self) -> Vec<&dyn Realizable<'pest>>
    {
        if is_with_macros!(self) {
            let doc = Box::leak(self.import_with_macros().unwrap());
            doc.resolve()
        } else {
            // TODO: execute decorator from env
            // 1. Parse the target CChunk to determine its type and pattern-match
            // 2. Split and assign body environment prior to macro call
            // 3. Return result of macro call in resolution process
            vec!()
        }
    }
}
