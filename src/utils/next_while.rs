use std::iter::{Peekable};

// TODO: Refactor `NextWhile` like `TakeWhile`
pub trait NextWhile<I>
where
    I: Iterator,
{
    fn next_while_count(&mut self, func: impl Fn(&I::Item) -> bool) -> usize;
}

impl<I> NextWhile<I> for Peekable<I>
where
    I: Iterator,
{
    fn next_while_count(&mut self, func: impl Fn(&I::Item) -> bool) -> usize {
        let mut result = 0;

        while let Some(_) = self.next_if(&func) {
            result += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::NextWhile;

    #[test]
    fn basics() {
        let mut iter = [1, 1, 2, 3].iter().peekable();

        assert_eq!(iter.next_while_count(|&x| x == &1), 2);
        assert_eq!(iter.next_while_count(|&x| x == &1), 0);

        let mut md_str = "### Hello World".chars().peekable();

        assert_eq!(md_str.next_while_count(|&x| x == '#'), 3);
        assert_eq!(md_str.next_while_count(|&x| x == '#'), 0);
    }
}
