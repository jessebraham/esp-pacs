#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `TARGET(0-2)` writer - Interrupt clear bit of system timer target %s."]
pub type TARGET_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Interrupt clear bit of system timer target (0-2)."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TARGET0` field.</div>"]
    #[inline(always)]
    pub fn target(&mut self, n: u8) -> TARGET_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TARGET_W::new(self, n)
    }
    #[doc = "Bit 0 - Interrupt clear bit of system timer target 0."]
    #[inline(always)]
    pub fn target0(&mut self) -> TARGET_W<INT_CLR_SPEC> {
        TARGET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt clear bit of system timer target 1."]
    #[inline(always)]
    pub fn target1(&mut self) -> TARGET_W<INT_CLR_SPEC> {
        TARGET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt clear bit of system timer target 2."]
    #[inline(always)]
    pub fn target2(&mut self) -> TARGET_W<INT_CLR_SPEC> {
        TARGET_W::new(self, 2)
    }
}
#[doc = "System timer interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
