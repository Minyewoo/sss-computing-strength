//! Нагрузка на судно: постоянный и переменный груз
use crate::math::*;


/// Абстрактный груз: контейнер, трюм или бак.
/// Имеет массу и может вернуть какая его часть попадает в указанные границы
pub trait ILoad {
    /// центер масс груза
    fn center(&self) -> Position;
    /// масса груза
    fn mass(&self, bound: Option<Bound>) -> f64;
    /// момент массы
    fn moment_mass(&self) -> MassMoment {
        MassMoment::from_pos(self.center(), self.mass(None))
    }
    /// момент свободной поверхности
    fn moment_surface(&self) -> SurfaceMoment {
        SurfaceMoment::new(0., 0.,)
    }
}

/// Груз, контенер, трюм и т.п. твердый груз, имеет границы, центр масс и значение
pub struct LoadSpace {
    /// общая масса
    mass: f64,     
    /// границы груза
    bound: Bound,  
    /// центер масс
    center: Position, 
    /// TODO: удалить и перенести в цистерны: Продольный момент свободной поверхности жидкости 
    m_f_s_y: f64,
    /// TODO: удалить и перенести в цистерны: Поперечный момент свободной поверхности жидкости 
    m_f_s_x: f64,
}

#[allow(dead_code)]
impl LoadSpace {
    ///
    pub fn new(mass: f64, bound: Bound, center: Position, m_f_s_y: f64, m_f_s_x: f64) -> Self {
        assert!(bound.start() < center.x(), "bound.start {} < pos.x {}", bound.start(), center.x());
        assert!(bound.end() > center.x(), "bound.end {} > pos.x {}", bound.end(), center.x());
        Self { bound, center, mass, m_f_s_y, m_f_s_x }
    }
}

impl ILoad for LoadSpace {
    fn mass(&self, bound: Option<Bound>) -> f64 {
        if let Some(bound) = bound {
            self.bound.part_ratio(&bound)*self.mass
        } else {
            self.mass
        }
    }

    fn center(&self) -> Position {
        self.center
    }

    /// момент свободной поверхности - TODO - удалить
    fn moment_surface(&self) -> SurfaceMoment {
        SurfaceMoment::new(self.m_f_s_x, self.m_f_s_y)
    }
}
