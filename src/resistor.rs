use crate::colorband::ColorBand;

// Represents a Resistor as a Vector of Color Bands
pub struct Resistor {
    bands: Vec<ColorBand>,
}

impl Resistor {
    pub fn new(bands: Vec<ColorBand>) -> Self {
        Resistor { bands: bands }
    }

    fn resistance(&self) -> Option<f64> {
        match self.bands.len() {
            // A one band resistor can only be 0
            1 => {
                if let Some(0) = self.bands[0].digit {
                    Some(0.0)
                } else {
                    None
                }
            }

            // 3 and 4 band resistors have two digit bands and one power 10 band
            // 4 band resistor also has a tolerance band
            3 | 4 => {
                if self.bands[0..3].iter().any(|x| x.digit.is_none()) {
                    None
                } else {
                    let mut resistance = self.bands[0].digit? as f64 * 10.0;
                    resistance += self.bands[1].digit? as f64;
                    resistance *= f64::powi(10.0, self.bands[2].digit?.into());
                    Some(resistance)
                }
            }

            // 5 and 6 band resistors have three digit bands and one power 10 band
            // 5 has a tolerance band
            // 6 has a tolerance band and a temperature band
            5 | 6 => {
                if self.bands[0..4].iter().any(|x| x.digit.is_none()) {
                    None
                } else {
                    let mut resistance = self.bands[0].digit? as f64 * 100.0;
                    resistance += self.bands[1].digit? as f64 * 10.0;
                    resistance += self.bands[2].digit? as f64;
                    resistance *= f64::powi(10.0, self.bands[3].digit?.into());
                    Some(resistance)
                }
            }

            // Not a valid resistor
            _ => None,
        }
    }

    fn tolerance(&self) -> Option<f64> {
        match self.bands.len() {
            1 => Some(0.0),
            3 => Some(20.0), // 3 Band Resistors Always have 20% tol
            4 => self.bands[3].tolerance,
            5 | 6 => self.bands[4].tolerance,
            _ => None,
        }
    }

    fn temperature_coeff(&self) -> Option<i16> {
        match self.bands.len() {
            6 => self.bands[5].tcr,
            _ => None,
        }
    }
}

impl ToString for Resistor {
    fn to_string(&self) -> String {
        let resistance = self.resistance();
        let tolerance = self.tolerance();

        if resistance.is_none() || tolerance.is_none() {
            return String::from("Invalid Resistor");
        }

        let resistance = resistance.unwrap();
        let tolerance = tolerance.unwrap();

        // Get pow10 of resistance
        let mut resistance_base = resistance.clone();
        let mut resistance_pow10: i8 = 0;

        while resistance_base >= 10.0 {
            resistance_base /= 10.0;
            resistance_pow10 += 1;
        }

        while resistance < 1.0 && resistance != 0.0 {
            resistance_base *= 10.0;
            resistance_pow10 -= 1;
        }

        // Get the units prefix
        let (prefix, pow10) = match resistance_pow10 {
            -12..=-10 => ("p", -12),
            -9..=-7 => ("n", -9),
            -6..=-4 => ("u", -6),
            -3..=-1 => ("m", -3),
            3..=5 => ("k", 3),
            6..=8 => ("M", 6),
            9..=11 => ("G", 9),
            12..=15 => ("T", 12),
            _ => ("", 0),
        };

        let resistance = format!("{}{}Ω", resistance / f64::powi(10., pow10), prefix);
        let tolerance = format!("±{}%", tolerance);

        let tcr = self.temperature_coeff();
        if tcr.is_some() {
            let tcr = format!("{} ppm/C", tcr.unwrap());
            return format!("{} {} {}", resistance, tolerance, tcr);
        }

        return format!("{} {}", resistance, tolerance);
    }
}
