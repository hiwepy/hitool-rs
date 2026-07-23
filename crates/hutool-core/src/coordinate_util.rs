use std::fmt;
use std::hash::{Hash, Hasher};

/// A longitude/latitude pair used by Hutool-compatible coordinate conversions.
#[derive(Debug, Clone, Copy, Default)]
pub struct Coordinate {
    lng: f64,
    lat: f64,
}

impl Coordinate {
    /// Creates a coordinate from longitude and latitude.
    #[must_use]
    pub const fn new(lng: f64, lat: f64) -> Self {
        Self { lng, lat }
    }

    /// Returns the longitude.
    #[must_use]
    pub const fn lng(self) -> f64 {
        self.lng
    }

    /// Changes the longitude and returns this coordinate for chaining.
    pub fn set_lng(&mut self, lng: f64) -> &mut Self {
        self.lng = lng;
        self
    }

    /// Returns the latitude.
    #[must_use]
    pub const fn lat(self) -> f64 {
        self.lat
    }

    /// Changes the latitude and returns this coordinate for chaining.
    pub fn set_lat(&mut self, lat: f64) -> &mut Self {
        self.lat = lat;
        self
    }

    /// Applies a longitude/latitude offset and returns this coordinate for chaining.
    pub fn offset(&mut self, offset: Self) -> &mut Self {
        self.lng += offset.lng;
        self.lat += offset.lat;
        self
    }

    /// Returns the value produced by Java's `Objects.hash(lng, lat)`.
    #[must_use]
    pub fn hash_code(self) -> i32 {
        let lng = double_hash_code(self.lng);
        let lat = double_hash_code(self.lat);
        31_i32
            .wrapping_mul(31_i32.wrapping_add(lng))
            .wrapping_add(lat)
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        java_double_bits(self.lng) == java_double_bits(other.lng)
            && java_double_bits(self.lat) == java_double_bits(other.lat)
    }
}

impl Eq for Coordinate {}

impl Hash for Coordinate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        java_double_bits(self.lng).hash(state);
        java_double_bits(self.lat).hash(state);
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Coordinate{{lng={}, lat={}}}",
            JavaDouble(self.lng),
            JavaDouble(self.lat)
        )
    }
}

struct JavaDouble(f64);

impl fmt::Display for JavaDouble {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.0;
        if value.is_nan() {
            return formatter.write_str("NaN");
        }
        if value == f64::INFINITY {
            return formatter.write_str("Infinity");
        }
        if value == f64::NEG_INFINITY {
            return formatter.write_str("-Infinity");
        }

        let absolute = value.abs();
        if value != 0.0 && !(0.001..10_000_000.0).contains(&absolute) {
            let scientific = format!("{value:e}");
            let (mantissa, exponent) = scientific
                .split_once('e')
                .expect("Rust scientific float formatting always contains an exponent");
            if mantissa.contains('.') {
                write!(formatter, "{mantissa}E{exponent}")
            } else {
                write!(formatter, "{mantissa}.0E{exponent}")
            }
        } else if value.fract() == 0.0 {
            write!(formatter, "{value:.1}")
        } else {
            write!(formatter, "{value}")
        }
    }
}

fn java_double_bits(value: f64) -> u64 {
    if value.is_nan() {
        0x7ff8_0000_0000_0000
    } else {
        value.to_bits()
    }
}

fn double_hash_code(value: f64) -> i32 {
    let bits = java_double_bits(value);
    let folded = (bits ^ (bits >> 32)).to_le_bytes();
    i32::from_le_bytes([folded[0], folded[1], folded[2], folded[3]])
}

/// Hutool-compatible WGS84, GCJ-02, BD-09 and Web Mercator conversions.
pub struct CoordinateUtil;

impl CoordinateUtil {
    /// GCJ-02/BD-09 intermediate conversion parameter.
    pub const X_PI: f64 = std::f64::consts::PI * 3000.0 / 180.0;
    /// Pi used by Hutool's formulas.
    pub const PI: f64 = std::f64::consts::PI;
    /// Krasovsky 1940 earth radius.
    pub const RADIUS: f64 = 6_378_245.0;
    /// Krasovsky eccentricity correction parameter.
    pub const CORRECTION_PARAM: f64 = 0.006_693_421_622_965_943;

    /// Returns true when GCJ-02 is not defined for the coordinate.
    #[must_use]
    pub fn out_of_china(lng: f64, lat: f64) -> bool {
        !(72.004..=137.8347).contains(&lng) || !(0.8293..=55.8271).contains(&lat)
    }

    /// Converts WGS84 to GCJ-02 using Hutool's approximate offset algorithm.
    #[must_use]
    pub fn wgs84_to_gcj02(lng: f64, lat: f64) -> Coordinate {
        let mut coordinate = Coordinate::new(lng, lat);
        coordinate.offset(offset(lng, lat, true));
        coordinate
    }

