#[doc = "Register `GEN%s_TSTMP_B` reader"]
pub type R = crate::R<GEN_TSTMP_B_SPEC>;
#[doc = "Register `GEN%s_TSTMP_B` writer"]
pub type W = crate::W<GEN_TSTMP_B_SPEC>;
#[doc = "Field `CMPR_B` reader - Configures the value of PWM generator %s time stamp B's shadow register."]
pub type CMPR_B_R = crate::FieldReader<u16>;
#[doc = "Field `CMPR_B` writer - Configures the value of PWM generator %s time stamp B's shadow register."]
pub type CMPR_B_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the value of PWM generator %s time stamp B's shadow register."]
    #[inline(always)]
    pub fn cmpr_b(&self) -> CMPR_B_R {
        CMPR_B_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN_TSTMP_B")
            .field("cmpr_b", &self.cmpr_b())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the value of PWM generator %s time stamp B's shadow register."]
    #[inline(always)]
    pub fn cmpr_b(&mut self) -> CMPR_B_W<GEN_TSTMP_B_SPEC> {
        CMPR_B_W::new(self, 0)
    }
}
#[doc = "Generator%s time stamp B's shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_tstmp_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_tstmp_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN_TSTMP_B_SPEC;
impl crate::RegisterSpec for GEN_TSTMP_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_tstmp_b::R`](R) reader structure"]
impl crate::Readable for GEN_TSTMP_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen_tstmp_b::W`](W) writer structure"]
impl crate::Writable for GEN_TSTMP_B_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GEN%s_TSTMP_B to value 0"]
impl crate::Resettable for GEN_TSTMP_B_SPEC {}
