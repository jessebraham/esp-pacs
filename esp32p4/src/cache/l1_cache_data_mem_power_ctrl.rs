#[doc = "Register `L1_CACHE_DATA_MEM_POWER_CTRL` reader"]
pub type R = crate::R<L1_CACHE_DATA_MEM_POWER_CTRL_SPEC>;
#[doc = "Register `L1_CACHE_DATA_MEM_POWER_CTRL` writer"]
pub type W = crate::W<L1_CACHE_DATA_MEM_POWER_CTRL_SPEC>;
#[doc = "Field `L1_ICACHE0_DATA_MEM_FORCE_ON` reader - The bit is used to close clock gating of L1-ICache0 data memory. 1: close gating, 0: open clock gating."]
pub type L1_ICACHE0_DATA_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_DATA_MEM_FORCE_ON` writer - The bit is used to close clock gating of L1-ICache0 data memory. 1: close gating, 0: open clock gating."]
pub type L1_ICACHE0_DATA_MEM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE0_DATA_MEM_FORCE_PD` reader - The bit is used to power L1-ICache0 data memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1_ICACHE0_DATA_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_DATA_MEM_FORCE_PD` writer - The bit is used to power L1-ICache0 data memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1_ICACHE0_DATA_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE0_DATA_MEM_FORCE_PU` reader - The bit is used to power L1-ICache0 data memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1_ICACHE0_DATA_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_DATA_MEM_FORCE_PU` writer - The bit is used to power L1-ICache0 data memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1_ICACHE0_DATA_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_DATA_MEM_FORCE_ON` reader - The bit is used to close clock gating of L1-ICache1 data memory. 1: close gating, 0: open clock gating."]
pub type L1_ICACHE1_DATA_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_DATA_MEM_FORCE_ON` writer - The bit is used to close clock gating of L1-ICache1 data memory. 1: close gating, 0: open clock gating."]
pub type L1_ICACHE1_DATA_MEM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_DATA_MEM_FORCE_PD` reader - The bit is used to power L1-ICache1 data memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1_ICACHE1_DATA_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_DATA_MEM_FORCE_PD` writer - The bit is used to power L1-ICache1 data memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1_ICACHE1_DATA_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_DATA_MEM_FORCE_PU` reader - The bit is used to power L1-ICache1 data memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1_ICACHE1_DATA_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_DATA_MEM_FORCE_PU` writer - The bit is used to power L1-ICache1 data memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1_ICACHE1_DATA_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_DATA_MEM_FORCE_ON` reader - Reserved"]
pub type L1_ICACHE2_DATA_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_DATA_MEM_FORCE_PD` reader - Reserved"]
pub type L1_ICACHE2_DATA_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_DATA_MEM_FORCE_PU` reader - Reserved"]
pub type L1_ICACHE2_DATA_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_DATA_MEM_FORCE_ON` reader - Reserved"]
pub type L1_ICACHE3_DATA_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_DATA_MEM_FORCE_PD` reader - Reserved"]
pub type L1_ICACHE3_DATA_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_DATA_MEM_FORCE_PU` reader - Reserved"]
pub type L1_ICACHE3_DATA_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_DATA_MEM_FORCE_ON` reader - The bit is used to close clock gating of L1-DCache data memory. 1: close gating, 0: open clock gating."]
pub type L1_DCACHE_DATA_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_DATA_MEM_FORCE_ON` writer - The bit is used to close clock gating of L1-DCache data memory. 1: close gating, 0: open clock gating."]
pub type L1_DCACHE_DATA_MEM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_DATA_MEM_FORCE_PD` reader - The bit is used to power L1-DCache data memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1_DCACHE_DATA_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_DATA_MEM_FORCE_PD` writer - The bit is used to power L1-DCache data memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1_DCACHE_DATA_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_DATA_MEM_FORCE_PU` reader - The bit is used to power L1-DCache data memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1_DCACHE_DATA_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_DATA_MEM_FORCE_PU` writer - The bit is used to power L1-DCache data memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1_DCACHE_DATA_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to close clock gating of L1-ICache0 data memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_icache0_data_mem_force_on(&self) -> L1_ICACHE0_DATA_MEM_FORCE_ON_R {
        L1_ICACHE0_DATA_MEM_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to power L1-ICache0 data memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_icache0_data_mem_force_pd(&self) -> L1_ICACHE0_DATA_MEM_FORCE_PD_R {
        L1_ICACHE0_DATA_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to power L1-ICache0 data memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_icache0_data_mem_force_pu(&self) -> L1_ICACHE0_DATA_MEM_FORCE_PU_R {
        L1_ICACHE0_DATA_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to close clock gating of L1-ICache1 data memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_icache1_data_mem_force_on(&self) -> L1_ICACHE1_DATA_MEM_FORCE_ON_R {
        L1_ICACHE1_DATA_MEM_FORCE_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to power L1-ICache1 data memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_icache1_data_mem_force_pd(&self) -> L1_ICACHE1_DATA_MEM_FORCE_PD_R {
        L1_ICACHE1_DATA_MEM_FORCE_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is used to power L1-ICache1 data memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_icache1_data_mem_force_pu(&self) -> L1_ICACHE1_DATA_MEM_FORCE_PU_R {
        L1_ICACHE1_DATA_MEM_FORCE_PU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_data_mem_force_on(&self) -> L1_ICACHE2_DATA_MEM_FORCE_ON_R {
        L1_ICACHE2_DATA_MEM_FORCE_ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_data_mem_force_pd(&self) -> L1_ICACHE2_DATA_MEM_FORCE_PD_R {
        L1_ICACHE2_DATA_MEM_FORCE_PD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_data_mem_force_pu(&self) -> L1_ICACHE2_DATA_MEM_FORCE_PU_R {
        L1_ICACHE2_DATA_MEM_FORCE_PU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_data_mem_force_on(&self) -> L1_ICACHE3_DATA_MEM_FORCE_ON_R {
        L1_ICACHE3_DATA_MEM_FORCE_ON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_data_mem_force_pd(&self) -> L1_ICACHE3_DATA_MEM_FORCE_PD_R {
        L1_ICACHE3_DATA_MEM_FORCE_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_data_mem_force_pu(&self) -> L1_ICACHE3_DATA_MEM_FORCE_PU_R {
        L1_ICACHE3_DATA_MEM_FORCE_PU_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - The bit is used to close clock gating of L1-DCache data memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_dcache_data_mem_force_on(&self) -> L1_DCACHE_DATA_MEM_FORCE_ON_R {
        L1_DCACHE_DATA_MEM_FORCE_ON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The bit is used to power L1-DCache data memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_dcache_data_mem_force_pd(&self) -> L1_DCACHE_DATA_MEM_FORCE_PD_R {
        L1_DCACHE_DATA_MEM_FORCE_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to power L1-DCache data memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_dcache_data_mem_force_pu(&self) -> L1_DCACHE_DATA_MEM_FORCE_PU_R {
        L1_DCACHE_DATA_MEM_FORCE_PU_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_DATA_MEM_POWER_CTRL")
            .field(
                "l1_icache0_data_mem_force_on",
                &self.l1_icache0_data_mem_force_on(),
            )
            .field(
                "l1_icache0_data_mem_force_pd",
                &self.l1_icache0_data_mem_force_pd(),
            )
            .field(
                "l1_icache0_data_mem_force_pu",
                &self.l1_icache0_data_mem_force_pu(),
            )
            .field(
                "l1_icache1_data_mem_force_on",
                &self.l1_icache1_data_mem_force_on(),
            )
            .field(
                "l1_icache1_data_mem_force_pd",
                &self.l1_icache1_data_mem_force_pd(),
            )
            .field(
                "l1_icache1_data_mem_force_pu",
                &self.l1_icache1_data_mem_force_pu(),
            )
            .field(
                "l1_icache2_data_mem_force_on",
                &self.l1_icache2_data_mem_force_on(),
            )
            .field(
                "l1_icache2_data_mem_force_pd",
                &self.l1_icache2_data_mem_force_pd(),
            )
            .field(
                "l1_icache2_data_mem_force_pu",
                &self.l1_icache2_data_mem_force_pu(),
            )
            .field(
                "l1_icache3_data_mem_force_on",
                &self.l1_icache3_data_mem_force_on(),
            )
            .field(
                "l1_icache3_data_mem_force_pd",
                &self.l1_icache3_data_mem_force_pd(),
            )
            .field(
                "l1_icache3_data_mem_force_pu",
                &self.l1_icache3_data_mem_force_pu(),
            )
            .field(
                "l1_dcache_data_mem_force_on",
                &self.l1_dcache_data_mem_force_on(),
            )
            .field(
                "l1_dcache_data_mem_force_pd",
                &self.l1_dcache_data_mem_force_pd(),
            )
            .field(
                "l1_dcache_data_mem_force_pu",
                &self.l1_dcache_data_mem_force_pu(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to close clock gating of L1-ICache0 data memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_icache0_data_mem_force_on(
        &mut self,
    ) -> L1_ICACHE0_DATA_MEM_FORCE_ON_W<L1_CACHE_DATA_MEM_POWER_CTRL_SPEC> {
        L1_ICACHE0_DATA_MEM_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to power L1-ICache0 data memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_icache0_data_mem_force_pd(
        &mut self,
    ) -> L1_ICACHE0_DATA_MEM_FORCE_PD_W<L1_CACHE_DATA_MEM_POWER_CTRL_SPEC> {
        L1_ICACHE0_DATA_MEM_FORCE_PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to power L1-ICache0 data memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_icache0_data_mem_force_pu(
        &mut self,
    ) -> L1_ICACHE0_DATA_MEM_FORCE_PU_W<L1_CACHE_DATA_MEM_POWER_CTRL_SPEC> {
        L1_ICACHE0_DATA_MEM_FORCE_PU_W::new(self, 2)
    }
    #[doc = "Bit 4 - The bit is used to close clock gating of L1-ICache1 data memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_icache1_data_mem_force_on(
        &mut self,
    ) -> L1_ICACHE1_DATA_MEM_FORCE_ON_W<L1_CACHE_DATA_MEM_POWER_CTRL_SPEC> {
        L1_ICACHE1_DATA_MEM_FORCE_ON_W::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to power L1-ICache1 data memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_icache1_data_mem_force_pd(
        &mut self,
    ) -> L1_ICACHE1_DATA_MEM_FORCE_PD_W<L1_CACHE_DATA_MEM_POWER_CTRL_SPEC> {
        L1_ICACHE1_DATA_MEM_FORCE_PD_W::new(self, 5)
    }
    #[doc = "Bit 6 - The bit is used to power L1-ICache1 data memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_icache1_data_mem_force_pu(
        &mut self,
    ) -> L1_ICACHE1_DATA_MEM_FORCE_PU_W<L1_CACHE_DATA_MEM_POWER_CTRL_SPEC> {
        L1_ICACHE1_DATA_MEM_FORCE_PU_W::new(self, 6)
    }
    #[doc = "Bit 16 - The bit is used to close clock gating of L1-DCache data memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_dcache_data_mem_force_on(
        &mut self,
    ) -> L1_DCACHE_DATA_MEM_FORCE_ON_W<L1_CACHE_DATA_MEM_POWER_CTRL_SPEC> {
        L1_DCACHE_DATA_MEM_FORCE_ON_W::new(self, 16)
    }
    #[doc = "Bit 17 - The bit is used to power L1-DCache data memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_dcache_data_mem_force_pd(
        &mut self,
    ) -> L1_DCACHE_DATA_MEM_FORCE_PD_W<L1_CACHE_DATA_MEM_POWER_CTRL_SPEC> {
        L1_DCACHE_DATA_MEM_FORCE_PD_W::new(self, 17)
    }
    #[doc = "Bit 18 - The bit is used to power L1-DCache data memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_dcache_data_mem_force_pu(
        &mut self,
    ) -> L1_DCACHE_DATA_MEM_FORCE_PU_W<L1_CACHE_DATA_MEM_POWER_CTRL_SPEC> {
        L1_DCACHE_DATA_MEM_FORCE_PU_W::new(self, 18)
    }
}
#[doc = "Cache data memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_data_mem_power_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_data_mem_power_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_DATA_MEM_POWER_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_DATA_MEM_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_data_mem_power_ctrl::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_DATA_MEM_POWER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_data_mem_power_ctrl::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_DATA_MEM_POWER_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_DATA_MEM_POWER_CTRL to value 0x0005_5555"]
impl crate::Resettable for L1_CACHE_DATA_MEM_POWER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0005_5555;
}
