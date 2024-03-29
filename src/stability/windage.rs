//! Парусность судна

use std::rc::Rc;

use crate::{ILoad, Moment, Position};

/// Парусность судна, площадь и положение 
/// центра относительно миделя и ОП
pub struct Windage {
    /// Все грузы судна
    loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
    /// Площадь парусности судна для минимальной осадки
    area: f64,
    /// Cтатический момент относительно миделя (x) и ОП (z) 
    /// для минимальной осадки
    moment: Moment,
    /// Смещение центра парусности судна для минимальной осадки
    shift: Position, 
    /// Разница в площадях парусности для осадки по ЛГВЛ и минимальной осадки
    delta_area: f64,
    /// Разница в статических моментах относительно миделя (x) и ОП (z) 
    /// соответствеено для осадки по ЛГВЛ и минимальной осадки
    delta_moment: Moment,
    /// Текущая осадка
    drought_current: f64,
    /// Минимальная осадка  
    draught_min: f64,    
    /// Осадка по ЛГВЛ
    draught_lgvl: f64,  
}
///
impl Windage {
    /// Аргументы конструктора:  
    /// * loads_cargo - грузы судна
    /// * area - Площадь парусности судна
    /// * moment - Cтатический момент относительно миделя (x) и ОП (z) 
    ///             для минимальной осадки
    /// * shift - Смещение центра парусности судна
    /// * delta_area - Разница в площадях парусности для осадки по ЛГВЛ и минимальной осадки
    /// * delta_moment - Разница в статических моментах относительно миделя (x) и ОП (z) 
    ///                 соответствеено для осадки по ЛГВЛ и минимальной осадки
    /// * drought_current - Текущая осадка
    /// * draught_min - Минимальная осадка  
    /// * draught_lgvl - Осадка по ЛГВЛ
    pub fn new(
        loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
        area: f64,
        moment: Moment,
        shift: Position,
        delta_area: f64,
        delta_moment: Moment,
        drought_current: f64,
        draught_min: f64,    
        draught_lgvl: f64,     
    ) -> Self {
        assert!(area >= 0., "area {area} >= 0");
        assert!(delta_area >= 0., "delta_area {delta_area} >= 0");
        assert!(drought_current >= 0., "drought_current {drought_current} >= 0");
        assert!(draught_min >= 0., "draught_min {draught_min} >= 0");
        assert!(draught_lgvl >= 0., "draught_lgvl {draught_lgvl} >= 0");
        Self {
            loads_cargo,
            area,
            moment,
            shift,
            delta_area,
            delta_moment,
            drought_current,
            draught_min,    
            draught_lgvl,  
        }
    }
    ///
    fn calculate(&self) -> f64 {
        let a_v_dmin = self.delta_area;
        let m_vx_dmin = self.delta_moment.x();
        let m_vz_dmin = self.delta_moment.z();

        let delta_a_v_summer = self.delta_area;
        let delta_m_vx_summer = self.delta_moment.x();
        let delta_m_vz_summer = self.delta_moment.z();

        let a_v_summer = a_v_dmin - delta_a_v_summer;
        let m_vx_summer = m_vx_dmin - delta_m_vx_summer;
        let m_vz_summer = m_vz_dmin - delta_m_vz_summer;

        let a_v_pg = self.loads_cargo.iter().map(|l| l.windage_area()).sum();
        let shift_pg: Position = self.loads_cargo.iter().map(|l| l.windage_shift()).sum();
        let m_pg = Moment::from_pos(shift_pg, a_v_pg);
        let a_v_cs_dmin1 = self.area;
        let m_vx_cs_dmin1 = self.moment.x();
        let m_vz_cs_dmin1 = self.moment.z(); 

        let a_v_cs_dmin = a_v_cs_dmin1 + a_v_pg;
        let m_vx_cs_dmin = m_vx_cs_dmin1 + m_pg.x();
        let m_vz_cs_dmin = m_vz_cs_dmin1 + m_pg.z();    

        let a_v_ds_ice = (0.15*a_v_cs_dmin, 0.1*a_v_cs_dmin);
        let m_vz_ds_ice = (0.15*m_vz_cs_dmin, 0.2*m_vz_cs_dmin);
        let m_vx_ds_ice = (0., 0.);
        let a_v_ds = 0.05*a_v_cn;
        let m_vx_ds = 0.;
        let m_vz_ds = 0.1*m_vz_cn;        

        let a_v_dmin = a_v_cs_dmin + a_v_ds + a_v_ds_ice;
        let m_v_x_dmin = m_vx_cs_dmin + m_vx_ds + m_vx_ds_ice;
        let m_v_z_dmin = m_vz_cs_dmin + m_vz_ds + m_vz_ds_ice;
    }
}
