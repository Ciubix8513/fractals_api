use std::sync::Mutex;

use super::rendering::Fractals;

///Represents types of fractals that the api can render, simplified so that serde can deserialize
///all of them
#[derive(Debug, Clone, Copy, serde_derive::Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, serde_derive::Deserialize, PartialEq)]
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

impl Eq for RequestBody {}

impl std::hash::Hash for RequestBody {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.width.hash(state);
        self.height.hash(state);
        self.formula.hash(state);
        self.colors.hash(state);
        self.max_iterations.hash(state);
        self.num_colors.hash(state);
        if let Some(zoom) = self.zoom {
            zoom.to_bits().hash(state);
        }
        if let Some(position_x) = self.position_x {
            position_x.to_bits().hash(state);
        }
        if let Some(position_y) = self.position_y {
            position_y.to_bits().hash(state);
        }
        self.msaa.hash(state);
        self.smooth.hash(state);
        self.debug.hash(state);
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RequestIdentifier {
    fractal: SimplifiedFractals,
    body: RequestBody,
}

impl RequestIdentifier {
    pub fn new(fractal: SimplifiedFractals, body: &RequestBody) -> Self {
        Self {
            fractal,
            body: body.clone(),
        }
    }
}

pub type Cache = Mutex<Vec<(RequestIdentifier, Vec<u8>)>>;
