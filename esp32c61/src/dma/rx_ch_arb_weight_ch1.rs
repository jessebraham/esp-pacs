#[doc = "Register `RX_CH_ARB_WEIGHT_CH1` reader"]
pub type R = crate::R<RX_CH_ARB_WEIGHT_CH1_SPEC>;
#[doc = "Register `RX_CH_ARB_WEIGHT_CH1` writer"]
pub type W = crate::W<RX_CH_ARB_WEIGHT_CH1_SPEC>;
#[doc = "Field `RX_ARB_WEIGHT_VALUE_CH1` reader - Configures the weight(i.e the number of tokens) of RX channel1"]
pub type RX_ARB_WEIGHT_VALUE_CH1_R = crate::FieldReader;
#[doc = "Field `RX_ARB_WEIGHT_VALUE_CH1` writer - Configures the weight(i.e the number of tokens) of RX channel1"]
pub type RX_ARB_WEIGHT_VALUE_CH1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configures the weight(i.e the number of tokens) of RX channel1"]
    #[inline(always)]
    pub fn rx_arb_weight_value_ch1(&self) -> RX_ARB_WEIGHT_VALUE_CH1_R {
        RX_ARB_WEIGHT_VALUE_CH1_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CH_ARB_WEIGHT_CH1")
            .field("rx_arb_weight_value_ch1", &self.rx_arb_weight_value_ch1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures the weight(i.e the number of tokens) of RX channel1"]
    #[inline(always)]
    pub fn rx_arb_weight_value_ch1(
        &mut self,
    ) -> RX_ARB_WEIGHT_VALUE_CH1_W<RX_CH_ARB_WEIGHT_CH1_SPEC> {
        RX_ARB_WEIGHT_VALUE_CH1_W::new(self, 0)
    }
}
#[doc = "RX channel 1 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch_arb_weight_ch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_arb_weight_ch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CH_ARB_WEIGHT_CH1_SPEC;
impl crate::RegisterSpec for RX_CH_ARB_WEIGHT_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ch_arb_weight_ch1::R`](R) reader structure"]
impl crate::Readable for RX_CH_ARB_WEIGHT_CH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_ch_arb_weight_ch1::W`](W) writer structure"]
impl crate::Writable for RX_CH_ARB_WEIGHT_CH1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CH_ARB_WEIGHT_CH1 to value 0"]
impl crate::Resettable for RX_CH_ARB_WEIGHT_CH1_SPEC {}
