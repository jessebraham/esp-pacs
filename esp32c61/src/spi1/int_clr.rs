#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `PER_END_INT_CLR` writer - The clear bit for SPI_MEM_PER_END_INT interrupt."]
pub type PER_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES_END_INT_CLR` writer - The clear bit for SPI_MEM_PES_END_INT interrupt."]
pub type PES_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPE_END_INT_CLR` writer - The clear bit for SPI_MEM_WPE_END_INT interrupt."]
pub type WPE_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SLV_ST_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MST_ST_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT_INT_CLR` writer - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type BROWN_OUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The clear bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn per_end_int_clr(&mut self) -> PER_END_INT_CLR_W<INT_CLR_SPEC> {
        PER_END_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The clear bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn pes_end_int_clr(&mut self) -> PES_END_INT_CLR_W<INT_CLR_SPEC> {
        PES_END_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - The clear bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    pub fn wpe_end_int_clr(&mut self) -> WPE_END_INT_CLR_W<INT_CLR_SPEC> {
        WPE_END_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end_int_clr(&mut self) -> SLV_ST_END_INT_CLR_W<INT_CLR_SPEC> {
        SLV_ST_END_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end_int_clr(&mut self) -> MST_ST_END_INT_CLR_W<INT_CLR_SPEC> {
        MST_ST_END_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 10 - The status bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn brown_out_int_clr(&mut self) -> BROWN_OUT_INT_CLR_W<INT_CLR_SPEC> {
        BROWN_OUT_INT_CLR_W::new(self, 10)
    }
}
#[doc = "SPI1 interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
