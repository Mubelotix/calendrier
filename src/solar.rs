
const UNIX_EPOCH_JD: f64 = 2440587.5;

// See https://eclipse.gsfc.nasa.gov/SEcat5/deltatpoly.html
pub fn delta_t(month: usize, year: isize) -> f64 {
    let y = year as f64 + (month as f64 - 0.5) / 12.0;

    match y {
        ..-500.0 => {
            let u = (y - 1820.0) / 100.0;
            -20.0 + 32.0 * u.powi(2)
        },
        -500.0..500.0 => {
            let u = y / 100.0;
            10583.6 - 1014.41 * u + 33.78311 * u * u - 5.952053 * u.powi(3) - 0.1798452 * u.powi(4) + 0.022174192 * u.powi(5) + 0.0090316521 * u.powi(6)
        },
        500.0..1600.0 => {
            let u = (y - 1000.0) / 100.0;
            1574.2 - 556.01 * u + 71.23472 * u.powi(2) + 0.319781 * u.powi(3) - 0.8503463 * u.powi(4) - 0.005050998 * u.powi(5) + 0.0083572073 * u.powi(6)
        },
        1600.0..1700.0 => {
            let t = y - 1600.0;
            120.0 - 0.9808 * t - 0.01532 * t.powi(2) + t.powi(3) / 7129.0
        },
        1700.0..1800.0 => {
            let t = y - 1700.0;
            8.83 + 0.1603 * t - 0.0059285 * t.powi(2) + 0.00013336 * t.powi(3) - t.powi(4) / 1174000.0
        },
        1800.0..1860.0 => {
            let t = y - 1800.0;
            13.72 - 0.332447 * t + 0.0068612 * t.powi(2) + 0.0041116 * t.powi(3) - 0.00037436 * t.powi(4) + 0.0000121272 * t.powi(5) - 0.0000001699 * t.powi(6) + 0.000000000875 * t.powi(7)
        },
        1860.0..1900.0 => {
            let t = y - 1860.0;
            7.62 + 0.5737 * t - 0.251754 * t.powi(2) + 0.01680668 * t.powi(3) -0.0004473624 * t.powi(4) + t.powi(5) / 233174.0
        },
        1900.0..1920.0 => {
            let t = y - 1900.0;
            -2.79 + 1.494119 * t - 0.0598939 * t.powi(2) + 0.0061966 * t.powi(3) - 0.000197 * t.powi(4)
        },
        1920.0..1941.0 => {
            let t = y - 1920.0;
            21.20 + 0.84493*t - 0.076100 * t.powi(2) + 0.0020936 * t.powi(3)
        },
        1941.0..1961.0 => {
            let t = y - 1950.0;
            29.07 + 0.407*t - t.powi(2) / 233.0 + t.powi(3) / 2547.0
        },
        1961.0..1986.0 => {
            let t = y - 1975.0;
            45.45 + 1.067*t - t.powi(2) / 260.0 - t.powi(3) / 718.0
        },
        1986.0..2005.0 => {
            let t = y - 2000.0;
            63.86 + 0.3345 * t - 0.060374 * t.powi(2) + 0.0017275 * t.powi(3) + 0.000651814 * t.powi(4) + 0.00002373599 * t.powi(5)
        },
        2005.0..2050.0 => {
            let t = y - 2000.0;
            62.92 + 0.32217 * t + 0.005589 * t.powi(2)
        }
        2050.0..2150.0 => {
            -20.0 + 32.0 * ((y-1820.0)/100.0).powi(2) - 0.5628 * (2150.0 - y)
        },
        _ => {
            let u = (y - 1820.0) / 100.0;
            -20.0 + 32.0 * u.powi(2)
        },
    }
}

