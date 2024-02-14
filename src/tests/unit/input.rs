#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use crate::core_::{
        debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
        testing::test_stuff::max_test_duration::TestDuration,
    }; 
    use crate::data::parse_input::*;
    
    #[test]
    fn input() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        println!("");
        let selfId = "test Input";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();

        let data = r#"
        {
            "gravity_const_g": 9.81,
            "water_density": 1.025,
            "mean_draught_curve": [[0.0, 0.0], [1000.0, 1.0], [100000.0, 5.0]],
            "trimming_moment_curve": [[0.0, 1000.0], [1.0, 2000.0], [5.0, 100000.0]],
            "buoyancy_centre_curve": [[0.0, -0.2], [1.0, 0.1], [5.0, 1.0]],
            "frames": [ 
                {"offset_x": -1.0, "immersion_area_curve": [[0.0, 0.0], [1.0, 1.0], [5.0, 10.0]]},
                {"offset_x": 0.0, "immersion_area_curve": [[0.0, 0.0], [1.0, 1.0], [5.0, 10.0]]},
                {"offset_x": 1.0, "immersion_area_curve": [[0.0, 0.0], [1.0, 1.0], [5.0, 10.0]]}
            ],
            "frame_spaces": [
                {"mass": 10.0, "displacement_curve": [[0.0, 0.0], [1.0, 1.0], [5.0, 10.0]]},
                {"mass": 10.0, "displacement_curve": [[0.0, 0.0], [1.0, 1.0], [5.0, 10.0]]},
                {"mass": 10.0, "displacement_curve": [[0.0, 0.0], [1.0, 1.0], [5.0, 10.0]]}
            ]
        }"#;
        
    
        let result = ParsedData::parse(&data).expect("parse error");
        let target = ParsedData {
            gravity_const_g: 9.81,
            water_density: 1.025,
            mean_draught_curve: vec![ (0.0, 0.0), (1000.0, 1.0), (100000.0, 5.0)],
            trimming_moment_curve: vec![(0.0, 1000.0), (1.0, 2000.0), (5.0, 100000.0)],
            buoyancy_centre_curve: vec![(0.0, -0.2), (1.0, 0.1), (5.0, 1.0)],
            frames: vec![ 
                FrameData { offset_x: -1.0, immersion_area_curve: vec![(0.0, 0.0), (1.0, 1.0), (5.0, 10.0)] },
                FrameData { offset_x: 0.0, immersion_area_curve: vec![(0.0, 0.0), (1.0, 1.0), (5.0, 10.0)] },
                FrameData { offset_x: 1.0, immersion_area_curve: vec![(0.0, 0.0), (1.0, 1.0), (5.0, 10.0)] },
            ],
            frame_spaces: vec![
                FrameSpaceData { mass: 10.0, displacement_curve: vec![(0.0, 0.0), (1.0, 1.0), (5.0, 10.0)], },
                FrameSpaceData { mass: 10.0, displacement_curve: vec![(0.0, 0.0), (1.0, 1.0), (5.0, 10.0)], },
                FrameSpaceData { mass: 10.0, displacement_curve: vec![(0.0, 0.0), (1.0, 1.0), (5.0, 10.0)], },
            ],            
        };

        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        testDuration.exit();
    }
}
