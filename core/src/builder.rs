pub(crate) trait Generator<'a, T> {
    type Ext = i64;
    fn generate(&self, ext: Self::Ext) -> T;
}

pub(crate) trait Consumer<'a, T> {
    type Ext = i64;
    fn consume(self, ext: Self::Ext) -> T;
}
