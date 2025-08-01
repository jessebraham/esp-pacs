#[doc = "Register `RAW` reader"]
pub type R = crate::R<RAW_SPEC>;
#[doc = "Register `RAW` writer"]
pub type W = crate::W<RAW_SPEC>;
#[doc = "Field `IN_DONE_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_IN_DONE_CH0_INT"]
pub type IN_DONE_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DONE_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_IN_DONE_CH0_INT"]
pub type IN_DONE_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT"]
pub type IN_SUC_EOF_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT"]
pub type IN_SUC_EOF_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT"]
pub type IN_ERR_EOF_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT"]
pub type IN_ERR_EOF_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT"]
pub type IN_DSCR_ERR_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT"]
pub type IN_DSCR_ERR_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
pub type IN_DSCR_EMPTY_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
pub type IN_DSCR_EMPTY_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT"]
pub type INFIFO_OVF_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT"]
pub type INFIFO_OVF_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT"]
pub type INFIFO_UDF_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT"]
pub type INFIFO_UDF_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_AHBINF_RESP_ERR_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_IN_RESP_ERR_CH0_INT"]
pub type IN_AHBINF_RESP_ERR_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_AHBINF_RESP_ERR_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_IN_RESP_ERR_CH0_INT"]
pub type IN_AHBINF_RESP_ERR_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of AHB_DMA_IN_DONE_CH0_INT"]
    #[inline(always)]
    pub fn in_done_ch0_int_raw(&self) -> IN_DONE_CH0_INT_RAW_R {
        IN_DONE_CH0_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_suc_eof_ch0_int_raw(&self) -> IN_SUC_EOF_CH0_INT_RAW_R {
        IN_SUC_EOF_CH0_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_err_eof_ch0_int_raw(&self) -> IN_ERR_EOF_CH0_INT_RAW_R {
        IN_ERR_EOF_CH0_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_err_ch0_int_raw(&self) -> IN_DSCR_ERR_CH0_INT_RAW_R {
        IN_DSCR_ERR_CH0_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_empty_ch0_int_raw(&self) -> IN_DSCR_EMPTY_CH0_INT_RAW_R {
        IN_DSCR_EMPTY_CH0_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_ovf_ch0_int_raw(&self) -> INFIFO_OVF_CH0_INT_RAW_R {
        INFIFO_OVF_CH0_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_udf_ch0_int_raw(&self) -> INFIFO_UDF_CH0_INT_RAW_R {
        INFIFO_UDF_CH0_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status of AHB_DMA_IN_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_ahbinf_resp_err_ch0_int_raw(&self) -> IN_AHBINF_RESP_ERR_CH0_INT_RAW_R {
        IN_AHBINF_RESP_ERR_CH0_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAW")
            .field("in_done_ch0_int_raw", &self.in_done_ch0_int_raw())
            .field("in_suc_eof_ch0_int_raw", &self.in_suc_eof_ch0_int_raw())
            .field("in_err_eof_ch0_int_raw", &self.in_err_eof_ch0_int_raw())
            .field("in_dscr_err_ch0_int_raw", &self.in_dscr_err_ch0_int_raw())
            .field(
                "in_dscr_empty_ch0_int_raw",
                &self.in_dscr_empty_ch0_int_raw(),
            )
            .field("infifo_ovf_ch0_int_raw", &self.infifo_ovf_ch0_int_raw())
            .field("infifo_udf_ch0_int_raw", &self.infifo_udf_ch0_int_raw())
            .field(
                "in_ahbinf_resp_err_ch0_int_raw",
                &self.in_ahbinf_resp_err_ch0_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt status of AHB_DMA_IN_DONE_CH0_INT"]
    #[inline(always)]
    pub fn in_done_ch0_int_raw(&mut self) -> IN_DONE_CH0_INT_RAW_W<RAW_SPEC> {
        IN_DONE_CH0_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_suc_eof_ch0_int_raw(&mut self) -> IN_SUC_EOF_CH0_INT_RAW_W<RAW_SPEC> {
        IN_SUC_EOF_CH0_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_err_eof_ch0_int_raw(&mut self) -> IN_ERR_EOF_CH0_INT_RAW_W<RAW_SPEC> {
        IN_ERR_EOF_CH0_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_err_ch0_int_raw(&mut self) -> IN_DSCR_ERR_CH0_INT_RAW_W<RAW_SPEC> {
        IN_DSCR_ERR_CH0_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_empty_ch0_int_raw(&mut self) -> IN_DSCR_EMPTY_CH0_INT_RAW_W<RAW_SPEC> {
        IN_DSCR_EMPTY_CH0_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_ovf_ch0_int_raw(&mut self) -> INFIFO_OVF_CH0_INT_RAW_W<RAW_SPEC> {
        INFIFO_OVF_CH0_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_udf_ch0_int_raw(&mut self) -> INFIFO_UDF_CH0_INT_RAW_W<RAW_SPEC> {
        INFIFO_UDF_CH0_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw interrupt status of AHB_DMA_IN_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_ahbinf_resp_err_ch0_int_raw(&mut self) -> IN_AHBINF_RESP_ERR_CH0_INT_RAW_W<RAW_SPEC> {
        IN_AHBINF_RESP_ERR_CH0_INT_RAW_W::new(self, 7)
    }
}
#[doc = "Raw interrupt status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAW_SPEC;
impl crate::RegisterSpec for RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw::R`](R) reader structure"]
impl crate::Readable for RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`raw::W`](W) writer structure"]
impl crate::Writable for RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAW to value 0"]
impl crate::Resettable for RAW_SPEC {}
