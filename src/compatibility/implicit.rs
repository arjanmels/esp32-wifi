#![allow(unused_variables)]

use crate::wprintln;
use cty::{c_char, c_int, c_uint, c_void};
use esp32_hal::units::*;

#[repr(C)]
#[derive(Debug)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

// Functions from esp-idf
#[no_mangle]
pub static WIFI_EVENT: StaticCString = StaticCString(b"WIFI_EVENT\0" as *const u8);

static mut INTERRUPT_MASK: u32 = 0;
//static mut PHY_SPINLOCK: spin::Mutex<()> = spin::Mutex::new(());
//static mut PHY_SPINLOCK_GUARD: Option<spin::MutexGuard<()>> = None;

#[no_mangle]
pub unsafe extern "C" fn phy_enter_critical() -> c_uint {
    wprintln!("phy_enter_critical()");

    INTERRUPT_MASK = xtensa_lx6::interrupt::disable();

    //    unimplemented!();
    // TODO: allow nesting on same core!
    //PHY_SPINLOCK_GUARD = Some(PHY_SPINLOCK.lock());
    INTERRUPT_MASK
}

#[no_mangle]
pub unsafe extern "C" fn phy_exit_critical(_level: c_uint) {
    wprintln!("phy_exit_critical({})", _level);

    //    unimplemented!();
    // TODO: allow nesting on same core!
    //PHY_SPINLOCK_GUARD = None;

    xtensa_lx6::interrupt::enable_mask(INTERRUPT_MASK);
}

#[no_mangle]
unsafe extern "C" fn phy_printf(fmt: *const c_char, ...) -> c_int {
    wprintln!(
        "phy_printf({})",
        cstr_core::CStr::from_ptr(fmt).to_str().unwrap()
    );
    1
    //    unimplemented!();
}

#[no_mangle]
unsafe extern "C" fn net80211_printf(fmt: *const c_char, ...) -> c_int {
    unimplemented!();
}

#[no_mangle]
unsafe extern "C" fn hexstr2bin(hex: *const c_char, buf: *const u8, len: usize) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn temprature_sens_read() -> u8 {
    // TODO: real temp sense read implementation
    75
    //unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn esp_dport_access_reg_read(mut reg: u32) -> u32 {
    wprintln!("esp_dport_access_reg_read({:x})", reg);
    // TODO: implement dport workaround

    let mut _apb: u32 = 0;
    let mut _int_lvl: u32 = 0;
    llvm_asm! (r#"
        rsil $2, 7
        movi $0, 0x3ff40078
        l32i $0, $0, 0
        l32i $1, $1, 0
        wsr  $2, PS
        rsync"#
         : "+r"(_apb), "+r"(reg), "+r"(_int_lvl):::"volatile");
    return reg;
    //unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn rtc_get_xtal() -> u32 {
    wprintln!("rtc_get_xtal()");

    esp32_hal::clock_control::ClockControlConfig {}.xtal_frequency() / Hertz(1_000_000)
}

// Functions available in ROM

#[no_mangle]
pub unsafe extern "C" fn roundup2(x: c_int, size: c_int) -> c_int {
    let res = (x + (size - 1)) & (-size);
    wprintln!("roundup2({}, {}) -> {}", x, size, res);
    res
}

#[no_mangle]
pub unsafe extern "C" fn __popcountsi2(x: c_int) -> c_uint {
    let res = x.count_ones();
    wprintln!("__popcountsi2({}) -> {}", x, res);
    res
}

#[no_mangle]
pub unsafe extern "C" fn gpio_output_set(
    set_mask: c_uint,
    clear_mask: c_uint,
    enable_mask: c_uint,
    disable_mask: c_uint,
) {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gpio_output_set_high(
    set_mask: c_uint,
    clear_mask: c_uint,
    enable_mask: c_uint,
    disable_mask: c_uint,
) {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn intr_matrix_set(cpu_no: c_int, model_num: c_uint, intr_num: c_uint) {
    wprintln!("intr_matrix_set({},{},{})", cpu_no, model_num, intr_num);
    // TODO: implement routine or refer to ROM
    //unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn ets_delay_us(us: c_uint) {
    wprintln!("ets_delay_us({})", us);
    //  let ticks = us.us() * esp32_hal::clock_control::ClockControlConfig {}.cpu_frequency();
    //  xtensa_lx6::timer::delay(ticks / Ticks(1));
}

#[no_mangle]
pub unsafe extern "C" fn phy_get_romfuncs() -> *const c_void {
    wprintln!("phy_get_romfuncs()");

    // Hardcoded phy_get_romfuncs address in ROM
    core::mem::transmute::<_, unsafe extern "C" fn() -> *const c_void>(0x40004100)()
}

// Functions from libc

#[no_mangle]
pub unsafe extern "C" fn strnlen(cs: *const c_char, maxlen: usize) -> usize {
    wprintln!("strnlen({:x}, {})", cs as u32, maxlen);

    for i in 0..maxlen {
        if *cs.add(i) == 0 {
            return i;
        }
    }
    return maxlen;
}

#[no_mangle]
pub unsafe extern "C" fn strlen(cs: *const c_char) -> usize {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn strncpy(dst: *mut c_char, src: *const c_char, n: usize) -> *mut c_char {
    wprintln!(
        "strncpy({:x}, {:x}, {}) -> {:x}",
        dst as u32,
        src as u32,
        n,
        dst as u32
    );

    for i in 0..n {
        if *dst.add(i) == 0 {
            for j in i..n {
                *dst.add(j) = 0;
            }
            break;
        }
        *dst.add(i) = *src.add(i);
    }

    dst
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *const c_void) {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn abort() -> ! {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn strncmp(cs: *const c_char, ct: *const c_char, n: usize) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int {
    wprintln!(
        "sprintf({:x}, {})",
        s as u32,
        cstr_core::CStr::from_ptr(format).to_str().unwrap()
    );

    //unimplemented!();
    let str = [0x44i8, 0x4d, 0x59, 0];
    strncpy(s, str.as_ptr(), 4);
    3
}

#[no_mangle]
pub unsafe extern "C" fn puts(a: *const c_char) -> c_int {
    wprintln!("{}", cstr_core::CStr::from_ptr(a).to_str().unwrap());
    true as c_int
}
