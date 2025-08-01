#[doc = "Register `INT_ST_PLD_CRC_FATAL` reader"]
pub type R = crate::R<INT_ST_PLD_CRC_FATAL_SPEC>;
#[doc = "Field `ST_ERR_CRC_VC0` reader - NA"]
pub type ST_ERR_CRC_VC0_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC1` reader - NA"]
pub type ST_ERR_CRC_VC1_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC2` reader - NA"]
pub type ST_ERR_CRC_VC2_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC3` reader - NA"]
pub type ST_ERR_CRC_VC3_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC4` reader - NA"]
pub type ST_ERR_CRC_VC4_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC5` reader - NA"]
pub type ST_ERR_CRC_VC5_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC6` reader - NA"]
pub type ST_ERR_CRC_VC6_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC7` reader - NA"]
pub type ST_ERR_CRC_VC7_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC8` reader - NA"]
pub type ST_ERR_CRC_VC8_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC9` reader - NA"]
pub type ST_ERR_CRC_VC9_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC10` reader - NA"]
pub type ST_ERR_CRC_VC10_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC11` reader - NA"]
pub type ST_ERR_CRC_VC11_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC12` reader - NA"]
pub type ST_ERR_CRC_VC12_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC13` reader - NA"]
pub type ST_ERR_CRC_VC13_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC14` reader - NA"]
pub type ST_ERR_CRC_VC14_R = crate::BitReader;
#[doc = "Field `ST_ERR_CRC_VC15` reader - NA"]
pub type ST_ERR_CRC_VC15_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc0(&self) -> ST_ERR_CRC_VC0_R {
        ST_ERR_CRC_VC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc1(&self) -> ST_ERR_CRC_VC1_R {
        ST_ERR_CRC_VC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc2(&self) -> ST_ERR_CRC_VC2_R {
        ST_ERR_CRC_VC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc3(&self) -> ST_ERR_CRC_VC3_R {
        ST_ERR_CRC_VC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc4(&self) -> ST_ERR_CRC_VC4_R {
        ST_ERR_CRC_VC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc5(&self) -> ST_ERR_CRC_VC5_R {
        ST_ERR_CRC_VC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc6(&self) -> ST_ERR_CRC_VC6_R {
        ST_ERR_CRC_VC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc7(&self) -> ST_ERR_CRC_VC7_R {
        ST_ERR_CRC_VC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc8(&self) -> ST_ERR_CRC_VC8_R {
        ST_ERR_CRC_VC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc9(&self) -> ST_ERR_CRC_VC9_R {
        ST_ERR_CRC_VC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc10(&self) -> ST_ERR_CRC_VC10_R {
        ST_ERR_CRC_VC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc11(&self) -> ST_ERR_CRC_VC11_R {
        ST_ERR_CRC_VC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc12(&self) -> ST_ERR_CRC_VC12_R {
        ST_ERR_CRC_VC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc13(&self) -> ST_ERR_CRC_VC13_R {
        ST_ERR_CRC_VC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc14(&self) -> ST_ERR_CRC_VC14_R {
        ST_ERR_CRC_VC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn st_err_crc_vc15(&self) -> ST_ERR_CRC_VC15_R {
        ST_ERR_CRC_VC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST_PLD_CRC_FATAL")
            .field("st_err_crc_vc0", &self.st_err_crc_vc0())
            .field("st_err_crc_vc1", &self.st_err_crc_vc1())
            .field("st_err_crc_vc2", &self.st_err_crc_vc2())
            .field("st_err_crc_vc3", &self.st_err_crc_vc3())
            .field("st_err_crc_vc4", &self.st_err_crc_vc4())
            .field("st_err_crc_vc5", &self.st_err_crc_vc5())
            .field("st_err_crc_vc6", &self.st_err_crc_vc6())
            .field("st_err_crc_vc7", &self.st_err_crc_vc7())
            .field("st_err_crc_vc8", &self.st_err_crc_vc8())
            .field("st_err_crc_vc9", &self.st_err_crc_vc9())
            .field("st_err_crc_vc10", &self.st_err_crc_vc10())
            .field("st_err_crc_vc11", &self.st_err_crc_vc11())
            .field("st_err_crc_vc12", &self.st_err_crc_vc12())
            .field("st_err_crc_vc13", &self.st_err_crc_vc13())
            .field("st_err_crc_vc14", &self.st_err_crc_vc14())
            .field("st_err_crc_vc15", &self.st_err_crc_vc15())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_pld_crc_fatal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_PLD_CRC_FATAL_SPEC;
impl crate::RegisterSpec for INT_ST_PLD_CRC_FATAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_pld_crc_fatal::R`](R) reader structure"]
impl crate::Readable for INT_ST_PLD_CRC_FATAL_SPEC {}
#[doc = "`reset()` method sets INT_ST_PLD_CRC_FATAL to value 0"]
impl crate::Resettable for INT_ST_PLD_CRC_FATAL_SPEC {}
