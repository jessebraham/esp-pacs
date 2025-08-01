#[doc = "Register `L2_MEM_RDN_ECO_CS` reader"]
pub type R = crate::R<L2_MEM_RDN_ECO_CS_SPEC>;
#[doc = "Register `L2_MEM_RDN_ECO_CS` writer"]
pub type W = crate::W<L2_MEM_RDN_ECO_CS_SPEC>;
#[doc = "Field `REG_L2_MEM_RDN_ECO_EN` reader - NA"]
pub type REG_L2_MEM_RDN_ECO_EN_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_RDN_ECO_EN` writer - NA"]
pub type REG_L2_MEM_RDN_ECO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_RDN_ECO_RESULT` reader - NA"]
pub type REG_L2_MEM_RDN_ECO_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_rdn_eco_en(&self) -> REG_L2_MEM_RDN_ECO_EN_R {
        REG_L2_MEM_RDN_ECO_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_rdn_eco_result(&self) -> REG_L2_MEM_RDN_ECO_RESULT_R {
        REG_L2_MEM_RDN_ECO_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_RDN_ECO_CS")
            .field("reg_l2_mem_rdn_eco_en", &self.reg_l2_mem_rdn_eco_en())
            .field(
                "reg_l2_mem_rdn_eco_result",
                &self.reg_l2_mem_rdn_eco_result(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_rdn_eco_en(&mut self) -> REG_L2_MEM_RDN_ECO_EN_W<L2_MEM_RDN_ECO_CS_SPEC> {
        REG_L2_MEM_RDN_ECO_EN_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_rdn_eco_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_rdn_eco_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_MEM_RDN_ECO_CS_SPEC;
impl crate::RegisterSpec for L2_MEM_RDN_ECO_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_rdn_eco_cs::R`](R) reader structure"]
impl crate::Readable for L2_MEM_RDN_ECO_CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_mem_rdn_eco_cs::W`](W) writer structure"]
impl crate::Writable for L2_MEM_RDN_ECO_CS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_MEM_RDN_ECO_CS to value 0"]
impl crate::Resettable for L2_MEM_RDN_ECO_CS_SPEC {}
