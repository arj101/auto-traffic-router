use serial2::SerialPort;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;
use std::time::Duration;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Led {
    C_RU_1_4,
    C_RD_1_4,
    B_RU_1_4,
    B_RD_1_4,
    E_RU_1_4,
    E_RD_1_4,

    B_L_a_1,
    B_U_a_1,
    C_L_a_1,
    C_U_a_1,
    D_L_a_1,
    D_U_a_1,

    C_L_1_2,
    C_U_1_2,
    D_L_1_2,
    D_U_1_2,

    A_L_b_2,
    A_U_b_2,
    A_R_b_2,
    C_L_b_2,
    C_U_b_2,
    C_R_b_2,
    D_L_b_2,
    D_U_b_2,
    D_R_b_2,

    A_R_3_2,
    A_U_3_2,
    D_R_3_2,
    D_U_3_2,

    A_L_c_3,
    A_R_c_3,
    B_L_c_3,
    B_R_c_3,
    D_L_c_3,
    D_R_c_3,

    A_L_3_4,
    A_U_3_4,
    B_L_3_4,
    B_U_3_4,

    A_LD_d_4,
    A_LU_d_4,
    A_U_d_4,
    B_LD_d_4,
    B_LU_d_4,
    B_U_d_4,
    C_LD_d_4,
    C_LU_d_4,
    C_U_d_4,
}

pub struct LedAddr(pub u8, pub u8);
impl LedAddr {
    pub fn i2c(&self) -> u8 {
        self.0
    }
    pub fn led(&self) -> u8 {
        self.1
    }
}

pub fn led_to_addr(led: Led) -> LedAddr {
    use Led::*;
    match led {
        C_RU_1_4 => LedAddr(1, 0),
        C_RD_1_4 => LedAddr(1, 1),
        B_RU_1_4 => LedAddr(1, 2),
        B_RD_1_4 => LedAddr(1, 3),
        E_RU_1_4 => LedAddr(1, 4),
        E_RD_1_4 => LedAddr(1, 5),

        B_L_a_1 => LedAddr(0, 1),
        B_U_a_1 => LedAddr(0, 0),
        C_L_a_1 => LedAddr(0, 3),
        C_U_a_1 => LedAddr(0, 2),
        D_L_a_1 => LedAddr(0, 5),
        D_U_a_1 => LedAddr(0, 4),

        C_L_1_2 => LedAddr(1, 8),
        C_U_1_2 => LedAddr(1, 6),
        D_L_1_2 => LedAddr(1, 9),
        D_U_1_2 => LedAddr(1, 7),

        A_L_b_2 => LedAddr(4, 0),
        A_U_b_2 => LedAddr(4, 1),
        A_R_b_2 => LedAddr(4, 2),
        C_L_b_2 => LedAddr(4, 3),
        C_U_b_2 => LedAddr(4, 4),
        C_R_b_2 => LedAddr(4, 5),
        D_L_b_2 => LedAddr(4, 6),
        D_U_b_2 => LedAddr(4, 7),
        D_R_b_2 => LedAddr(4, 8),

        A_R_3_2 => LedAddr(4, 12),
        A_U_3_2 => LedAddr(4, 13),
        D_R_3_2 => LedAddr(4, 14),
        D_U_3_2 => LedAddr(4, 15),

        A_L_c_3 => LedAddr(2, 0),
        A_R_c_3 => LedAddr(2, 1),
        B_L_c_3 => LedAddr(2, 2),
        B_R_c_3 => LedAddr(2, 3),
        D_L_c_3 => LedAddr(2, 4),
        D_R_c_3 => LedAddr(2, 5),

        A_L_3_4 => LedAddr(3, 14),
        A_U_3_4 => LedAddr(3, 13),
        B_L_3_4 => LedAddr(3, 12),
        B_U_3_4 => LedAddr(3, 15),

        A_LD_d_4 => LedAddr(3, 0),
        A_LU_d_4 => LedAddr(3, 1),
        A_U_d_4 => LedAddr(3, 2),
        B_LD_d_4 => LedAddr(3, 3),
        B_LU_d_4 => LedAddr(3, 4),
        B_U_d_4 => LedAddr(3, 5),
        C_LD_d_4 => LedAddr(3, 6),
        C_LU_d_4 => LedAddr(3, 7),
        C_U_d_4 => LedAddr(3, 8),
    }
}

pub struct LightController {
    port: SerialPort,
    buffer: [u8; 256],
    state: HashMap<Led, bool>,
}

impl LightController {
    pub fn create_and_init(port_name: &str, baudrate: u32) -> Self {
        let mut port = SerialPort::open(port_name, baudrate).expect("Failed to open serial port");
        port.set_read_timeout(Duration::from_millis(300)).unwrap();
        port.set_write_timeout(Duration::from_millis(300)).unwrap();
        let mut buffer = [0; 256];
        let mut off = 0;
        println!("====starting initialisation=====");
        println!("Waiting for start message(\"Listening for input...\")\n");
        while &buffer[..22] != "Listening for input...".as_bytes() {
            if let Ok(read) = port.read(&mut buffer[off..]) {
                off += read;
                println!(
                    "[start message] {}",
                    buffer[..off + 1]
                        .iter()
                        .map(|c| *c as char)
                        .collect::<String>(),
                );
            }
        }
        println!("====successfully initialised serial port!=====\n");
        buffer = [0; 256];
        let mut controller = Self {
            port,
            buffer,
            state: HashMap::new(),
        };
        crate::delay(2);
        controller.clear_all();
        controller
    }

    fn clear_all(&mut self) {
        for i2c_addr in 0..5 {
            for led in 0..16 {
                self.set_led_from_addr(i2c_addr, led, 0);
            }
        }
    }

    pub fn clear(&mut self) {
        for (led, state) in self.state.clone() {
            if state {
                self.set_led(led, false)
            }
        }
    }

    pub fn set_led(&mut self, led: Led, state: bool) {
        let led_addr = led_to_addr(led);
        let i2c_addr = led_addr.i2c();
        let led_addr = led_addr.led();
        self.set_led_from_addr(i2c_addr, led_addr, if state { 1 } else { 0 });
        self.state.insert(led, state);
    }

    fn set_led_from_addr(&mut self, i2c_addr: u8, led_addr: u8, state: u8) {
        if let Err(e) = write!(&mut self.port, "{}{:02}{}\n", i2c_addr, led_addr, state) {
            println!("Error setting led: {:?}", e);
        } else {
            // self.port.flush().unwrap();
            if let Err(e) = self.port.read(&mut self.buffer) {
                // println!(
                //     "response: {:?}",
                //     self.buffer[..read]
                //         .iter()
                //         .map(|c| *c as char)
                //         .collect::<String>(),
                // );
                // } else {
                println!("**No response after setting led**, {}", e);
            }
        }
    }
}
