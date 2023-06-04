use super::rendering::Fractals;

///Represents types of fractals that the api can render, simplified so that serde can deserialize
///all of them
#[derive(Debug, Clone, Copy, serde_derive::Deserialize, PartialEq, Eq)]
pub enum SimplifiedFractals {
    Mandelbrot,
    BurningShip,
    Tricorn,
    Feather,
    Eye,
    Custom,
}

impl SimplifiedFractals {
    pub fn into_fractals(self, formula: Option<String>) -> Fractals {
        match self {
            Self::Mandelbrot => Fractals::Mandelbrot,
            Self::Custom => {
                assert_ne!(formula, None);
                Fractals::Custom(formula.unwrap())
            }
            Self::BurningShip => Fractals::BurningShip,
            Self::Tricorn => Fractals::Tricorn,
            Self::Feather => Fractals::Feather,
            Self::Eye => Fractals::Eye,
        }
    }
}

#[derive(Debug, Clone, serde_derive::Deserialize)]
pub struct RequestBody {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub formula: Option<String>,
    ///Strings representing hex colors split with ,
    pub colors: Option<String>,
    pub max_iterations: Option<u32>,
    pub num_colors: Option<u32>,
    pub zoom: Option<f32>,
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
    pub msaa: Option<u8>,
    pub smooth: Option<bool>,
    pub debug: Option<bool>,
}
