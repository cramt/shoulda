use crate::{Should, ShouldNot, Shoulda};
use std::borrow::Borrow;
use std::fmt::Debug;

fn contains_sequence<K: Shoulda, L: Borrow<K>>(sequence: &Vec<L>, v: &Vec<&K>) -> bool {
    if sequence.is_empty() {
        return true;
    }
    let mut curr: Option<usize> = None;
    let mut iter = v.iter();
    (loop {
        match iter.next() {
            None => break false,
            Some(v) => {
                curr = match curr {
                    None => {
                        if sequence.first().unwrap().borrow().test_eq(v) {
                            Some(1)
                        } else {
                            None
                        }
                    }
                    Some(curr) => {
                        if curr == sequence.len() {
                            break true;
                        }
                        if sequence[curr].borrow().test_eq(v) {
                            Some(curr + 1)
                        } else {
                            None
                        }
                    }
                }
            }
        }
    } || (curr.is_some() && curr.unwrap() == sequence.len()))
}

impl<'a, T, K> Should<'a, T>
    where
        &'a T: IntoIterator<Item=&'a K>,
        T: Debug,
        K: Debug,
        K: Shoulda,
        K: 'a,
{
    pub fn contain<I: Borrow<K>>(self, item: I) -> Self {
        let item = item.borrow();
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            v.iter().any(|x| x.test_eq(&item)),
            "{:?} did not contain {:?}",
            v,
            item,
        );
        self
    }

    pub fn contain_sequence<L: Borrow<K>, I: IntoIterator<Item=L>>(self, items: I) -> Self {
        let sequence = items.into_iter().collect::<Vec<L>>();
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            contains_sequence(&sequence, &v),
            "{:?} does not contain the sequence {:?}",
            v,
            sequence.iter().map(|x| x.borrow()).collect::<Vec<&K>>()
        );
        self
    }

    pub fn contains<I: Fn(&K) -> bool>(self, predicate: I) -> Self {
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            v.iter().map(|x| *x).any(predicate),
            "{:?} did not fufill the predicate",
            v
        );
        self
    }
}

impl<'a, T, K> ShouldNot<'a, T>
    where
        &'a T: IntoIterator<Item=&'a K>,
        T: Debug,
        T: Shoulda,
        K: Debug,
        K: Shoulda,
        K: 'a,
{
    pub fn contain<I: Borrow<K>>(self, item: I) -> Should<'a, T> {
        let item = item.borrow();
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            !v.iter().any(|x| x.test_eq(&item)),
            "{:?} did contain {:?}",
            v,
            item,
        );
        self.not()
    }

    pub fn contain_sequence<L: Borrow<K>, I: IntoIterator<Item=L>>(self, items: I) -> Should<'a, T> {
        let sequence = items.into_iter().collect::<Vec<L>>();
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            !contains_sequence(&sequence, &v),
            "{:?} does contain the sequence {:?}",
            v,
            sequence.iter().map(|x| x.borrow()).collect::<Vec<&K>>()
        );
        self.not()
    }

    pub fn contains<I: Fn(&K) -> bool>(self, predicate: I) -> Should<'a, T> {
        let v = self.inner.into_iter().collect::<Vec<&K>>();
        assert!(
            !v.iter().map(|x| *x).any(predicate),
            "{:?} did fufill the predicate",
            v
        );
        self.not()
    }
}

#[cfg(test)]
mod tests {
    use crate::Shoulda;

    #[test]
    fn should_contain() {
        vec![1, 2, 3].should().contain(2);
    }

    #[test]
    fn should_not_contain() {
        vec![1, 2, 3].should().not().contain(4);
    }

    #[test]
    fn should_contains() {
        vec![1, 2, 3].should().contains(|x| x % 2 == 0);
    }

    #[test]
    fn should_not_contains() {
        vec![1, 2, 3].should().not().contains(|x| x % 4 == 0);
    }

    #[test]
    fn should_contain_sequence() {
        vec![1, 2, 3, 4]
            .should()
            .contain_sequence(vec![2, 3])
            .and()
            .contain_sequence(vec![1, 2])
            .and()
            .contain_sequence(vec![2, 3]);
    }

    #[test]
    fn should_not_contain_sequence() {
        vec![1, 2, 3, 4]
            .should()
            .not()
            .contain_sequence(vec![1, 3])
            .and()
            .not()
            .contain_sequence(vec![4, 3])
            .and()
            .not()
            .contain_sequence(vec![3, 2]);
    }
}
