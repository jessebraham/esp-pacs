#[doc = "Register `FUNC68_IN_SEL_CFG` reader"]
pub type R = crate::R<FUNC68_IN_SEL_CFG_SPEC>;
#[doc = "Register `FUNC68_IN_SEL_CFG` writer"]
pub type W = crate::W<FUNC68_IN_SEL_CFG_SPEC>;
#[doc = "Field `FUNC68_IN_SEL` reader - set this value: s=0-33: connect GPIO\\[s\\] to this port. s=0x40: set this port always high level. s=0x60: set this port always low level."]
pub type FUNC68_IN_SEL_R = crate::FieldReader;
#[doc = "Field `FUNC68_IN_SEL` writer - set this value: s=0-33: connect GPIO\\[s\\] to this port. s=0x40: set this port always high level. s=0x60: set this port always low level."]
pub type FUNC68_IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FUNC68_IN_INV_SEL` reader - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type FUNC68_IN_INV_SEL_R = crate::BitReader;
#[doc = "Field `FUNC68_IN_INV_SEL` writer - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type FUNC68_IN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIG68_IN_SEL` reader - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SIG68_IN_SEL_R = crate::BitReader;
#[doc = "Field `SIG68_IN_SEL` writer - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SIG68_IN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - set this value: s=0-33: connect GPIO\\[s\\] to this port. s=0x40: set this port always high level. s=0x60: set this port always low level."]
    #[inline(always)]
    pub fn func68_in_sel(&self) -> FUNC68_IN_SEL_R {
        FUNC68_IN_SEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn func68_in_inv_sel(&self) -> FUNC68_IN_INV_SEL_R {
        FUNC68_IN_INV_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sig68_in_sel(&self) -> SIG68_IN_SEL_R {
        SIG68_IN_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC68_IN_SEL_CFG")
            .field("func68_in_sel", &self.func68_in_sel())
            .field("func68_in_inv_sel", &self.func68_in_inv_sel())
            .field("sig68_in_sel", &self.sig68_in_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - set this value: s=0-33: connect GPIO\\[s\\] to this port. s=0x40: set this port always high level. s=0x60: set this port always low level."]
    #[inline(always)]
    pub fn func68_in_sel(&mut self) -> FUNC68_IN_SEL_W<FUNC68_IN_SEL_CFG_SPEC> {
        FUNC68_IN_SEL_W::new(self, 0)
    }
    #[doc = "Bit 7 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn func68_in_inv_sel(&mut self) -> FUNC68_IN_INV_SEL_W<FUNC68_IN_SEL_CFG_SPEC> {
        FUNC68_IN_INV_SEL_W::new(self, 7)
    }
    #[doc = "Bit 8 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sig68_in_sel(&mut self) -> SIG68_IN_SEL_W<FUNC68_IN_SEL_CFG_SPEC> {
        SIG68_IN_SEL_W::new(self, 8)
    }
}
#[doc = "GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func68_in_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func68_in_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC68_IN_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC68_IN_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func68_in_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC68_IN_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func68_in_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC68_IN_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC68_IN_SEL_CFG to value 0x60"]
impl crate::Resettable for FUNC68_IN_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0x60;
}
