use crate::println;

pub fn runner(tests: &[&dyn Fn()]) {
    let count = tests.len();

    for (i, test) in tests.into_iter().enumerate() {
        println!("Running test {} out of {}:\n", i + 1, count);
        test();
    }
}

#[cfg(test)]
mod test {
    use crate::{print, println};

    #[test_case]
    fn trivial_assertion() {
        print!("trivial assertion... ");
        assert_eq!(1, 1);
        print!("[ok]");
    }
}
