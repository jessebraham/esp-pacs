#[doc = "Register `PIN%s` reader"]
pub type R = crate::R<PIN_SPEC>;
#[doc = "Register `PIN%s` writer"]
pub type W = crate::W<PIN_SPEC>;
#[doc = "Field `SYNC2_BYPASS` reader - Configures whether or not to synchronize GPIO input data on either edge of LP IO MUX operating clock for the second-level synchronization.\\\\ 0: Not synchronize\\\\ 1: Synchronize on falling edge\\\\ 2: Synchronize on rising edge\\\\ 3: Synchronize on rising edge\\\\"]
pub type SYNC2_BYPASS_R = crate::FieldReader;
#[doc = "Field `SYNC2_BYPASS` writer - Configures whether or not to synchronize GPIO input data on either edge of LP IO MUX operating clock for the second-level synchronization.\\\\ 0: Not synchronize\\\\ 1: Synchronize on falling edge\\\\ 2: Synchronize on rising edge\\\\ 3: Synchronize on rising edge\\\\"]
pub type SYNC2_BYPASS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PAD_DRIVER` reader - Configures to select the pin dirve mode of GPIO0.\\\\ 0: Normal output\\\\ 1: Open drain output\\\\"]
pub type PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `PAD_DRIVER` writer - Configures to select the pin dirve mode of GPIO0.\\\\ 0: Normal output\\\\ 1: Open drain output\\\\"]
pub type PAD_DRIVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC1_BYPASS` reader - Configures whether or not to synchronize GPIO input data on either edge of LP IO MUX operating clock for the first-level synchronization.\\\\ 0: Not synchronize\\\\ 1: Synchronize on falling edge\\\\ 2: Synchronize on rising edge\\\\ 3: Synchronize on rising edge\\\\"]
pub type SYNC1_BYPASS_R = crate::FieldReader;
#[doc = "Field `SYNC1_BYPASS` writer - Configures whether or not to synchronize GPIO input data on either edge of LP IO MUX operating clock for the first-level synchronization.\\\\ 0: Not synchronize\\\\ 1: Synchronize on falling edge\\\\ 2: Synchronize on rising edge\\\\ 3: Synchronize on rising edge\\\\"]
pub type SYNC1_BYPASS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGE_WAKEUP_CLR` writer - Configures whether or not to clear the edge wake-up status of GPIO0 ~ GPIO7.\\\\ - bit0 ~ bit7 are corresponding to GPIO0 ~ GPIO7. - If the value 1 is written to a bit here, the edge wake-up status of corresponding GPIO will be cleared."]
pub type EDGE_WAKEUP_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_TYPE` reader - Configures GPIO0 interrupt type.\\\\ 0: GPIO interrupt disabled \\\\ 1: Rising edge trigger \\\\ 2: Falling edge trigger \\\\ 3: Any edge trigger \\\\ 4: Low level trigger \\\\ 5: High level trigger \\\\"]
pub type INT_TYPE_R = crate::FieldReader;
#[doc = "Field `INT_TYPE` writer - Configures GPIO0 interrupt type.\\\\ 0: GPIO interrupt disabled \\\\ 1: Rising edge trigger \\\\ 2: Falling edge trigger \\\\ 3: Any edge trigger \\\\ 4: Low level trigger \\\\ 5: High level trigger \\\\"]
pub type INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WAKEUP_ENABLE` reader - Configures whether or not to enable GPIO0 wake-up function.\\\\ 0: Not enable\\\\ 1: Enable\\\\ This function is disabled when PD_LP_PERI is powered off.\\\\"]
pub type WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `WAKEUP_ENABLE` writer - Configures whether or not to enable GPIO0 wake-up function.\\\\ 0: Not enable\\\\ 1: Enable\\\\ This function is disabled when PD_LP_PERI is powered off.\\\\"]
pub type WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures whether or not to synchronize GPIO input data on either edge of LP IO MUX operating clock for the second-level synchronization.\\\\ 0: Not synchronize\\\\ 1: Synchronize on falling edge\\\\ 2: Synchronize on rising edge\\\\ 3: Synchronize on rising edge\\\\"]
    #[inline(always)]
    pub fn sync2_bypass(&self) -> SYNC2_BYPASS_R {
        SYNC2_BYPASS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Configures to select the pin dirve mode of GPIO0.\\\\ 0: Normal output\\\\ 1: Open drain output\\\\"]
    #[inline(always)]
    pub fn pad_driver(&self) -> PAD_DRIVER_R {
        PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Configures whether or not to synchronize GPIO input data on either edge of LP IO MUX operating clock for the first-level synchronization.\\\\ 0: Not synchronize\\\\ 1: Synchronize on falling edge\\\\ 2: Synchronize on rising edge\\\\ 3: Synchronize on rising edge\\\\"]
    #[inline(always)]
    pub fn sync1_bypass(&self) -> SYNC1_BYPASS_R {
        SYNC1_BYPASS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 7:9 - Configures GPIO0 interrupt type.\\\\ 0: GPIO interrupt disabled \\\\ 1: Rising edge trigger \\\\ 2: Falling edge trigger \\\\ 3: Any edge trigger \\\\ 4: Low level trigger \\\\ 5: High level trigger \\\\"]
    #[inline(always)]
    pub fn int_type(&self) -> INT_TYPE_R {
        INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - Configures whether or not to enable GPIO0 wake-up function.\\\\ 0: Not enable\\\\ 1: Enable\\\\ This function is disabled when PD_LP_PERI is powered off.\\\\"]
    #[inline(always)]
    pub fn wakeup_enable(&self) -> WAKEUP_ENABLE_R {
        WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN")
            .field("sync2_bypass", &self.sync2_bypass())
            .field("pad_driver", &self.pad_driver())
            .field("sync1_bypass", &self.sync1_bypass())
            .field("int_type", &self.int_type())
            .field("wakeup_enable", &self.wakeup_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures whether or not to synchronize GPIO input data on either edge of LP IO MUX operating clock for the second-level synchronization.\\\\ 0: Not synchronize\\\\ 1: Synchronize on falling edge\\\\ 2: Synchronize on rising edge\\\\ 3: Synchronize on rising edge\\\\"]
    #[inline(always)]
    pub fn sync2_bypass(&mut self) -> SYNC2_BYPASS_W<PIN_SPEC> {
        SYNC2_BYPASS_W::new(self, 0)
    }
    #[doc = "Bit 2 - Configures to select the pin dirve mode of GPIO0.\\\\ 0: Normal output\\\\ 1: Open drain output\\\\"]
    #[inline(always)]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W<PIN_SPEC> {
        PAD_DRIVER_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Configures whether or not to synchronize GPIO input data on either edge of LP IO MUX operating clock for the first-level synchronization.\\\\ 0: Not synchronize\\\\ 1: Synchronize on falling edge\\\\ 2: Synchronize on rising edge\\\\ 3: Synchronize on rising edge\\\\"]
    #[inline(always)]
    pub fn sync1_bypass(&mut self) -> SYNC1_BYPASS_W<PIN_SPEC> {
        SYNC1_BYPASS_W::new(self, 3)
    }
    #[doc = "Bit 5 - Configures whether or not to clear the edge wake-up status of GPIO0 ~ GPIO7.\\\\ - bit0 ~ bit7 are corresponding to GPIO0 ~ GPIO7. - If the value 1 is written to a bit here, the edge wake-up status of corresponding GPIO will be cleared."]
    #[inline(always)]
    pub fn edge_wakeup_clr(&mut self) -> EDGE_WAKEUP_CLR_W<PIN_SPEC> {
        EDGE_WAKEUP_CLR_W::new(self, 5)
    }
    #[doc = "Bits 7:9 - Configures GPIO0 interrupt type.\\\\ 0: GPIO interrupt disabled \\\\ 1: Rising edge trigger \\\\ 2: Falling edge trigger \\\\ 3: Any edge trigger \\\\ 4: Low level trigger \\\\ 5: High level trigger \\\\"]
    #[inline(always)]
    pub fn int_type(&mut self) -> INT_TYPE_W<PIN_SPEC> {
        INT_TYPE_W::new(self, 7)
    }
    #[doc = "Bit 10 - Configures whether or not to enable GPIO0 wake-up function.\\\\ 0: Not enable\\\\ 1: Enable\\\\ This function is disabled when PD_LP_PERI is powered off.\\\\"]
    #[inline(always)]
    pub fn wakeup_enable(&mut self) -> WAKEUP_ENABLE_W<PIN_SPEC> {
        WAKEUP_ENABLE_W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin::R`](R) reader structure"]
impl crate::Readable for PIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin::W`](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN%s to value 0"]
impl crate::Resettable for PIN_SPEC {}
