#[doc = "Register `TRV_DELAY_SSP_CFG` reader"]
pub type R = crate::R<TRV_DELAY_SSP_CFG_SPEC>;
#[doc = "Register `TRV_DELAY_SSP_CFG` writer"]
pub type W = crate::W<TRV_DELAY_SSP_CFG_SPEC>;
#[doc = "Field `TRV_DELAY_VALUE` reader - Measured Transmitter delay in multiple of minimal Time quanta."]
pub type TRV_DELAY_VALUE_R = crate::FieldReader;
#[doc = "Field `SSP_OFFSET` reader - Secondary sampling point offset. Value is given as multiple of minimal Time quanta."]
pub type SSP_OFFSET_R = crate::FieldReader;
#[doc = "Field `SSP_OFFSET` writer - Secondary sampling point offset. Value is given as multiple of minimal Time quanta."]
pub type SSP_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SSP_SRC` reader - Source of Secondary sampling point. 0b00 - SSP_SRC_MEAS_N_OFFSET - SSP position = TRV_DELAY (Measured Transmitter delay) + SSP_OFFSET. 0b01 - SSP_SRC_NO_SSP - SSP is not used. Transmitter uses regular Sampling Point during data bit rate. 0b10 - SSP_SRC_OFFSET - SSP position = SSP_OFFSET. Measured Transmitter delay value is ignored."]
pub type SSP_SRC_R = crate::FieldReader;
#[doc = "Field `SSP_SRC` writer - Source of Secondary sampling point. 0b00 - SSP_SRC_MEAS_N_OFFSET - SSP position = TRV_DELAY (Measured Transmitter delay) + SSP_OFFSET. 0b01 - SSP_SRC_NO_SSP - SSP is not used. Transmitter uses regular Sampling Point during data bit rate. 0b10 - SSP_SRC_OFFSET - SSP position = SSP_OFFSET. Measured Transmitter delay value is ignored."]
pub type SSP_SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Measured Transmitter delay in multiple of minimal Time quanta."]
    #[inline(always)]
    pub fn trv_delay_value(&self) -> TRV_DELAY_VALUE_R {
        TRV_DELAY_VALUE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Secondary sampling point offset. Value is given as multiple of minimal Time quanta."]
    #[inline(always)]
    pub fn ssp_offset(&self) -> SSP_OFFSET_R {
        SSP_OFFSET_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Source of Secondary sampling point. 0b00 - SSP_SRC_MEAS_N_OFFSET - SSP position = TRV_DELAY (Measured Transmitter delay) + SSP_OFFSET. 0b01 - SSP_SRC_NO_SSP - SSP is not used. Transmitter uses regular Sampling Point during data bit rate. 0b10 - SSP_SRC_OFFSET - SSP position = SSP_OFFSET. Measured Transmitter delay value is ignored."]
    #[inline(always)]
    pub fn ssp_src(&self) -> SSP_SRC_R {
        SSP_SRC_R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRV_DELAY_SSP_CFG")
            .field("trv_delay_value", &self.trv_delay_value())
            .field("ssp_offset", &self.ssp_offset())
            .field("ssp_src", &self.ssp_src())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:23 - Secondary sampling point offset. Value is given as multiple of minimal Time quanta."]
    #[inline(always)]
    pub fn ssp_offset(&mut self) -> SSP_OFFSET_W<TRV_DELAY_SSP_CFG_SPEC> {
        SSP_OFFSET_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Source of Secondary sampling point. 0b00 - SSP_SRC_MEAS_N_OFFSET - SSP position = TRV_DELAY (Measured Transmitter delay) + SSP_OFFSET. 0b01 - SSP_SRC_NO_SSP - SSP is not used. Transmitter uses regular Sampling Point during data bit rate. 0b10 - SSP_SRC_OFFSET - SSP position = SSP_OFFSET. Measured Transmitter delay value is ignored."]
    #[inline(always)]
    pub fn ssp_src(&mut self) -> SSP_SRC_W<TRV_DELAY_SSP_CFG_SPEC> {
        SSP_SRC_W::new(self, 24)
    }
}
#[doc = "TWAI FD transmit delay & secondary sample point configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`trv_delay_ssp_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trv_delay_ssp_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRV_DELAY_SSP_CFG_SPEC;
impl crate::RegisterSpec for TRV_DELAY_SSP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trv_delay_ssp_cfg::R`](R) reader structure"]
impl crate::Readable for TRV_DELAY_SSP_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trv_delay_ssp_cfg::W`](W) writer structure"]
impl crate::Writable for TRV_DELAY_SSP_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRV_DELAY_SSP_CFG to value 0x000a_0000"]
impl crate::Resettable for TRV_DELAY_SSP_CFG_SPEC {
    const RESET_VALUE: u32 = 0x000a_0000;
}
