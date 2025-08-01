#[doc = "Register `DELAY_CONF_SYNC` reader"]
pub type R = crate::R<DELAY_CONF_SYNC_SPEC>;
#[doc = "Register `DELAY_CONF_SYNC` writer"]
pub type W = crate::W<DELAY_CONF_SYNC_SPEC>;
#[doc = "Field `DL0_EN` reader - Configures whether or not to add a turnaround delay of 1 bit before the start bit.\\\\ 0: Not add\\\\ 1: Add\\\\"]
pub type DL0_EN_R = crate::BitReader;
#[doc = "Field `DL0_EN` writer - Configures whether or not to add a turnaround delay of 1 bit before the start bit.\\\\ 0: Not add\\\\ 1: Add\\\\"]
pub type DL0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1_EN` reader - Configures whether or not to add a turnaround delay of 1 bit after the stop bit.\\\\ 0: Not add\\\\ 1: Add\\\\"]
pub type DL1_EN_R = crate::BitReader;
#[doc = "Field `DL1_EN` writer - Configures whether or not to add a turnaround delay of 1 bit after the stop bit.\\\\ 0: Not add\\\\ 1: Add\\\\"]
pub type DL1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Configures whether or not to add a turnaround delay of 1 bit before the start bit.\\\\ 0: Not add\\\\ 1: Add\\\\"]
    #[inline(always)]
    pub fn dl0_en(&self) -> DL0_EN_R {
        DL0_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to add a turnaround delay of 1 bit after the stop bit.\\\\ 0: Not add\\\\ 1: Add\\\\"]
    #[inline(always)]
    pub fn dl1_en(&self) -> DL1_EN_R {
        DL1_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DELAY_CONF_SYNC")
            .field("dl0_en", &self.dl0_en())
            .field("dl1_en", &self.dl1_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Configures whether or not to add a turnaround delay of 1 bit before the start bit.\\\\ 0: Not add\\\\ 1: Add\\\\"]
    #[inline(always)]
    pub fn dl0_en(&mut self) -> DL0_EN_W<DELAY_CONF_SYNC_SPEC> {
        DL0_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to add a turnaround delay of 1 bit after the stop bit.\\\\ 0: Not add\\\\ 1: Add\\\\"]
    #[inline(always)]
    pub fn dl1_en(&mut self) -> DL1_EN_W<DELAY_CONF_SYNC_SPEC> {
        DL1_EN_W::new(self, 2)
    }
}
#[doc = "RS485 mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`delay_conf_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delay_conf_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DELAY_CONF_SYNC_SPEC;
impl crate::RegisterSpec for DELAY_CONF_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`delay_conf_sync::R`](R) reader structure"]
impl crate::Readable for DELAY_CONF_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`delay_conf_sync::W`](W) writer structure"]
impl crate::Writable for DELAY_CONF_SYNC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DELAY_CONF_SYNC to value 0"]
impl crate::Resettable for DELAY_CONF_SYNC_SPEC {}
