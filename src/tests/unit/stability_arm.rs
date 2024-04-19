#[cfg(test)]

mod tests {

    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use std::{rc::Rc, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;

    use crate::{
        math::*,
        stability::{metacentric_height::*, stability_arm::*},
        FakeMass,
    };

    static INIT: Once = Once::new();

    unsafe impl Sync for StabilityArm {} //for static
    static mut STABILITY_ARM: Option<StabilityArm> = None;

    fn init_once() {
        INIT.call_once(|| {
            let mass = Rc::new(FakeMass::new(
                50.0,
                vec![0.],
                Position::new(0., 0., 0.),
                DeltaMH::new(0., 0.),
                Position::new(0., 0., 0.),
                SurfaceMoment::new(0., 0.),
            ));

            let center_draught_shift = Position::new(0., 0., 0.);

            let metacentric_height: Rc<dyn IMetacentricHeight> =
                Rc::new(FakeMetacentricHeight::new(0., 0., 0.));

            let pantocaren = vec![
                (
                    1.,
                    vec![
                        (0., 0.),
                        (15., 1.),
                        (30., 2.),
                        (45., 3.),
                        (60., 2.),
                        (75., 1.),
                        (90., 0.),
                    ],
                ),
                (
                    10.,
                    vec![
                        (0., 0.),
                        (15., 1.),
                        (30., 2.),
                        (45., 3.),
                        (60., 2.),
                        (75., 1.),
                        (90., 0.),
                    ],
                ),
            ];
            let mut stability_arm = StabilityArm::new(
                mass,
                center_draught_shift,
                Curve2D::from_values_linear(pantocaren),
                5.,
                metacentric_height,
            );
            stability_arm.dso();
            unsafe {
                STABILITY_ARM.replace(stability_arm);
            }
        })
    }

    #[test]
    fn angle() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test StabilityArm angle";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { STABILITY_ARM.clone().unwrap().angle(1.) };
        let target = vec![15., 75.];
        result.iter().zip(target.iter()).for_each(|(r, t)| {
            assert!((r - t).abs() < 0.001, "\nresult: {:?}\ntarget: {:?}", r, t)
        });

        test_duration.exit();
    }

    #[test]
    #[ignore = "TODO"]
    fn dso() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        println!("");
        let self_id = "test StabilityArm diagram";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let result = unsafe { STABILITY_ARM.clone().unwrap().dso() };
        let target = vec![(0., 0.), (30., 2.), (45., 3.), (60., 2.), (90., 0.)];
        assert!(
            result == target,
            "\nresult: {:?}\ntarget: {:?}",
            result,
            target
        );

        test_duration.exit();
    }
}