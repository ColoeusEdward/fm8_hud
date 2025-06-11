use std::{collections::BTreeMap, sync::{Mutex, OnceLock}};

pub static TELEMETRY_FIELDS: &[TelemetryDataField] = &[
    TelemetryDataField { name: "IsRaceOn", type_name: "S32", offset: 0, bytes: 4, description: "= 1 when race is on. = 0 when in menus/race stopped." },
    TelemetryDataField { name: "TimestampMS", type_name: "U32", offset: 4, bytes: 4, description: "Can overflow to 0 eventually." },
    TelemetryDataField { name: "EngineMaxRpm", type_name: "F32", offset: 8, bytes: 4, description: "Maximum RPM of the engine." },
    TelemetryDataField { name: "EngineIdleRpm", type_name: "F32", offset: 12, bytes: 4, description: "Idle RPM of the engine." },
    TelemetryDataField { name: "CurrentEngineRpm", type_name: "F32", offset: 16, bytes: 4, description: "Current RPM of the engine." },
    TelemetryDataField { name: "AccelerationX", type_name: "F32", offset: 20, bytes: 4, description: "Acceleration in the car's local space (X = right)." },
    TelemetryDataField { name: "AccelerationY", type_name: "F32", offset: 24, bytes: 4, description: "Acceleration in the car's local space (Y = up)." },
    TelemetryDataField { name: "AccelerationZ", type_name: "F32", offset: 28, bytes: 4, description: "Acceleration in the car's local space (Z = forward)." },
    TelemetryDataField { name: "VelocityX", type_name: "F32", offset: 32, bytes: 4, description: "Velocity in the car's local space (X = right)." },
    TelemetryDataField { name: "VelocityY", type_name: "F32", offset: 36, bytes: 4, description: "Velocity in the car's local space (Y = up)." },
    TelemetryDataField { name: "VelocityZ", type_name: "F32", offset: 40, bytes: 4, description: "Velocity in the car's local space (Z = forward)." },
    TelemetryDataField { name: "AngularVelocityX", type_name: "F32", offset: 44, bytes: 4, description: "Angular velocity in the car's local space (X = pitch)." },
    TelemetryDataField { name: "AngularVelocityY", type_name: "F32", offset: 48, bytes: 4, description: "Angular velocity in the car's local space (Y = yaw)." },
    TelemetryDataField { name: "AngularVelocityZ", type_name: "F32", offset: 52, bytes: 4, description: "Angular velocity in the car's local space (Z = roll)." },
    TelemetryDataField { name: "Yaw", type_name: "F32", offset: 56, bytes: 4, description: "Yaw angle of the car." },
    TelemetryDataField { name: "Pitch", type_name: "F32", offset: 60, bytes: 4, description: "Pitch angle of the car." },
    TelemetryDataField { name: "Roll", type_name: "F32", offset: 64, bytes: 4, description: "Roll angle of the car." },
    TelemetryDataField { name: "NormalizedSuspensionTravelFrontLeft", type_name: "F32", offset: 68, bytes: 4, description: "Normalized suspension travel (0.0 = max stretch, 1.0 = max compression) for front left wheel." },
    TelemetryDataField { name: "NormalizedSuspensionTravelFrontRight", type_name: "F32", offset: 72, bytes: 4, description: "Normalized suspension travel (0.0 = max stretch, 1.0 = max compression) for front right wheel." },
    TelemetryDataField { name: "NormalizedSuspensionTravelRearLeft", type_name: "F32", offset: 76, bytes: 4, description: "Normalized suspension travel (0.0 = max stretch, 1.0 = max compression) for rear left wheel." },
    TelemetryDataField { name: "NormalizedSuspensionTravelRearRight", type_name: "F32", offset: 80, bytes: 4, description: "Normalized suspension travel (0.0 = max stretch, 1.0 = max compression) for rear right wheel." },
    TelemetryDataField { name: "TireSlipRatioFrontLeft", type_name: "F32", offset: 84, bytes: 4, description: "Normalized tire slip ratio (0 = 100% grip, |ratio| > 1.0 = loss of grip) for front left tire." },
    TelemetryDataField { name: "TireSlipRatioFrontRight", type_name: "F32", offset: 88, bytes: 4, description: "Normalized tire slip ratio (0 = 100% grip, |ratio| > 1.0 = loss of grip) for front right tire." },
    TelemetryDataField { name: "TireSlipRatioRearLeft", type_name: "F32", offset: 92, bytes: 4, description: "Normalized tire slip ratio (0 = 100% grip, |ratio| > 1.0 = loss of grip) for rear left tire." },
    TelemetryDataField { name: "TireSlipRatioRearRight", type_name: "F32", offset: 96, bytes: 4, description: "Normalized tire slip ratio (0 = 100% grip, |ratio| > 1.0 = loss of grip) for rear right tire." },
    TelemetryDataField { name: "WheelRotationSpeedFrontLeft", type_name: "F32", offset: 100, bytes: 4, description: "Wheel rotation speed in radians/sec for front left wheel." },
    TelemetryDataField { name: "WheelRotationSpeedFrontRight", type_name: "F32", offset: 104, bytes: 4, description: "Wheel rotation speed in radians/sec for front right wheel." },
    TelemetryDataField { name: "WheelRotationSpeedRearLeft", type_name: "F32", offset: 108, bytes: 4, description: "Wheel rotation speed in radians/sec for rear left wheel." },
    TelemetryDataField { name: "WheelRotationSpeedRearRight", type_name: "F32", offset: 112, bytes: 4, description: "Wheel rotation speed in radians/sec for rear right wheel." },
    TelemetryDataField { name: "WheelOnRumbleStripFrontLeft", type_name: "S32", offset: 116, bytes: 4, description: "= 1 when wheel is on rumble strip, = 0 when off (front left wheel)." },
    TelemetryDataField { name: "WheelOnRumbleStripFrontRight", type_name: "S32", offset: 120, bytes: 4, description: "= 1 when wheel is on rumble strip, = 0 when off (front right wheel)." },
    TelemetryDataField { name: "WheelOnRumbleStripRearLeft", type_name: "S32", offset: 124, bytes: 4, description: "= 1 when wheel is on rumble strip, = 0 when off (rear left wheel)." },
    TelemetryDataField { name: "WheelOnRumbleStripRearRight", type_name: "S32", offset: 128, bytes: 4, description: "= 1 when wheel is on rumble strip, = 0 when off (rear right wheel)." },
    TelemetryDataField { name: "WheelInPuddleDepthFrontLeft", type_name: "F32", offset: 132, bytes: 4, description: "= from 0 to 1, where 1 is the deepest puddle (front left wheel)." },
    TelemetryDataField { name: "WheelInPuddleDepthFrontRight", type_name: "F32", offset: 136, bytes: 4, description: "= from 0 to 1, where 1 is the deepest puddle (front right wheel)." },
    TelemetryDataField { name: "WheelInPuddleDepthRearLeft", type_name: "F32", offset: 140, bytes: 4, description: "= from 0 to 1, where 1 is the deepest puddle (rear left wheel)." },
    TelemetryDataField { name: "WheelInPuddleDepthRearRight", type_name: "F32", offset: 144, bytes: 4, description: "= from 0 to 1, where 1 is the deepest puddle (rear right wheel)." },
    TelemetryDataField { name: "SurfaceRumbleFrontLeft", type_name: "F32", offset: 148, bytes: 4, description: "Non-dimensional surface rumble values passed to controller force feedback (front left wheel)." },
    TelemetryDataField { name: "SurfaceRumbleFrontRight", type_name: "F32", offset: 152, bytes: 4, description: "Non-dimensional surface rumble values passed to controller force feedback (front right wheel)." },
    TelemetryDataField { name: "SurfaceRumbleRearLeft", type_name: "F32", offset: 156, bytes: 4, description: "Non-dimensional surface rumble values passed to controller force feedback (rear left wheel)." },
    TelemetryDataField { name: "SurfaceRumbleRearRight", type_name: "F32", offset: 160, bytes: 4, description: "Non-dimensional surface rumble values passed to controller force feedback (rear right wheel)." },
    TelemetryDataField { name: "TireSlipAngleFrontLeft", type_name: "F32", offset: 164, bytes: 4, description: "Normalized tire slip angle (0 = 100% grip, |angle| > 1.0 = loss of grip) for front left tire." },
    TelemetryDataField { name: "TireSlipAngleFrontRight", type_name: "F32", offset: 168, bytes: 4, description: "Normalized tire slip angle (0 = 100% grip, |angle| > 1.0 = loss of grip) for front right tire." },
    TelemetryDataField { name: "TireSlipAngleRearLeft", type_name: "F32", offset: 172, bytes: 4, description: "Normalized tire slip angle (0 = 100% grip, |angle| > 1.0 = loss of grip) for rear left tire." },
    TelemetryDataField { name: "TireSlipAngleRearRight", type_name: "F32", offset: 176, bytes: 4, description: "Normalized tire slip angle (0 = 100% grip, |angle| > 1.0 = loss of grip) for rear right tire." },
    TelemetryDataField { name: "TireCombinedSlipFrontLeft", type_name: "F32", offset: 180, bytes: 4, description: "Normalized tire combined slip (0 = 100% grip, |slip| > 1.0 = loss of grip) for front left tire." },
    TelemetryDataField { name: "TireCombinedSlipFrontRight", type_name: "F32", offset: 184, bytes: 4, description: "Normalized tire combined slip (0 = 100% grip, |slip| > 1.0 = loss of grip) for front right tire." },
    TelemetryDataField { name: "TireCombinedSlipRearLeft", type_name: "F32", offset: 188, bytes: 4, description: "Normalized tire combined slip (0 = 100% grip, |slip| > 1.0= loss of grip) for rear left tire." },
    TelemetryDataField { name: "TireCombinedSlipRearRight", type_name: "F32", offset: 192, bytes: 4, description: "Normalized tire combined slip (0 = 100% grip, |slip| > 1.0 = loss of grip) for rear right tire." },
    TelemetryDataField { name: "SuspensionTravelMetersFrontLeft", type_name: "F32", offset: 196, bytes: 4, description: "Actual suspension travel in meters for front left wheel." },
    TelemetryDataField { name: "SuspensionTravelMetersFrontRight", type_name: "F32", offset: 200, bytes: 4, description: "Actual suspension travel in meters for front right wheel." },
    TelemetryDataField { name: "SuspensionTravelMetersRearLeft", type_name: "F32", offset: 204, bytes: 4, description: "Actual suspension travel in meters for rear left wheel." },
    TelemetryDataField { name: "SuspensionTravelMetersRearRight", type_name: "F32", offset: 208, bytes: 4, description: "Actual suspension travel in meters for rear right wheel." },
    TelemetryDataField { name: "CarOrdinal", type_name: "S32", offset: 212, bytes: 4, description: "Unique ID of the car make/model." },
    TelemetryDataField { name: "CarClass", type_name: "S32", offset: 216, bytes: 4, description: "Between 0 (D -- worst cars) and 7 (X class -- best cars) inclusive." },
    TelemetryDataField { name: "CarPerformanceIndex", type_name: "S32", offset: 220, bytes: 4, description: "Between 100 (worst car) and 999 (best car) inclusive." },
    TelemetryDataField { name: "DrivetrainType", type_name: "S32", offset: 224, bytes: 4, description: "0 = FWD, 1 = RWD, 2 = AWD." },
    TelemetryDataField { name: "NumCylinders", type_name: "S32", offset: 228, bytes: 4, description: "Number of cylinders in the engine." },
    TelemetryDataField { name: "PositionX", type_name: "F32", offset: 232, bytes: 4, description: "X coordinate of the car's position." },
    TelemetryDataField { name: "PositionY", type_name: "F32", offset: 236, bytes: 4, description: "Y coordinate of the car's position." },
    TelemetryDataField { name: "PositionZ", type_name: "F32", offset: 240, bytes: 4, description: "Z coordinate of the car's position." },
    TelemetryDataField { name: "Speed", type_name: "F32", offset: 244, bytes: 4, description: "Current speed of the car." },
    TelemetryDataField { name: "Power", type_name: "F32", offset: 248, bytes: 4, description: "Current power output of the engine." },
    TelemetryDataField { name: "Torque", type_name: "F32", offset: 252, bytes: 4, description: "Current torque output of the engine." },
    TelemetryDataField { name: "TireTempFrontLeft", type_name: "F32", offset: 256, bytes: 4, description: "Temperature of the front left tire." },
    TelemetryDataField { name: "TireTempFrontRight", type_name: "F32", offset: 260, bytes: 4, description: "Temperature of the front right tire." },
    TelemetryDataField { name: "TireTempRearLeft", type_name: "F32", offset: 264, bytes: 4, description: "Temperature of the rear left tire." },
    TelemetryDataField { name: "TireTempRearRight", type_name: "F32", offset: 268, bytes: 4, description: "Temperature of the rear right tire." },
    TelemetryDataField { name: "Boost", type_name: "F32", offset: 272, bytes: 4, description: "Current boost level." },
    TelemetryDataField { name: "Fuel", type_name: "F32", offset: 276, bytes: 4, description: "Remaining fuel in the car." },
    TelemetryDataField { name: "DistanceTraveled", type_name: "F32", offset: 280, bytes: 4, description: "Total distance traveled by the car." },
    TelemetryDataField { name: "BestLap", type_name: "F32", offset: 284, bytes: 4, description: "Best lap time of the car." },
    TelemetryDataField { name: "LastLap", type_name: "F32", offset: 288, bytes: 4, description: "Time of the last completed lap." },
    TelemetryDataField { name: "CurrentLap", type_name: "F32", offset: 292, bytes: 4, description: "Time of the current lap." },
    TelemetryDataField { name: "CurrentRaceTime", type_name: "F32", offset: 296, bytes: 4, description: "Total race time elapsed." },
    TelemetryDataField { name: "LapNumber", type_name: "U16", offset: 300, bytes: 2, description: "Current lap number." },
    TelemetryDataField { name: "RacePosition", type_name: "U8", offset: 302, bytes: 1, description: "Current race position of the car." },
    TelemetryDataField { name: "Accel", type_name: "U8", offset: 303, bytes: 1, description: "Current acceleration input." },
    TelemetryDataField { name: "Brake", type_name: "U8", offset: 304, bytes: 1, description: "Current brake input." },
    TelemetryDataField { name: "Clutch", type_name: "U8", offset: 305, bytes: 1, description: "Current clutch input." },
    TelemetryDataField { name: "HandBrake", type_name: "U8", offset: 306, bytes: 1, description: "Current handbrake input." },
    TelemetryDataField { name: "Gear", type_name: "U8", offset: 307, bytes: 1, description: "Current gear selected." },
    TelemetryDataField { name: "Steer", type_name: "S8", offset: 308, bytes: 1, description: "Current steering input." },
    TelemetryDataField { name: "NormalizedDrivingLine", type_name: "S8", offset: 309, bytes: 1, description: "Normalized driving line input." },
    TelemetryDataField { name: "NormalizedAIBrakeDifference", type_name: "S8", offset: 310, bytes: 1, description: "Normalized AI brake difference." },
    TelemetryDataField { name: "TireWearFrontLeft", type_name: "F32", offset: 311, bytes: 4, description: "Wear level of the front left tire." },
    TelemetryDataField { name: "TireWearFrontRight", type_name: "F32", offset: 315, bytes: 4, description: "Wear level of the front right tire." },
    TelemetryDataField { name: "TireWearRearLeft", type_name: "F32", offset: 319, bytes: 4, description: "Wear level of the rear left tire." },
    TelemetryDataField { name: "TireWearRearRight", type_name: "F32", offset: 323, bytes: 4, description: "Wear level of the rear right tire." },
    TelemetryDataField { name: "TrackOrdinal", type_name: "S32", offset: 327, bytes: 4, description: "Unique ID for the track." },
];

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct TelemetryDataField {
    pub name: &'static str,
    pub type_name: &'static str,
    pub offset: usize,
    pub bytes: usize,
    pub description: &'static str,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct TrackData {
    pub length: u16,
    pub s1_end: u16,
    pub s2_end: u16,
}

