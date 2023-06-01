use crate::*;

#[test_case]
fn trivial() {
    assert_eq!(1,1);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
 where T: Fn() {
    fn run(&self) {
        serial_print!("{}...\t",core::any::type_name::<T>());
        self();
        serial_println!("[OK]");
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {


    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}