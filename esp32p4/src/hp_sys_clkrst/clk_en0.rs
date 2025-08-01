#[doc = "Register `CLK_EN0` reader"]
pub type R = crate::R<CLK_EN0_SPEC>;
#[doc = "Register `CLK_EN0` writer"]
pub type W = crate::W<CLK_EN0_SPEC>;
#[doc = "Field `CLK_EN` reader - Reserved"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Reserved"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_EN0")
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<CLK_EN0_SPEC> {
        CLK_EN_W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_EN0_SPEC;
impl crate::RegisterSpec for CLK_EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_en0::R`](R) reader structure"]
impl crate::Readable for CLK_EN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_en0::W`](W) writer structure"]
impl crate::Writable for CLK_EN0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_EN0 to value 0x01"]
impl crate::Resettable for CLK_EN0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
