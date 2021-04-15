mod thing;

fn main() {
    println!("Hello, world!");
}

use mockall_double::double;
#[double]
use thing::Thing;

fn do_stuff(thing: &Thing) -> u32 {
    thing.some
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {
        let mut mock = Thing::default();
        mock.expect_foo().returning(|| 42);
        do_stuff(&mock);
    }
}
