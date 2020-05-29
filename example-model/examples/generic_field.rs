fn main() {
    let x = GenericField::new();
}

struct GenericField<I> {
    iter: I,
}

impl<I: Iterator<Item = u8>> GenericField<I> {
    fn filter_non_zero(self) -> GenericField<impl Iterator<Item = u8>> {
        GenericField {
            iter: self.iter.filter(|&x| x != 0),
        }
    }
}

struct Dummy;

impl GenericField<Dummy> {
    fn new() -> GenericField<impl Iterator<Item = u8>> {
        GenericField {
            iter: std::iter::empty(),
        }
    }
}
