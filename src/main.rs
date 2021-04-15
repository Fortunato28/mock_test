mod thing;

fn main() {
    println!("Hello, world!");
}

use thing::Thing;

fn do_stuff(thing: &Thing) -> u32 {
    thing.foo()
}

#[cfg(test)]
mod test {
    use super::*;
    #[double]
    use crate::thing::Thing;
    use mockall_double::double;

    #[test]
    fn test_foo() {
        let mut mock = Thing::default();
        mock.expect_foo().returning(|| 42);
        do_stuff(&mock);
    }
}
