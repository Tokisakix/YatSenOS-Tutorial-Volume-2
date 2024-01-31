use core::fmt;

/// A port-mapped UART 16550 serial interface.
pub struct SerialPort{
    port : u16,
}

fn inb(port: u16) -> u8{
    unsafe{
        x86::io::inb(port)
    }
}

fn outb(port: u16, data: u8) {
    unsafe{
        x86::io::outb(port, data);
    }
}

impl SerialPort {
    pub const fn new(port: u16) -> Self {
        SerialPort { port }
    }

    /// Initializes the serial port.
    pub fn init(&self) {
        // Initialize the serial port
        outb(self.port + 1, 0x00);    // Disable all interrupts
        outb(self.port + 3, 0x80);    // Enable DLAB (set baud rate divisor)
        outb(self.port + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
        outb(self.port + 1, 0x00);    //                  (hi byte)
        outb(self.port + 3, 0x03);    // 8 bits, no parity, one stop bit
        outb(self.port + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
        outb(self.port + 4, 0x0B);    // IRQs enabled, RTS/DSR set
        outb(self.port + 4, 0x1E);    // Set in loopback mode, test the serial chip
        outb(self.port + 0, 0xAE);    // Test serial chip (send byte 0xAE and check if serial returns same byte)

        // Check if serial is faulty (i.e: not same byte as sent)
        if inb(self.port + 0) != 0xAE {
            return;
        }

        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        outb(self.port + 4, 0x0F);

        outb(self.port + 1, 0x01);    // Enable interrupts
    }

    /// Sends a byte on the serial port.
    pub fn send(&mut self, data: u8) {
        // Send a byte on the serial port
        while(inb(self.port + 5) & 0x20) == 0 {}
        outb(self.port, data);
    }

    /// Receives a byte on the serial port no wait.
    pub fn receive(&mut self) -> Option<u8> {
        // Receive a byte on the serial port no wait
        if (inb(self.port + 5) & 1) != 0 {
            return Some(inb(self.port));
        }
        None
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}
