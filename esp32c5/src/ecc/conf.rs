#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `START` reader - Configures whether to start calculation of ECC Accelerator. This bit will be self-cleared after the calculation is done. \\\\ 0: No effect\\\\ 1: Start calculation of ECC Accelerator\\\\"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Configures whether to start calculation of ECC Accelerator. This bit will be self-cleared after the calculation is done. \\\\ 0: No effect\\\\ 1: Start calculation of ECC Accelerator\\\\"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` writer - Configures whether to reset ECC Accelerator. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_LENGTH` reader - Configures the key length mode bit of ECC Accelerator. \\\\ 0: P-192\\\\ 1: P-256\\\\"]
pub type KEY_LENGTH_R = crate::BitReader;
#[doc = "Field `KEY_LENGTH` writer - Configures the key length mode bit of ECC Accelerator. \\\\ 0: P-192\\\\ 1: P-256\\\\"]
pub type KEY_LENGTH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD_BASE` reader - Configures the mod base of mod operation, only valid in work_mode 8-11. \\\\ 0: n(order of curve)\\\\ 1: p(mod base of curve)\\\\"]
pub type MOD_BASE_R = crate::BitReader;
#[doc = "Field `MOD_BASE` writer - Configures the mod base of mod operation, only valid in work_mode 8-11. \\\\ 0: n(order of curve)\\\\ 1: p(mod base of curve)\\\\"]
pub type MOD_BASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_MODE` reader - Configures the work mode of ECC Accelerator.\\\\ 0: Point Multi mode\\\\ 1: Reserved\\\\ 2: Point Verif mode\\\\ 3: Point Verif + Multi mode\\\\ 4: Jacobian Point Multi mode\\\\ 5: Reserved\\\\ 6: Jacobian Point Verif mode\\\\ 7: Point Verif + Jacobian Point Multi mode\\\\ 8: Mod Add mode\\\\ 9. Mod Sub mode\\\\ 10: Mod Multi mode\\\\ 11: Mod Div mode\\\\"]
pub type WORK_MODE_R = crate::FieldReader;
#[doc = "Field `WORK_MODE` writer - Configures the work mode of ECC Accelerator.\\\\ 0: Point Multi mode\\\\ 1: Reserved\\\\ 2: Point Verif mode\\\\ 3: Point Verif + Multi mode\\\\ 4: Jacobian Point Multi mode\\\\ 5: Reserved\\\\ 6: Jacobian Point Verif mode\\\\ 7: Point Verif + Jacobian Point Multi mode\\\\ 8: Mod Add mode\\\\ 9. Mod Sub mode\\\\ 10: Mod Multi mode\\\\ 11: Mod Div mode\\\\"]
pub type WORK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SECURITY_MODE` reader - Configures the security mode of ECC Accelerator.\\\\ 0: no secure function enabled.\\\\ 1: enable constant-time calculation in all point multiplication modes.\\\\"]
pub type SECURITY_MODE_R = crate::BitReader;
#[doc = "Field `SECURITY_MODE` writer - Configures the security mode of ECC Accelerator.\\\\ 0: no secure function enabled.\\\\ 1: enable constant-time calculation in all point multiplication modes.\\\\"]
pub type SECURITY_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VERIFICATION_RESULT` reader - Represents the verification result of ECC Accelerator, valid only when calculation is done."]
pub type VERIFICATION_RESULT_R = crate::BitReader;
#[doc = "Field `CLK_EN` reader - Configures whether to force on register clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Configures whether to force on register clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CLOCK_GATE_FORCE_ON` reader - Configures whether to force on ECC memory clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
pub type MEM_CLOCK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MEM_CLOCK_GATE_FORCE_ON` writer - Configures whether to force on ECC memory clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
pub type MEM_CLOCK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether to start calculation of ECC Accelerator. This bit will be self-cleared after the calculation is done. \\\\ 0: No effect\\\\ 1: Start calculation of ECC Accelerator\\\\"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Configures the key length mode bit of ECC Accelerator. \\\\ 0: P-192\\\\ 1: P-256\\\\"]
    #[inline(always)]
    pub fn key_length(&self) -> KEY_LENGTH_R {
        KEY_LENGTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures the mod base of mod operation, only valid in work_mode 8-11. \\\\ 0: n(order of curve)\\\\ 1: p(mod base of curve)\\\\"]
    #[inline(always)]
    pub fn mod_base(&self) -> MOD_BASE_R {
        MOD_BASE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Configures the work mode of ECC Accelerator.\\\\ 0: Point Multi mode\\\\ 1: Reserved\\\\ 2: Point Verif mode\\\\ 3: Point Verif + Multi mode\\\\ 4: Jacobian Point Multi mode\\\\ 5: Reserved\\\\ 6: Jacobian Point Verif mode\\\\ 7: Point Verif + Jacobian Point Multi mode\\\\ 8: Mod Add mode\\\\ 9. Mod Sub mode\\\\ 10: Mod Multi mode\\\\ 11: Mod Div mode\\\\"]
    #[inline(always)]
    pub fn work_mode(&self) -> WORK_MODE_R {
        WORK_MODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Configures the security mode of ECC Accelerator.\\\\ 0: no secure function enabled.\\\\ 1: enable constant-time calculation in all point multiplication modes.\\\\"]
    #[inline(always)]
    pub fn security_mode(&self) -> SECURITY_MODE_R {
        SECURITY_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents the verification result of ECC Accelerator, valid only when calculation is done."]
    #[inline(always)]
    pub fn verification_result(&self) -> VERIFICATION_RESULT_R {
        VERIFICATION_RESULT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Configures whether to force on register clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configures whether to force on ECC memory clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
    #[inline(always)]
    pub fn mem_clock_gate_force_on(&self) -> MEM_CLOCK_GATE_FORCE_ON_R {
        MEM_CLOCK_GATE_FORCE_ON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("start", &self.start())
            .field("key_length", &self.key_length())
            .field("mod_base", &self.mod_base())
            .field("work_mode", &self.work_mode())
            .field("security_mode", &self.security_mode())
            .field("verification_result", &self.verification_result())
            .field("clk_en", &self.clk_en())
            .field("mem_clock_gate_force_on", &self.mem_clock_gate_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to start calculation of ECC Accelerator. This bit will be self-cleared after the calculation is done. \\\\ 0: No effect\\\\ 1: Start calculation of ECC Accelerator\\\\"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<CONF_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether to reset ECC Accelerator. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<CONF_SPEC> {
        RESET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures the key length mode bit of ECC Accelerator. \\\\ 0: P-192\\\\ 1: P-256\\\\"]
    #[inline(always)]
    pub fn key_length(&mut self) -> KEY_LENGTH_W<CONF_SPEC> {
        KEY_LENGTH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures the mod base of mod operation, only valid in work_mode 8-11. \\\\ 0: n(order of curve)\\\\ 1: p(mod base of curve)\\\\"]
    #[inline(always)]
    pub fn mod_base(&mut self) -> MOD_BASE_W<CONF_SPEC> {
        MOD_BASE_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - Configures the work mode of ECC Accelerator.\\\\ 0: Point Multi mode\\\\ 1: Reserved\\\\ 2: Point Verif mode\\\\ 3: Point Verif + Multi mode\\\\ 4: Jacobian Point Multi mode\\\\ 5: Reserved\\\\ 6: Jacobian Point Verif mode\\\\ 7: Point Verif + Jacobian Point Multi mode\\\\ 8: Mod Add mode\\\\ 9. Mod Sub mode\\\\ 10: Mod Multi mode\\\\ 11: Mod Div mode\\\\"]
    #[inline(always)]
    pub fn work_mode(&mut self) -> WORK_MODE_W<CONF_SPEC> {
        WORK_MODE_W::new(self, 4)
    }
    #[doc = "Bit 8 - Configures the security mode of ECC Accelerator.\\\\ 0: no secure function enabled.\\\\ 1: enable constant-time calculation in all point multiplication modes.\\\\"]
    #[inline(always)]
    pub fn security_mode(&mut self) -> SECURITY_MODE_W<CONF_SPEC> {
        SECURITY_MODE_W::new(self, 8)
    }
    #[doc = "Bit 30 - Configures whether to force on register clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF_SPEC> {
        CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether to force on ECC memory clock gate. \\\\ 0: No effect\\\\ 1: Force on\\\\"]
    #[inline(always)]
    pub fn mem_clock_gate_force_on(&mut self) -> MEM_CLOCK_GATE_FORCE_ON_W<CONF_SPEC> {
        MEM_CLOCK_GATE_FORCE_ON_W::new(self, 31)
    }
}
#[doc = "ECC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {}
