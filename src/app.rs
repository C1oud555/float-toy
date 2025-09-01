#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FloatFormat {
    Fp32,
    Fp16,
    Bf16,
    Tf32,
    Fp8e4m3,
    Fp8e5m2,
    Fp4,
    Ue8m0,
    Custom,
}

impl FloatFormat {
    pub fn get_params(&self) -> (u8, u8) {
        match self {
            FloatFormat::Fp32 => (8, 23),
            FloatFormat::Fp16 => (5, 10),
            FloatFormat::Bf16 => (8, 7),
            FloatFormat::Tf32 => (8, 10),
            FloatFormat::Fp8e4m3 => (4, 3),
            FloatFormat::Fp8e5m2 => (5, 2),
            FloatFormat::Fp4 => (2, 1),
            FloatFormat::Ue8m0 => (8, 0),
            FloatFormat::Custom => (8, 23), // Default custom
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            FloatFormat::Fp32 => "fp32",
            FloatFormat::Fp16 => "fp16",
            FloatFormat::Bf16 => "bf16",
            FloatFormat::Tf32 => "tf32",
            FloatFormat::Fp8e4m3 => "fp8e4m3",
            FloatFormat::Fp8e5m2 => "fp8e5m2",
            FloatFormat::Fp4 => "fp4",
            FloatFormat::Ue8m0 => "ue8m0",
            FloatFormat::Custom => "Custom",
        }
    }

    pub fn all() -> Vec<FloatFormat> {
        vec![
            FloatFormat::Fp32,
            FloatFormat::Fp16,
            FloatFormat::Bf16,
            FloatFormat::Tf32,
            FloatFormat::Fp8e4m3,
            FloatFormat::Fp8e5m2,
            FloatFormat::Fp4,
            FloatFormat::Ue8m0,
        ]
    }
}

pub struct App {
    pub hex_input: String,
    pub float_output: String,
    pub bits: Vec<u8>,
    pub exponent_bits_input: String,
    pub mantissa_bits_input: String,
    pub exponent_bits: u8,
    pub mantissa_bits: u8,
    pub current_format: FloatFormat,
    pub active_input: InputField,
}

#[derive(PartialEq)]
pub enum InputField {
    Hex,
    Exponent,
    Mantissa,
}


impl App {
    pub fn new() -> App {
        let (exponent_bits, mantissa_bits) = FloatFormat::Fp32.get_params();
        let mut app = App {
            hex_input: "40490fdb".to_string(),
            float_output: String::new(),
            bits: vec![0; 32],
            exponent_bits_input: exponent_bits.to_string(),
            mantissa_bits_input: mantissa_bits.to_string(),
            exponent_bits,
            mantissa_bits,
            current_format: FloatFormat::Fp32,
            active_input: InputField::Hex,
        };
        app.convert();
        app
    }

    pub fn set_format(&mut self, format: FloatFormat) {
        self.current_format = format;
        if format != FloatFormat::Custom {
            let (e, m) = format.get_params();
            self.exponent_bits = e;
            self.mantissa_bits = m;
            self.exponent_bits_input = e.to_string();
            self.mantissa_bits_input = m.to_string();
        }
        let total_bits = 1 + self.exponent_bits + self.mantissa_bits;
        self.bits.resize(total_bits as usize, 0);
        self.convert();
    }

    pub fn update_custom_format(&mut self) {
        let e = self.exponent_bits_input.parse::<u8>().unwrap_or(self.exponent_bits);
        let m = self.mantissa_bits_input.parse::<u8>().unwrap_or(self.mantissa_bits);

        if e != self.exponent_bits || m != self.mantissa_bits {
             self.exponent_bits = e;
             self.mantissa_bits = m;
             self.current_format = FloatFormat::Custom;
             let total_bits = 1 + self.exponent_bits + self.mantissa_bits;
             self.bits.resize(total_bits as usize, 0);
             self.convert();
        }
    }

    pub fn convert(&mut self) {
        let total_bits = (1 + self.exponent_bits + self.mantissa_bits) as usize;
        if total_bits > 64 {
            self.float_output = "Total bits > 64 not supported".to_string();
            return;
        }
        let hex_len = (total_bits + 3) / 4;

        let mut padded_hex = self.hex_input.clone();
        if padded_hex.len() > hex_len {
            padded_hex = padded_hex.chars().take(hex_len).collect();
        }

        if let Ok(bits_val) = u64::from_str_radix(&padded_hex, 16) {
            self.bits.fill(0);
            for i in 0..total_bits {
                if (bits_val >> (total_bits - 1 - i)) & 1 == 1 {
                    if i < self.bits.len() {
                        self.bits[i] = 1;
                    }
                }
            }

            let sign_bit = (bits_val >> (total_bits - 1)) & 1;
            let sign = if sign_bit == 1 { -1.0 } else { 1.0 };

            let exponent_mask = (1u64 << self.exponent_bits) - 1;
            let exponent_val = (bits_val >> self.mantissa_bits) & exponent_mask;

            let mantissa_mask = (1u64 << self.mantissa_bits) - 1;
            let mantissa_val = bits_val & mantissa_mask;

            let bias = (1 << (self.exponent_bits - 1)) - 1;

            if exponent_val == exponent_mask {
                if mantissa_val == 0 {
                    self.float_output = if sign > 0.0 { "inf".to_string() } else { "-inf".to_string() };
                } else {
                    self.float_output = "NaN".to_string();
                }
                return;
            }

            let (mantissa_float, exponent) = if exponent_val == 0 { // Subnormal
                let mut m = 0.0;
                for i in 0..self.mantissa_bits {
                    if (mantissa_val >> (self.mantissa_bits - 1 - i)) & 1 == 1 {
                        m += 2.0f64.powf(-(i as f64 + 1.0));
                    }
                }
                (m, 1 - bias)
            } else { // Normal
                let mut m = 1.0;
                for i in 0..self.mantissa_bits {
                    if (mantissa_val >> (self.mantissa_bits - 1 - i)) & 1 == 1 {
                        m += 2.0f64.powf(-(i as f64 + 1.0));
                    }
                }
                (m, exponent_val as i32 - bias)
            };

            let float_value = sign * mantissa_float * 2.0f64.powi(exponent);
            self.float_output = float_value.to_string();

        } else {
            self.float_output = "Invalid hex input".to_string();
            self.bits.fill(0);
        }
    }
}