#[doc = "Register `RTC_GPIO_IN` reader"]
pub type R = crate::R<RTC_GPIO_IN_SPEC>;
#[doc = "Field `GPIO_IN_NEXT` reader - GPIO0 ~ 21 input value. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. Each bit represents a pad input value, 1 for high level, and 0 for low level."]
pub type GPIO_IN_NEXT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 input value. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. Each bit represents a pad input value, 1 for high level, and 0 for low level."]
    #[inline(always)]
    pub fn gpio_in_next(&self) -> GPIO_IN_NEXT_R {
        GPIO_IN_NEXT_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_GPIO_IN")
            .field("gpio_in_next", &self.gpio_in_next())
            .finish()
    }
}
#[doc = "RTC GPIO input register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_gpio_in::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_GPIO_IN_SPEC;
impl crate::RegisterSpec for RTC_GPIO_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_gpio_in::R`](R) reader structure"]
impl crate::Readable for RTC_GPIO_IN_SPEC {}
#[doc = "`reset()` method sets RTC_GPIO_IN to value 0"]
impl crate::Resettable for RTC_GPIO_IN_SPEC {}
