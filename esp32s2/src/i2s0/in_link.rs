#[doc = "Register `IN_LINK` reader"]
pub type R = crate::R<IN_LINK_SPEC>;
#[doc = "Register `IN_LINK` writer"]
pub type W = crate::W<IN_LINK_SPEC>;
#[doc = "Field `INLINK_ADDR` reader - The address of first inlink descriptor."]
pub type INLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `INLINK_ADDR` writer - The address of first inlink descriptor."]
pub type INLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `INLINK_STOP` reader - Set this bit to stop inlink descriptor."]
pub type INLINK_STOP_R = crate::BitReader;
#[doc = "Field `INLINK_STOP` writer - Set this bit to stop inlink descriptor."]
pub type INLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_START` reader - Set this bit to start inlink descriptor."]
pub type INLINK_START_R = crate::BitReader;
#[doc = "Field `INLINK_START` writer - Set this bit to start inlink descriptor."]
pub type INLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_RESTART` reader - Set this bit to restart inlink descriptor."]
pub type INLINK_RESTART_R = crate::BitReader;
#[doc = "Field `INLINK_RESTART` writer - Set this bit to restart inlink descriptor."]
pub type INLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_PARK` reader - "]
pub type INLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19 - The address of first inlink descriptor."]
    #[inline(always)]
    pub fn inlink_addr(&self) -> INLINK_ADDR_R {
        INLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28 - Set this bit to stop inlink descriptor."]
    #[inline(always)]
    pub fn inlink_stop(&self) -> INLINK_STOP_R {
        INLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to start inlink descriptor."]
    #[inline(always)]
    pub fn inlink_start(&self) -> INLINK_START_R {
        INLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to restart inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart(&self) -> INLINK_RESTART_R {
        INLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn inlink_park(&self) -> INLINK_PARK_R {
        INLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_LINK")
            .field("inlink_addr", &self.inlink_addr())
            .field("inlink_stop", &self.inlink_stop())
            .field("inlink_start", &self.inlink_start())
            .field("inlink_restart", &self.inlink_restart())
            .field("inlink_park", &self.inlink_park())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - The address of first inlink descriptor."]
    #[inline(always)]
    pub fn inlink_addr(&mut self) -> INLINK_ADDR_W<IN_LINK_SPEC> {
        INLINK_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 28 - Set this bit to stop inlink descriptor."]
    #[inline(always)]
    pub fn inlink_stop(&mut self) -> INLINK_STOP_W<IN_LINK_SPEC> {
        INLINK_STOP_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set this bit to start inlink descriptor."]
    #[inline(always)]
    pub fn inlink_start(&mut self) -> INLINK_START_W<IN_LINK_SPEC> {
        INLINK_START_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set this bit to restart inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart(&mut self) -> INLINK_RESTART_W<IN_LINK_SPEC> {
        INLINK_RESTART_W::new(self, 30)
    }
}
#[doc = "I2S DMA RX configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_LINK_SPEC;
impl crate::RegisterSpec for IN_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_link::R`](R) reader structure"]
impl crate::Readable for IN_LINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_link::W`](W) writer structure"]
impl crate::Writable for IN_LINK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_LINK to value 0"]
impl crate::Resettable for IN_LINK_SPEC {}
