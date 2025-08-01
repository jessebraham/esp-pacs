#[doc = "Register `BS_PD_CTRL` reader"]
pub type R = crate::R<BS_PD_CTRL_SPEC>;
#[doc = "Register `BS_PD_CTRL` writer"]
pub type W = crate::W<BS_PD_CTRL_SPEC>;
#[doc = "Field `BS_MEM_FORCE_PU` reader - Set this bit to force power up bs memory."]
pub type BS_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `BS_MEM_FORCE_PU` writer - Set this bit to force power up bs memory."]
pub type BS_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_MEM_FORCE_PD` reader - Set this bit to force power down bs memory."]
pub type BS_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `BS_MEM_FORCE_PD` writer - Set this bit to force power down bs memory."]
pub type BS_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set this bit to force power up bs memory."]
    #[inline(always)]
    pub fn bs_mem_force_pu(&self) -> BS_MEM_FORCE_PU_R {
        BS_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power down bs memory."]
    #[inline(always)]
    pub fn bs_mem_force_pd(&self) -> BS_MEM_FORCE_PD_R {
        BS_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BS_PD_CTRL")
            .field("bs_mem_force_pu", &self.bs_mem_force_pu())
            .field("bs_mem_force_pd", &self.bs_mem_force_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to force power up bs memory."]
    #[inline(always)]
    pub fn bs_mem_force_pu(&mut self) -> BS_MEM_FORCE_PU_W<BS_PD_CTRL_SPEC> {
        BS_MEM_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force power down bs memory."]
    #[inline(always)]
    pub fn bs_mem_force_pd(&mut self) -> BS_MEM_FORCE_PD_W<BS_PD_CTRL_SPEC> {
        BS_MEM_FORCE_PD_W::new(self, 2)
    }
}
#[doc = "BS power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BS_PD_CTRL_SPEC;
impl crate::RegisterSpec for BS_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bs_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for BS_PD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bs_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for BS_PD_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BS_PD_CTRL to value 0x04"]
impl crate::Resettable for BS_PD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
