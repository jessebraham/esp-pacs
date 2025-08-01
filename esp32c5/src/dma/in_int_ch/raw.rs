#[doc = "Register `RAW` reader"]
pub type R = crate::R<RAW_SPEC>;
#[doc = "Register `RAW` writer"]
pub type W = crate::W<RAW_SPEC>;
#[doc = "Field `IN_DONE` reader - The raw interrupt status of AHB_DMA_IN_DONE_CH0_INT."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_DONE` writer - The raw interrupt status of AHB_DMA_IN_DONE_CH0_INT."]
pub type IN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF` reader - The raw interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` writer - The raw interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT."]
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF` reader - The raw interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT."]
pub type IN_ERR_EOF_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF` writer - The raw interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT."]
pub type IN_ERR_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR` reader - The raw interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT."]
pub type IN_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` writer - The raw interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT."]
pub type IN_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY` reader - The raw interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT."]
pub type IN_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` writer - The raw interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT."]
pub type IN_DSCR_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF` reader - The raw interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT."]
pub type INFIFO_OVF_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF` writer - The raw interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT."]
pub type INFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF` reader - The raw interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT."]
pub type INFIFO_UDF_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF` writer - The raw interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT."]
pub type INFIFO_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of AHB_DMA_IN_DONE_CH0_INT."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT."]
    #[inline(always)]
    pub fn infifo_ovf(&self) -> INFIFO_OVF_R {
        INFIFO_OVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT."]
    #[inline(always)]
    pub fn infifo_udf(&self) -> INFIFO_UDF_R {
        INFIFO_UDF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAW")
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("in_err_eof", &self.in_err_eof())
            .field("in_dscr_err", &self.in_dscr_err())
            .field("in_dscr_empty", &self.in_dscr_empty())
            .field("infifo_ovf", &self.infifo_ovf())
            .field("infifo_udf", &self.infifo_udf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt status of AHB_DMA_IN_DONE_CH0_INT."]
    #[inline(always)]
    pub fn in_done(&mut self) -> IN_DONE_W<RAW_SPEC> {
        IN_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT."]
    #[inline(always)]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<RAW_SPEC> {
        IN_SUC_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT."]
    #[inline(always)]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<RAW_SPEC> {
        IN_ERR_EOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT."]
    #[inline(always)]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<RAW_SPEC> {
        IN_DSCR_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT."]
    #[inline(always)]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<RAW_SPEC> {
        IN_DSCR_EMPTY_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT."]
    #[inline(always)]
    pub fn infifo_ovf(&mut self) -> INFIFO_OVF_W<RAW_SPEC> {
        INFIFO_OVF_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT."]
    #[inline(always)]
    pub fn infifo_udf(&mut self) -> INFIFO_UDF_W<RAW_SPEC> {
        INFIFO_UDF_W::new(self, 6)
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
