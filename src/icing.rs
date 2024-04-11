//! Учет обледенения

use std::rc::Rc;
use crate::{icing_stab::IcingStab, ILoad};

/// Учет обледенения судна. Может быть без обледенения, частичным и полным.  
/// При расчете обледенения необходимо учитывать изменения водоизмещения и  
/// возвышения центра тяжести. При учете обледенения к массе судна добавляются  
/// масса льда на бортах, палубах, палубном грузе. Масса льда и его моменты,  
/// рассчитываются для осадки 𝑑𝑚𝑖𝑛 и распространяются на все случаи загрузки. 
pub struct Icing {
    /// Тип обледенения
    icing_stab: IcingStab,
    /// Все грузы судна
    loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
    /// Площадь обледенения горизонтальных поверхностей
    icing_area_h: Vec<ParsedIcingArea>,
    /// Площадь обледенения поверхности парусности
    icing_area_v: Vec<ParsedIcingArea>,
}
///
impl Icing {
    /// Основной конструктор
    /// * icing_stab - Тип обледенения
    /// * loads_cargo - Грузы судна
    /// * icing_area_h - Площадь обледенения горизонтальных поверхностей
    /// * icing_area_v - Площадь обледенения поверхности парусности
    pub fn new(
        /// Тип обледенения
        icing_stab: IcingStab,
        /// Все грузы судна
        loads_cargo: Vec<Rc<Box<dyn ILoad>>>,
        /// Площадь обледенения горизонтальных поверхностей
        icing_area_h: Vec<ParsedIcingArea>,
        /// Площадь обледенения поверхности парусности
        icing_area_v: Vec<ParsedIcingArea>,
    ) -> Self {
        Self{
            icing_stab, 
            loads_cargo,
            icing_area_h,
            icing_area_v,
        }
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
        let m_x_v = (m_vx_cs_dmin + m_vx_ds) * self.w_v;
        let m_z_v = (m_vz_cs_dmin + m_vz_ds) * self.w_v;
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
#[doc(hidden)]
impl IIcing for FakeIcing {
}




