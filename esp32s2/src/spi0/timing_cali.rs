#[doc = "Register `TIMING_CALI` reader"]
pub type R = crate::R<TIMING_CALI_SPEC>;
#[doc = "Register `TIMING_CALI` writer"]
pub type W = crate::W<TIMING_CALI_SPEC>;
#[doc = "Field `TIMING_CLK_ENA` reader - "]
pub type TIMING_CLK_ENA_R = crate::BitReader;
#[doc = "Field `TIMING_CLK_ENA` writer - "]
pub type TIMING_CLK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMING_CALI` reader - "]
pub type TIMING_CALI_R = crate::BitReader;
#[doc = "Field `TIMING_CALI` writer - "]
pub type TIMING_CALI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRA_DUMMY_CYCLELEN` reader - "]
pub type EXTRA_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `EXTRA_DUMMY_CYCLELEN` writer - "]
pub type EXTRA_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timing_clk_ena(&self) -> TIMING_CLK_ENA_R {
        TIMING_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timing_cali(&self) -> TIMING_CALI_R {
        TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn extra_dummy_cyclelen(&self) -> EXTRA_DUMMY_CYCLELEN_R {
        EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMING_CALI")
            .field("extra_dummy_cyclelen", &self.extra_dummy_cyclelen())
            .field("timing_cali", &self.timing_cali())
            .field("timing_clk_ena", &self.timing_clk_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timing_clk_ena(&mut self) -> TIMING_CLK_ENA_W<TIMING_CALI_SPEC> {
        TIMING_CLK_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timing_cali(&mut self) -> TIMING_CALI_W<TIMING_CALI_SPEC> {
        TIMING_CALI_W::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn extra_dummy_cyclelen(&mut self) -> EXTRA_DUMMY_CYCLELEN_W<TIMING_CALI_SPEC> {
        EXTRA_DUMMY_CYCLELEN_W::new(self, 2)
    }
}
#[doc = "SPI Memory Timing Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timing_cali::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing_cali::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMING_CALI_SPEC;
impl crate::RegisterSpec for TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timing_cali::R`](R) reader structure"]
impl crate::Readable for TIMING_CALI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timing_cali::W`](W) writer structure"]
impl crate::Writable for TIMING_CALI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMING_CALI to value 0"]
impl crate::Resettable for TIMING_CALI_SPEC {}
