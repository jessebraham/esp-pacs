#[doc = "Register `LEDC_PD_CTRL` reader"]
pub type R = crate::R<LEDC_PD_CTRL_SPEC>;
#[doc = "Register `LEDC_PD_CTRL` writer"]
pub type W = crate::W<LEDC_PD_CTRL_SPEC>;
#[doc = "Field `LEDC_MEM_FORCE_PU` reader - Set this bit to force power down LEDC memory."]
pub type LEDC_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `LEDC_MEM_FORCE_PU` writer - Set this bit to force power down LEDC memory."]
pub type LEDC_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_MEM_FORCE_PD` reader - Set this bit to force power up LEDC memory."]
pub type LEDC_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `LEDC_MEM_FORCE_PD` writer - Set this bit to force power up LEDC memory."]
pub type LEDC_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set this bit to force power down LEDC memory."]
    #[inline(always)]
    pub fn ledc_mem_force_pu(&self) -> LEDC_MEM_FORCE_PU_R {
        LEDC_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power up LEDC memory."]
    #[inline(always)]
    pub fn ledc_mem_force_pd(&self) -> LEDC_MEM_FORCE_PD_R {
        LEDC_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LEDC_PD_CTRL")
            .field("ledc_mem_force_pu", &self.ledc_mem_force_pu())
            .field("ledc_mem_force_pd", &self.ledc_mem_force_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to force power down LEDC memory."]
    #[inline(always)]
    pub fn ledc_mem_force_pu(&mut self) -> LEDC_MEM_FORCE_PU_W<LEDC_PD_CTRL_SPEC> {
        LEDC_MEM_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force power up LEDC memory."]
    #[inline(always)]
    pub fn ledc_mem_force_pd(&mut self) -> LEDC_MEM_FORCE_PD_W<LEDC_PD_CTRL_SPEC> {
        LEDC_MEM_FORCE_PD_W::new(self, 2)
    }
}
#[doc = "LEDC power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ledc_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LEDC_PD_CTRL_SPEC;
impl crate::RegisterSpec for LEDC_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ledc_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for LEDC_PD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ledc_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for LEDC_PD_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEDC_PD_CTRL to value 0x02"]
impl crate::Resettable for LEDC_PD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
