//! coordinate_util parity tests
//! 对齐: hutool-core CoordinateUtilTest

use hitool_core::{Coordinate, CoordinateUtil};

#[test]
fn coordinate_new() {
    let c = Coordinate::new(116.4074, 39.9042);
    assert_eq!(c.lng(), 116.4074);
    assert_eq!(c.lat(), 39.9042);
}

#[test]
fn coordinate_set_lng_lat() {
    let mut c = Coordinate::new(0.0, 0.0);
    c.set_lng(116.4074).set_lat(39.9042);
    assert_eq!(c.lng(), 116.4074);
    assert_eq!(c.lat(), 39.9042);
}

#[test]
fn coordinate_offset() {
    let mut c = Coordinate::new(116.4074, 39.9042);
    let offset = Coordinate::new(0.001, 0.001);
    c.offset(offset);
    assert!((c.lng() - 116.4084).abs() < 0.0001);
    assert!((c.lat() - 39.9052).abs() < 0.0001);
}

#[test]
fn coordinate_hash_code() {
    let c = Coordinate::new(116.4074, 39.9042);
    let h = c.hash_code();
    assert_ne!(h, 0);
}

#[test]
fn out_of_china_beijing() {
    assert!(!CoordinateUtil::out_of_china(116.4074, 39.9042));
}

#[test]
fn out_of_china_london() {
    assert!(CoordinateUtil::out_of_china(-0.1278, 51.5074));
}

#[test]
fn wgs84_to_gcj02_roundtrip() {
    let gcj = CoordinateUtil::wgs84_to_gcj02(116.4074, 39.9042);
    let wgs = CoordinateUtil::gcj02_to_wgs84(gcj.lng(), gcj.lat());
    assert!((wgs.lng() - 116.4074).abs() < 0.01);
    assert!((wgs.lat() - 39.9042).abs() < 0.01);
}

#[test]
fn wgs84_to_bd09_roundtrip() {
    let bd = CoordinateUtil::wgs84_to_bd09(116.4074, 39.9042);
    let wgs = CoordinateUtil::bd09_to_wgs84(bd.lng(), bd.lat());
    assert!((wgs.lng() - 116.4074).abs() < 0.01);
    assert!((wgs.lat() - 39.9042).abs() < 0.01);
}

#[test]
fn gcj02_to_bd09_roundtrip() {
    let gcj = Coordinate::new(116.4074, 39.9042);
    let bd = CoordinateUtil::gcj02_to_bd09(gcj.lng(), gcj.lat());
    let gcj2 = CoordinateUtil::bd09_to_gcj02(bd.lng(), bd.lat());
    assert!((gcj2.lng() - gcj.lng()).abs() < 0.0001);
    assert!((gcj2.lat() - gcj.lat()).abs() < 0.0001);
}

#[test]
fn wgs84_to_mercator_roundtrip() {
    let mercator = CoordinateUtil::wgs84_to_mercator(116.4074, 39.9042);
    let wgs = CoordinateUtil::mercator_to_wgs84(mercator.lng(), mercator.lat());
    assert!((wgs.lng() - 116.4074).abs() < 0.01);
    assert!((wgs.lat() - 39.9042).abs() < 0.01);
}

#[test]
fn out_of_china_southern_border() {
    assert!(!CoordinateUtil::out_of_china(108.0, 0.83));
    assert!(CoordinateUtil::out_of_china(108.0, 0.82));
}

#[test]
fn out_of_china_northern_border() {
    assert!(!CoordinateUtil::out_of_china(108.0, 55.82));
    assert!(CoordinateUtil::out_of_china(108.0, 55.83));
}
