pub trait Token {

}

pub trait BaseLexer {
    fn create_from(str: &str) -> Self;
    fn next<T: Token + Sized>(&self) -> Result<Option<T>, ()>;
}