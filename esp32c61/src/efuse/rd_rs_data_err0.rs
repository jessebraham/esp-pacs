#[doc = "Register `RD_RS_DATA_ERR0` reader"]
pub type R = crate::R<RD_RS_DATA_ERR0_SPEC>;
#[doc = "Field `RD_MAC_SYS_ERR_NUM` reader - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_mac_sys"]
pub type RD_MAC_SYS_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `RD_MAC_SYS_FAIL` reader - Represents error status of register.\\\\0: Means no failure and that the data of rd_mac_sys is reliable\\\\ 1: Means that programming rd_mac_sys failed and the number of error bytes is over 6."]
pub type RD_MAC_SYS_FAIL_R = crate::BitReader;
#[doc = "Field `RD_SYS_PART1_DATA_ERR_NUM` reader - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_sys_part1_data"]
pub type RD_SYS_PART1_DATA_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `RD_SYS_PART1_DATA_FAIL` reader - Represents error status of register.\\\\0: Means no failure and that the data of rd_sys_part1_data is reliable\\\\ 1: Means that programming rd_sys_part1_data failed and the number of error bytes is over 6."]
pub type RD_SYS_PART1_DATA_FAIL_R = crate::BitReader;
#[doc = "Field `RD_USR_DATA_ERR_NUM` reader - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_usr_data"]
pub type RD_USR_DATA_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `RD_USR_DATA_FAIL` reader - Represents error status of register.\\\\0: Means no failure and that the data of rd_usr_data is reliable\\\\ 1: Means that programming rd_usr_data failed and the number of error bytes is over 6."]
pub type RD_USR_DATA_FAIL_R = crate::BitReader;
#[doc = "Field `RD_KEY0_DATA_ERR_NUM` reader - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_key0_data"]
pub type RD_KEY0_DATA_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `RD_KEY0_DATA_FAIL` reader - Represents error status of register.\\\\0: Means no failure and that the data of rd_key0_data is reliable\\\\ 1: Means that programming rd_key0_data failed and the number of error bytes is over 6."]
pub type RD_KEY0_DATA_FAIL_R = crate::BitReader;
#[doc = "Field `RD_KEY1_DATA_ERR_NUM` reader - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_key1_data"]
pub type RD_KEY1_DATA_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `RD_KEY1_DATA_FAIL` reader - Represents error status of register.\\\\0: Means no failure and that the data of rd_key1_data is reliable\\\\ 1: Means that programming rd_key1_data failed and the number of error bytes is over 6."]
pub type RD_KEY1_DATA_FAIL_R = crate::BitReader;
#[doc = "Field `RD_KEY2_DATA_ERR_NUM` reader - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_key2_data"]
pub type RD_KEY2_DATA_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `RD_KEY2_DATA_FAIL` reader - Represents error status of register.\\\\0: Means no failure and that the data of rd_key2_data is reliable\\\\ 1: Means that programming rd_key2_data failed and the number of error bytes is over 6."]
pub type RD_KEY2_DATA_FAIL_R = crate::BitReader;
#[doc = "Field `RD_KEY3_DATA_ERR_NUM` reader - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_key3_data"]
pub type RD_KEY3_DATA_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `RD_KEY3_DATA_FAIL` reader - Represents error status of register.\\\\0: Means no failure and that the data of rd_key3_data is reliable\\\\ 1: Means that programming rd_key3_data failed and the number of error bytes is over 6."]
pub type RD_KEY3_DATA_FAIL_R = crate::BitReader;
#[doc = "Field `RD_KEY4_DATA_ERR_NUM` reader - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_key4_data"]
pub type RD_KEY4_DATA_ERR_NUM_R = crate::FieldReader;
#[doc = "Field `RD_KEY4_DATA_FAIL` reader - Represents error status of register.\\\\0: Means no failure and that the data of rd_key4_data is reliable\\\\ 1: Means that programming rd_key4_data failed and the number of error bytes is over 6."]
pub type RD_KEY4_DATA_FAIL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_mac_sys"]
    #[inline(always)]
    pub fn rd_mac_sys_err_num(&self) -> RD_MAC_SYS_ERR_NUM_R {
        RD_MAC_SYS_ERR_NUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Represents error status of register.\\\\0: Means no failure and that the data of rd_mac_sys is reliable\\\\ 1: Means that programming rd_mac_sys failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn rd_mac_sys_fail(&self) -> RD_MAC_SYS_FAIL_R {
        RD_MAC_SYS_FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_sys_part1_data"]
    #[inline(always)]
    pub fn rd_sys_part1_data_err_num(&self) -> RD_SYS_PART1_DATA_ERR_NUM_R {
        RD_SYS_PART1_DATA_ERR_NUM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Represents error status of register.\\\\0: Means no failure and that the data of rd_sys_part1_data is reliable\\\\ 1: Means that programming rd_sys_part1_data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn rd_sys_part1_data_fail(&self) -> RD_SYS_PART1_DATA_FAIL_R {
        RD_SYS_PART1_DATA_FAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_usr_data"]
    #[inline(always)]
    pub fn rd_usr_data_err_num(&self) -> RD_USR_DATA_ERR_NUM_R {
        RD_USR_DATA_ERR_NUM_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Represents error status of register.\\\\0: Means no failure and that the data of rd_usr_data is reliable\\\\ 1: Means that programming rd_usr_data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn rd_usr_data_fail(&self) -> RD_USR_DATA_FAIL_R {
        RD_USR_DATA_FAIL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_key0_data"]
    #[inline(always)]
    pub fn rd_key0_data_err_num(&self) -> RD_KEY0_DATA_ERR_NUM_R {
        RD_KEY0_DATA_ERR_NUM_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Represents error status of register.\\\\0: Means no failure and that the data of rd_key0_data is reliable\\\\ 1: Means that programming rd_key0_data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn rd_key0_data_fail(&self) -> RD_KEY0_DATA_FAIL_R {
        RD_KEY0_DATA_FAIL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_key1_data"]
    #[inline(always)]
    pub fn rd_key1_data_err_num(&self) -> RD_KEY1_DATA_ERR_NUM_R {
        RD_KEY1_DATA_ERR_NUM_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Represents error status of register.\\\\0: Means no failure and that the data of rd_key1_data is reliable\\\\ 1: Means that programming rd_key1_data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn rd_key1_data_fail(&self) -> RD_KEY1_DATA_FAIL_R {
        RD_KEY1_DATA_FAIL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_key2_data"]
    #[inline(always)]
    pub fn rd_key2_data_err_num(&self) -> RD_KEY2_DATA_ERR_NUM_R {
        RD_KEY2_DATA_ERR_NUM_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Represents error status of register.\\\\0: Means no failure and that the data of rd_key2_data is reliable\\\\ 1: Means that programming rd_key2_data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn rd_key2_data_fail(&self) -> RD_KEY2_DATA_FAIL_R {
        RD_KEY2_DATA_FAIL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_key3_data"]
    #[inline(always)]
    pub fn rd_key3_data_err_num(&self) -> RD_KEY3_DATA_ERR_NUM_R {
        RD_KEY3_DATA_ERR_NUM_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Represents error status of register.\\\\0: Means no failure and that the data of rd_key3_data is reliable\\\\ 1: Means that programming rd_key3_data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn rd_key3_data_fail(&self) -> RD_KEY3_DATA_FAIL_R {
        RD_KEY3_DATA_FAIL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Represents the error number of registers.\\\\The value of this signal means the number of error bytes in rd_key4_data"]
    #[inline(always)]
    pub fn rd_key4_data_err_num(&self) -> RD_KEY4_DATA_ERR_NUM_R {
        RD_KEY4_DATA_ERR_NUM_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Represents error status of register.\\\\0: Means no failure and that the data of rd_key4_data is reliable\\\\ 1: Means that programming rd_key4_data failed and the number of error bytes is over 6."]
    #[inline(always)]
    pub fn rd_key4_data_fail(&self) -> RD_KEY4_DATA_FAIL_R {
        RD_KEY4_DATA_FAIL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_RS_DATA_ERR0")
            .field("rd_mac_sys_err_num", &self.rd_mac_sys_err_num())
            .field("rd_mac_sys_fail", &self.rd_mac_sys_fail())
            .field(
                "rd_sys_part1_data_err_num",
                &self.rd_sys_part1_data_err_num(),
            )
            .field("rd_sys_part1_data_fail", &self.rd_sys_part1_data_fail())
            .field("rd_usr_data_err_num", &self.rd_usr_data_err_num())
            .field("rd_usr_data_fail", &self.rd_usr_data_fail())
            .field("rd_key0_data_err_num", &self.rd_key0_data_err_num())
            .field("rd_key0_data_fail", &self.rd_key0_data_fail())
            .field("rd_key1_data_err_num", &self.rd_key1_data_err_num())
            .field("rd_key1_data_fail", &self.rd_key1_data_fail())
            .field("rd_key2_data_err_num", &self.rd_key2_data_err_num())
            .field("rd_key2_data_fail", &self.rd_key2_data_fail())
            .field("rd_key3_data_err_num", &self.rd_key3_data_err_num())
            .field("rd_key3_data_fail", &self.rd_key3_data_fail())
            .field("rd_key4_data_err_num", &self.rd_key4_data_err_num())
            .field("rd_key4_data_fail", &self.rd_key4_data_fail())
            .finish()
    }
}
#[doc = "Represents rd_rs_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_rs_data_err0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_RS_DATA_ERR0_SPEC;
impl crate::RegisterSpec for RD_RS_DATA_ERR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_rs_data_err0::R`](R) reader structure"]
impl crate::Readable for RD_RS_DATA_ERR0_SPEC {}
#[doc = "`reset()` method sets RD_RS_DATA_ERR0 to value 0"]
impl crate::Resettable for RD_RS_DATA_ERR0_SPEC {}
