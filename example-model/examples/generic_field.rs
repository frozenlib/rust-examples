fn main() {}

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

// 引数と戻り値からimpl<I>のIが確定できない場合（今後変更される可能性があるも含む）は関連関数ではなく、普通の関数として実装
fn new() -> GenericField<impl Iterator<Item = u8>> {
    GenericField {
        iter: std::iter::empty(),
    }
}
