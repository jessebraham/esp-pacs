#[doc = "Register `MODEM_BUS` reader"]
pub type R = crate::R<MODEM_BUS_SPEC>;
#[doc = "Register `MODEM_BUS` writer"]
pub type W = crate::W<MODEM_BUS_SPEC>;
#[doc = "Field `MODEM_SYNC_BRIDGE_EN` reader - enable modem sync bridge or not 1: enable 0: disable"]
pub type MODEM_SYNC_BRIDGE_EN_R = crate::BitReader;
#[doc = "Field `MODEM_SYNC_BRIDGE_EN` writer - enable modem sync bridge or not 1: enable 0: disable"]
pub type MODEM_SYNC_BRIDGE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - enable modem sync bridge or not 1: enable 0: disable"]
    #[inline(always)]
    pub fn modem_sync_bridge_en(&self) -> MODEM_SYNC_BRIDGE_EN_R {
        MODEM_SYNC_BRIDGE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_BUS")
            .field("modem_sync_bridge_en", &self.modem_sync_bridge_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - enable modem sync bridge or not 1: enable 0: disable"]
    #[inline(always)]
    pub fn modem_sync_bridge_en(&mut self) -> MODEM_SYNC_BRIDGE_EN_W<MODEM_BUS_SPEC> {
        MODEM_SYNC_BRIDGE_EN_W::new(self, 31)
    }
}
#[doc = "configure modem sync bridge\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_bus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_bus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_BUS_SPEC;
impl crate::RegisterSpec for MODEM_BUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_bus::R`](R) reader structure"]
impl crate::Readable for MODEM_BUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_bus::W`](W) writer structure"]
impl crate::Writable for MODEM_BUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEM_BUS to value 0"]
impl crate::Resettable for MODEM_BUS_SPEC {}
