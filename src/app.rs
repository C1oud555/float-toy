pub struct App {
    pub hex_input: String,
    pub float_output: String,
    pub bits: [u8; 32],
}

impl App {
    pub fn new() -> App {
        App {
            hex_input: String::new(),
            float_output: String::new(),
            bits: [0; 32],
        }
    }

    pub fn convert(&mut self) {
        if let Ok(bits_val) = u32::from_str_radix(&self.hex_input, 16) {
            let float_value = f32::from_bits(bits_val);
            self.float_output = float_value.to_string();

            for i in 0..32 {
                if (bits_val >> (31 - i)) & 1 == 1 {
                    self.bits[i] = 1;
                } else {
                    self.bits[i] = 0;
                }
            }
        } else {
            self.float_output = "Invalid hex input".to_string();
            self.bits = [0; 32];
        }
    }
}
