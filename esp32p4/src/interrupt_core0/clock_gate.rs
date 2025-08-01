#[doc = "Register `CLOCK_GATE` reader"]
pub type R = crate::R<CLOCK_GATE_SPEC>;
#[doc = "Register `CLOCK_GATE` writer"]
pub type W = crate::W<CLOCK_GATE_SPEC>;
#[doc = "Field `CORE0_REG_CLK_EN` reader - NA"]
pub type CORE0_REG_CLK_EN_R = crate::BitReader;
#[doc = "Field `CORE0_REG_CLK_EN` writer - NA"]
pub type CORE0_REG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn core0_reg_clk_en(&self) -> CORE0_REG_CLK_EN_R {
        CORE0_REG_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK_GATE")
            .field("core0_reg_clk_en", &self.core0_reg_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn core0_reg_clk_en(&mut self) -> CORE0_REG_CLK_EN_W<CLOCK_GATE_SPEC> {
        CORE0_REG_CLK_EN_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCK_GATE_SPEC;
impl crate::RegisterSpec for CLOCK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_gate::R`](R) reader structure"]
impl crate::Readable for CLOCK_GATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clock_gate::W`](W) writer structure"]
impl crate::Writable for CLOCK_GATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCK_GATE to value 0x01"]
impl crate::Resettable for CLOCK_GATE_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
