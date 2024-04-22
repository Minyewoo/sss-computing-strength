//! Цистерна с жидкостью
use crate::{math::*};

use super::IMass;

/// Цистерна с жидкостью.
/// Имеет свойства свободной поверхности жидкости.
pub trait ITank {
    /// Момент свободной поверхности 
    fn moment_surface(&self) -> SurfaceMoment;
}


/// Цистерна с жидкостью.
/// Имеет свойства свободной поверхности жидкости.
#[derive(Clone)]
pub struct Tank {
    /// Плотность жидкости в цистерне
    density: f64,
    /// Объем жидкости в цистерне
    volume: f64,
    /// Границы
    bound_x: Bound,
    /// Кривая координат центра объема жидкости в цистерне в системе координат судна
    center: PosShift,
    /// Кривая момента инерции площади свободной поверхности жидкости
    free_surf_inertia: InertiaShift,
    /// Продольный момент свободной поверхности жидкости
    m_f_s_y: Option<f64>,
    /// Поперечный момент свободной поверхности жидкости
    m_f_s_x: Option<f64>,
}
///
impl Tank {
    /// Основной конструктор
    /// * density - Плотность жидкости в цистерне
    /// * volume - Объем жидкости в цистерне
    /// * bound_x - Границы цистерны по оси Х
    /// * center - Кривая координат центра объема жидкости в цистерне в системе координат судна
    /// * free_surf_inertia - Кривая момента инерции площади свободной поверхности жидкости
    /// * m_f_s_y - Продольный момент свободной поверхности жидкости
    /// * m_f_s_x - Поперечный момент свободной поверхности жидкости
    pub fn new(
        density: f64,
        volume: f64,
        bound_x: Bound,
        center: PosShift,
        free_surf_inertia: InertiaShift,
        m_f_s_y: Option<f64>,
        m_f_s_x: Option<f64>,
    ) -> Self {
        assert!(density > 0., "density {} > 0", density);
        assert!(volume >= 0., "volume {} >= 0", volume);
        Self {
            density,
            volume,
            bound_x,
            center,
            free_surf_inertia,
            m_f_s_y,
            m_f_s_x,
        }
    }
}
///
impl ITank for Tank {
    /// Момент свободной поверхности 
    fn moment_surface(&self) -> SurfaceMoment {
        if self.m_f_s_x.is_some() && self.m_f_s_y.is_some() {
            return SurfaceMoment::new(self.m_f_s_x.unwrap(), self.m_f_s_y.unwrap());
        }
        let result =
            SurfaceMoment::from_inertia(self.free_surf_inertia.value(self.volume), self.density);
        log::info!("\t Tank result:{:?}", result);    
        result
    }
}
///
impl IMass for Tank {
    ///
    fn sum(&self) -> f64 {
        self.density*self.volume
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
