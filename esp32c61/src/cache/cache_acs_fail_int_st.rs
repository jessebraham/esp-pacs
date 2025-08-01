#[doc = "Register `CACHE_ACS_FAIL_INT_ST` reader"]
pub type R = crate::R<CACHE_ACS_FAIL_INT_ST_SPEC>;
#[doc = "Field `ICACHE0_FAIL_INT_ST` reader - The bit indicates the interrupt status of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache."]
pub type ICACHE0_FAIL_INT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE1_FAIL_INT_ST` reader - The bit indicates the interrupt status of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache."]
pub type ICACHE1_FAIL_INT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE2_FAIL_INT_ST` reader - Reserved"]
pub type ICACHE2_FAIL_INT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE3_FAIL_INT_ST` reader - Reserved"]
pub type ICACHE3_FAIL_INT_ST_R = crate::BitReader;
#[doc = "Field `CACHE_FAIL_INT_ST` reader - The bit indicates the interrupt status of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
pub type CACHE_FAIL_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit indicates the interrupt status of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache."]
    #[inline(always)]
    pub fn icache0_fail_int_st(&self) -> ICACHE0_FAIL_INT_ST_R {
        ICACHE0_FAIL_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit indicates the interrupt status of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache."]
    #[inline(always)]
    pub fn icache1_fail_int_st(&self) -> ICACHE1_FAIL_INT_ST_R {
        ICACHE1_FAIL_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_fail_int_st(&self) -> ICACHE2_FAIL_INT_ST_R {
        ICACHE2_FAIL_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn icache3_fail_int_st(&self) -> ICACHE3_FAIL_INT_ST_R {
        ICACHE3_FAIL_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit indicates the interrupt status of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
    #[inline(always)]
    pub fn cache_fail_int_st(&self) -> CACHE_FAIL_INT_ST_R {
        CACHE_FAIL_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ACS_FAIL_INT_ST")
            .field("icache0_fail_int_st", &self.icache0_fail_int_st())
            .field("icache1_fail_int_st", &self.icache1_fail_int_st())
            .field("icache2_fail_int_st", &self.icache2_fail_int_st())
            .field("icache3_fail_int_st", &self.icache3_fail_int_st())
            .field("cache_fail_int_st", &self.cache_fail_int_st())
            .finish()
    }
}
#[doc = "Cache Access Fail Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_fail_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_FAIL_INT_ST_SPEC;
impl crate::RegisterSpec for CACHE_ACS_FAIL_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_acs_fail_int_st::R`](R) reader structure"]
impl crate::Readable for CACHE_ACS_FAIL_INT_ST_SPEC {}
#[doc = "`reset()` method sets CACHE_ACS_FAIL_INT_ST to value 0"]
impl crate::Resettable for CACHE_ACS_FAIL_INT_ST_SPEC {}
