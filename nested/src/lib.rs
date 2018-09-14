struct SomeStruct;

fn uses_it(arg: &SomeStruct) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let val = SomeStruct;
        uses_it(&val);
    }
}
