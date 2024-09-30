use core::time::Duration;

#[cfg(feature = "M")]
#[allow(unused)]
#[inline]
pub fn read_mtime(map_addr: u64) -> u64 {
    let addr = map_addr as *mut u64;
    unsafe { addr.read_volatile() }
}
#[cfg(feature = "M")]
#[allow(unused)]
#[inline]
pub fn set_mtime(map_addr: u64, tick: u64) {
    let addr = map_addr as *mut u64;
    unsafe {
        addr.write_volatile(tick);
    }
}
#[cfg(feature = "M")]
#[allow(unused)]
#[inline]
pub fn read_mtimecmp(map_addr: u64) -> u64 {
    let addr = map_addr as *mut u64;
    unsafe { addr.read_volatile() }
}
#[cfg(feature = "M")]
#[allow(unused)]
#[inline]
pub fn set_mtimecmp(map_addr: u64, tick: u64) {
    let addr = map_addr as *mut u64;
    unsafe {
        addr.write_volatile(tick);
    }
}

/// Clock ticks turns into time quota
#[inline(always)]
#[allow(unused)]
pub const fn tick_to_nanos(tick: u64, time_base: u64) -> Duration {
    Duration::from_nanos(tick.saturating_div(time_base).saturating_mul(1000000000))
}

#[inline(always)]
#[allow(unused)]
pub const fn tick_to_micros(tick: u64, time_base: u64) -> Duration {
    Duration::from_nanos(tick.saturating_div(time_base).saturating_mul(1000000))
}

#[inline(always)]
#[allow(unused)]
pub const fn tick_to_millis(tick: u64, time_base: u64) -> Duration {
    Duration::from_nanos(tick.saturating_div(time_base).saturating_mul(1000))
}

#[inline(always)]
#[allow(unused)]
pub const fn tick_to_secs(tick: u64, time_base: u64) -> Duration {
    Duration::from_nanos(tick.saturating_div(time_base))
}

/// Time quota turns into clock ticks
#[inline(always)]
#[allow(unused)]
pub const fn nanos_to_tick(nanos: u64, time_base: u64) -> u64 {
    nanos.saturating_mul(time_base).saturating_div(1000000000)
}

#[inline(always)]
#[allow(unused)]
pub const fn micros_to_tick(micros: u64, time_base: u64) -> u64 {
    micros.saturating_mul(time_base).saturating_div(1000000)
}

#[inline(always)]
#[allow(unused)]
pub const fn millis_to_tick(millis: u64, time_base: u64) -> u64 {
    millis.saturating_mul(time_base).saturating_div(1000)
}

#[inline(always)]
#[allow(unused)]
pub const fn secs_to_tick(secs: u64, time_base: u64) -> u64 {
    secs.saturating_mul(time_base)
}
