extern crate time;

use std::thread;

use std::ops::{Div, Range};

extern crate num;
use num::{One};

#[inline]
pub fn ns_to_s(ns: u64) -> f64 {
    (ns as f64) / 1_000_000_000.
}

/// seconds to nanoseconds
#[inline]
pub fn s_to_ns(s: f64) -> f64 {
    s * 1_000_000_000.
}

#[inline]
pub fn ns_to_ms(ns: u64) -> f64 {
    (ns as f64) / 1_000_000.
}

#[inline]
pub fn ms_to_ns(ns: f64) -> f64 {
    ns * 1_000_000.
}

#[inline]
pub fn fps_to_ns_per_frame(fps: usize) -> u64 {
    (s_to_ns(1.0) / (fps as f64)).round() as u64
}

#[inline]
pub fn current_time_ns() -> u64 {
    time::precise_time_ns()
}

pub fn sleep_for_constant_rate(fps: usize, ns_before: u64) {
    let ns_per_frame = fps_to_ns_per_frame(fps);
    let ns_after: u64 = time::precise_time_ns();
    let ns_elapsed = ns_after - ns_before;
    if ns_elapsed < ns_per_frame {
        let ns_left_to_sleep = ns_per_frame - ns_elapsed;
        thread::sleep_ms(ns_to_ms(ns_left_to_sleep) as u32);
    }
}

/// frequency range that can be modeled when taking the short
/// time fourier transform of a signal with `sample_rate` with
/// a sliding window of `window_sizew`.
/// equivalent to `rayleigh(sample_rate, window_size)..nyquist(sample_rate)`
/// increase the window size to increase the lower frequency
/// increase the sample rate to increase the upper frequency
fn hertz_range<T>(sample_rate: T, window_size: T) -> Range<T>
    where T: Div<T, Output=T> + From<u16> + Clone
{
    rayleigh(sample_rate.clone(), window_size)..nyquist(sample_rate)
}

#[test]
fn test_hertz_range() {
    assert_eq!(
        hertz_range(44100., 1024. * 8.),
        (5.38330078125)..22050.);
    assert_eq!(
        hertz_range(44100., 1024. * 4.),
        (10.7666015625)..22050.);
    assert_eq!(
        hertz_range(44100., 1024.),
        (43.06640625)..22050.);
    assert_eq!(
        hertz_range(44100., 512.),
        (86.1328125)..22050.);
}

/// maximum frequency in hertz that can be modeled with a given rate
fn nyquist<T>(sample_rate: T) -> T
    where T: Div<T, Output=T> + From<u16>
{
    sample_rate / T::from(2 as u16)
}

#[test]
fn test_nyquist() {
    assert_eq!(nyquist(44100.), 22050.);
}

/// minimum frequency in hertz
fn rayleigh<T>(sample_rate: T, window_size: T) -> T
    where T: Div<T, Output=T> + From<u16>
{
    T::from(1 as u16) / seconds_per_window(sample_rate, window_size)
}

#[test]
fn test_rayleigh() {
    assert_eq!(rayleigh(44100., 1024.), 43.06640625);
}

/// window duration in seconds
/// sample_rate in hertz
fn seconds_per_window<T>(samples_per_second: T, samples_per_window: T) -> T
    where T: Div<T, Output=T>
{
    samples_per_window / samples_per_second
}

#[test]
fn test_seconds_per_window() {
    // 11ms time resolution
    assert_eq!(seconds_per_window(44100., 512.), 0.011609977324263039);
    assert_eq!(seconds_per_window(44100., 1024.), 0.023219954648526078);
}
