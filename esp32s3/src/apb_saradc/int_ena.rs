#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `THRES1_LOW` reader - interrupt of thres1 low"]
pub type THRES1_LOW_R = crate::BitReader;
#[doc = "Field `THRES1_LOW` writer - interrupt of thres1 low"]
pub type THRES1_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES0_LOW` reader - interrupt of thres0 low"]
pub type THRES0_LOW_R = crate::BitReader;
#[doc = "Field `THRES0_LOW` writer - interrupt of thres0 low"]
pub type THRES0_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES1_HIGH` reader - interrupt of thres1 high"]
pub type THRES1_HIGH_R = crate::BitReader;
#[doc = "Field `THRES1_HIGH` writer - interrupt of thres1 high"]
pub type THRES1_HIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES0_HIGH` reader - interrupt of thres0 high"]
pub type THRES0_HIGH_R = crate::BitReader;
#[doc = "Field `THRES0_HIGH` writer - interrupt of thres0 high"]
pub type THRES0_HIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_DONE` reader - interrupt of sar2 done"]
pub type ADC2_DONE_R = crate::BitReader;
#[doc = "Field `ADC2_DONE` writer - interrupt of sar2 done"]
pub type ADC2_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_DONE` reader - interrupt of sar1 done"]
pub type ADC1_DONE_R = crate::BitReader;
#[doc = "Field `ADC1_DONE` writer - interrupt of sar1 done"]
pub type ADC1_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 26 - interrupt of thres1 low"]
    #[inline(always)]
    pub fn thres1_low(&self) -> THRES1_LOW_R {
        THRES1_LOW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - interrupt of thres0 low"]
    #[inline(always)]
    pub fn thres0_low(&self) -> THRES0_LOW_R {
        THRES0_LOW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - interrupt of thres1 high"]
    #[inline(always)]
    pub fn thres1_high(&self) -> THRES1_HIGH_R {
        THRES1_HIGH_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - interrupt of thres0 high"]
    #[inline(always)]
    pub fn thres0_high(&self) -> THRES0_HIGH_R {
        THRES0_HIGH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - interrupt of sar2 done"]
    #[inline(always)]
    pub fn adc2_done(&self) -> ADC2_DONE_R {
        ADC2_DONE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - interrupt of sar1 done"]
    #[inline(always)]
    pub fn adc1_done(&self) -> ADC1_DONE_R {
        ADC1_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("thres1_low", &self.thres1_low())
            .field("thres0_low", &self.thres0_low())
            .field("thres1_high", &self.thres1_high())
            .field("thres0_high", &self.thres0_high())
            .field("adc2_done", &self.adc2_done())
            .field("adc1_done", &self.adc1_done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 26 - interrupt of thres1 low"]
    #[inline(always)]
    pub fn thres1_low(&mut self) -> THRES1_LOW_W<INT_ENA_SPEC> {
        THRES1_LOW_W::new(self, 26)
    }
    #[doc = "Bit 27 - interrupt of thres0 low"]
    #[inline(always)]
    pub fn thres0_low(&mut self) -> THRES0_LOW_W<INT_ENA_SPEC> {
        THRES0_LOW_W::new(self, 27)
    }
    #[doc = "Bit 28 - interrupt of thres1 high"]
    #[inline(always)]
    pub fn thres1_high(&mut self) -> THRES1_HIGH_W<INT_ENA_SPEC> {
        THRES1_HIGH_W::new(self, 28)
    }
    #[doc = "Bit 29 - interrupt of thres0 high"]
    #[inline(always)]
    pub fn thres0_high(&mut self) -> THRES0_HIGH_W<INT_ENA_SPEC> {
        THRES0_HIGH_W::new(self, 29)
    }
    #[doc = "Bit 30 - interrupt of sar2 done"]
    #[inline(always)]
    pub fn adc2_done(&mut self) -> ADC2_DONE_W<INT_ENA_SPEC> {
        ADC2_DONE_W::new(self, 30)
    }
    #[doc = "Bit 31 - interrupt of sar1 done"]
    #[inline(always)]
    pub fn adc1_done(&mut self) -> ADC1_DONE_W<INT_ENA_SPEC> {
        ADC1_DONE_W::new(self, 31)
    }
}
#[doc = "enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
