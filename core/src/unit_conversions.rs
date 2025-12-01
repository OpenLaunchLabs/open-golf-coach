use crate::vector::Vector3;

pub const METERS_TO_YARDS: f64 = 1.093_613_3;
pub const YARDS_TO_METERS: f64 = 1.0 / METERS_TO_YARDS;
pub const MPS_TO_MPH: f64 = 2.236_936_29;
pub const MPH_TO_MPS: f64 = 1.0 / MPS_TO_MPH;

#[inline]
pub fn meters_to_yards(value: f64) -> f64 {
    value * METERS_TO_YARDS
}

#[inline]
pub fn yards_to_meters(value: f64) -> f64 {
    value * YARDS_TO_METERS
}

#[inline]
pub fn meters_per_second_to_mph(value: f64) -> f64 {
    value * MPS_TO_MPH
}

#[inline]
pub fn mph_to_meters_per_second(value: f64) -> f64 {
    value * MPH_TO_MPS
}

#[inline]
pub fn vector_meters_to_yards(vec: &Vector3) -> Vector3 {
    Vector3::new(
        meters_to_yards(vec.x),
        meters_to_yards(vec.y),
        meters_to_yards(vec.z),
    )
}

#[inline]
pub fn vector_yards_to_meters(vec: &Vector3) -> Vector3 {
    Vector3::new(
        yards_to_meters(vec.x),
        yards_to_meters(vec.y),
        yards_to_meters(vec.z),
    )
}

#[inline]
pub fn vector_mps_to_mph(vec: &Vector3) -> Vector3 {
    Vector3::new(
        meters_per_second_to_mph(vec.x),
        meters_per_second_to_mph(vec.y),
        meters_per_second_to_mph(vec.z),
    )
}

#[inline]
pub fn vector_mph_to_mps(vec: &Vector3) -> Vector3 {
    Vector3::new(
        mph_to_meters_per_second(vec.x),
        mph_to_meters_per_second(vec.y),
        mph_to_meters_per_second(vec.z),
    )
}
