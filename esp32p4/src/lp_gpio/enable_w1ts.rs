#[doc = "Register `ENABLE_W1TS` writer"]
pub type W = crate::W<ENABLE_W1TS_SPEC>;
#[doc = "Field `REG_GPIO_ENABLE_DATA_W1TS` writer - Reserved"]
pub type REG_GPIO_ENABLE_DATA_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_enable_data_w1ts(&mut self) -> REG_GPIO_ENABLE_DATA_W1TS_W<ENABLE_W1TS_SPEC> {
        REG_GPIO_ENABLE_DATA_W1TS_W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE_W1TS_SPEC;
impl crate::RegisterSpec for ENABLE_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enable_w1ts::W`](W) writer structure"]
impl crate::Writable for ENABLE_W1TS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE_W1TS to value 0"]
impl crate::Resettable for ENABLE_W1TS_SPEC {}
