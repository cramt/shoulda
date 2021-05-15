use std::ops::{Deref, DerefMut};

pub struct ShouldResult {
    pass: bool,
    message: String,
}

impl ShouldResult {
    pub fn new(pass: bool, message: String) -> Self {
        Self { pass, message }
    }

    pub fn or(self, other: Self) -> Self {
        Self::new(
            self.pass || other.pass,
            format!("({} || {})", self.message, other.message),
        )
    }

    pub fn assert(&self) {
        assert!(self.pass, "{}", self.message)
    }
}

impl From<(bool, String)> for ShouldResult {
    fn from((pass, message): (bool, String)) -> Self {
        Self::new(pass, message)
    }
}

pub struct ResultsContainer {
    inner: Vec<ShouldResult>,
}

impl Default for ResultsContainer {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl Drop for ResultsContainer {
    fn drop(&mut self) {
        if std::thread::panicking() {
            return;
        }
        for result in self.inner.iter() {
            result.assert()
        }
    }
}

impl Deref for ResultsContainer {
    type Target = Vec<ShouldResult>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ResultsContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
