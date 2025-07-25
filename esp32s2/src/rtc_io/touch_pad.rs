#[doc = "Register `TOUCH_PAD%s` reader"]
pub type R = crate::R<TOUCH_PAD_SPEC>;
#[doc = "Register `TOUCH_PAD%s` writer"]
pub type W = crate::W<TOUCH_PAD_SPEC>;
#[doc = "Field `FUN_IE` reader - Input enable in normal execution."]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - Input enable in normal execution."]
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_OE` reader - Output enable in sleep mode."]
pub type SLP_OE_R = crate::BitReader;
#[doc = "Field `SLP_OE` writer - Output enable in sleep mode."]
pub type SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_IE` reader - Input enable in sleep mode."]
pub type SLP_IE_R = crate::BitReader;
#[doc = "Field `SLP_IE` writer - Input enable in sleep mode."]
pub type SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_SEL` reader - 0: no sleep mode. 1: enable sleep mode."]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - 0: no sleep mode. 1: enable sleep mode."]
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_SEL` reader - Function selection."]
pub type FUN_SEL_R = crate::FieldReader;
#[doc = "Field `FUN_SEL` writer - Function selection."]
pub type FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MUX_SEL` reader - Connect the RTC pad input to digital pad input. 0 is available."]
pub type MUX_SEL_R = crate::BitReader;
#[doc = "Field `MUX_SEL` writer - Connect the RTC pad input to digital pad input. 0 is available."]
pub type MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD` reader - Touch sensor power on."]
pub type XPD_R = crate::BitReader;
#[doc = "Field `XPD` writer - Touch sensor power on."]
pub type XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_OPT` reader - The tie option of touch sensor. 0: tie low. 1: tie high."]
pub type TIE_OPT_R = crate::BitReader;
#[doc = "Field `TIE_OPT` writer - The tie option of touch sensor. 0: tie low. 1: tie high."]
pub type TIE_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start touch sensor."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start touch sensor."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC` reader - Touch sensor slope control. 3-bit for each touch pad, defaults to 0x4."]
pub type DAC_R = crate::FieldReader;
#[doc = "Field `DAC` writer - Touch sensor slope control. 3-bit for each touch pad, defaults to 0x4."]
pub type DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RUE` reader - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type RUE_R = crate::BitReader;
#[doc = "Field `RUE` writer - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDE` reader - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type RDE_R = crate::BitReader;
#[doc = "Field `RDE` writer - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRV` reader - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type DRV_R = crate::FieldReader;
#[doc = "Field `DRV` writer - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in sleep mode."]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input enable in sleep mode."]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 0: no sleep mode. 1: enable sleep mode."]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Function selection."]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Connect the RTC pad input to digital pad input. 0 is available."]
    #[inline(always)]
    pub fn mux_sel(&self) -> MUX_SEL_R {
        MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Touch sensor power on."]
    #[inline(always)]
    pub fn xpd(&self) -> XPD_R {
        XPD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The tie option of touch sensor. 0: tie low. 1: tie high."]
    #[inline(always)]
    pub fn tie_opt(&self) -> TIE_OPT_R {
        TIE_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Start touch sensor."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:25 - Touch sensor slope control. 3-bit for each touch pad, defaults to 0x4."]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_PAD")
            .field("fun_ie", &self.fun_ie())
            .field("slp_oe", &self.slp_oe())
            .field("slp_ie", &self.slp_ie())
            .field("slp_sel", &self.slp_sel())
            .field("fun_sel", &self.fun_sel())
            .field("mux_sel", &self.mux_sel())
            .field("xpd", &self.xpd())
            .field("tie_opt", &self.tie_opt())
            .field("start", &self.start())
            .field("dac", &self.dac())
            .field("rue", &self.rue())
            .field("rde", &self.rde())
            .field("drv", &self.drv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W<TOUCH_PAD_SPEC> {
        FUN_IE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Output enable in sleep mode."]
    #[inline(always)]
    pub fn slp_oe(&mut self) -> SLP_OE_W<TOUCH_PAD_SPEC> {
        SLP_OE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Input enable in sleep mode."]
    #[inline(always)]
    pub fn slp_ie(&mut self) -> SLP_IE_W<TOUCH_PAD_SPEC> {
        SLP_IE_W::new(self, 15)
    }
    #[doc = "Bit 16 - 0: no sleep mode. 1: enable sleep mode."]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<TOUCH_PAD_SPEC> {
        SLP_SEL_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Function selection."]
    #[inline(always)]
    pub fn fun_sel(&mut self) -> FUN_SEL_W<TOUCH_PAD_SPEC> {
        FUN_SEL_W::new(self, 17)
    }
    #[doc = "Bit 19 - Connect the RTC pad input to digital pad input. 0 is available."]
    #[inline(always)]
    pub fn mux_sel(&mut self) -> MUX_SEL_W<TOUCH_PAD_SPEC> {
        MUX_SEL_W::new(self, 19)
    }
    #[doc = "Bit 20 - Touch sensor power on."]
    #[inline(always)]
    pub fn xpd(&mut self) -> XPD_W<TOUCH_PAD_SPEC> {
        XPD_W::new(self, 20)
    }
    #[doc = "Bit 21 - The tie option of touch sensor. 0: tie low. 1: tie high."]
    #[inline(always)]
    pub fn tie_opt(&mut self) -> TIE_OPT_W<TOUCH_PAD_SPEC> {
        TIE_OPT_W::new(self, 21)
    }
    #[doc = "Bit 22 - Start touch sensor."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<TOUCH_PAD_SPEC> {
        START_W::new(self, 22)
    }
    #[doc = "Bits 23:25 - Touch sensor slope control. 3-bit for each touch pad, defaults to 0x4."]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W<TOUCH_PAD_SPEC> {
        DAC_W::new(self, 23)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn rue(&mut self) -> RUE_W<TOUCH_PAD_SPEC> {
        RUE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn rde(&mut self) -> RDE_W<TOUCH_PAD_SPEC> {
        RDE_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    pub fn drv(&mut self) -> DRV_W<TOUCH_PAD_SPEC> {
        DRV_W::new(self, 29)
    }
}
#[doc = "Touch pad %s configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_pad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_pad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_PAD_SPEC;
impl crate::RegisterSpec for TOUCH_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_pad::R`](R) reader structure"]
impl crate::Readable for TOUCH_PAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_pad::W`](W) writer structure"]
impl crate::Writable for TOUCH_PAD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_PAD%s to value 0x5200_0000"]
impl crate::Resettable for TOUCH_PAD_SPEC {
    const RESET_VALUE: u32 = 0x5200_0000;
}
