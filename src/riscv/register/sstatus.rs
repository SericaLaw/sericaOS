use bit_field::BitField;
use core::mem::size_of;
#[inline]
pub fn read() -> Sstatus {
    let mut bits: usize;
    unsafe {
        asm!("csrr x10, sstatus"
            :"={x10}"(bits)
            :::"volatile");
    }
    Sstatus {
        bits: bits
    }
}


/// Supervisor Status Register
#[derive(Clone, Copy, Debug)]
pub struct Sstatus {
    bits: usize,
}

// TODO: 是否能和read一样抽出来即可？
// 这个结构体的作用是抽象出一个Sstatus，但对其的操作是在内存中，不会对实际的sstatus进行读写
impl Sstatus {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn set_spp(&mut self, val: SPP) {
        match val {
            SPP::Supervisor => self.bits = self.bits | 1 << 8,
            SPP::User => self.bits = self.bits & 0 << 8,
        }
        // TODO: 不写回..？
//        unsafe {
//            asm!("csrw sstatus, x10"
//            ::"{x10}"(self.bits)
//            ::"volatile");
//        }
    }
}
/// Supervisor Previous Privilege Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SPP {
    Supervisor = 1,
    User = 0,
}

pub unsafe fn set_sie() {
    // TODO: 理论上可以换成另一条指令一步到位，但好像不work
    let mut sstatus: usize;
    asm!("csrr x10, sstatus"
            :"={x10}"(sstatus)
            :::"volatile");
    sstatus |= 0b10;
    asm!("csrw sstatus, x10"
                    ::"{x10}"(sstatus)
                    ::"volatile");
}

pub unsafe fn set_sum() {
    let mut sstatus: usize;
    asm!("csrr x10, sstatus"
            :"={x10}"(sstatus)
            :::"volatile");
    sstatus |= 1 << 18;
    asm!("csrw sstatus, x10"
                    ::"{x10}"(sstatus)
                    ::"volatile");
}