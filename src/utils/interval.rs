#[derive(Clone)]
pub struct Interval {
    pub min : f32,
    pub max : f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f32{
        self.max - self.min
    } 

    pub fn is_empty(&self) -> bool {
        self.min > self.max
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }


    pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
        if x < min {
            return min;
        }
        if x > max {
            return max;
        }
        x
    }

    // Commenter ou supprimer si inutilisées
    // const EMPTY : Interval = Interval { min: f32::INFINITY, max: f32::NEG_INFINITY };
    // const UNIVERSE : Interval = Interval { min: f32::NEG_INFINITY, max: f32::INFINITY };
}
