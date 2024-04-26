//! Критерии проверки остойчивости

use std::rc::Rc;

use crate::{
    Curve, Error, IAcceleration, ICirculation, ICurve, IGrain, ILeverDiagram, IMetacentricHeight,
    IStability, IWind,
};

/// Критерии проверки остойчивости
struct Criterion {
    /// Ship type: "Tanker  container Barge-carrying Roll-on/roll-off Dry-bulk  General cargo timber"
    /// Угол заливания отверстий
    flooding_angle: f64,
    /// Длина судна
    ship_length: f64,
    /// Статический угол крена от действия постоянного ветра.
    /// Предполагаемое давление ветра 𝑝𝑣 принимается как для судна
    /// неограниченного района плавания судна.
    wind: Rc<dyn IWind>,
    /// Диаграмма плеч статической и динамической остойчивости
    lever_diagram: Rc<dyn ILeverDiagram>,
    /// Критерий погоды K
    stability: Rc<dyn IStability>,
    /// Продольная и поперечная исправленная метацентрическая высота
    metacentric_height: Rc<dyn IMetacentricHeight>,
    /// Расчет критерия ускорения
    acceleration: Rc<dyn IAcceleration>,
    /// Расчет крена на циркуляции
    circulation: Rc<dyn ICirculation>,
    /// Смещение груза при перевозки навалочных смещаемых грузов (зерна)
    grain: Rc<dyn IGrain>,
}
///
impl Criterion {
    /// Главный конструктор:
    /// * flooding_angle - Угол заливания отверстий
    /// * ship_length - Длина судна
    /// * wind - Статический угол крена от действия постоянного ветра
    /// * lever_diagram - Диаграмма плеч статической и динамической остойчивости
    /// * stability - Критерий погоды K
    /// * metacentric_height - Продольная и поперечная исправленная метацентрическая высота
    /// * acceleration - Расчет критерия ускорения
    /// * circulation - Расчет крена на циркуляции
    /// * grain - Смещение груза при перевозки навалочных смещаемых грузов (зерна)
    pub fn new(
        flooding_angle: f64,
        ship_length: f64,
        wind: Rc<dyn IWind>,
        lever_diagram: Rc<dyn ILeverDiagram>,
        stability: Rc<dyn IStability>,
        metacentric_height: Rc<dyn IMetacentricHeight>,
        acceleration: Rc<dyn IAcceleration>,
        circulation: Rc<dyn ICirculation>,
        grain: Rc<dyn IGrain>,
    ) -> Self {
        Self {
            flooding_angle,
            ship_length,
            wind,
            stability,
            lever_diagram,
            metacentric_height,
            acceleration,
            circulation,
            grain,
        }
    }
    /// Критерий погоды K
    pub fn weather(&mut self) -> Result<(f64, f64), Error> {
        Ok((self.stability.k()?, 1.))
    }
    /// Статический угол крена от действия постоянного ветра.
    /// При расчете плеча кренящего момента от давления ветра 𝑙𝑤1, используемое при
    /// определении угла крена θ𝑤1, предполагаемое давление ветра 𝑝𝑣 принимается как для судна
    /// неограниченного района плавания судна.
    pub fn static_angle(&mut self) -> Result<(f64, f64), Error> {
        // Для всех судов (кроме района плавания R3):
        // статического угла крена θ𝑤1, вызванного постоянным ветром
        let wind_lever = self.wind.arm_wind_static();
        let binding = self
            .lever_diagram
            .angle(wind_lever);
        let wind_angle = binding
            .first()
            .ok_or(Error::FromString("Moment of wind too height!".to_owned()))?;
        Ok((*wind_angle, 16.0f64.min(0.8 * self.flooding_angle)))
        // TODO: Для лесовозов:
        // theta_w_1 <= 16.0
        // Для контейнеровозов:
        // theta_w_1 <= 16.0.min( 0.5*flooding_angle )
    }
    /// Площади под диаграммой статической остойчивости
    pub fn dso(&self) -> Vec<(f64, f64)> {
        //    Все суда
        vec![
            (self.lever_diagram.dso_area(0., 30.), 0.055),
            (
                self.lever_diagram
                    .dso_area(0., 40.0f64.min(self.flooding_angle)),
                0.09,
            ),
            (
                self.lever_diagram
                    .dso_area(30., 40.0f64.min(self.flooding_angle)),
                0.03,
            ),
        ]
        // TODO:    При перевозке палубного лесного груза
        //    self.lever_diagram.area(0, 40.min(flooding_angle)) >= 0,08 м·рад
    }
    /// Максимум диаграммы статической остойчивости
    pub fn dso_lever(&self) -> Result<(f64, f64), Error> {
        // Все суда (за исключением лесовозов)
        let curve = Curve::new_linear(&vec![(105., 0.25), (80., 20.)]);
        Ok((
            self.lever_diagram.lever_moment(30.),
            curve.value(self.ship_length),
        ))
        // TODO:    При перевозке палубного лесного груза и обледенении
    }
    /// Угол, соответствующий максимуму диаграммы статической остойчивости
    pub fn dso_lever_max_angle(&self) -> Result<(f64, f64), Error> {
        //   Все суда
        let binding = self
            .lever_diagram
            .max_angles();
        let mas_angle = binding
            .first()
            .ok_or(Error::FromString("No max angles!".to_owned()))?;
        if self.lever_diagram.max_angles().len() == 1 {
            Ok((mas_angle.0, 30.))
        } else {
            Ok((mas_angle.0, 25.))
        }

        //    Судам, имеющим отношение 𝐵/𝐷>2,
        //    delta_theta_max = 40.*((b/d).min(2.5) - 2.)*(k.min(1.5) - 1)*0.5;
        //    theta_max - delta_theta_max <= ()

        //    Судам, имеющим отношение 𝐵/𝐷>2,5,
        //    theta_max >= 15

        //    if theta_max == 15. {
        //        dso_area(0, 15.) >= 0,07 м·рад,
        //    } else if theta_max >= 30. {
        //        dso_area(0, 30.) >= 0,055 м·рад,
        //    } else {
        //        dso_area(0, StabilityArm.theta_last()) >= 0.055 + 0.001*(30.0 - theta_max)
        //    }
    }
    /// Метацентрическая высота
    pub fn metacentric_height(&self) -> Result<(f64, f64), Error> {
        // Все суда
        // TODO: за исключением «судна порожнем» (если балласт и груз != 0)
        Ok((self.metacentric_height.h_cross_fix(), 0.15))

        // Сухогрузное накатное судно 0.2
        // При перевозке леса 0,1 м
        // При перевозке зерна 0.3 м

        // ДОПОЛНИТЕЛЬНЫЕ ТРЕБОВАНИЯ К СУХОГРУЗАМ
        // if MetacentricHeight.h_cross_fix().sqrt()/B > 0.08 || B/d > 2.5
        // то проверяем критерий ускорения 𝐾∗
        //accelleration(&self)
    }
    /// Критерий ускорения 𝐾∗
    pub fn accelleration(&self) -> Result<(f64, f64), Error> {
        // ДОПОЛНИТЕЛЬНЫЕ ТРЕБОВАНИЯ К СУДАМ СМЕШАННОГО РЕКА МОРЯ
        Ok((self.acceleration.calculate(), 1.))
    }
    /// Критерий крена на циркуляции
    pub fn circulation(&self) -> Result<(f64, f64), Error> {
        // ДОПОЛНИТЕЛЬНЫЕ ТРЕБОВАНИЯ К СУДАМ, ПЕРЕВОЗЯЩИМ КОНТЕЙНЕРЫ
        let target_angle = 16.0f64.min(self.flooding_angle);
        if let Some(angle) = self.circulation.angle() {
            if angle <= target_angle {
                return Ok((angle, target_angle));
            }            
        }
        let velocity = self.circulation.velocity(target_angle);
        Err(Error::FromString(format!("An angle of {target_angle} degree requires speed {velocity} m/s")))

        // В случаях, когда палубный груз контейнеров размещается только на крышках грузовых
        // люков, вместо угла входа кромки верхней палубы может приниматься меньший из углов
        // входа в воду верхней кромки комингса люка или входа контейнера в воду (в случае, когда
        // контейнеры выходят за пределы этого комингса).

        // В случае если требование к величине угла крена на циркуляции при
        // эксплуатационной скорости хода не выполняется, в Информации об остойчивости
        // должна быть указана максимально допустимая скорость судна перед выходом на
        // циркуляцию, определенная из выполнения указанного требования.
    }
    /// Критерий при перевозки навалочных смещаемых грузов
    pub fn grain(&self) -> Result<(f64, f64), Error> {
        // ДОПОЛНИЕТЕЛЬНЫЕ ТРЕБОВАНИЯ К ЗЕРНОВОЗАМ
        Ok((self.grain.area(), 0.075))

        // В случаях, когда палубный груз контейнеров размещается только на крышках грузовых
        // люков, вместо угла входа кромки верхней палубы может приниматься меньший из углов
        // входа в воду верхней кромки комингса люка или входа контейнера в воду (в случае, когда
        // контейнеры выходят за пределы этого комингса).
    }
}
