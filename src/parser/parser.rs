use crate::parser::work_session;

pub trait BaseParser {
    fn parse<S: work_session::BaseWorkSession + Sized>(&self, str: &str) -> Result<S, ()>;
    fn clear_cache(&self) -> ();
}

pub struct FiberMarkParser {}

impl BaseParser for FiberMarkParser {
    fn parse<S: work_session::BaseWorkSession + Sized>(&self, _str: &str) -> Result<S, ()> {
        unimplemented!()
    }

    fn clear_cache(&self) -> () {
        unimplemented!()
    }
}