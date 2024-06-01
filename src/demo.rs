
#[cfg(test)]
mod test {
    use crate::printer::Printer;

    #[test]
    fn demo() {
        let p = Printer::new();
        p.print();
    }
}
