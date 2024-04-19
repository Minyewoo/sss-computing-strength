----------- Критерий погоды
𝐾=𝑏/а 𝐾≥1,


------------ Статический угол крена от действия постоянного ветра

При расчете плеча кренящего момента от давления ветра 𝑙𝑤1, используемое при
определении угла крена θ𝑤1, предполагаемое давление ветра 𝑝𝑣 принимается как для судна
неограниченного района плавания судна. - пересчитать все для этого района

Для всех судов (кроме района плавания R3):
статического угла крена θ𝑤1, вызванного постоянным ветром theta_w_1 <= 16.0.min( 0.8*flooding_angle )
Для лесовозов:
theta_w_1 <= 16.0

Для контейнеровозов:
theta_w_1 <= 16.0.min( 0.5*flooding_angle )


Stability
dso_area(&mut self, angle1: f64, angle2: f64) -> Result<f64, Error>
theta_max(&mut self) -> Result<f64, Error> 

--------ДИАГРАММА СТАТИЧЕСКОЙ ОСТОЙЧИВОСТИ
Все суда
Stability.dso_area(0, 30) >= 0,055 м·рад
dso_area(0, 40.min(flooding_angle)) >= 0,09 м·рад 
dso_area(30, 40.min(flooding_angle)) >= 0,03 м·рад 

При перевозке палубного лесного груза
dso_area(0, 40.min(flooding_angle)) >= 0,08 м·рад 

--------- Плечо диаграммы
if ship_lenght <= 80. {
    StabilityArm.angle(30°) >= 0,25 м
} else if ship_lenght >= 105. {
    StabilityArm.angle(30°) >= 0,20 м    
}
Для промежуточных значений 𝐿 величина плеча определяется линейной интерполяцией.

При перевозке палубного лесного груза
StabilityArm.angle_max(30°) >= 0,25 м   


---------- Угол, соответствующий максимуму диаграммы
Stability
max_angles.first >= 30 или 25 если max_angles.len > 1

Судам, имеющим отношение 𝐵/𝐷>2,
delta_theta_max = 40.*((b/d).min(2.5) - 2.)*(k.min(1.5) - 1)*0.5;
theta_max - delta_theta_max <= ()

Судам, имеющим отношение 𝐵/𝐷>2,5,
theta_max >= 15

if theta_max == 15. {
    dso_area(0, 15.) >= 0,07 м·рад,
} else if theta_max >= 30. {
    dso_area(0, 30.) >= 0,055 м·рад, 
} else {
    dso_area(0, StabilityArm.theta_last()) >= 0.055 + 0.001*(30.0 - theta_max)
}


---------- МЕТАЦЕНТРИЧЕСКАЯ ВЫСОТА
MetacentricHeight.h_cross_fix() >=
Все суда за исключением «судна порожнем» (если балласт и груз != 0)  0.15
Сухогрузное накатное судно 0.2
При перевозке леса 0,1 м
При перевозке зерна 0.3 м


------------  ДОПОЛНИТЕЛЬНЫЕ ТРЕБОВАНИЯ К СУХОГРУЗАМ
if MetacentricHeight.h_cross_fix().sqrt()/B > 0.08 || B/d > 2.5 
то проверяем критерий ускорения 𝐾∗


----------- ДОПОЛНИТЕЛЬНЫЕ ТРЕБОВАНИЯ К СУДАМ СМЕШАННОГО РЕКА МОРЯ (критерий ускорения 𝐾∗)
Accelleration 
K* = 0.3/a_calc >= 1;
