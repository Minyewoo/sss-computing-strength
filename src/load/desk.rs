//! Палубный груз
use crate::math::*;

use super::IMass;

/// Палубный груз, имеет площадь и парусность
pub trait IDeskLoad {
    /// Парусность попадающая в Bound или вся если Bound отсутствует
    fn windage_area(&self, bound: Option<Bound>) -> f64;
    /// Смещение центра парусности
    fn windage_shift(&self) -> Position;
    /// Площадь горизонтальной поверхности, м^2
    fn horizontal_area(&self, bound: Option<Bound>) -> f64;
    /// Высота груза, м
    fn height(&self) -> f64;
}
/// Палубный груз, имеет площадь и парусность
pub struct DeskLoad { 
    /// Масса груза 
    mass: f64,
    /// Границы груза 
    bound_x: Bound,
    bound_y: Bound,
    bound_z: Bound,
    /// Площадь парусности
    windage_area: Option<f64>,
    /// Смещение центра парусности
    windage_shift: Option<Position>,
}
///
impl DeskLoad {
    /// Основной конструктор
    /// * mass - Масса груза
    /// * bound_x - границы груза вдоль продольной оси
    /// * bound_y - границы груза вдоль поперечной оси
    /// * bound_z - границы груза вдоль вертикальной оси
    /// * windage_shift - Смещение центра парусности
    pub fn new(
        mass: f64,
        bound_x: Bound,
        bound_y: Bound,
        bound_z: Bound,
        windage_area: Option<f64>,
        windage_shift: Option<Position>,
    ) -> Self {
        Self {
            mass,
            bound_x,
            bound_y,
            bound_z,
            windage_area,
            windage_shift,
        }
    }
}
///
///
impl IDeskLoad for DeskLoad {
    /// Парусность попадающая в Bound или вся если Bound отсутствует
    fn windage_area(&self, bound: Option<Bound>) -> f64 {
        self.bound_x.part_ratio(&bound.unwrap_or(self.bound_x)) * 
        self.windage_area.unwrap_or(self.bound_x.length()*self.bound_z.length())
    }
    /// Смещение центра парусности
    fn windage_shift(&self) -> Position {
        if let Some(windage_shift) = self.windage_shift.clone() {
            windage_shift
        } else {
            Position::new(self.bound_x.center(), self.bound_y.center(), self.bound_y.center(),)
        }
    }
    /// Площадь горизонтальной поверхности, м^2
    fn horizontal_area(&self, bound: Option<Bound>) -> f64 {
        self.bound_x.part_ratio(&bound.unwrap_or(self.bound_x)) *
        self.bound_y.length()
    }
    /// Высота груза, м
    fn height(&self) -> f64 {
        self.bound_z.length()
    }
}

///
impl IMass for DeskLoad {
    ///
    fn sum(&self) -> f64 {
        self.mass
    }
    ///
    fn bound_x(&self) -> Bound {
        self.bound_x
    }    
    ///
    fn mass_shift(&self) -> Position {
        self.center.value(self.volume)
    } 
}