    /// Converts WGS84 to BD-09 through GCJ-02.
    #[must_use]
    pub fn wgs84_to_bd09(lng: f64, lat: f64) -> Coordinate {
        let gcj02 = Self::wgs84_to_gcj02(lng, lat);
        Self::gcj02_to_bd09(gcj02.lng, gcj02.lat)
    }

    /// Converts GCJ-02 to WGS84 using Hutool's approximate inverse offset.
    #[must_use]
    pub fn gcj02_to_wgs84(lng: f64, lat: f64) -> Coordinate {
        let mut coordinate = Coordinate::new(lng, lat);
        coordinate.offset(offset(lng, lat, false));
        coordinate
    }

    /// Converts GCJ-02 to BD-09.
    #[must_use]
    pub fn gcj02_to_bd09(lng: f64, lat: f64) -> Coordinate {
        let z = (lng * lng + lat * lat).sqrt() + 0.00002 * (lat * Self::X_PI).sin();
        let theta = lat.atan2(lng) + 0.000_003 * (lng * Self::X_PI).cos();
        Coordinate::new(z * theta.cos() + 0.0065, z * theta.sin() + 0.006)
    }

    /// Converts BD-09 to GCJ-02.
    #[must_use]
    pub fn bd09_to_gcj02(lng: f64, lat: f64) -> Coordinate {
        let x = lng - 0.0065;
        let y = lat - 0.006;
        let z = (x * x + y * y).sqrt() - 0.00002 * (y * Self::X_PI).sin();
        let theta = y.atan2(x) - 0.000_003 * (x * Self::X_PI).cos();
        Coordinate::new(z * theta.cos(), z * theta.sin())
    }

    /// Converts BD-09 to WGS84 through GCJ-02.
    #[must_use]
    pub fn bd09_to_wgs84(lng: f64, lat: f64) -> Coordinate {
        let gcj02 = Self::bd09_to_gcj02(lng, lat);
        Self::gcj02_to_wgs84(gcj02.lng, gcj02.lat)
    }

    /// Projects WGS84 longitude/latitude to Web Mercator.
    #[must_use]
    pub fn wgs84_to_mercator(lng: f64, lat: f64) -> Coordinate {
        let x = lng * 20_037_508.342_789_244 / 180.0;
        let y = ((90.0 + lat) * Self::PI / 360.0).tan().ln() / (Self::PI / 180.0);
        Coordinate::new(x, y * 20_037_508.342_789_244 / 180.0)
    }

    /// Converts Web Mercator coordinates back to WGS84.
    #[must_use]
    pub fn mercator_to_wgs84(mercator_x: f64, mercator_y: f64) -> Coordinate {
        let x = mercator_x / 20_037_508.342_789_244 * 180.0;
        let y = mercator_y / 20_037_508.342_789_244 * 180.0;
        let y = 180.0 / Self::PI * (2.0 * (y * Self::PI / 180.0).exp().atan() - Self::PI / 2.0);
        Coordinate::new(x, y)
    }
}

fn offset(lng: f64, lat: f64, is_plus: bool) -> Coordinate {
    let mut dlng = trans_lng(lng - 105.0, lat - 35.0);
    let mut dlat = trans_lat(lng - 105.0, lat - 35.0);
    let mut magic = (lat / 180.0 * CoordinateUtil::PI).sin();
    magic = 1.0 - CoordinateUtil::CORRECTION_PARAM * magic * magic;
    let sqrt_magic = magic.sqrt();
    dlng = dlng * 180.0
        / (CoordinateUtil::RADIUS / sqrt_magic
            * (lat / 180.0 * CoordinateUtil::PI).cos()
            * CoordinateUtil::PI);
    dlat = dlat * 180.0
        / (CoordinateUtil::RADIUS * (1.0 - CoordinateUtil::CORRECTION_PARAM)
            / (magic * sqrt_magic)
            * CoordinateUtil::PI);
    if !is_plus {
        dlng = -dlng;
        dlat = -dlat;
    }
    Coordinate::new(dlng, dlat)
}

fn trans_lng(lng: f64, lat: f64) -> f64 {
    let mut result =
        300.0 + lng + 2.0 * lat + 0.1 * lng * lng + 0.1 * lng * lat + 0.1 * lng.abs().sqrt();
    result += (20.0 * (6.0 * lng * CoordinateUtil::PI).sin()
        + 20.0 * (2.0 * lng * CoordinateUtil::PI).sin())
        * 2.0
        / 3.0;
    result += (20.0 * (lng * CoordinateUtil::PI).sin()
        + 40.0 * (lng / 3.0 * CoordinateUtil::PI).sin())
        * 2.0
        / 3.0;
    result += (150.0 * (lng / 12.0 * CoordinateUtil::PI).sin()
        + 300.0 * (lng / 30.0 * CoordinateUtil::PI).sin())
        * 2.0
        / 3.0;
    result
}

