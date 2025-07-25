#[doc = "Register `XTS_PSEUDO_ROUND_CONF` reader"]
pub type R = crate::R<XTS_PSEUDO_ROUND_CONF_SPEC>;
#[doc = "Register `XTS_PSEUDO_ROUND_CONF` writer"]
pub type W = crate::W<XTS_PSEUDO_ROUND_CONF_SPEC>;
#[doc = "Field `MODE_PSEUDO` reader - Set the mode of pseudo. 2'b00: crypto without pseudo. 2'b01: state T with pseudo and state D without pseudo. 2'b10: state T with pseudo and state D with few pseudo. 2'b11: crypto with pseudo."]
pub type MODE_PSEUDO_R = crate::FieldReader;
#[doc = "Field `MODE_PSEUDO` writer - Set the mode of pseudo. 2'b00: crypto without pseudo. 2'b01: state T with pseudo and state D without pseudo. 2'b10: state T with pseudo and state D with few pseudo. 2'b11: crypto with pseudo."]
pub type MODE_PSEUDO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PSEUDO_RNG_CNT` reader - xts aes peseudo function base round that must be peformed."]
pub type PSEUDO_RNG_CNT_R = crate::FieldReader;
#[doc = "Field `PSEUDO_RNG_CNT` writer - xts aes peseudo function base round that must be peformed."]
pub type PSEUDO_RNG_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PSEUDO_BASE` reader - xts aes peseudo function base round that must be peformed."]
pub type PSEUDO_BASE_R = crate::FieldReader;
#[doc = "Field `PSEUDO_BASE` writer - xts aes peseudo function base round that must be peformed."]
pub type PSEUDO_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PSEUDO_INC` reader - xts aes peseudo function increment round that will be peformed randomly between 0 & 2**(inc+1)."]
pub type PSEUDO_INC_R = crate::FieldReader;
#[doc = "Field `PSEUDO_INC` writer - xts aes peseudo function increment round that will be peformed randomly between 0 & 2**(inc+1)."]
pub type PSEUDO_INC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Set the mode of pseudo. 2'b00: crypto without pseudo. 2'b01: state T with pseudo and state D without pseudo. 2'b10: state T with pseudo and state D with few pseudo. 2'b11: crypto with pseudo."]
    #[inline(always)]
    pub fn mode_pseudo(&self) -> MODE_PSEUDO_R {
        MODE_PSEUDO_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - xts aes peseudo function base round that must be peformed."]
    #[inline(always)]
    pub fn pseudo_rng_cnt(&self) -> PSEUDO_RNG_CNT_R {
        PSEUDO_RNG_CNT_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - xts aes peseudo function base round that must be peformed."]
    #[inline(always)]
    pub fn pseudo_base(&self) -> PSEUDO_BASE_R {
        PSEUDO_BASE_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:10 - xts aes peseudo function increment round that will be peformed randomly between 0 & 2**(inc+1)."]
    #[inline(always)]
    pub fn pseudo_inc(&self) -> PSEUDO_INC_R {
        PSEUDO_INC_R::new(((self.bits >> 9) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTS_PSEUDO_ROUND_CONF")
            .field("mode_pseudo", &self.mode_pseudo())
            .field("pseudo_rng_cnt", &self.pseudo_rng_cnt())
            .field("pseudo_base", &self.pseudo_base())
            .field("pseudo_inc", &self.pseudo_inc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Set the mode of pseudo. 2'b00: crypto without pseudo. 2'b01: state T with pseudo and state D without pseudo. 2'b10: state T with pseudo and state D with few pseudo. 2'b11: crypto with pseudo."]
    #[inline(always)]
    pub fn mode_pseudo(&mut self) -> MODE_PSEUDO_W<XTS_PSEUDO_ROUND_CONF_SPEC> {
        MODE_PSEUDO_W::new(self, 0)
    }
    #[doc = "Bits 2:4 - xts aes peseudo function base round that must be peformed."]
    #[inline(always)]
    pub fn pseudo_rng_cnt(&mut self) -> PSEUDO_RNG_CNT_W<XTS_PSEUDO_ROUND_CONF_SPEC> {
        PSEUDO_RNG_CNT_W::new(self, 2)
    }
    #[doc = "Bits 5:8 - xts aes peseudo function base round that must be peformed."]
    #[inline(always)]
    pub fn pseudo_base(&mut self) -> PSEUDO_BASE_W<XTS_PSEUDO_ROUND_CONF_SPEC> {
        PSEUDO_BASE_W::new(self, 5)
    }
    #[doc = "Bits 9:10 - xts aes peseudo function increment round that will be peformed randomly between 0 & 2**(inc+1)."]
    #[inline(always)]
    pub fn pseudo_inc(&mut self) -> PSEUDO_INC_W<XTS_PSEUDO_ROUND_CONF_SPEC> {
        PSEUDO_INC_W::new(self, 9)
    }
}
#[doc = "SPI memory cryption PSEUDO register\n\nYou can [`read`](crate::Reg::read) this register and get [`xts_pseudo_round_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_pseudo_round_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTS_PSEUDO_ROUND_CONF_SPEC;
impl crate::RegisterSpec for XTS_PSEUDO_ROUND_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xts_pseudo_round_conf::R`](R) reader structure"]
impl crate::Readable for XTS_PSEUDO_ROUND_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xts_pseudo_round_conf::W`](W) writer structure"]
impl crate::Writable for XTS_PSEUDO_ROUND_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTS_PSEUDO_ROUND_CONF to value 0x045c"]
impl crate::Resettable for XTS_PSEUDO_ROUND_CONF_SPEC {
    const RESET_VALUE: u32 = 0x045c;
}
