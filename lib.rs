
pub use aaa::exp;

/// The abc mod
pub mod abc {
    /// This is public
    pub fn def() {}

    /// This is not
    fn hij() {}
}

/// This should be seen with --no-defaults
mod aaa {
    /// So should this
    fn kkk() {}
    /// And this
    pub fn lll() {}

    /// This is exported
    pub fn exp() {}
}
