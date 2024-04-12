//! Учет момента массы льда
use std::rc::Rc;
use crate::ILoad;

/// Учет момента массы льда при обледенения судна. 
/// Может быть без обледенения, частичным и полным.  
/// При расчете обледенения необходимо учитывать изменения водоизмещения и  
/// возвышения центра тяжести. При учете обледенения к массе судна добавляются  
/// масса льда на бортах, палубах, палубном грузе. Масса льда и его моменты,  
/// рассчитываются для осадки 𝑑𝑚𝑖𝑛 и распространяются на все случаи загрузки. 
pub struct IcingMoment {
    /// Тип обледенения
    icing_stab: IcingStab,
    /// Все грузы судна
    loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
    /// Площадь горизонтальных поверхностей
    area_h: Vec<ParsedIcingArea>,
    /// Площадь поверхности парусности
    area_v: Vec<ParsedIcingArea>,
}
///
impl IcingMoment {
    /// Основной конструктор
    /// * icing_stab - Тип обледенения
    /// * loads_cargo - Грузы судна
    /// * area_h - Площадь горизонтальных поверхностей
    /// * area_v - Площадь поверхности парусности
    pub fn new(
        icing_stab: IcingStab,
        loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
        area_h: Vec<ParsedIcingArea>,
        area_v: Vec<ParsedIcingArea>,
    ) -> Self {
        Self{
            icing_stab, 
            loads_cargo,
            area_h,
            area_v,
        }
    }
    /// Суммарная масса льда
    pub fn mass(&self, bound: Option<Bound>) -> f64 {
        
        if let Some(bound) = bound {
            self.bound_x.part_ratio(&bound) * self.mass
        } else {
            self.mass
        }
    }
    /// Распределение массы по вектору разбиения
    fn values(&self) -> Vec<f64> {
    }
    /// Суммарный статический момент массы льда.
    pub fn moment_mass(&self) -> Moment {
    }
    /// Отстояние центра масс
    fn shift(&self) -> Position {
    }
    /// Расчет обледенения
    fn calculate(&mut self) {
        // Масса льда на общей горизонтальной проекции открытых палуб и палубного груза, т
        let p_h = self.desc_area;
        // Момент массы льда на общей горизонтальной проекции открытых палуб и палубного груза
        let m_x_h = ;
        let m_y_h = ;
        let m_z_h = ;
        // Масса льда на площади парусности, т
        let p_v = (a_v_cs_dmin + a_v_ds) * self.w_v;
        // Момент массы льда на площади парусности
        let m_x_v = (m_vx_cs_dmin + m_vx_ds) * self.icing_stab.mass_h();
        let m_z_v = (m_vz_cs_dmin + m_vz_ds) * self.icing_stab.mass_v();
        // Масса льда
        let p = p_h + p_v;
        // Момент массы льда
        let m_x = m_x_h + m_x_v;
        let m_z = m_z_h + m_z_v;
    }
}
///
impl IIcing for Icing {
}
#[doc(hidden)]
pub trait IIcing {
}
// заглушка для тестирования
#[doc(hidden)]
pub struct FakeIcing {

}
#[doc(hidden)]
#[allow(dead_code)]
impl FakeIcing {
    pub fn new(

    ) -> Self {
        Self {

        }
    }
}




