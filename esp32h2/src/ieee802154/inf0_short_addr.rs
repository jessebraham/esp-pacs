#[doc = "Register `INF0_SHORT_ADDR` reader"]
pub type R = crate::R<INF0_SHORT_ADDR_SPEC>;
#[doc = "Register `INF0_SHORT_ADDR` writer"]
pub type W = crate::W<INF0_SHORT_ADDR_SPEC>;
#[doc = "Field `MAC_INF0_SHORT_ADDR` reader - "]
pub type MAC_INF0_SHORT_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `MAC_INF0_SHORT_ADDR` writer - "]
pub type MAC_INF0_SHORT_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mac_inf0_short_addr(&self) -> MAC_INF0_SHORT_ADDR_R {
        MAC_INF0_SHORT_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INF0_SHORT_ADDR")
            .field("mac_inf0_short_addr", &self.mac_inf0_short_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mac_inf0_short_addr(&mut self) -> MAC_INF0_SHORT_ADDR_W<INF0_SHORT_ADDR_SPEC> {
        MAC_INF0_SHORT_ADDR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`inf0_short_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inf0_short_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INF0_SHORT_ADDR_SPEC;
impl crate::RegisterSpec for INF0_SHORT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inf0_short_addr::R`](R) reader structure"]
impl crate::Readable for INF0_SHORT_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inf0_short_addr::W`](W) writer structure"]
impl crate::Writable for INF0_SHORT_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INF0_SHORT_ADDR to value 0"]
impl crate::Resettable for INF0_SHORT_ADDR_SPEC {}
