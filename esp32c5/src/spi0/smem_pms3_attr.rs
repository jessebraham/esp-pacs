#[doc = "Register `SMEM_PMS3_ATTR` reader"]
pub type R = crate::R<SMEM_PMS3_ATTR_SPEC>;
#[doc = "Register `SMEM_PMS3_ATTR` writer"]
pub type W = crate::W<SMEM_PMS3_ATTR_SPEC>;
#[doc = "Field `SMEM_PMS3_RD_ATTR` reader - 1: SPI1 external RAM PMS section 3 read accessible. 0: Not allowed."]
pub type SMEM_PMS3_RD_ATTR_R = crate::BitReader;
#[doc = "Field `SMEM_PMS3_RD_ATTR` writer - 1: SPI1 external RAM PMS section 3 read accessible. 0: Not allowed."]
pub type SMEM_PMS3_RD_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_PMS3_WR_ATTR` reader - 1: SPI1 external RAM PMS section 3 write accessible. 0: Not allowed."]
pub type SMEM_PMS3_WR_ATTR_R = crate::BitReader;
#[doc = "Field `SMEM_PMS3_WR_ATTR` writer - 1: SPI1 external RAM PMS section 3 write accessible. 0: Not allowed."]
pub type SMEM_PMS3_WR_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_PMS3_ECC` reader - SPI1 external RAM PMS section 3 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 3 is configured by registers SPI_SMEM_PMS3_ADDR_REG and SPI_SMEM_PMS3_SIZE_REG."]
pub type SMEM_PMS3_ECC_R = crate::BitReader;
#[doc = "Field `SMEM_PMS3_ECC` writer - SPI1 external RAM PMS section 3 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 3 is configured by registers SPI_SMEM_PMS3_ADDR_REG and SPI_SMEM_PMS3_SIZE_REG."]
pub type SMEM_PMS3_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: SPI1 external RAM PMS section 3 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn smem_pms3_rd_attr(&self) -> SMEM_PMS3_RD_ATTR_R {
        SMEM_PMS3_RD_ATTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: SPI1 external RAM PMS section 3 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn smem_pms3_wr_attr(&self) -> SMEM_PMS3_WR_ATTR_R {
        SMEM_PMS3_WR_ATTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI1 external RAM PMS section 3 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 3 is configured by registers SPI_SMEM_PMS3_ADDR_REG and SPI_SMEM_PMS3_SIZE_REG."]
    #[inline(always)]
    pub fn smem_pms3_ecc(&self) -> SMEM_PMS3_ECC_R {
        SMEM_PMS3_ECC_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_PMS3_ATTR")
            .field("smem_pms3_rd_attr", &self.smem_pms3_rd_attr())
            .field("smem_pms3_wr_attr", &self.smem_pms3_wr_attr())
            .field("smem_pms3_ecc", &self.smem_pms3_ecc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1: SPI1 external RAM PMS section 3 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn smem_pms3_rd_attr(&mut self) -> SMEM_PMS3_RD_ATTR_W<SMEM_PMS3_ATTR_SPEC> {
        SMEM_PMS3_RD_ATTR_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: SPI1 external RAM PMS section 3 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn smem_pms3_wr_attr(&mut self) -> SMEM_PMS3_WR_ATTR_W<SMEM_PMS3_ATTR_SPEC> {
        SMEM_PMS3_WR_ATTR_W::new(self, 1)
    }
    #[doc = "Bit 2 - SPI1 external RAM PMS section 3 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 3 is configured by registers SPI_SMEM_PMS3_ADDR_REG and SPI_SMEM_PMS3_SIZE_REG."]
    #[inline(always)]
    pub fn smem_pms3_ecc(&mut self) -> SMEM_PMS3_ECC_W<SMEM_PMS3_ATTR_SPEC> {
        SMEM_PMS3_ECC_W::new(self, 2)
    }
}
#[doc = "SPI1 external RAM PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_pms3_attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_pms3_attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_PMS3_ATTR_SPEC;
impl crate::RegisterSpec for SMEM_PMS3_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_pms3_attr::R`](R) reader structure"]
impl crate::Readable for SMEM_PMS3_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_pms3_attr::W`](W) writer structure"]
impl crate::Writable for SMEM_PMS3_ATTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_PMS3_ATTR to value 0x03"]
impl crate::Resettable for SMEM_PMS3_ATTR_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
