use x86_64::structures::idt::{InterruptDescriptorTable,InterruptStackFrame};
use crate::{print,println};
use crate::gdt;
use pic8259::ChainedPics;
use spin;
use lazy_static::lazy_static;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		unsafe {
			idt.double_fault.set_handler_fn(double_fault_handler)
							.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
		}
		idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_handler);
		idt
	};
}

#[derive(Debug,Clone,Copy)]
#[repr(u8)]
pub enum InterruptIndex {
	Timer = PIC_1_OFFSET,
}

impl InterruptIndex {
	fn as_u8(self) -> u8{
		self as u8
	}

	fn as_usize(self) -> usize {
		usize::from(self.as_u8())
	}
}

pub fn init_idt () {
	IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
	stack_frame: InterruptStackFrame)
{
	println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern  "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
	panic!("EXCEPTION: DOUBLE FAULT\n{:#?}",stack_frame);
}

extern "x86-interrupt" fn timer_handler(_stack_fram: InterruptStackFrame) {
	print!(".");
	unsafe {
		PICS.lock()
			.notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
	}
}

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(
	unsafe{
		ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
	}
);





// --------- TESTS ----------- //
#[test_case]
fn test_int3() {
	x86_64::instructions::interrupts::int3();
}
