pub struct ColorBand {
    /// Digit of color, both as digit and as power of 10 for multiplier. Use negative to specify none.
    pub digit: Option<u8>,
    /// Tolerance, as a percent.
    pub tolerance: Option<f64>,
    /// Temperature coefficient in ppm/C.
    pub tcr: Option<i16>,
}

pub const BLACK: ColorBand = ColorBand {
    digit: Some(0),
    tolerance: None,
    tcr: Some(250),
};

pub const BROWN: ColorBand = ColorBand {
    digit: Some(1),
    tolerance: Some(1.0),
    tcr: Some(100),
};

pub const RED: ColorBand = ColorBand {
    digit: Some(2),
    tolerance: Some(2.0),
    tcr: Some(50),
};

pub const ORANGE: ColorBand = ColorBand {
    digit: Some(3),
    tolerance: None,
    tcr: Some(15),
};

pub const YELLOW: ColorBand = ColorBand {
    digit: Some(4),
    tolerance: None,
    tcr: Some(25),
};

pub const GREEN: ColorBand = ColorBand {
    digit: Some(5),
    tolerance: Some(0.5),
    tcr: Some(20),
};

pub const BLUE: ColorBand = ColorBand {
    digit: Some(6),
    tolerance: Some(0.25),
    tcr: Some(10),
};

pub const VIOLET: ColorBand = ColorBand {
    digit: Some(7),
    tolerance: Some(0.1),
    tcr: Some(5),
};

pub const GREY: ColorBand = ColorBand {
    digit: Some(8),
    tolerance: Some(0.05),
    tcr: Some(1),
};

pub const WHITE: ColorBand = ColorBand {
    digit: Some(9),
    tolerance: None,
    tcr: None,
};

pub const GOLD: ColorBand = ColorBand {
    digit: None,
    tolerance: Some(5.0),
    tcr: None,
};

pub const SILVER: ColorBand = ColorBand {
    digit: None,
    tolerance: Some(10.0),
    tcr: None,
};
