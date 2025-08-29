pub struct App {
    pub binary_input: String,
    pub float_output: String,
}

impl App {
    pub fn new() -> App {
        App {
            binary_input: String::new(),
            float_output: String::new(),
        }
    }

    pub fn convert(&mut self) {
        if let Ok(bits) = u32::from_str_radix(&self.binary_input, 2) {
            let float_value = f32::from_bits(bits);
            self.float_output = float_value.to_string();
        } else {
            self.float_output = "Invalid binary input".to_string();
        }
    }
}