pub static TRACK_DATA_MAP: OnceLock<Mutex<BTreeMap<u16, TrackData>>> = OnceLock::new();
pub static DEFAULT_TRACK_DATA: TrackData = TrackData { length: 5000, s1_end: 1700, s2_end: 3350 };


pub fn init_track_data_map(){
    let mut track_data_map = TRACK_DATA_MAP.get_or_init(|| Mutex::new(BTreeMap::new())).lock().unwrap();
    track_data_map.insert(0, TrackData{length: 0, s1_end: 0, s2_end: 0});
    // Populate the BTreeMap with your data
    track_data_map.insert(1660, TrackData { length: 1642, s1_end: 558, s2_end: 1100 });
    track_data_map.insert(1661, TrackData { length: 1722, s1_end: 585, s2_end: 1154 });
    track_data_map.insert(1663, TrackData { length: 1722, s1_end: 585, s2_end: 1154 });
    track_data_map.insert(860, TrackData { length: 3915, s1_end: 1331, s2_end: 2623 });
    track_data_map.insert(861, TrackData { length: 1944, s1_end: 661, s2_end: 1302 });
    track_data_map.insert(110, TrackData { length: 4657, s1_end: 1583, s2_end: 3120 });
    track_data_map.insert(111, TrackData { length: 3071, s1_end: 1044, s2_end: 2058 });
    track_data_map.insert(113, TrackData { length: 2997, s1_end: 1019, s2_end: 2008 });
    track_data_map.insert(530, TrackData { length: 7004, s1_end: 2381, s2_end: 4693 });
    track_data_map.insert(840, TrackData { length: 5777, s1_end: 1964, s2_end: 3871 });
    track_data_map.insert(841, TrackData { length: 4023, s1_end: 1368, s2_end: 2695 });
    track_data_map.insert(1630, TrackData { length: 3734, s1_end: 1270, s2_end: 2502 });
    track_data_map.insert(1631, TrackData { length: 2623, s1_end: 892, s2_end: 1757 });
    track_data_map.insert(1632, TrackData { length: 3734, s1_end: 1270, s2_end: 2502 });
    track_data_map.insert(1640, TrackData { length: 4281, s1_end: 1456, s2_end: 2868 });
    track_data_map.insert(1641, TrackData { length: 3315, s1_end: 1127, s2_end: 2221 });
    track_data_map.insert(1643, TrackData { length: 3315, s1_end: 1127, s2_end: 2221 });
    track_data_map.insert(250, TrackData { length: 4574, s1_end: 1555, s2_end: 3065 });
    track_data_map.insert(251, TrackData { length: 3692, s1_end: 1255, s2_end: 2474 });
    track_data_map.insert(252, TrackData { length: 2638, s1_end: 897, s2_end: 1767 });
    track_data_map.insert(1110, TrackData { length: 2414, s1_end: 821, s2_end: 1617 });
    track_data_map.insert(1111, TrackData { length: 3556, s1_end: 1209, s2_end: 2383 });
    track_data_map.insert(231, TrackData { length: 4023, s1_end: 1368, s2_end: 2695 });
    track_data_map.insert(232, TrackData { length: 3925, s1_end: 1335, s2_end: 2630 });
    track_data_map.insert(1590, TrackData { length: 4529, s1_end: 1540, s2_end: 3034 });
    track_data_map.insert(100, TrackData { length: 13626, s1_end: 4633, s2_end: 9129 });
    track_data_map.insert(101, TrackData { length: 13535, s1_end: 4602, s2_end: 9068 });
    track_data_map.insert(880, TrackData { length: 2414, s1_end: 821, s2_end: 1617 });
    track_data_map.insert(882, TrackData { length: 2366, s1_end: 804, s2_end: 1585 });
    track_data_map.insert(883, TrackData { length: 2414, s1_end: 821, s2_end: 1617 });
    track_data_map.insert(67, TrackData { length: 4812, s1_end: 1636, s2_end: 3224 });
    track_data_map.insert(68, TrackData { length: 1851, s1_end: 629, s2_end: 1240 });
    track_data_map.insert(70, TrackData { length: 1851, s1_end: 629, s2_end: 1240 });
    track_data_map.insert(16, TrackData { length: 4088, s1_end: 1390, s2_end: 2739 });
    track_data_map.insert(17, TrackData { length: 2881, s1_end: 980, s2_end: 1930 });
    track_data_map.insert(1450, TrackData { length: 3622, s1_end: 1231, s2_end: 2427 });
    track_data_map.insert(1452, TrackData { length: 2817, s1_end: 958, s2_end: 1887 });
    track_data_map.insert(540, TrackData { length: 6213, s1_end: 2112, s2_end: 4163 });
    track_data_map.insert(35, TrackData { length: 5246, s1_end: 1784, s2_end: 3515 });
    track_data_map.insert(36, TrackData { length: 2575, s1_end: 876, s2_end: 1725 });
    track_data_map.insert(31, TrackData { length: 25378, s1_end: 8629, s2_end: 17003 });
    track_data_map.insert(32, TrackData { length: 20830, s1_end: 7082, s2_end: 13956 });
    track_data_map.insert(33, TrackData { length: 5148, s1_end: 1750, s2_end: 3449 });
    track_data_map.insert(34, TrackData { length: 3629, s1_end: 1234, s2_end: 2431 });
    track_data_map.insert(3, TrackData { length: 6515, s1_end: 2215, s2_end: 4365 });
    track_data_map.insert(5, TrackData { length: 3520, s1_end: 1197, s2_end: 2358 });
    track_data_map.insert(40, TrackData { length: 5954, s1_end: 2024, s2_end: 3989 });
    track_data_map.insert(42, TrackData { length: 3219, s1_end: 1094, s2_end: 2157 });
    track_data_map.insert(21, TrackData { length: 5891, s1_end: 2003, s2_end: 3947 });
    track_data_map.insert(22, TrackData { length: 2639, s1_end: 897, s2_end: 1768 });
    track_data_map.insert(23, TrackData { length: 2979, s1_end: 1013, s2_end: 1996 });
    track_data_map.insert(1620, TrackData { length: 4474, s1_end: 1521, s2_end: 2998 });
    track_data_map.insert(1621, TrackData { length: 3122, s1_end: 1061, s2_end: 2092 });
    track_data_map.insert(1622, TrackData { length: 4474, s1_end: 1521, s2_end: 2998 });
    track_data_map.insert(1623, TrackData { length: 3122, s1_end: 1061, s2_end: 2092 });
    track_data_map.insert(1624, TrackData { length: 3927, s1_end: 1335, s2_end: 2631 });
    track_data_map.insert(37, TrackData { length: 5809, s1_end: 1975, s2_end: 3892 });
    track_data_map.insert(38, TrackData { length: 2253, s1_end: 766, s2_end: 1510 });
    track_data_map.insert(990, TrackData { length: 5260, s1_end: 1788, s2_end: 3524 });
    track_data_map.insert(991, TrackData { length: 3620, s1_end: 1231, s2_end: 2425 });
    track_data_map.insert(992, TrackData { length: 2660, s1_end: 904, s2_end: 1782 });
    track_data_map.insert(995, TrackData { length: 6598, s1_end: 2243, s2_end: 4421 });
    track_data_map.insert(996, TrackData { length: 6759, s1_end: 2298, s2_end: 4529 });
    track_data_map.insert(870, TrackData { length: 5552, s1_end: 1888, s2_end: 3720 });
    track_data_map.insert(873, TrackData { length: 3943, s1_end: 1341, s2_end: 2642 });
    track_data_map.insert(0, TrackData { length: 3602, s1_end: 1225, s2_end: 2413 });
    track_data_map.insert(1, TrackData { length: 2961, s1_end: 1007, s2_end: 1984 });
    track_data_map.insert(510, TrackData { length: 5280, s1_end: 1795, s2_end: 3538 });
    track_data_map.insert(511, TrackData { length: 3000, s1_end: 1020, s2_end: 2010 });
    track_data_map.insert(512, TrackData { length: 2190, s1_end: 745, s2_end: 1467 });
    track_data_map.insert(513, TrackData { length: 2290, s1_end: 779, s2_end: 1534 });
    track_data_map.insert(78, TrackData { length: 16480, s1_end: 5603, s2_end: 11042 });
}

pub fn get_track_data_map(track_id: &u16) -> TrackData {
    let track_data = TRACK_DATA_MAP.get().unwrap().lock().unwrap();
    let track_data = track_data.get(track_id).unwrap_or(&DEFAULT_TRACK_DATA);
    track_data.clone()
}
