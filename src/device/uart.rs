// uart.rs
// UART routines and driver

use core::{convert::TryInto,
           fmt::{Error, Write}};
use core::ptr::{read_volatile, write_volatile};

pub struct Uart {
    base_address: usize,
}

impl Write for Uart {
    fn write_str(&mut self, out: &str) -> Result<(), Error> {
        for c in out.bytes() {
            self.put(c);
        }
        Ok(())
    }
}

impl Uart {
    pub fn new(base_address: usize) -> Self {
        Uart { base_address }
    }

    pub fn init(&mut self) {
        let ptr = self.base_address as *mut u8;


            // First, set the word length, which
            // are bits 0 and 1 of the line control register (LCR)
            // which is at base_address + 3
            // We can easily write the value 3 here or 0b11, but I'm
            // extending it so that it is clear we're setting two
            // individual fields
            //             Word 0     Word 1
            //             ~~~~~~     ~~~~~~
            let lcr: u8 = (1 << 0) | (1 << 1);
            write(self.base_address + COM_LCR, lcr);
//            *ptr.add(3) = lcr;
        unsafe {
            // Now, enable the FIFO, which is bit index 0 of the
            // FIFO control register (FCR at offset 2).
            // Again, we can just write 1 here, but when we use left
            // shift, it's easier to see that we're trying to write
            // bit index #0.
            ptr.add(2).write_volatile(1 << 0);

            // Enable receiver buffer interrupts, which is at bit
            // index 0 of the interrupt enable register (IER at
            // offset 1).
            ptr.add(1).write_volatile(1 << 0);

            // If we cared about the divisor, the code below would
            // set the divisor from a global clock rate of 22.729
            // MHz (22,729,000 cycles per second) to a signaling
            // rate of 2400 (BAUD). We usually have much faster
            // signalling rates nowadays, but this demonstrates what
            // the divisor actually does. The formula given in the
            // NS16500A specification for calculating the divisor
            // is:
            // divisor = ceil( (clock_hz) / (baud_sps x 16) )
            // So, we substitute our values and get:
            // divisor = ceil( 22_729_000 / (2400 x 16) )
            // divisor = ceil( 22_729_000 / 38_400 )
            // divisor = ceil( 591.901 ) = 592

            // The divisor register is two bytes (16 bits), so we
            // need to split the value 592 into two bytes.
            // Typically, we would calculate this based on measuring
            // the clock rate, but again, for our purposes [qemu],
            // this doesn't really do anything.
            let divisor: u16 = 592;
            let divisor_least: u8 =
                (divisor & 0xff).try_into().unwrap();
            let divisor_most: u8 =
                (divisor >> 8).try_into().unwrap();

            // Notice that the divisor register DLL (divisor latch
            // least) and DLM (divisor latch most) have the same
            // base address as the receiver/transmitter and the
            // interrupt enable register. To change what the base
            // address points to, we open the "divisor latch" by
            // writing 1 into the Divisor Latch Access Bit (DLAB),
            // which is bit index 7 of the Line Control Register
            // (LCR) which is at base_address + 3.
            ptr.add(3).write_volatile(lcr | 1 << 7);

            // Now, base addresses 0 and 1 point to DLL and DLM,
            // respectively. Put the lower 8 bits of the divisor
            // into DLL
            ptr.add(0).write_volatile(divisor_least);
            ptr.add(1).write_volatile(divisor_most);

            // Now that we've written the divisor, we never have to
            // touch this again. In hardware, this will divide the
            // global clock (22.729 MHz) into one suitable for 2,400
            // signals per second. So, to once again get access to
            // the RBR/THR/IER registers, we need to close the DLAB
            // bit by clearing it to 0.
            ptr.add(3).write_volatile(lcr);
        }
    }

    pub fn put(&mut self, c: u8) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            ptr.add(0).write_volatile(c);
        }
    }

    pub fn get(&mut self) -> Option<u8> {
        let ptr = self.base_address as *mut u8;
        unsafe {
            if ptr.add(5).read_volatile() & 1 == 0 {
                // The DR bit is 0, meaning no data
                None
            }
            else {
                // The DR bit is 1, meaning data!
                Some(ptr.add(0).read_volatile())
            }
        }
    }
}

#[inline(always)]
pub fn write<T>(addr: usize, content: T) {
    let cell = (addr) as *mut T;
    unsafe {
        write_volatile(cell, content);
    }
}

#[inline(always)]
pub fn read<T>(addr: usize) -> T {
    let cell = (addr) as *const T;
    unsafe { read_volatile(cell) }
}

const COM_RX: usize = 0; // In:  Receive buffer (DLAB=0)
const COM_TX: usize = 0; // Out: Transmit buffer (DLAB=0)
const COM_DLL: usize = 0; // Out: Divisor Latch Low (DLAB=1)
const COM_DLM: usize = 1; // Out: Divisor Latch High (DLAB=1)
const COM_IER: usize = 1; // Out: Interrupt Enable Register
const COM_IER_RDI: u8 = 0x01; // Enable receiver data interrupt
const COM_IIR: usize = 2; // In:  Interrupt ID Register
const COM_FCR: usize = 2; // Out: FIFO Control Register
const COM_LCR: usize = 3; // Out: Line Control Register
const COM_LCR_DLAB: u8 = 0x80; // Divisor latch access bit
const COM_LCR_WLEN8: u8 = 0x03; // Wordlength: 8 bits
const COM_MCR: usize = 4; // Out: Modem Control Register
const COM_MCR_RTS: u8 = 0x02; // RTS complement
const COM_MCR_DTR: u8 = 0x01; // DTR complement
const COM_MCR_OUT2: u8 = 0x08; // Out2 complement
const COM_LSR: usize = 5; // In:  Line Status Register
const COM_LSR_DATA: u8 = 0x01; // Data available
const COM_LSR_TXRDY: u8 = 0x20; // Transmit buffer avail
const COM_LSR_TSRE: u8 = 0x40; // Transmitter off