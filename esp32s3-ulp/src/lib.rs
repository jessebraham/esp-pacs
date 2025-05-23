#![doc = "Peripheral access API for ESP32-S3-ULP microcontrollers (generated using svd2rust v0.35.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next] svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.35.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/46717278")]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn TOUCH_DONE_INT();
    fn TOUCH_INACTIVE_INT();
    fn TOUCH_ACTIVE_INT();
    fn SARADC1_DONE_INT();
    fn SARADC2_DONE_INT();
    fn TSENS_DONE_INT();
    fn RISCV_START_INT();
    fn SW_INT();
    fn SWD_INT();
    fn TOUCH_TIME_OUT_INT();
    fn TOUCH_APPROACH_LOOP_DONE_INT();
    fn TOUCH_SCAN_DONE_INT();
}
#[doc(hidden)]
#[repr(C)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    pub _reserved: usize,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".rwtext"]
#[no_mangle]
pub static __EXTERNAL_INTERRUPTS: [Vector; 12] = [
    Vector {
        _handler: TOUCH_DONE_INT,
    },
    Vector {
        _handler: TOUCH_INACTIVE_INT,
    },
    Vector {
        _handler: TOUCH_ACTIVE_INT,
    },
    Vector {
        _handler: SARADC1_DONE_INT,
    },
    Vector {
        _handler: SARADC2_DONE_INT,
    },
    Vector {
        _handler: TSENS_DONE_INT,
    },
    Vector {
        _handler: RISCV_START_INT,
    },
    Vector { _handler: SW_INT },
    Vector { _handler: SWD_INT },
    Vector {
        _handler: TOUCH_TIME_OUT_INT,
    },
    Vector {
        _handler: TOUCH_APPROACH_LOOP_DONE_INT,
    },
    Vector {
        _handler: TOUCH_SCAN_DONE_INT,
    },
];
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[doc = "Real-Time Clock Control"]
pub struct RTC_CNTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_CNTL {}
impl RTC_CNTL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc_cntl::RegisterBlock = 0x8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc_cntl::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for RTC_CNTL {
    type Target = rtc_cntl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC_CNTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_CNTL").finish()
    }
}
#[doc = "Real-Time Clock Control"]
pub mod rtc_cntl;
#[doc = "Low-power I2C (Inter-Integrated Circuit) Controller"]
pub struct RTC_I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_I2C {}
impl RTC_I2C {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc_i2c::RegisterBlock = 0xec00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc_i2c::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for RTC_I2C {
    type Target = rtc_i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC_I2C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_I2C").finish()
    }
}
#[doc = "Low-power I2C (Inter-Integrated Circuit) Controller"]
pub mod rtc_i2c;
#[doc = "Low-power Input/Output"]
pub struct RTC_IO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_IO {}
impl RTC_IO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc_io::RegisterBlock = 0xa400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc_io::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for RTC_IO {
    type Target = rtc_io::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC_IO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_IO").finish()
    }
}
#[doc = "Low-power Input/Output"]
pub mod rtc_io;
#[doc = "SENS Peripheral"]
pub struct SENS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SENS {}
impl SENS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sens::RegisterBlock = 0xc800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sens::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for SENS {
    type Target = sens::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SENS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SENS").finish()
    }
}
#[doc = "SENS Peripheral"]
pub mod sens;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "RTC_CNTL"]
    pub RTC_CNTL: RTC_CNTL,
    #[doc = "RTC_I2C"]
    pub RTC_I2C: RTC_I2C,
    #[doc = "RTC_IO"]
    pub RTC_IO: RTC_IO,
    #[doc = "SENS"]
    pub SENS: SENS,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            RTC_CNTL: RTC_CNTL::steal(),
            RTC_I2C: RTC_I2C::steal(),
            RTC_IO: RTC_IO::steal(),
            SENS: SENS::steal(),
        }
    }
}