/// Converts a standard Unix timestamp (Mean Time) into a "Solar" Unix timestamp 
/// (Apparent Time) by applying the Equation of Time correction.
pub fn get_solar_unix_timestamp(unix_timestamp: i64) -> f64 {
    let seconds_since_epoch = unix_timestamp as f64;
    let jd_utc = UNIX_EPOCH_JD + seconds_since_epoch / 86400.0;

    // 1. Get the Calendar Date to calculate dynamic Delta T
    // We can use NOVAS's own cal_date to get year/month from JD
    let mut year = 0;
    let mut month = 0;
    let mut day = 0;
    let mut hour = 0.0;
    unsafe {
        novas::cal_date(jd_utc, &mut year, &mut month, &mut day, &mut hour);
    }

    // 2. Calculate Delta T using your polynomial function
    let dt = delta_t(month as usize, year as isize);

    // 3. Set up Julian Dates for NOVAS
    let jd_ut1 = jd_utc; 
    let jd_tt = jd_utc + dt / 86400.0;
    let jd_high = jd_ut1.floor();
    let jd_low = jd_ut1 - jd_high;

// 4. Calculate Sun's Apparent Right Ascension
    let mut sun_ra = 0.0;
    let mut sun_dec = 0.0;
    let mut sun_dis = 0.0;

    let mut sun_obj = unsafe { core::mem::zeroed::<novas::object>() };
    sun_obj.type_ = 0;
    sun_obj.number = 10; // Sun

    // Attempt high-precision (requires ephemeris for dates outside 1900-2100)
    let sun_status = unsafe { 
        novas::app_planet(jd_tt, &mut sun_obj, 1, &mut sun_ra, &mut sun_dec, &mut sun_dis) 
    };

    if sun_status != 0 {
        // Fallback: Use internal analytic theory (valid for a much wider range, e.g., 1793)
        let mut sun_pos = [0.0f64; 3];
        let mut sun_vel = [0.0f64; 3];

        unsafe {
            // body: 0 = Sun, origin: 1 = Earth.
            novas::sys::solarsystem(jd_tt, 0, 1, sun_pos.as_mut_ptr(), sun_vel.as_mut_ptr());
            // Convert the position vector [x, y, z] back to RA/Dec
            novas::sys::vector2radec(sun_pos.as_mut_ptr(), &mut sun_ra, &mut sun_dec);
        }
    }

    // 5. Calculate Sidereal Time (GAST)
    let mut gast = 0.0;
    let sidereal_status = unsafe { 
        novas::sidereal_time(jd_high, jd_low, dt, 1, 1, 0, &mut gast) 
    };
    assert_eq!(sidereal_status, 0, "sidereal_time failed");

    // 6. Calculate Equation of Time (EoT)
    // Mean Solar Time (at Greenwich) is derived from UTC
    let utc_hours = (seconds_since_epoch / 3600.0).rem_euclid(24.0);
    
    // Apparent Solar Time (at Greenwich) = GAST - RA + 12h
    // GAST and RA are in hours.
    let apparent_solar_time = (gast - sun_ra + 12.0).rem_euclid(24.0);
    
    // EoT is the signed difference in hours
    let mut eot_hours = apparent_solar_time - utc_hours;
    
    // Normalize the difference to [-12, 12] range to handle the midnight wrap
    if eot_hours > 12.0 {
        eot_hours -= 24.0;
    } else if eot_hours < -12.0 {
        eot_hours += 24.0;
    }

    // 7. Return corrected Unix timestamp
    seconds_since_epoch + (eot_hours * 3600.0)
}

pub fn get_unix_from_solar_timestamp(solar_timestamp: f64) -> i64 {
    // Initial guess: assume EoT is zero, so Mean Time = Solar Time
    let mut estimate_unix_f64 = solar_timestamp;
    
    // Iteratively refine the estimate
    for _ in 0..3 {
        // Calculate what the solar time would be for our current estimate
        let solar_at_estimate = get_solar_unix_timestamp(estimate_unix_f64 as i64);
        
        // Calculate the error (difference)
        let error = solar_at_estimate - solar_timestamp;
        
        // Adjust the estimate to close the gap
        estimate_unix_f64 -= error;
    }

    estimate_unix_f64.round() as i64
}

/// Returns the length of a solar second in SI seconds.
/// A value of 1.0001 means a solar second is slightly longer than an SI second.
pub fn get_solar_time_speed_ratio(unix_timestamp: i64, window: usize) -> f64 {
    // 1. Calculate the solar timestamp at the current second
    let t1 = get_solar_unix_timestamp(unix_timestamp);
    
    // 2. Calculate the solar timestamp exactly one second later
    let t2 = get_solar_unix_timestamp(unix_timestamp + window as i64);
    
    // 3. The difference is the "speed" or ratio
    // If solar time moved exactly with Unix time, this would be 1.0.
    (t2 - t1) / (window as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_conversion() {
        let random_timestamps = [
            0, // Unix epoch
            1776724770, // Random timestamp in 2026
            -5364662400, // Random timestamp in 1800
            4102444800, // Random timestamp in 2100
        ];
        for timestamp in random_timestamps {
            let solar_timestamp = get_solar_unix_timestamp(timestamp);
            let converted_back = get_unix_from_solar_timestamp(solar_timestamp);
            assert!((converted_back - timestamp).abs() <= 1, "Conversion should be accurate within 2 seconds for timestamp {timestamp}, got {converted_back}");
        }
    }

    #[test]
    fn test_solar_time_speed_ratio() {
        use chrono::TimeZone;

        // Plot the values by pasting into sheets at C4 and computing C5=-C4*10000+10000
        // Compare with the reference: https://en.wikipedia.org/wiki/Equation_of_time#:~:text=Derivative%20of%20%E2%88%92%CE%94t
        let base = chrono::Utc.with_ymd_and_hms(2000, 3, 1, 0, 0, 0).single().unwrap();
        for i in 0..=365 {
            let date = base + chrono::Duration::days(i);
            let unix_timestamp = date.timestamp();
            let ratio = get_solar_time_speed_ratio(unix_timestamp, 3600);
            println!("{ratio:.7}");
        }
    }

    #[test]
    fn test_solar_speed_stability() {
        let unix_timestamp = 1776724768;

        for i in 0..500 {
            let ratio = get_solar_time_speed_ratio(unix_timestamp + i, 3600);
            assert!((ratio - 1.0001412).abs() < 0.0000001, "Solar speed ratio should be stable around 1.0001412 (found {ratio}) at timestamp {unix_timestamp} with offset {i}");
        }
    }

}
