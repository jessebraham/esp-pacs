#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<ENABLE_SPEC>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<ENABLE_SPEC>;
#[doc = "Field `REG_RTCIO_REG_GPIO_ENABLE` reader - GPIO0 ~ 21 output enable. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. If the bit is set to 1, it means this GPIO pad is output."]
pub type REG_RTCIO_REG_GPIO_ENABLE_R = crate::FieldReader<u32>;
#[doc = "Field `REG_RTCIO_REG_GPIO_ENABLE` writer - GPIO0 ~ 21 output enable. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. If the bit is set to 1, it means this GPIO pad is output."]
pub type REG_RTCIO_REG_GPIO_ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 output enable. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. If the bit is set to 1, it means this GPIO pad is output."]
    #[inline(always)]
    pub fn reg_rtcio_reg_gpio_enable(&self) -> REG_RTCIO_REG_GPIO_ENABLE_R {
        REG_RTCIO_REG_GPIO_ENABLE_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE")
            .field(
                "reg_rtcio_reg_gpio_enable",
                &self.reg_rtcio_reg_gpio_enable(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 output enable. Bit10 corresponds to GPIO0, bit11 corresponds to GPIO1, etc. If the bit is set to 1, it means this GPIO pad is output."]
    #[inline(always)]
    pub fn reg_rtcio_reg_gpio_enable(&mut self) -> REG_RTCIO_REG_GPIO_ENABLE_W<ENABLE_SPEC> {
        REG_RTCIO_REG_GPIO_ENABLE_W::new(self, 10)
    }
}
#[doc = "RTC GPIO output enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for ENABLE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for ENABLE_SPEC {}