fn trans_lat(lng: f64, lat: f64) -> f64 {
    let mut result =
        -100.0 + 2.0 * lng + 3.0 * lat + 0.2 * lat * lat + 0.1 * lng * lat + 0.2 * lng.abs().sqrt();
    result += (20.0 * (6.0 * lng * CoordinateUtil::PI).sin()
        + 20.0 * (2.0 * lng * CoordinateUtil::PI).sin())
        * 2.0
        / 3.0;
    result += (20.0 * (lat * CoordinateUtil::PI).sin()
        + 40.0 * (lat / 3.0 * CoordinateUtil::PI).sin())
        * 2.0
        / 3.0;
    result += (160.0 * (lat / 12.0 * CoordinateUtil::PI).sin()
        + 320.0 * (lat * CoordinateUtil::PI / 30.0).sin())
        * 2.0
        / 3.0;
    result
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;

    use super::*;

    fn close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "{actual} != {expected}"
        );
    }

    #[test]
    fn conversions_match_hutool_reference_vectors_and_round_trip_mercator() {
        let gcj = CoordinateUtil::wgs84_to_gcj02(116.404, 39.915);
        close(gcj.lng(), 116.410_244_499_169_38, 1e-14);
        close(gcj.lat(), 39.916_404_281_501_64, 1e-14);

        let wgs = CoordinateUtil::gcj02_to_wgs84(116.404, 39.915);
        close(wgs.lng(), 116.397_755_500_830_61, 1e-14);
        close(wgs.lat(), 39.913_595_718_498_36, 1e-14);

        let bd = CoordinateUtil::wgs84_to_bd09(122.993_955_97, 44.998_040_71);
        close(bd.lng(), 123.006_365_160_288_85, 1e-13);
        close(bd.lat(), 45.006_369_091_895_89, 1e-13);

        let bd = CoordinateUtil::gcj02_to_bd09(116.404, 39.915);
        close(bd.lng(), 116.410_369_493_710_29, 1e-14);
        close(bd.lat(), 39.921_336_993_510_22, 1e-14);
        let gcj = CoordinateUtil::bd09_to_gcj02(116.404, 39.915);
        close(gcj.lng(), 116.397_627_291_193_15, 1e-14);
        close(gcj.lat(), 39.908_656_739_576_31, 1e-14);
        let wgs = CoordinateUtil::bd09_to_wgs84(116.404, 39.915);
        close(wgs.lng(), 116.391_383_699_512_5, 1e-14);
        close(wgs.lat(), 39.907_253_214_522_164, 1e-14);

        let source = Coordinate::new(12.345, 45.678);
        let restored = CoordinateUtil::mercator_to_wgs84(
            CoordinateUtil::wgs84_to_mercator(source.lng(), source.lat()).lng(),
            CoordinateUtil::wgs84_to_mercator(source.lng(), source.lat()).lat(),
        );
        close(restored.lng(), source.lng(), 1e-12);
        close(restored.lat(), source.lat(), 1e-12);
    }

    #[test]
    fn boundaries_mutation_equality_hash_and_display_are_real_value_semantics() {
        assert!(!CoordinateUtil::out_of_china(72.004, 0.8293));
        assert!(!CoordinateUtil::out_of_china(137.8347, 55.8271));
        assert!(CoordinateUtil::out_of_china(72.0039, 30.0));
        assert!(CoordinateUtil::out_of_china(120.0, 55.8272));

        let mut coordinate = Coordinate::new(1.0, 2.0);
        coordinate
            .set_lng(3.0)
            .set_lat(4.0)
            .offset(Coordinate::new(0.5, -0.5));
        assert_eq!(coordinate, Coordinate::new(3.5, 3.5));
        assert_eq!(coordinate.to_string(), "Coordinate{lng=3.5, lat=3.5}");
        assert_eq!(
            Coordinate::new(1.0, 2.0).to_string(),
            "Coordinate{lng=1.0, lat=2.0}"
        );
        assert_eq!(
            Coordinate::new(f64::INFINITY, f64::NEG_INFINITY).to_string(),
            "Coordinate{lng=Infinity, lat=-Infinity}"
        );
        assert_eq!(
            Coordinate::new(10_000_000.0, 0.000_1).to_string(),
            "Coordinate{lng=1.0E7, lat=1.0E-4}"
        );
        assert_eq!(
            Coordinate::new(12_345_678.9, 0.001).to_string(),
            "Coordinate{lng=1.23456789E7, lat=0.001}"
        );
        assert_eq!(
            Coordinate::new(f64::NAN, -0.0).to_string(),
            "Coordinate{lng=NaN, lat=-0.0}"
        );
        assert_ne!(Coordinate::new(0.0, 0.0), Coordinate::new(-0.0, 0.0));
        assert_eq!(
            Coordinate::new(f64::NAN, 0.0),
            Coordinate::new(f64::NAN, 0.0)
        );
        assert_eq!(Coordinate::new(1.0, 2.0).hash_code(), -32_504_895);

        let mut first = DefaultHasher::new();
        let mut second = DefaultHasher::new();
        Coordinate::new(f64::NAN, 0.0).hash(&mut first);
        Coordinate::new(f64::NAN, 0.0).hash(&mut second);
        assert_eq!(first.finish(), second.finish());
    }
}
