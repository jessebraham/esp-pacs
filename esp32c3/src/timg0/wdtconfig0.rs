#[doc = "Register `WDTCONFIG0` reader"]
pub struct R(crate::R<WDTCONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCONFIG0` writer"]
pub struct W(crate::W<WDTCONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WDTCONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_APPCPU_RESET_EN` reader - reg_wdt_appcpu_reset_en."]
pub type WDT_APPCPU_RESET_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_APPCPU_RESET_EN` writer - reg_wdt_appcpu_reset_en."]
pub type WDT_APPCPU_RESET_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WDTCONFIG0_SPEC, bool, O>;
#[doc = "Field `WDT_PROCPU_RESET_EN` reader - reg_wdt_procpu_reset_en."]
pub type WDT_PROCPU_RESET_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_PROCPU_RESET_EN` writer - reg_wdt_procpu_reset_en."]
pub type WDT_PROCPU_RESET_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WDTCONFIG0_SPEC, bool, O>;
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` reader - reg_wdt_flashboot_mod_en."]
pub type WDT_FLASHBOOT_MOD_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` writer - reg_wdt_flashboot_mod_en."]
pub type WDT_FLASHBOOT_MOD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WDTCONFIG0_SPEC, bool, O>;
#[doc = "Field `WDT_SYS_RESET_LENGTH` reader - reg_wdt_sys_reset_length."]
pub type WDT_SYS_RESET_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_SYS_RESET_LENGTH` writer - reg_wdt_sys_reset_length."]
pub type WDT_SYS_RESET_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WDTCONFIG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `WDT_CPU_RESET_LENGTH` reader - reg_wdt_cpu_reset_length."]
pub type WDT_CPU_RESET_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_CPU_RESET_LENGTH` writer - reg_wdt_cpu_reset_length."]
pub type WDT_CPU_RESET_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WDTCONFIG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `WDT_USE_XTAL` reader - reg_wdt_use_xtal."]
pub type WDT_USE_XTAL_R = crate::BitReader<bool>;
#[doc = "Field `WDT_USE_XTAL` writer - reg_wdt_use_xtal."]
pub type WDT_USE_XTAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDTCONFIG0_SPEC, bool, O>;
#[doc = "Field `WDT_CONF_UPDATE_EN` writer - reg_wdt_conf_update_en."]
pub type WDT_CONF_UPDATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WDTCONFIG0_SPEC, bool, O>;
#[doc = "Field `WDT_STG3` reader - reg_wdt_stg3."]
pub type WDT_STG3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_STG3` writer - reg_wdt_stg3."]
pub type WDT_STG3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTCONFIG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `WDT_STG2` reader - reg_wdt_stg2."]
pub type WDT_STG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_STG2` writer - reg_wdt_stg2."]
pub type WDT_STG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTCONFIG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `WDT_STG1` reader - reg_wdt_stg1."]
pub type WDT_STG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_STG1` writer - reg_wdt_stg1."]
pub type WDT_STG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTCONFIG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `WDT_STG0` reader - reg_wdt_stg0."]
pub type WDT_STG0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_STG0` writer - reg_wdt_stg0."]
pub type WDT_STG0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTCONFIG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `WDT_EN` reader - reg_wdt_en."]
pub type WDT_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_EN` writer - reg_wdt_en."]
pub type WDT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDTCONFIG0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12 - reg_wdt_appcpu_reset_en."]
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&self) -> WDT_APPCPU_RESET_EN_R {
        WDT_APPCPU_RESET_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_wdt_procpu_reset_en."]
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&self) -> WDT_PROCPU_RESET_EN_R {
        WDT_PROCPU_RESET_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reg_wdt_flashboot_mod_en."]
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&self) -> WDT_FLASHBOOT_MOD_EN_R {
        WDT_FLASHBOOT_MOD_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:17 - reg_wdt_sys_reset_length."]
    #[inline(always)]
    pub fn wdt_sys_reset_length(&self) -> WDT_SYS_RESET_LENGTH_R {
        WDT_SYS_RESET_LENGTH_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - reg_wdt_cpu_reset_length."]
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&self) -> WDT_CPU_RESET_LENGTH_R {
        WDT_CPU_RESET_LENGTH_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - reg_wdt_use_xtal."]
    #[inline(always)]
    pub fn wdt_use_xtal(&self) -> WDT_USE_XTAL_R {
        WDT_USE_XTAL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 23:24 - reg_wdt_stg3."]
    #[inline(always)]
    pub fn wdt_stg3(&self) -> WDT_STG3_R {
        WDT_STG3_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - reg_wdt_stg2."]
    #[inline(always)]
    pub fn wdt_stg2(&self) -> WDT_STG2_R {
        WDT_STG2_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - reg_wdt_stg1."]
    #[inline(always)]
    pub fn wdt_stg1(&self) -> WDT_STG1_R {
        WDT_STG1_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - reg_wdt_stg0."]
    #[inline(always)]
    pub fn wdt_stg0(&self) -> WDT_STG0_R {
        WDT_STG0_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - reg_wdt_en."]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - reg_wdt_appcpu_reset_en."]
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&mut self) -> WDT_APPCPU_RESET_EN_W<12> {
        WDT_APPCPU_RESET_EN_W::new(self)
    }
    #[doc = "Bit 13 - reg_wdt_procpu_reset_en."]
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&mut self) -> WDT_PROCPU_RESET_EN_W<13> {
        WDT_PROCPU_RESET_EN_W::new(self)
    }
    #[doc = "Bit 14 - reg_wdt_flashboot_mod_en."]
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&mut self) -> WDT_FLASHBOOT_MOD_EN_W<14> {
        WDT_FLASHBOOT_MOD_EN_W::new(self)
    }
    #[doc = "Bits 15:17 - reg_wdt_sys_reset_length."]
    #[inline(always)]
    pub fn wdt_sys_reset_length(&mut self) -> WDT_SYS_RESET_LENGTH_W<15> {
        WDT_SYS_RESET_LENGTH_W::new(self)
    }
    #[doc = "Bits 18:20 - reg_wdt_cpu_reset_length."]
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&mut self) -> WDT_CPU_RESET_LENGTH_W<18> {
        WDT_CPU_RESET_LENGTH_W::new(self)
    }
    #[doc = "Bit 21 - reg_wdt_use_xtal."]
    #[inline(always)]
    pub fn wdt_use_xtal(&mut self) -> WDT_USE_XTAL_W<21> {
        WDT_USE_XTAL_W::new(self)
    }
    #[doc = "Bit 22 - reg_wdt_conf_update_en."]
    #[inline(always)]
    pub fn wdt_conf_update_en(&mut self) -> WDT_CONF_UPDATE_EN_W<22> {
        WDT_CONF_UPDATE_EN_W::new(self)
    }
    #[doc = "Bits 23:24 - reg_wdt_stg3."]
    #[inline(always)]
    pub fn wdt_stg3(&mut self) -> WDT_STG3_W<23> {
        WDT_STG3_W::new(self)
    }
    #[doc = "Bits 25:26 - reg_wdt_stg2."]
    #[inline(always)]
    pub fn wdt_stg2(&mut self) -> WDT_STG2_W<25> {
        WDT_STG2_W::new(self)
    }
    #[doc = "Bits 27:28 - reg_wdt_stg1."]
    #[inline(always)]
    pub fn wdt_stg1(&mut self) -> WDT_STG1_W<27> {
        WDT_STG1_W::new(self)
    }
    #[doc = "Bits 29:30 - reg_wdt_stg0."]
    #[inline(always)]
    pub fn wdt_stg0(&mut self) -> WDT_STG0_W<29> {
        WDT_STG0_W::new(self)
    }
    #[doc = "Bit 31 - reg_wdt_en."]
    #[inline(always)]
    pub fn wdt_en(&mut self) -> WDT_EN_W<31> {
        WDT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMG_WDTCONFIG0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig0](index.html) module"]
pub struct WDTCONFIG0_SPEC;
impl crate::RegisterSpec for WDTCONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtconfig0::R](R) reader structure"]
impl crate::Readable for WDTCONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtconfig0::W](W) writer structure"]
impl crate::Writable for WDTCONFIG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCONFIG0 to value 0x0004_c000"]
impl crate::Resettable for WDTCONFIG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_c000
    }
}