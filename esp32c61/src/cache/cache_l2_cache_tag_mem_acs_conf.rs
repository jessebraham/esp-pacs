#[doc = "Register `CACHE_L2_CACHE_TAG_MEM_ACS_CONF` reader"]
pub type R = crate::R<CACHE_L2_CACHE_TAG_MEM_ACS_CONF_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_TAG_MEM_RD_EN` reader - The bit is used to enable config-bus read L2-Cache tag memoryory. 0: disable, 1: enable."]
pub type CACHE_L2_CACHE_TAG_MEM_RD_EN_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_TAG_MEM_WR_EN` reader - The bit is used to enable config-bus write L2-Cache tag memoryory. 0: disable, 1: enable."]
pub type CACHE_L2_CACHE_TAG_MEM_WR_EN_R = crate::BitReader;
impl R {
    #[doc = "Bit 20 - The bit is used to enable config-bus read L2-Cache tag memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn cache_l2_cache_tag_mem_rd_en(&self) -> CACHE_L2_CACHE_TAG_MEM_RD_EN_R {
        CACHE_L2_CACHE_TAG_MEM_RD_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The bit is used to enable config-bus write L2-Cache tag memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn cache_l2_cache_tag_mem_wr_en(&self) -> CACHE_L2_CACHE_TAG_MEM_WR_EN_R {
        CACHE_L2_CACHE_TAG_MEM_WR_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_TAG_MEM_ACS_CONF")
            .field(
                "cache_l2_cache_tag_mem_rd_en",
                &self.cache_l2_cache_tag_mem_rd_en(),
            )
            .field(
                "cache_l2_cache_tag_mem_wr_en",
                &self.cache_l2_cache_tag_mem_wr_en(),
            )
            .finish()
    }
}
#[doc = "Cache tag memory access configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_tag_mem_acs_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_TAG_MEM_ACS_CONF_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_TAG_MEM_ACS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_tag_mem_acs_conf::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_TAG_MEM_ACS_CONF_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_TAG_MEM_ACS_CONF to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_TAG_MEM_ACS_CONF_SPEC {}
