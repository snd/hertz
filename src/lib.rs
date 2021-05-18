/*!

**useful functions for working with frame-rates, sample-rates,
other rates,
time durations,
frequencies, etc and for keeping a constant framerate.**

`rate`, `sample rate`, `frame rate`, `fps`, `frequency`, etc
express the same concept and are therefore used interchangeably.

you can use hertz to compute the time resolution
and frequency range that can be meaningfully analyzed by a
[short-time fourier transform](https://en.wikipedia.org/wiki/Short-time_Fourier_transform#Explanation)
of a signal:

```
extern crate hertz;

fn main() {
    let sample_rate = 44100;
    let window_size = 4096;
    let step_size = 512;

    // 11 hertz is the maximum frequency that can be meaningfully analyzed
    assert_eq!(
        11.,
        hertz::rayleigh(sample_rate as f64, window_size as f64).round());

    // 22050 hertz is the maximum frequency that can be meaningfully analyzed
    assert_eq!(
        22050.,
        hertz::nyquist(sample_rate as f64).round());

    // 12 ms is the time resolution we get when analyzing a 44100 hertz
    // signal with a step size of 512
    assert_eq!(
        12.,
        hertz::s_to_ms(hertz::cycles_per_second_to_seconds_per_cycle(
            sample_rate as f64,
            step_size as f64)).round());
}
```

you can use hertz to keep a constant framerate in a game or other
computer graphics application:

```no_run
fn main() {
    let frames_per_second: usize = 60;

    loop {
        let instant_at_frame_start = std::time::Instant::now();

        // here's where logic and rendering would go.
        // this is never called more than frames_per_second
        // times per second.

        hertz::sleep_for_constant_rate(
            frames_per_second, instant_at_frame_start);
    }
}
```
*/

#![cfg_attr(not(feature = "std"), no_std)]

use core::ops::{Mul, Div, Range};

/// nanoseconds to seconds
#[inline]
pub fn ns_to_s(ns: u64) -> f64 {
    (ns as f64) / 1_000_000_000.
}

/// seconds to nanoseconds
#[inline]
pub fn s_to_ns(s: f64) -> f64 {
    s * 1_000_000_000.
}

/// nanoseconds to milliseconds
#[inline]
pub fn ns_to_ms(ns: u64) -> f64 {
    (ns as f64) / 1_000_000.
}

/// seconds to milliseconds
#[inline]
pub fn s_to_ms<T>(s: T) -> T
    where T: Mul<T, Output=T> + From<u16>
{
    s * T::from(1000 as u16)
}

#[test]
fn test_s_to_ms() {
    assert_eq!(s_to_ms(1), 1000);
    assert_eq!(s_to_ms(1.), 1000.);
}

/// milliseconds to seconds
#[inline]
pub fn ms_to_s<T>(ms: T) -> T
    where T: Div<T, Output=T> + From<u16>
{
    ms / T::from(1000 as u16)
}

#[test]
fn test_ms_to_s() {
    assert_eq!(ms_to_s(1000), 1);
    assert_eq!(ms_to_s(1000.), 1.);
}

/// milliseconds to nanoseconds
#[inline]
pub fn ms_to_ns(ns: f64) -> f64 {
    ns * 1_000_000.
}

/// when given frames per second (or sample rate)
/// returns the duration of a single frame
#[inline]
pub fn fps_to_ns_per_frame(fps: usize) -> u64 {
    libm::round(s_to_ns(1.0) / (fps as f64)) as u64
}

// /// returns the nanoseconds of sleep which are needed to keep a constant
// /// frame rate
// pub fn ns_sleep_needed_for_constant_rate(fps: usize, ns_at_last_frame_start: u64) -> u64 {

/// useful for keeping a constant framerate
#[cfg(feature = "std")]
pub fn sleep_for_constant_rate(fps: usize, instant_at_last_frame_start: std::time::Instant) {
    let ns_per_frame = fps_to_ns_per_frame(fps);
    let frame_duration = std::time::Duration::new(ns_per_frame * 1000000000, (ns_per_frame % 1000000000) as u32);
    let elapsed = instant_at_last_frame_start.elapsed();
    if elapsed < frame_duration {
        std::thread::sleep(frame_duration - elapsed);
    }
}

/// frequency range that can be modeled when taking the short
/// time fourier transform of a signal with `sample_rate` with
/// a sliding window of `window_sizew`.
/// equivalent to `rayleigh(sample_rate, window_size)..nyquist(sample_rate)`
/// increase the window size to increase the lower frequency
/// increase the sample rate to increase the upper frequency
pub fn hertz_range<T>(sample_rate: T, window_size: T) -> Range<T>
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

/// maximum frequency in hertz that can be meaningfully analyzed with a given sample rate
/// [https://en.wikipedia.org/wiki/Short-time_Fourier_transform#Explanation](https://en.wikipedia.org/wiki/Short-time_Fourier_transform#Explanation)
pub fn nyquist<T>(sample_rate: T) -> T
    where T: Div<T, Output=T> + From<u16>
{
    sample_rate / T::from(2 as u16)
}

#[test]
fn test_nyquist() {
    assert_eq!(nyquist(44100.), 22050.);
}

/// minimum frequency in hertz that can be meaningfully analyzed with a given sample rate and
/// window size
/// [https://en.wikipedia.org/wiki/Short-time_Fourier_transform#Rayleigh_frequency](https://en.wikipedia.org/wiki/Short-time_Fourier_transform#Rayleigh_frequency)
pub fn rayleigh<T>(sample_rate: T, window_size: T) -> T
    where T: Div<T, Output=T> + From<u16>
{
    T::from(1 as u16) / cycles_per_second_to_seconds_per_cycle(sample_rate, window_size)
}

#[test]
fn test_rayleigh() {
    assert_eq!(rayleigh(44100., 1024.), 43.06640625);
}

/// window duration in seconds
/// sample_rate in hertz
pub fn cycles_per_second_to_seconds_per_cycle<T>(cycles_per_second: T, cycles: T) -> T
    where T: Div<T, Output=T>
{
    cycles / cycles_per_second
}

#[test]
fn test_seconds_per_window() {
    // 11ms time resolution
    assert_eq!(cycles_per_second_to_seconds_per_cycle(44100., 512.), 0.011609977324263039);
    assert_eq!(cycles_per_second_to_seconds_per_cycle(44100., 1024.), 0.023219954648526078);
}
