use core::mem::size_of;
use bit_field::BitField;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Scause {
    bits: usize,
}


/// Trap Cause
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}

/// Interrupt
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interrupt {
    UserSoftwareInterrupt,
    SupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,
    UserTimerInterrupt,
    SupervisorTimerInterrupt,
    MachineTimerInterrupt,
    UserExternalInterrupt,
    SupervisorExternalInterrupt,
    MachineExternalInterrupt,
    Unknown,
}

/// Exception
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Exception {
    InstructionMisaligned,
    InstructionFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    UserEnvCall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    Unknown,
}

impl Interrupt {
    pub fn from(nr: usize) -> Self {
        match nr {
            0   => Interrupt::UserSoftwareInterrupt,
            1   => Interrupt::SupervisorSoftwareInterrupt,
            3   => Interrupt::MachineSoftwareInterrupt,
            4   => Interrupt::UserTimerInterrupt,
            5   => Interrupt::SupervisorTimerInterrupt,
            7   => Interrupt::MachineTimerInterrupt,
            8   => Interrupt::UserExternalInterrupt,
            9   => Interrupt::SupervisorExternalInterrupt,
            11  => Interrupt::MachineExternalInterrupt,
            _   => Interrupt::Unknown,
        }
    }
}


impl Exception {
    pub fn from(nr: usize) -> Self {
        match nr {
            0   => Exception::InstructionMisaligned,
            1   => Exception::InstructionFault,
            2   => Exception::IllegalInstruction,
            3   => Exception::Breakpoint,
            4   => Exception::LoadAddressMisaligned,
            5   => Exception::LoadAccessFault,
            6   => Exception::StoreAddressMisaligned,
            7   => Exception::StoreAccessFault,
            8   => Exception::UserEnvCall,
            12  => Exception::InstructionPageFault,
            13  => Exception::LoadPageFault,
            15  => Exception::StorePageFault,
            _   => Exception::Unknown,
        }
    }
}

impl Scause {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the code field
    pub fn code(&self) -> usize {
        let bit = 1 << (size_of::<usize>() * 8 - 1);
        self.bits & !bit
    }

    /// Trap Cause
    #[inline]
    pub fn cause(&self) -> Trap {
        if self.is_interrupt() {
            Trap::Interrupt(Interrupt::from(self.code()))
        } else {
            Trap::Exception(Exception::from(self.code()))
        }
    }

    /// Is trap cause an interrupt.
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        self.bits.get_bit(size_of::<usize>() * 8 - 1)
    }

    /// Is trap cause an exception.
    #[inline]
    pub fn is_exception(&self) -> bool {
        !self.is_interrupt()
    }
}