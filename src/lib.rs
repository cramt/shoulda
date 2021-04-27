use std::path::Component::CurDir;
use std::mem;

#[cfg(test)]
mod tests {
    use crate::{RomanNumerals, RomanNumeral, RomanNumeralInstance};
    use crate::RomanNumeral::X;

    #[test]
    fn idk() {
        assert_eq!(RomanNumeralInstance::new(X, 4), RomanNumerals::from(40).0[2])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RomanNumeral {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl RomanNumeral {
    pub fn size(&self) -> usize {
        match self {
            Self::I => 1,
            Self::V => 5,
            Self::X => 10,
            Self::L => 50,
            Self::C => 100,
            Self::D => 500,
            Self::M => 1000
        }
    }
    pub fn previous(&self) -> Option<Self> {
        match self {
            Self::I => None,
            Self::V => Some(Self::I),
            Self::X => Some(Self::V),
            Self::L => Some(Self::X),
            Self::C => Some(Self::L),
            Self::D => Some(Self::C),
            Self::M => Some(Self::D),
        }
    }
    pub fn previous_size(&self) -> usize {
        self.previous().map(|x| x.size()).unwrap_or(0)
    }
    pub fn largest() -> Self {
        Self::M
    }
    pub fn take(self, n: &mut usize) -> RomanNumeralInstance {
        let cut_off = self.size();
        let amount = *n / cut_off;
        *n -= cut_off * amount;
        RomanNumeralInstance::new(self, amount)
    }
}

impl IntoIterator for RomanNumeral {
    type Item = Self;
    type IntoIter = RomanNumeralIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        RomanNumeralIntoIter::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct RomanNumeralIntoIter {
    curr: Option<RomanNumeral>
}

impl RomanNumeralIntoIter {
    pub fn new(roman_num: RomanNumeral) -> Self {
        Self {
            curr: Some(roman_num)
        }
    }
}

impl Default for RomanNumeralIntoIter {
    fn default() -> Self {
        Self::new(RomanNumeral::largest())
    }
}

impl Iterator for RomanNumeralIntoIter {
    type Item = RomanNumeral;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.curr.as_ref()?.previous();
        mem::swap(&mut next, &mut self.curr);
        next
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RomanNumeralInstance {
    numeral: RomanNumeral,
    doube_amount: usize,
}

impl RomanNumeralInstance {
    pub fn new_half(num: RomanNumeral) -> Self {
        Self {
            numeral: num,
            doube_amount: 1,
        }
    }
    pub fn new_empty(num: RomanNumeral) -> Self {
        Self {
            numeral: num,
            doube_amount: 0,
        }
    }
    pub fn new(num: RomanNumeral, amount: usize) -> Self {
        Self {
            numeral: num,
            doube_amount: amount * 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RomanNumerals(pub Vec<RomanNumeralInstance>);

impl From<usize> for RomanNumerals {
    fn from(mut n: usize) -> Self {
        let mut v: Vec<RomanNumeralInstance> = RomanNumeralIntoIter::default().map(|x| x.take(&mut n)).collect();
        v.reverse();
        Self(v)
    }
}
