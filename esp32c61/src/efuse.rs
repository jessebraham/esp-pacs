#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pgm_data: [PGM_DATA; 8],
    pgm_check_value: [PGM_CHECK_VALUE; 3],
    rd_wr_dis0: RD_WR_DIS0,
    rd_repeat_data0: RD_REPEAT_DATA0,
    rd_repeat_data1: RD_REPEAT_DATA1,
    rd_repeat_data2: RD_REPEAT_DATA2,
    rd_repeat_data3: RD_REPEAT_DATA3,
    rd_repeat_data4: RD_REPEAT_DATA4,
    rd_mac_sys0: RD_MAC_SYS0,
    rd_mac_sys1: RD_MAC_SYS1,
    rd_mac_sys2: RD_MAC_SYS2,
    rd_mac_sys3: RD_MAC_SYS3,
    rd_mac_sys4: RD_MAC_SYS4,
    rd_mac_sys5: RD_MAC_SYS5,
    rd_sys_part1_data: [RD_SYS_PART1_DATA; 8],
    rd_usr_data: [RD_USR_DATA; 8],
    rd_key0_data: [RD_KEY0_DATA; 8],
    rd_key1_data: [RD_KEY1_DATA; 8],
    rd_key2_data: [RD_KEY2_DATA; 8],
    rd_key3_data: [RD_KEY3_DATA; 8],
    rd_key4_data: [RD_KEY4_DATA; 8],
    rd_key5_data: [RD_KEY5_DATA; 8],
    rd_sys_part2_data: [RD_SYS_PART2_DATA; 8],
    rd_repeat_data_err0: RD_REPEAT_DATA_ERR0,
    rd_repeat_data_err1: RD_REPEAT_DATA_ERR1,
    rd_repeat_data_err2: RD_REPEAT_DATA_ERR2,
    rd_repeat_data_err3: RD_REPEAT_DATA_ERR3,
    rd_repeat_data_err4: RD_REPEAT_DATA_ERR4,
    rd_rs_data_err0: RD_RS_DATA_ERR0,
    rd_rs_data_err1: RD_RS_DATA_ERR1,
    date: DATE,
    _reserved31: [u8; 0x2c],
    clk: CLK,
    conf: CONF,
    status: STATUS,
    cmd: CMD,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    dac_conf: DAC_CONF,
    rd_tim_conf: RD_TIM_CONF,
    wr_tim_conf1: WR_TIM_CONF1,
    wr_tim_conf2: WR_TIM_CONF2,
    wr_tim_conf0_rs_bypass: WR_TIM_CONF0_RS_BYPASS,
    _reserved44: [u8; 0x0304],
    apb2otp_wr_dis: APB2OTP_WR_DIS,
    apb2otp_blk0_backup1_w1: APB2OTP_BLK0_BACKUP1_W1,
    apb2otp_blk0_backup1_w2: APB2OTP_BLK0_BACKUP1_W2,
    apb2otp_blk0_backup1_w3: APB2OTP_BLK0_BACKUP1_W3,
    apb2otp_blk0_backup1_w4: APB2OTP_BLK0_BACKUP1_W4,
    apb2otp_blk0_backup1_w5: APB2OTP_BLK0_BACKUP1_W5,
    apb2otp_blk0_backup2_w1: APB2OTP_BLK0_BACKUP2_W1,
    apb2otp_blk0_backup2_w2: APB2OTP_BLK0_BACKUP2_W2,
    apb2otp_blk0_backup2_w3: APB2OTP_BLK0_BACKUP2_W3,
    apb2otp_blk0_backup2_w4: APB2OTP_BLK0_BACKUP2_W4,
    apb2otp_blk0_backup2_w5: APB2OTP_BLK0_BACKUP2_W5,
    apb2otp_blk0_backup3_w1: APB2OTP_BLK0_BACKUP3_W1,
    apb2otp_blk0_backup3_w2: APB2OTP_BLK0_BACKUP3_W2,
    apb2otp_blk0_backup3_w3: APB2OTP_BLK0_BACKUP3_W3,
    apb2otp_blk0_backup3_w4: APB2OTP_BLK0_BACKUP3_W4,
    apb2otp_blk0_backup3_w5: APB2OTP_BLK0_BACKUP3_W5,
    apb2otp_blk0_backup4_w1: APB2OTP_BLK0_BACKUP4_W1,
    apb2otp_blk0_backup4_w2: APB2OTP_BLK0_BACKUP4_W2,
    apb2otp_blk0_backup4_w3: APB2OTP_BLK0_BACKUP4_W3,
    apb2otp_blk0_backup4_w4: APB2OTP_BLK0_BACKUP4_W4,
    apb2otp_blk0_backup4_w5: APB2OTP_BLK0_BACKUP4_W5,
    apb2otp_blk1_w1: APB2OTP_BLK1_W1,
    apb2otp_blk1_w2: APB2OTP_BLK1_W2,
    apb2otp_blk1_w3: APB2OTP_BLK1_W3,
    apb2otp_blk1_w4: APB2OTP_BLK1_W4,
    apb2otp_blk1_w5: APB2OTP_BLK1_W5,
    apb2otp_blk1_w6: APB2OTP_BLK1_W6,
    apb2otp_blk1_w7: APB2OTP_BLK1_W7,
    apb2otp_blk1_w8: APB2OTP_BLK1_W8,
    apb2otp_blk1_w9: APB2OTP_BLK1_W9,
    apb2otp_blk2_w1: APB2OTP_BLK2_W1,
    apb2otp_blk2_w2: APB2OTP_BLK2_W2,
    apb2otp_blk2_w3: APB2OTP_BLK2_W3,
    apb2otp_blk2_w4: APB2OTP_BLK2_W4,
    apb2otp_blk2_w5: APB2OTP_BLK2_W5,
    apb2otp_blk2_w6: APB2OTP_BLK2_W6,
    apb2otp_blk2_w7: APB2OTP_BLK2_W7,
    apb2otp_blk2_w8: APB2OTP_BLK2_W8,
    apb2otp_blk2_w9: APB2OTP_BLK2_W9,
    apb2otp_blk2_w10: APB2OTP_BLK2_W10,
    apb2otp_blk2_w11: APB2OTP_BLK2_W11,
    apb2otp_blk3_w1: APB2OTP_BLK3_W1,
    apb2otp_blk3_w2: APB2OTP_BLK3_W2,
    apb2otp_blk3_w3: APB2OTP_BLK3_W3,
    apb2otp_blk3_w4: APB2OTP_BLK3_W4,
    apb2otp_blk3_w5: APB2OTP_BLK3_W5,
    apb2otp_blk3_w6: APB2OTP_BLK3_W6,
    apb2otp_blk3_w7: APB2OTP_BLK3_W7,
    apb2otp_blk3_w8: APB2OTP_BLK3_W8,
    apb2otp_blk3_w9: APB2OTP_BLK3_W9,
    apb2otp_blk3_w10: APB2OTP_BLK3_W10,
    apb2otp_blk3_w11: APB2OTP_BLK3_W11,
    apb2otp_blk4_w1: APB2OTP_BLK4_W1,
    apb2otp_blk4_w2: APB2OTP_BLK4_W2,
    apb2otp_blk4_w3: APB2OTP_BLK4_W3,
    apb2otp_blk4_w4: APB2OTP_BLK4_W4,
    apb2otp_blk4_w5: APB2OTP_BLK4_W5,
    apb2otp_blk4_w6: APB2OTP_BLK4_W6,
    apb2otp_blk4_w7: APB2OTP_BLK4_W7,
    apb2otp_blk4_w8: APB2OTP_BLK4_W8,
    apb2otp_blk4_w9: APB2OTP_BLK4_W9,
    apb2otp_blk4_w10: APB2OTP_BLK4_W10,
    apb2otp_blk4_w11: APB2OTP_BLK4_W11,
    apb2otp_blk5_w1: APB2OTP_BLK5_W1,
    apb2otp_blk5_w2: APB2OTP_BLK5_W2,
    apb2otp_blk5_w3: APB2OTP_BLK5_W3,
    apb2otp_blk5_w4: APB2OTP_BLK5_W4,
    apb2otp_blk5_w5: APB2OTP_BLK5_W5,
    apb2otp_blk5_w6: APB2OTP_BLK5_W6,
    apb2otp_blk5_w7: APB2OTP_BLK5_W7,
    apb2otp_blk5_w8: APB2OTP_BLK5_W8,
    apb2otp_blk5_w9: APB2OTP_BLK5_W9,
    apb2otp_blk5_w10: APB2OTP_BLK5_W10,
    apb2otp_blk5_w11: APB2OTP_BLK5_W11,
    apb2otp_blk6_w1: APB2OTP_BLK6_W1,
    apb2otp_blk6_w2: APB2OTP_BLK6_W2,
    apb2otp_blk6_w3: APB2OTP_BLK6_W3,
    apb2otp_blk6_w4: APB2OTP_BLK6_W4,
    apb2otp_blk6_w5: APB2OTP_BLK6_W5,
    apb2otp_blk6_w6: APB2OTP_BLK6_W6,
    apb2otp_blk6_w7: APB2OTP_BLK6_W7,
    apb2otp_blk6_w8: APB2OTP_BLK6_W8,
    apb2otp_blk6_w9: APB2OTP_BLK6_W9,
    apb2otp_blk6_w10: APB2OTP_BLK6_W10,
    apb2otp_blk6_w11: APB2OTP_BLK6_W11,
    apb2otp_blk7_w1: APB2OTP_BLK7_W1,
    apb2otp_blk7_w2: APB2OTP_BLK7_W2,
    apb2otp_blk7_w3: APB2OTP_BLK7_W3,
    apb2otp_blk7_w4: APB2OTP_BLK7_W4,
    apb2otp_blk7_w5: APB2OTP_BLK7_W5,
    apb2otp_blk7_w6: APB2OTP_BLK7_W6,
    apb2otp_blk7_w7: APB2OTP_BLK7_W7,
    apb2otp_blk7_w8: APB2OTP_BLK7_W8,
    apb2otp_blk7_w9: APB2OTP_BLK7_W9,
    apb2otp_blk7_w10: APB2OTP_BLK7_W10,
    apb2otp_blk7_w11: APB2OTP_BLK7_W11,
    apb2otp_blk8_w1: APB2OTP_BLK8_W1,
    apb2otp_blk8_w2: APB2OTP_BLK8_W2,
    apb2otp_blk8_w3: APB2OTP_BLK8_W3,
    apb2otp_blk8_w4: APB2OTP_BLK8_W4,
    apb2otp_blk8_w5: APB2OTP_BLK8_W5,
    apb2otp_blk8_w6: APB2OTP_BLK8_W6,
    apb2otp_blk8_w7: APB2OTP_BLK8_W7,
    apb2otp_blk8_w8: APB2OTP_BLK8_W8,
    apb2otp_blk8_w9: APB2OTP_BLK8_W9,
    apb2otp_blk8_w10: APB2OTP_BLK8_W10,
    apb2otp_blk8_w11: APB2OTP_BLK8_W11,
    apb2otp_blk9_w1: APB2OTP_BLK9_W1,
    apb2otp_blk9_w2: APB2OTP_BLK9_W2,
    apb2otp_blk9_w3: APB2OTP_BLK9_W3,
    apb2otp_blk9_w4: APB2OTP_BLK9_W4,
    apb2otp_blk9_w5: APB2OTP_BLK9_W5,
    apb2otp_blk9_w6: APB2OTP_BLK9_W6,
    apb2otp_blk9_w7: APB2OTP_BLK9_W7,
    apb2otp_blk9_w8: APB2OTP_BLK9_W8,
    apb2otp_blk9_w9: APB2OTP_BLK9_W9,
    apb2otp_blk9_w10: APB2OTP_BLK9_W10,
    apb2otp_blk9_w11: APB2OTP_BLK9_W11,
    apb2otp_blk10_w1: APB2OTP_BLK10_W1,
    apb2otp_blk10_w2: APB2OTP_BLK10_W2,
    apb2otp_blk10_w3: APB2OTP_BLK10_W3,
    apb2otp_blk10_w4: APB2OTP_BLK10_W4,
    apb2otp_blk10_w5: APB2OTP_BLK10_W5,
    apb2otp_blk10_w6: APB2OTP_BLK10_W6,
    apb2otp_blk10_w7: APB2OTP_BLK10_W7,
    apb2otp_blk10_w8: APB2OTP_BLK10_W8,
    apb2otp_blk10_w9: APB2OTP_BLK10_W9,
    apb2otp_blk10_w10: APB2OTP_BLK10_W10,
    apb2otp_blk10_w11: APB2OTP_BLK10_W11,
    _reserved173: [u8; 0x04],
    apb2otp_en: APB2OTP_EN,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - Represents pgm_data%s"]
    #[inline(always)]
    pub const fn pgm_data(&self, n: usize) -> &PGM_DATA {
        &self.pgm_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Represents pgm_data%s"]
    #[inline(always)]
    pub fn pgm_data_iter(&self) -> impl Iterator<Item = &PGM_DATA> {
        self.pgm_data.iter()
    }
    #[doc = "0x20..0x2c - Represents pgm_check_value%s"]
    #[inline(always)]
    pub const fn pgm_check_value(&self, n: usize) -> &PGM_CHECK_VALUE {
        &self.pgm_check_value[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x2c - Represents pgm_check_value%s"]
    #[inline(always)]
    pub fn pgm_check_value_iter(&self) -> impl Iterator<Item = &PGM_CHECK_VALUE> {
        self.pgm_check_value.iter()
    }
    #[doc = "0x2c - Represents rd_wr_dis"]
    #[inline(always)]
    pub const fn rd_wr_dis0(&self) -> &RD_WR_DIS0 {
        &self.rd_wr_dis0
    }
    #[doc = "0x30 - Represents rd_repeat_data"]
    #[inline(always)]
    pub const fn rd_repeat_data0(&self) -> &RD_REPEAT_DATA0 {
        &self.rd_repeat_data0
    }
    #[doc = "0x34 - Represents rd_repeat_data"]
    #[inline(always)]
    pub const fn rd_repeat_data1(&self) -> &RD_REPEAT_DATA1 {
        &self.rd_repeat_data1
    }
    #[doc = "0x38 - Represents rd_repeat_data"]
    #[inline(always)]
    pub const fn rd_repeat_data2(&self) -> &RD_REPEAT_DATA2 {
        &self.rd_repeat_data2
    }
    #[doc = "0x3c - Represents rd_repeat_data"]
    #[inline(always)]
    pub const fn rd_repeat_data3(&self) -> &RD_REPEAT_DATA3 {
        &self.rd_repeat_data3
    }
    #[doc = "0x40 - Represents rd_repeat_data"]
    #[inline(always)]
    pub const fn rd_repeat_data4(&self) -> &RD_REPEAT_DATA4 {
        &self.rd_repeat_data4
    }
    #[doc = "0x44 - Represents rd_mac_sys"]
    #[inline(always)]
    pub const fn rd_mac_sys0(&self) -> &RD_MAC_SYS0 {
        &self.rd_mac_sys0
    }
    #[doc = "0x48 - Represents rd_mac_sys"]
    #[inline(always)]
    pub const fn rd_mac_sys1(&self) -> &RD_MAC_SYS1 {
        &self.rd_mac_sys1
    }
    #[doc = "0x4c - Represents rd_mac_sys"]
    #[inline(always)]
    pub const fn rd_mac_sys2(&self) -> &RD_MAC_SYS2 {
        &self.rd_mac_sys2
    }
    #[doc = "0x50 - Represents rd_mac_sys"]
    #[inline(always)]
    pub const fn rd_mac_sys3(&self) -> &RD_MAC_SYS3 {
        &self.rd_mac_sys3
    }
    #[doc = "0x54 - Represents rd_mac_sys"]
    #[inline(always)]
    pub const fn rd_mac_sys4(&self) -> &RD_MAC_SYS4 {
        &self.rd_mac_sys4
    }
    #[doc = "0x58 - Represents rd_mac_sys"]
    #[inline(always)]
    pub const fn rd_mac_sys5(&self) -> &RD_MAC_SYS5 {
        &self.rd_mac_sys5
    }
    #[doc = "0x5c..0x7c - Represents rd_sys_part1_data%s"]
    #[inline(always)]
    pub const fn rd_sys_part1_data(&self, n: usize) -> &RD_SYS_PART1_DATA {
        &self.rd_sys_part1_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c..0x7c - Represents rd_sys_part1_data%s"]
    #[inline(always)]
    pub fn rd_sys_part1_data_iter(&self) -> impl Iterator<Item = &RD_SYS_PART1_DATA> {
        self.rd_sys_part1_data.iter()
    }
    #[doc = "0x7c..0x9c - Represents rd_usr_data%s"]
    #[inline(always)]
    pub const fn rd_usr_data(&self, n: usize) -> &RD_USR_DATA {
        &self.rd_usr_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x7c..0x9c - Represents rd_usr_data%s"]
    #[inline(always)]
    pub fn rd_usr_data_iter(&self) -> impl Iterator<Item = &RD_USR_DATA> {
        self.rd_usr_data.iter()
    }
    #[doc = "0x9c..0xbc - Represents rd_key0_data%s"]
    #[inline(always)]
    pub const fn rd_key0_data(&self, n: usize) -> &RD_KEY0_DATA {
        &self.rd_key0_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9c..0xbc - Represents rd_key0_data%s"]
    #[inline(always)]
    pub fn rd_key0_data_iter(&self) -> impl Iterator<Item = &RD_KEY0_DATA> {
        self.rd_key0_data.iter()
    }
    #[doc = "0xbc..0xdc - Represents rd_key1_data%s"]
    #[inline(always)]
    pub const fn rd_key1_data(&self, n: usize) -> &RD_KEY1_DATA {
        &self.rd_key1_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xbc..0xdc - Represents rd_key1_data%s"]
    #[inline(always)]
    pub fn rd_key1_data_iter(&self) -> impl Iterator<Item = &RD_KEY1_DATA> {
        self.rd_key1_data.iter()
    }
    #[doc = "0xdc..0xfc - Represents rd_key2_data%s"]
    #[inline(always)]
    pub const fn rd_key2_data(&self, n: usize) -> &RD_KEY2_DATA {
        &self.rd_key2_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xdc..0xfc - Represents rd_key2_data%s"]
    #[inline(always)]
    pub fn rd_key2_data_iter(&self) -> impl Iterator<Item = &RD_KEY2_DATA> {
        self.rd_key2_data.iter()
    }
    #[doc = "0xfc..0x11c - Represents rd_key3_data%s"]
    #[inline(always)]
    pub const fn rd_key3_data(&self, n: usize) -> &RD_KEY3_DATA {
        &self.rd_key3_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xfc..0x11c - Represents rd_key3_data%s"]
    #[inline(always)]
    pub fn rd_key3_data_iter(&self) -> impl Iterator<Item = &RD_KEY3_DATA> {
        self.rd_key3_data.iter()
    }
    #[doc = "0x11c..0x13c - Represents rd_key4_data%s"]
    #[inline(always)]
    pub const fn rd_key4_data(&self, n: usize) -> &RD_KEY4_DATA {
        &self.rd_key4_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x11c..0x13c - Represents rd_key4_data%s"]
    #[inline(always)]
    pub fn rd_key4_data_iter(&self) -> impl Iterator<Item = &RD_KEY4_DATA> {
        self.rd_key4_data.iter()
    }
    #[doc = "0x13c..0x15c - Represents rd_key5_data%s"]
    #[inline(always)]
    pub const fn rd_key5_data(&self, n: usize) -> &RD_KEY5_DATA {
        &self.rd_key5_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x13c..0x15c - Represents rd_key5_data%s"]
    #[inline(always)]
    pub fn rd_key5_data_iter(&self) -> impl Iterator<Item = &RD_KEY5_DATA> {
        self.rd_key5_data.iter()
    }
    #[doc = "0x15c..0x17c - Represents rd_sys_part2_data%s"]
    #[inline(always)]
    pub const fn rd_sys_part2_data(&self, n: usize) -> &RD_SYS_PART2_DATA {
        &self.rd_sys_part2_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x15c..0x17c - Represents rd_sys_part2_data%s"]
    #[inline(always)]
    pub fn rd_sys_part2_data_iter(&self) -> impl Iterator<Item = &RD_SYS_PART2_DATA> {
        self.rd_sys_part2_data.iter()
    }
    #[doc = "0x17c - Represents rd_repeat_data_err"]
    #[inline(always)]
    pub const fn rd_repeat_data_err0(&self) -> &RD_REPEAT_DATA_ERR0 {
        &self.rd_repeat_data_err0
    }
    #[doc = "0x180 - Represents rd_repeat_data_err"]
    #[inline(always)]
    pub const fn rd_repeat_data_err1(&self) -> &RD_REPEAT_DATA_ERR1 {
        &self.rd_repeat_data_err1
    }
    #[doc = "0x184 - Represents rd_repeat_data_err"]
    #[inline(always)]
    pub const fn rd_repeat_data_err2(&self) -> &RD_REPEAT_DATA_ERR2 {
        &self.rd_repeat_data_err2
    }
    #[doc = "0x188 - Represents rd_repeat_data_err"]
    #[inline(always)]
    pub const fn rd_repeat_data_err3(&self) -> &RD_REPEAT_DATA_ERR3 {
        &self.rd_repeat_data_err3
    }
    #[doc = "0x18c - Represents rd_repeat_data_err"]
    #[inline(always)]
    pub const fn rd_repeat_data_err4(&self) -> &RD_REPEAT_DATA_ERR4 {
        &self.rd_repeat_data_err4
    }
    #[doc = "0x190 - Represents rd_rs_data_err"]
    #[inline(always)]
    pub const fn rd_rs_data_err0(&self) -> &RD_RS_DATA_ERR0 {
        &self.rd_rs_data_err0
    }
    #[doc = "0x194 - Represents rd_rs_data_err"]
    #[inline(always)]
    pub const fn rd_rs_data_err1(&self) -> &RD_RS_DATA_ERR1 {
        &self.rd_rs_data_err1
    }
    #[doc = "0x198 - eFuse version register."]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x1c8 - eFuse clcok configuration register."]
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    #[doc = "0x1cc - eFuse operation mode configuraiton register"]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x1d0 - eFuse status register."]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x1d4 - eFuse command register."]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x1d8 - eFuse raw interrupt register."]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x1dc - eFuse interrupt status register."]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x1e0 - eFuse interrupt enable register."]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x1e4 - eFuse interrupt clear register."]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x1e8 - Controls the eFuse programming voltage."]
    #[inline(always)]
    pub const fn dac_conf(&self) -> &DAC_CONF {
        &self.dac_conf
    }
    #[doc = "0x1ec - Configures read timing parameters."]
    #[inline(always)]
    pub const fn rd_tim_conf(&self) -> &RD_TIM_CONF {
        &self.rd_tim_conf
    }
    #[doc = "0x1f0 - Configurarion register 1 of eFuse programming timing parameters."]
    #[inline(always)]
    pub const fn wr_tim_conf1(&self) -> &WR_TIM_CONF1 {
        &self.wr_tim_conf1
    }
    #[doc = "0x1f4 - Configurarion register 2 of eFuse programming timing parameters."]
    #[inline(always)]
    pub const fn wr_tim_conf2(&self) -> &WR_TIM_CONF2 {
        &self.wr_tim_conf2
    }
    #[doc = "0x1f8 - Configurarion register0 of eFuse programming time parameters and rs bypass operation."]
    #[inline(always)]
    pub const fn wr_tim_conf0_rs_bypass(&self) -> &WR_TIM_CONF0_RS_BYPASS {
        &self.wr_tim_conf0_rs_bypass
    }
    #[doc = "0x500 - eFuse apb2otp block0 data register1."]
    #[inline(always)]
    pub const fn apb2otp_wr_dis(&self) -> &APB2OTP_WR_DIS {
        &self.apb2otp_wr_dis
    }
    #[doc = "0x504 - eFuse apb2otp block0 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup1_w1(&self) -> &APB2OTP_BLK0_BACKUP1_W1 {
        &self.apb2otp_blk0_backup1_w1
    }
    #[doc = "0x508 - eFuse apb2otp block0 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup1_w2(&self) -> &APB2OTP_BLK0_BACKUP1_W2 {
        &self.apb2otp_blk0_backup1_w2
    }
    #[doc = "0x50c - eFuse apb2otp block0 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup1_w3(&self) -> &APB2OTP_BLK0_BACKUP1_W3 {
        &self.apb2otp_blk0_backup1_w3
    }
    #[doc = "0x510 - eFuse apb2otp block0 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup1_w4(&self) -> &APB2OTP_BLK0_BACKUP1_W4 {
        &self.apb2otp_blk0_backup1_w4
    }
    #[doc = "0x514 - eFuse apb2otp block0 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup1_w5(&self) -> &APB2OTP_BLK0_BACKUP1_W5 {
        &self.apb2otp_blk0_backup1_w5
    }
    #[doc = "0x518 - eFuse apb2otp block0 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup2_w1(&self) -> &APB2OTP_BLK0_BACKUP2_W1 {
        &self.apb2otp_blk0_backup2_w1
    }
    #[doc = "0x51c - eFuse apb2otp block0 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup2_w2(&self) -> &APB2OTP_BLK0_BACKUP2_W2 {
        &self.apb2otp_blk0_backup2_w2
    }
    #[doc = "0x520 - eFuse apb2otp block0 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup2_w3(&self) -> &APB2OTP_BLK0_BACKUP2_W3 {
        &self.apb2otp_blk0_backup2_w3
    }
    #[doc = "0x524 - eFuse apb2otp block0 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup2_w4(&self) -> &APB2OTP_BLK0_BACKUP2_W4 {
        &self.apb2otp_blk0_backup2_w4
    }
    #[doc = "0x528 - eFuse apb2otp block0 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup2_w5(&self) -> &APB2OTP_BLK0_BACKUP2_W5 {
        &self.apb2otp_blk0_backup2_w5
    }
    #[doc = "0x52c - eFuse apb2otp block0 data register12."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup3_w1(&self) -> &APB2OTP_BLK0_BACKUP3_W1 {
        &self.apb2otp_blk0_backup3_w1
    }
    #[doc = "0x530 - eFuse apb2otp block0 data register13."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup3_w2(&self) -> &APB2OTP_BLK0_BACKUP3_W2 {
        &self.apb2otp_blk0_backup3_w2
    }
    #[doc = "0x534 - eFuse apb2otp block0 data register14."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup3_w3(&self) -> &APB2OTP_BLK0_BACKUP3_W3 {
        &self.apb2otp_blk0_backup3_w3
    }
    #[doc = "0x538 - eFuse apb2otp block0 data register15."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup3_w4(&self) -> &APB2OTP_BLK0_BACKUP3_W4 {
        &self.apb2otp_blk0_backup3_w4
    }
    #[doc = "0x53c - eFuse apb2otp block0 data register16."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup3_w5(&self) -> &APB2OTP_BLK0_BACKUP3_W5 {
        &self.apb2otp_blk0_backup3_w5
    }
    #[doc = "0x540 - eFuse apb2otp block0 data register17."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup4_w1(&self) -> &APB2OTP_BLK0_BACKUP4_W1 {
        &self.apb2otp_blk0_backup4_w1
    }
    #[doc = "0x544 - eFuse apb2otp block0 data register18."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup4_w2(&self) -> &APB2OTP_BLK0_BACKUP4_W2 {
        &self.apb2otp_blk0_backup4_w2
    }
    #[doc = "0x548 - eFuse apb2otp block0 data register19."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup4_w3(&self) -> &APB2OTP_BLK0_BACKUP4_W3 {
        &self.apb2otp_blk0_backup4_w3
    }
    #[doc = "0x54c - eFuse apb2otp block0 data register20."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup4_w4(&self) -> &APB2OTP_BLK0_BACKUP4_W4 {
        &self.apb2otp_blk0_backup4_w4
    }
    #[doc = "0x550 - eFuse apb2otp block0 data register21."]
    #[inline(always)]
    pub const fn apb2otp_blk0_backup4_w5(&self) -> &APB2OTP_BLK0_BACKUP4_W5 {
        &self.apb2otp_blk0_backup4_w5
    }
    #[doc = "0x554 - eFuse apb2otp block1 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w1(&self) -> &APB2OTP_BLK1_W1 {
        &self.apb2otp_blk1_w1
    }
    #[doc = "0x558 - eFuse apb2otp block1 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w2(&self) -> &APB2OTP_BLK1_W2 {
        &self.apb2otp_blk1_w2
    }
    #[doc = "0x55c - eFuse apb2otp block1 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w3(&self) -> &APB2OTP_BLK1_W3 {
        &self.apb2otp_blk1_w3
    }
    #[doc = "0x560 - eFuse apb2otp block1 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w4(&self) -> &APB2OTP_BLK1_W4 {
        &self.apb2otp_blk1_w4
    }
    #[doc = "0x564 - eFuse apb2otp block1 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w5(&self) -> &APB2OTP_BLK1_W5 {
        &self.apb2otp_blk1_w5
    }
    #[doc = "0x568 - eFuse apb2otp block1 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w6(&self) -> &APB2OTP_BLK1_W6 {
        &self.apb2otp_blk1_w6
    }
    #[doc = "0x56c - eFuse apb2otp block1 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w7(&self) -> &APB2OTP_BLK1_W7 {
        &self.apb2otp_blk1_w7
    }
    #[doc = "0x570 - eFuse apb2otp block1 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w8(&self) -> &APB2OTP_BLK1_W8 {
        &self.apb2otp_blk1_w8
    }
    #[doc = "0x574 - eFuse apb2otp block1 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk1_w9(&self) -> &APB2OTP_BLK1_W9 {
        &self.apb2otp_blk1_w9
    }
    #[doc = "0x578 - eFuse apb2otp block2 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w1(&self) -> &APB2OTP_BLK2_W1 {
        &self.apb2otp_blk2_w1
    }
    #[doc = "0x57c - eFuse apb2otp block2 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w2(&self) -> &APB2OTP_BLK2_W2 {
        &self.apb2otp_blk2_w2
    }
    #[doc = "0x580 - eFuse apb2otp block2 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w3(&self) -> &APB2OTP_BLK2_W3 {
        &self.apb2otp_blk2_w3
    }
    #[doc = "0x584 - eFuse apb2otp block2 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w4(&self) -> &APB2OTP_BLK2_W4 {
        &self.apb2otp_blk2_w4
    }
    #[doc = "0x588 - eFuse apb2otp block2 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w5(&self) -> &APB2OTP_BLK2_W5 {
        &self.apb2otp_blk2_w5
    }
    #[doc = "0x58c - eFuse apb2otp block2 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w6(&self) -> &APB2OTP_BLK2_W6 {
        &self.apb2otp_blk2_w6
    }
    #[doc = "0x590 - eFuse apb2otp block2 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w7(&self) -> &APB2OTP_BLK2_W7 {
        &self.apb2otp_blk2_w7
    }
    #[doc = "0x594 - eFuse apb2otp block2 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w8(&self) -> &APB2OTP_BLK2_W8 {
        &self.apb2otp_blk2_w8
    }
    #[doc = "0x598 - eFuse apb2otp block2 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w9(&self) -> &APB2OTP_BLK2_W9 {
        &self.apb2otp_blk2_w9
    }
    #[doc = "0x59c - eFuse apb2otp block2 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w10(&self) -> &APB2OTP_BLK2_W10 {
        &self.apb2otp_blk2_w10
    }
    #[doc = "0x5a0 - eFuse apb2otp block2 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk2_w11(&self) -> &APB2OTP_BLK2_W11 {
        &self.apb2otp_blk2_w11
    }
    #[doc = "0x5a4 - eFuse apb2otp block3 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w1(&self) -> &APB2OTP_BLK3_W1 {
        &self.apb2otp_blk3_w1
    }
    #[doc = "0x5a8 - eFuse apb2otp block3 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w2(&self) -> &APB2OTP_BLK3_W2 {
        &self.apb2otp_blk3_w2
    }
    #[doc = "0x5ac - eFuse apb2otp block3 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w3(&self) -> &APB2OTP_BLK3_W3 {
        &self.apb2otp_blk3_w3
    }
    #[doc = "0x5b0 - eFuse apb2otp block3 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w4(&self) -> &APB2OTP_BLK3_W4 {
        &self.apb2otp_blk3_w4
    }
    #[doc = "0x5b4 - eFuse apb2otp block3 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w5(&self) -> &APB2OTP_BLK3_W5 {
        &self.apb2otp_blk3_w5
    }
    #[doc = "0x5b8 - eFuse apb2otp block3 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w6(&self) -> &APB2OTP_BLK3_W6 {
        &self.apb2otp_blk3_w6
    }
    #[doc = "0x5bc - eFuse apb2otp block3 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w7(&self) -> &APB2OTP_BLK3_W7 {
        &self.apb2otp_blk3_w7
    }
    #[doc = "0x5c0 - eFuse apb2otp block3 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w8(&self) -> &APB2OTP_BLK3_W8 {
        &self.apb2otp_blk3_w8
    }
    #[doc = "0x5c4 - eFuse apb2otp block3 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w9(&self) -> &APB2OTP_BLK3_W9 {
        &self.apb2otp_blk3_w9
    }
    #[doc = "0x5c8 - eFuse apb2otp block3 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w10(&self) -> &APB2OTP_BLK3_W10 {
        &self.apb2otp_blk3_w10
    }
    #[doc = "0x5cc - eFuse apb2otp block3 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk3_w11(&self) -> &APB2OTP_BLK3_W11 {
        &self.apb2otp_blk3_w11
    }
    #[doc = "0x5d0 - eFuse apb2otp BLOCK7 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w1(&self) -> &APB2OTP_BLK4_W1 {
        &self.apb2otp_blk4_w1
    }
    #[doc = "0x5d4 - eFuse apb2otp block4 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w2(&self) -> &APB2OTP_BLK4_W2 {
        &self.apb2otp_blk4_w2
    }
    #[doc = "0x5d8 - eFuse apb2otp block4 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w3(&self) -> &APB2OTP_BLK4_W3 {
        &self.apb2otp_blk4_w3
    }
    #[doc = "0x5dc - eFuse apb2otp block4 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w4(&self) -> &APB2OTP_BLK4_W4 {
        &self.apb2otp_blk4_w4
    }
    #[doc = "0x5e0 - eFuse apb2otp block4 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w5(&self) -> &APB2OTP_BLK4_W5 {
        &self.apb2otp_blk4_w5
    }
    #[doc = "0x5e4 - eFuse apb2otp block4 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w6(&self) -> &APB2OTP_BLK4_W6 {
        &self.apb2otp_blk4_w6
    }
    #[doc = "0x5e8 - eFuse apb2otp block4 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w7(&self) -> &APB2OTP_BLK4_W7 {
        &self.apb2otp_blk4_w7
    }
    #[doc = "0x5ec - eFuse apb2otp block4 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w8(&self) -> &APB2OTP_BLK4_W8 {
        &self.apb2otp_blk4_w8
    }
    #[doc = "0x5f0 - eFuse apb2otp block4 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w9(&self) -> &APB2OTP_BLK4_W9 {
        &self.apb2otp_blk4_w9
    }
    #[doc = "0x5f4 - eFuse apb2otp block4 data registe10."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w10(&self) -> &APB2OTP_BLK4_W10 {
        &self.apb2otp_blk4_w10
    }
    #[doc = "0x5f8 - eFuse apb2otp block4 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk4_w11(&self) -> &APB2OTP_BLK4_W11 {
        &self.apb2otp_blk4_w11
    }
    #[doc = "0x5fc - eFuse apb2otp block5 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w1(&self) -> &APB2OTP_BLK5_W1 {
        &self.apb2otp_blk5_w1
    }
    #[doc = "0x600 - eFuse apb2otp block5 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w2(&self) -> &APB2OTP_BLK5_W2 {
        &self.apb2otp_blk5_w2
    }
    #[doc = "0x604 - eFuse apb2otp block5 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w3(&self) -> &APB2OTP_BLK5_W3 {
        &self.apb2otp_blk5_w3
    }
    #[doc = "0x608 - eFuse apb2otp block5 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w4(&self) -> &APB2OTP_BLK5_W4 {
        &self.apb2otp_blk5_w4
    }
    #[doc = "0x60c - eFuse apb2otp block5 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w5(&self) -> &APB2OTP_BLK5_W5 {
        &self.apb2otp_blk5_w5
    }
    #[doc = "0x610 - eFuse apb2otp block5 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w6(&self) -> &APB2OTP_BLK5_W6 {
        &self.apb2otp_blk5_w6
    }
    #[doc = "0x614 - eFuse apb2otp block5 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w7(&self) -> &APB2OTP_BLK5_W7 {
        &self.apb2otp_blk5_w7
    }
    #[doc = "0x618 - eFuse apb2otp block5 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w8(&self) -> &APB2OTP_BLK5_W8 {
        &self.apb2otp_blk5_w8
    }
    #[doc = "0x61c - eFuse apb2otp block5 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w9(&self) -> &APB2OTP_BLK5_W9 {
        &self.apb2otp_blk5_w9
    }
    #[doc = "0x620 - eFuse apb2otp block5 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w10(&self) -> &APB2OTP_BLK5_W10 {
        &self.apb2otp_blk5_w10
    }
    #[doc = "0x624 - eFuse apb2otp block5 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk5_w11(&self) -> &APB2OTP_BLK5_W11 {
        &self.apb2otp_blk5_w11
    }
    #[doc = "0x628 - eFuse apb2otp block6 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w1(&self) -> &APB2OTP_BLK6_W1 {
        &self.apb2otp_blk6_w1
    }
    #[doc = "0x62c - eFuse apb2otp block6 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w2(&self) -> &APB2OTP_BLK6_W2 {
        &self.apb2otp_blk6_w2
    }
    #[doc = "0x630 - eFuse apb2otp block6 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w3(&self) -> &APB2OTP_BLK6_W3 {
        &self.apb2otp_blk6_w3
    }
    #[doc = "0x634 - eFuse apb2otp block6 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w4(&self) -> &APB2OTP_BLK6_W4 {
        &self.apb2otp_blk6_w4
    }
    #[doc = "0x638 - eFuse apb2otp block6 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w5(&self) -> &APB2OTP_BLK6_W5 {
        &self.apb2otp_blk6_w5
    }
    #[doc = "0x63c - eFuse apb2otp block6 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w6(&self) -> &APB2OTP_BLK6_W6 {
        &self.apb2otp_blk6_w6
    }
    #[doc = "0x640 - eFuse apb2otp block6 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w7(&self) -> &APB2OTP_BLK6_W7 {
        &self.apb2otp_blk6_w7
    }
    #[doc = "0x644 - eFuse apb2otp block6 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w8(&self) -> &APB2OTP_BLK6_W8 {
        &self.apb2otp_blk6_w8
    }
    #[doc = "0x648 - eFuse apb2otp block6 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w9(&self) -> &APB2OTP_BLK6_W9 {
        &self.apb2otp_blk6_w9
    }
    #[doc = "0x64c - eFuse apb2otp block6 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w10(&self) -> &APB2OTP_BLK6_W10 {
        &self.apb2otp_blk6_w10
    }
    #[doc = "0x650 - eFuse apb2otp block6 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk6_w11(&self) -> &APB2OTP_BLK6_W11 {
        &self.apb2otp_blk6_w11
    }
    #[doc = "0x654 - eFuse apb2otp block7 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w1(&self) -> &APB2OTP_BLK7_W1 {
        &self.apb2otp_blk7_w1
    }
    #[doc = "0x658 - eFuse apb2otp block7 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w2(&self) -> &APB2OTP_BLK7_W2 {
        &self.apb2otp_blk7_w2
    }
    #[doc = "0x65c - eFuse apb2otp block7 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w3(&self) -> &APB2OTP_BLK7_W3 {
        &self.apb2otp_blk7_w3
    }
    #[doc = "0x660 - eFuse apb2otp block7 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w4(&self) -> &APB2OTP_BLK7_W4 {
        &self.apb2otp_blk7_w4
    }
    #[doc = "0x664 - eFuse apb2otp block7 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w5(&self) -> &APB2OTP_BLK7_W5 {
        &self.apb2otp_blk7_w5
    }
    #[doc = "0x668 - eFuse apb2otp block7 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w6(&self) -> &APB2OTP_BLK7_W6 {
        &self.apb2otp_blk7_w6
    }
    #[doc = "0x66c - eFuse apb2otp block7 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w7(&self) -> &APB2OTP_BLK7_W7 {
        &self.apb2otp_blk7_w7
    }
    #[doc = "0x670 - eFuse apb2otp block7 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w8(&self) -> &APB2OTP_BLK7_W8 {
        &self.apb2otp_blk7_w8
    }
    #[doc = "0x674 - eFuse apb2otp block7 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w9(&self) -> &APB2OTP_BLK7_W9 {
        &self.apb2otp_blk7_w9
    }
    #[doc = "0x678 - eFuse apb2otp block7 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w10(&self) -> &APB2OTP_BLK7_W10 {
        &self.apb2otp_blk7_w10
    }
    #[doc = "0x67c - eFuse apb2otp block7 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk7_w11(&self) -> &APB2OTP_BLK7_W11 {
        &self.apb2otp_blk7_w11
    }
    #[doc = "0x680 - eFuse apb2otp block8 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w1(&self) -> &APB2OTP_BLK8_W1 {
        &self.apb2otp_blk8_w1
    }
    #[doc = "0x684 - eFuse apb2otp block8 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w2(&self) -> &APB2OTP_BLK8_W2 {
        &self.apb2otp_blk8_w2
    }
    #[doc = "0x688 - eFuse apb2otp block8 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w3(&self) -> &APB2OTP_BLK8_W3 {
        &self.apb2otp_blk8_w3
    }
    #[doc = "0x68c - eFuse apb2otp block8 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w4(&self) -> &APB2OTP_BLK8_W4 {
        &self.apb2otp_blk8_w4
    }
    #[doc = "0x690 - eFuse apb2otp block8 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w5(&self) -> &APB2OTP_BLK8_W5 {
        &self.apb2otp_blk8_w5
    }
    #[doc = "0x694 - eFuse apb2otp block8 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w6(&self) -> &APB2OTP_BLK8_W6 {
        &self.apb2otp_blk8_w6
    }
    #[doc = "0x698 - eFuse apb2otp block8 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w7(&self) -> &APB2OTP_BLK8_W7 {
        &self.apb2otp_blk8_w7
    }
    #[doc = "0x69c - eFuse apb2otp block8 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w8(&self) -> &APB2OTP_BLK8_W8 {
        &self.apb2otp_blk8_w8
    }
    #[doc = "0x6a0 - eFuse apb2otp block8 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w9(&self) -> &APB2OTP_BLK8_W9 {
        &self.apb2otp_blk8_w9
    }
    #[doc = "0x6a4 - eFuse apb2otp block8 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w10(&self) -> &APB2OTP_BLK8_W10 {
        &self.apb2otp_blk8_w10
    }
    #[doc = "0x6a8 - eFuse apb2otp block8 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk8_w11(&self) -> &APB2OTP_BLK8_W11 {
        &self.apb2otp_blk8_w11
    }
    #[doc = "0x6ac - eFuse apb2otp block9 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w1(&self) -> &APB2OTP_BLK9_W1 {
        &self.apb2otp_blk9_w1
    }
    #[doc = "0x6b0 - eFuse apb2otp block9 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w2(&self) -> &APB2OTP_BLK9_W2 {
        &self.apb2otp_blk9_w2
    }
    #[doc = "0x6b4 - eFuse apb2otp block9 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w3(&self) -> &APB2OTP_BLK9_W3 {
        &self.apb2otp_blk9_w3
    }
    #[doc = "0x6b8 - eFuse apb2otp block9 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w4(&self) -> &APB2OTP_BLK9_W4 {
        &self.apb2otp_blk9_w4
    }
    #[doc = "0x6bc - eFuse apb2otp block9 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w5(&self) -> &APB2OTP_BLK9_W5 {
        &self.apb2otp_blk9_w5
    }
    #[doc = "0x6c0 - eFuse apb2otp block9 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w6(&self) -> &APB2OTP_BLK9_W6 {
        &self.apb2otp_blk9_w6
    }
    #[doc = "0x6c4 - eFuse apb2otp block9 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w7(&self) -> &APB2OTP_BLK9_W7 {
        &self.apb2otp_blk9_w7
    }
    #[doc = "0x6c8 - eFuse apb2otp block9 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w8(&self) -> &APB2OTP_BLK9_W8 {
        &self.apb2otp_blk9_w8
    }
    #[doc = "0x6cc - eFuse apb2otp block9 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w9(&self) -> &APB2OTP_BLK9_W9 {
        &self.apb2otp_blk9_w9
    }
    #[doc = "0x6d0 - eFuse apb2otp block9 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w10(&self) -> &APB2OTP_BLK9_W10 {
        &self.apb2otp_blk9_w10
    }
    #[doc = "0x6d4 - eFuse apb2otp block9 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk9_w11(&self) -> &APB2OTP_BLK9_W11 {
        &self.apb2otp_blk9_w11
    }
    #[doc = "0x6d8 - eFuse apb2otp block10 data register1."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w1(&self) -> &APB2OTP_BLK10_W1 {
        &self.apb2otp_blk10_w1
    }
    #[doc = "0x6dc - eFuse apb2otp block10 data register2."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w2(&self) -> &APB2OTP_BLK10_W2 {
        &self.apb2otp_blk10_w2
    }
    #[doc = "0x6e0 - eFuse apb2otp block10 data register3."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w3(&self) -> &APB2OTP_BLK10_W3 {
        &self.apb2otp_blk10_w3
    }
    #[doc = "0x6e4 - eFuse apb2otp block10 data register4."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w4(&self) -> &APB2OTP_BLK10_W4 {
        &self.apb2otp_blk10_w4
    }
    #[doc = "0x6e8 - eFuse apb2otp block10 data register5."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w5(&self) -> &APB2OTP_BLK10_W5 {
        &self.apb2otp_blk10_w5
    }
    #[doc = "0x6ec - eFuse apb2otp block10 data register6."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w6(&self) -> &APB2OTP_BLK10_W6 {
        &self.apb2otp_blk10_w6
    }
    #[doc = "0x6f0 - eFuse apb2otp block10 data register7."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w7(&self) -> &APB2OTP_BLK10_W7 {
        &self.apb2otp_blk10_w7
    }
    #[doc = "0x6f4 - eFuse apb2otp block10 data register8."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w8(&self) -> &APB2OTP_BLK10_W8 {
        &self.apb2otp_blk10_w8
    }
    #[doc = "0x6f8 - eFuse apb2otp block10 data register9."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w9(&self) -> &APB2OTP_BLK10_W9 {
        &self.apb2otp_blk10_w9
    }
    #[doc = "0x6fc - eFuse apb2otp block10 data register10."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w10(&self) -> &APB2OTP_BLK10_W10 {
        &self.apb2otp_blk10_w10
    }
    #[doc = "0x700 - eFuse apb2otp block10 data register11."]
    #[inline(always)]
    pub const fn apb2otp_blk10_w11(&self) -> &APB2OTP_BLK10_W11 {
        &self.apb2otp_blk10_w11
    }
    #[doc = "0x708 - eFuse apb2otp enable configuration register."]
    #[inline(always)]
    pub const fn apb2otp_en(&self) -> &APB2OTP_EN {
        &self.apb2otp_en
    }
}
#[doc = "PGM_DATA (rw) register accessor: Represents pgm_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`pgm_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgm_data`] module"]
pub type PGM_DATA = crate::Reg<pgm_data::PGM_DATA_SPEC>;
#[doc = "Represents pgm_data%s"]
pub mod pgm_data;
#[doc = "PGM_CHECK_VALUE (rw) register accessor: Represents pgm_check_value%s\n\nYou can [`read`](crate::Reg::read) this register and get [`pgm_check_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_check_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgm_check_value`] module"]
pub type PGM_CHECK_VALUE = crate::Reg<pgm_check_value::PGM_CHECK_VALUE_SPEC>;
#[doc = "Represents pgm_check_value%s"]
pub mod pgm_check_value;
#[doc = "RD_WR_DIS0 (r) register accessor: Represents rd_wr_dis\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_wr_dis0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_wr_dis0`] module"]
pub type RD_WR_DIS0 = crate::Reg<rd_wr_dis0::RD_WR_DIS0_SPEC>;
#[doc = "Represents rd_wr_dis"]
pub mod rd_wr_dis0;
#[doc = "RD_REPEAT_DATA0 (r) register accessor: Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data0`] module"]
pub type RD_REPEAT_DATA0 = crate::Reg<rd_repeat_data0::RD_REPEAT_DATA0_SPEC>;
#[doc = "Represents rd_repeat_data"]
pub mod rd_repeat_data0;
#[doc = "RD_REPEAT_DATA1 (r) register accessor: Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data1`] module"]
pub type RD_REPEAT_DATA1 = crate::Reg<rd_repeat_data1::RD_REPEAT_DATA1_SPEC>;
#[doc = "Represents rd_repeat_data"]
pub mod rd_repeat_data1;
#[doc = "RD_REPEAT_DATA2 (r) register accessor: Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data2`] module"]
pub type RD_REPEAT_DATA2 = crate::Reg<rd_repeat_data2::RD_REPEAT_DATA2_SPEC>;
#[doc = "Represents rd_repeat_data"]
pub mod rd_repeat_data2;
#[doc = "RD_REPEAT_DATA3 (r) register accessor: Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data3`] module"]
pub type RD_REPEAT_DATA3 = crate::Reg<rd_repeat_data3::RD_REPEAT_DATA3_SPEC>;
#[doc = "Represents rd_repeat_data"]
pub mod rd_repeat_data3;
#[doc = "RD_REPEAT_DATA4 (r) register accessor: Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data4`] module"]
pub type RD_REPEAT_DATA4 = crate::Reg<rd_repeat_data4::RD_REPEAT_DATA4_SPEC>;
#[doc = "Represents rd_repeat_data"]
pub mod rd_repeat_data4;
#[doc = "RD_MAC_SYS0 (r) register accessor: Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_sys0`] module"]
pub type RD_MAC_SYS0 = crate::Reg<rd_mac_sys0::RD_MAC_SYS0_SPEC>;
#[doc = "Represents rd_mac_sys"]
pub mod rd_mac_sys0;
#[doc = "RD_MAC_SYS1 (r) register accessor: Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_sys1`] module"]
pub type RD_MAC_SYS1 = crate::Reg<rd_mac_sys1::RD_MAC_SYS1_SPEC>;
#[doc = "Represents rd_mac_sys"]
pub mod rd_mac_sys1;
#[doc = "RD_MAC_SYS2 (r) register accessor: Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_sys2`] module"]
pub type RD_MAC_SYS2 = crate::Reg<rd_mac_sys2::RD_MAC_SYS2_SPEC>;
#[doc = "Represents rd_mac_sys"]
pub mod rd_mac_sys2;
#[doc = "RD_MAC_SYS3 (r) register accessor: Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_sys3`] module"]
pub type RD_MAC_SYS3 = crate::Reg<rd_mac_sys3::RD_MAC_SYS3_SPEC>;
#[doc = "Represents rd_mac_sys"]
pub mod rd_mac_sys3;
#[doc = "RD_MAC_SYS4 (r) register accessor: Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_sys4`] module"]
pub type RD_MAC_SYS4 = crate::Reg<rd_mac_sys4::RD_MAC_SYS4_SPEC>;
#[doc = "Represents rd_mac_sys"]
pub mod rd_mac_sys4;
#[doc = "RD_MAC_SYS5 (r) register accessor: Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_sys5`] module"]
pub type RD_MAC_SYS5 = crate::Reg<rd_mac_sys5::RD_MAC_SYS5_SPEC>;
#[doc = "Represents rd_mac_sys"]
pub mod rd_mac_sys5;
#[doc = "RD_SYS_PART1_DATA (r) register accessor: Represents rd_sys_part1_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part1_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_sys_part1_data`] module"]
pub type RD_SYS_PART1_DATA = crate::Reg<rd_sys_part1_data::RD_SYS_PART1_DATA_SPEC>;
#[doc = "Represents rd_sys_part1_data%s"]
pub mod rd_sys_part1_data;
#[doc = "RD_USR_DATA (r) register accessor: Represents rd_usr_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_usr_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_usr_data`] module"]
pub type RD_USR_DATA = crate::Reg<rd_usr_data::RD_USR_DATA_SPEC>;
#[doc = "Represents rd_usr_data%s"]
pub mod rd_usr_data;
#[doc = "RD_KEY0_DATA (r) register accessor: Represents rd_key0_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key0_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_key0_data`] module"]
pub type RD_KEY0_DATA = crate::Reg<rd_key0_data::RD_KEY0_DATA_SPEC>;
#[doc = "Represents rd_key0_data%s"]
pub mod rd_key0_data;
#[doc = "RD_KEY1_DATA (r) register accessor: Represents rd_key1_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key1_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_key1_data`] module"]
pub type RD_KEY1_DATA = crate::Reg<rd_key1_data::RD_KEY1_DATA_SPEC>;
#[doc = "Represents rd_key1_data%s"]
pub mod rd_key1_data;
#[doc = "RD_KEY2_DATA (r) register accessor: Represents rd_key2_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key2_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_key2_data`] module"]
pub type RD_KEY2_DATA = crate::Reg<rd_key2_data::RD_KEY2_DATA_SPEC>;
#[doc = "Represents rd_key2_data%s"]
pub mod rd_key2_data;
#[doc = "RD_KEY3_DATA (r) register accessor: Represents rd_key3_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key3_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_key3_data`] module"]
pub type RD_KEY3_DATA = crate::Reg<rd_key3_data::RD_KEY3_DATA_SPEC>;
#[doc = "Represents rd_key3_data%s"]
pub mod rd_key3_data;
#[doc = "RD_KEY4_DATA (r) register accessor: Represents rd_key4_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key4_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_key4_data`] module"]
pub type RD_KEY4_DATA = crate::Reg<rd_key4_data::RD_KEY4_DATA_SPEC>;
#[doc = "Represents rd_key4_data%s"]
pub mod rd_key4_data;
#[doc = "RD_KEY5_DATA (r) register accessor: Represents rd_key5_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_key5_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_key5_data`] module"]
pub type RD_KEY5_DATA = crate::Reg<rd_key5_data::RD_KEY5_DATA_SPEC>;
#[doc = "Represents rd_key5_data%s"]
pub mod rd_key5_data;
#[doc = "RD_SYS_PART2_DATA (r) register accessor: Represents rd_sys_part2_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_sys_part2_data`] module"]
pub type RD_SYS_PART2_DATA = crate::Reg<rd_sys_part2_data::RD_SYS_PART2_DATA_SPEC>;
#[doc = "Represents rd_sys_part2_data%s"]
pub mod rd_sys_part2_data;
#[doc = "RD_REPEAT_DATA_ERR0 (r) register accessor: Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data_err0`] module"]
pub type RD_REPEAT_DATA_ERR0 = crate::Reg<rd_repeat_data_err0::RD_REPEAT_DATA_ERR0_SPEC>;
#[doc = "Represents rd_repeat_data_err"]
pub mod rd_repeat_data_err0;
#[doc = "RD_REPEAT_DATA_ERR1 (r) register accessor: Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data_err1`] module"]
pub type RD_REPEAT_DATA_ERR1 = crate::Reg<rd_repeat_data_err1::RD_REPEAT_DATA_ERR1_SPEC>;
#[doc = "Represents rd_repeat_data_err"]
pub mod rd_repeat_data_err1;
#[doc = "RD_REPEAT_DATA_ERR2 (r) register accessor: Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data_err2`] module"]
pub type RD_REPEAT_DATA_ERR2 = crate::Reg<rd_repeat_data_err2::RD_REPEAT_DATA_ERR2_SPEC>;
#[doc = "Represents rd_repeat_data_err"]
pub mod rd_repeat_data_err2;
#[doc = "RD_REPEAT_DATA_ERR3 (r) register accessor: Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data_err3`] module"]
pub type RD_REPEAT_DATA_ERR3 = crate::Reg<rd_repeat_data_err3::RD_REPEAT_DATA_ERR3_SPEC>;
#[doc = "Represents rd_repeat_data_err"]
pub mod rd_repeat_data_err3;
#[doc = "RD_REPEAT_DATA_ERR4 (r) register accessor: Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data_err4`] module"]
pub type RD_REPEAT_DATA_ERR4 = crate::Reg<rd_repeat_data_err4::RD_REPEAT_DATA_ERR4_SPEC>;
#[doc = "Represents rd_repeat_data_err"]
pub mod rd_repeat_data_err4;
#[doc = "RD_RS_DATA_ERR0 (r) register accessor: Represents rd_rs_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_rs_data_err0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_rs_data_err0`] module"]
pub type RD_RS_DATA_ERR0 = crate::Reg<rd_rs_data_err0::RD_RS_DATA_ERR0_SPEC>;
#[doc = "Represents rd_rs_data_err"]
pub mod rd_rs_data_err0;
#[doc = "RD_RS_DATA_ERR1 (r) register accessor: Represents rd_rs_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_rs_data_err1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_rs_data_err1`] module"]
pub type RD_RS_DATA_ERR1 = crate::Reg<rd_rs_data_err1::RD_RS_DATA_ERR1_SPEC>;
#[doc = "Represents rd_rs_data_err"]
pub mod rd_rs_data_err1;
#[doc = "DATE (rw) register accessor: eFuse version register.\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "eFuse version register."]
pub mod date;
#[doc = "CLK (rw) register accessor: eFuse clcok configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "eFuse clcok configuration register."]
pub mod clk;
#[doc = "CONF (rw) register accessor: eFuse operation mode configuraiton register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "eFuse operation mode configuraiton register"]
pub mod conf;
#[doc = "STATUS (r) register accessor: eFuse status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "eFuse status register."]
pub mod status;
#[doc = "CMD (rw) register accessor: eFuse command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "eFuse command register."]
pub mod cmd;
#[doc = "INT_RAW (r) register accessor: eFuse raw interrupt register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "eFuse raw interrupt register."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: eFuse interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "eFuse interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: eFuse interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "eFuse interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: eFuse interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "eFuse interrupt clear register."]
pub mod int_clr;
#[doc = "DAC_CONF (rw) register accessor: Controls the eFuse programming voltage.\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_conf`] module"]
pub type DAC_CONF = crate::Reg<dac_conf::DAC_CONF_SPEC>;
#[doc = "Controls the eFuse programming voltage."]
pub mod dac_conf;
#[doc = "RD_TIM_CONF (rw) register accessor: Configures read timing parameters.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_tim_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_tim_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_tim_conf`] module"]
pub type RD_TIM_CONF = crate::Reg<rd_tim_conf::RD_TIM_CONF_SPEC>;
#[doc = "Configures read timing parameters."]
pub mod rd_tim_conf;
#[doc = "WR_TIM_CONF1 (rw) register accessor: Configurarion register 1 of eFuse programming timing parameters.\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_tim_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_tim_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_tim_conf1`] module"]
pub type WR_TIM_CONF1 = crate::Reg<wr_tim_conf1::WR_TIM_CONF1_SPEC>;
#[doc = "Configurarion register 1 of eFuse programming timing parameters."]
pub mod wr_tim_conf1;
#[doc = "WR_TIM_CONF2 (rw) register accessor: Configurarion register 2 of eFuse programming timing parameters.\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_tim_conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_tim_conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_tim_conf2`] module"]
pub type WR_TIM_CONF2 = crate::Reg<wr_tim_conf2::WR_TIM_CONF2_SPEC>;
#[doc = "Configurarion register 2 of eFuse programming timing parameters."]
pub mod wr_tim_conf2;
#[doc = "WR_TIM_CONF0_RS_BYPASS (rw) register accessor: Configurarion register0 of eFuse programming time parameters and rs bypass operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_tim_conf0_rs_bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_tim_conf0_rs_bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_tim_conf0_rs_bypass`] module"]
pub type WR_TIM_CONF0_RS_BYPASS = crate::Reg<wr_tim_conf0_rs_bypass::WR_TIM_CONF0_RS_BYPASS_SPEC>;
#[doc = "Configurarion register0 of eFuse programming time parameters and rs bypass operation."]
pub mod wr_tim_conf0_rs_bypass;
#[doc = "APB2OTP_WR_DIS (r) register accessor: eFuse apb2otp block0 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_wr_dis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_wr_dis`] module"]
pub type APB2OTP_WR_DIS = crate::Reg<apb2otp_wr_dis::APB2OTP_WR_DIS_SPEC>;
#[doc = "eFuse apb2otp block0 data register1."]
pub mod apb2otp_wr_dis;
#[doc = "APB2OTP_BLK0_BACKUP1_W1 (r) register accessor: eFuse apb2otp block0 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup1_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup1_w1`] module"]
pub type APB2OTP_BLK0_BACKUP1_W1 =
    crate::Reg<apb2otp_blk0_backup1_w1::APB2OTP_BLK0_BACKUP1_W1_SPEC>;
#[doc = "eFuse apb2otp block0 data register2."]
pub mod apb2otp_blk0_backup1_w1;
#[doc = "APB2OTP_BLK0_BACKUP1_W2 (r) register accessor: eFuse apb2otp block0 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup1_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup1_w2`] module"]
pub type APB2OTP_BLK0_BACKUP1_W2 =
    crate::Reg<apb2otp_blk0_backup1_w2::APB2OTP_BLK0_BACKUP1_W2_SPEC>;
#[doc = "eFuse apb2otp block0 data register3."]
pub mod apb2otp_blk0_backup1_w2;
#[doc = "APB2OTP_BLK0_BACKUP1_W3 (r) register accessor: eFuse apb2otp block0 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup1_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup1_w3`] module"]
pub type APB2OTP_BLK0_BACKUP1_W3 =
    crate::Reg<apb2otp_blk0_backup1_w3::APB2OTP_BLK0_BACKUP1_W3_SPEC>;
#[doc = "eFuse apb2otp block0 data register4."]
pub mod apb2otp_blk0_backup1_w3;
#[doc = "APB2OTP_BLK0_BACKUP1_W4 (r) register accessor: eFuse apb2otp block0 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup1_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup1_w4`] module"]
pub type APB2OTP_BLK0_BACKUP1_W4 =
    crate::Reg<apb2otp_blk0_backup1_w4::APB2OTP_BLK0_BACKUP1_W4_SPEC>;
#[doc = "eFuse apb2otp block0 data register5."]
pub mod apb2otp_blk0_backup1_w4;
#[doc = "APB2OTP_BLK0_BACKUP1_W5 (r) register accessor: eFuse apb2otp block0 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup1_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup1_w5`] module"]
pub type APB2OTP_BLK0_BACKUP1_W5 =
    crate::Reg<apb2otp_blk0_backup1_w5::APB2OTP_BLK0_BACKUP1_W5_SPEC>;
#[doc = "eFuse apb2otp block0 data register6."]
pub mod apb2otp_blk0_backup1_w5;
#[doc = "APB2OTP_BLK0_BACKUP2_W1 (r) register accessor: eFuse apb2otp block0 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup2_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup2_w1`] module"]
pub type APB2OTP_BLK0_BACKUP2_W1 =
    crate::Reg<apb2otp_blk0_backup2_w1::APB2OTP_BLK0_BACKUP2_W1_SPEC>;
#[doc = "eFuse apb2otp block0 data register7."]
pub mod apb2otp_blk0_backup2_w1;
#[doc = "APB2OTP_BLK0_BACKUP2_W2 (r) register accessor: eFuse apb2otp block0 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup2_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup2_w2`] module"]
pub type APB2OTP_BLK0_BACKUP2_W2 =
    crate::Reg<apb2otp_blk0_backup2_w2::APB2OTP_BLK0_BACKUP2_W2_SPEC>;
#[doc = "eFuse apb2otp block0 data register8."]
pub mod apb2otp_blk0_backup2_w2;
#[doc = "APB2OTP_BLK0_BACKUP2_W3 (r) register accessor: eFuse apb2otp block0 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup2_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup2_w3`] module"]
pub type APB2OTP_BLK0_BACKUP2_W3 =
    crate::Reg<apb2otp_blk0_backup2_w3::APB2OTP_BLK0_BACKUP2_W3_SPEC>;
#[doc = "eFuse apb2otp block0 data register9."]
pub mod apb2otp_blk0_backup2_w3;
#[doc = "APB2OTP_BLK0_BACKUP2_W4 (r) register accessor: eFuse apb2otp block0 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup2_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup2_w4`] module"]
pub type APB2OTP_BLK0_BACKUP2_W4 =
    crate::Reg<apb2otp_blk0_backup2_w4::APB2OTP_BLK0_BACKUP2_W4_SPEC>;
#[doc = "eFuse apb2otp block0 data register10."]
pub mod apb2otp_blk0_backup2_w4;
#[doc = "APB2OTP_BLK0_BACKUP2_W5 (r) register accessor: eFuse apb2otp block0 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup2_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup2_w5`] module"]
pub type APB2OTP_BLK0_BACKUP2_W5 =
    crate::Reg<apb2otp_blk0_backup2_w5::APB2OTP_BLK0_BACKUP2_W5_SPEC>;
#[doc = "eFuse apb2otp block0 data register11."]
pub mod apb2otp_blk0_backup2_w5;
#[doc = "APB2OTP_BLK0_BACKUP3_W1 (r) register accessor: eFuse apb2otp block0 data register12.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup3_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup3_w1`] module"]
pub type APB2OTP_BLK0_BACKUP3_W1 =
    crate::Reg<apb2otp_blk0_backup3_w1::APB2OTP_BLK0_BACKUP3_W1_SPEC>;
#[doc = "eFuse apb2otp block0 data register12."]
pub mod apb2otp_blk0_backup3_w1;
#[doc = "APB2OTP_BLK0_BACKUP3_W2 (r) register accessor: eFuse apb2otp block0 data register13.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup3_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup3_w2`] module"]
pub type APB2OTP_BLK0_BACKUP3_W2 =
    crate::Reg<apb2otp_blk0_backup3_w2::APB2OTP_BLK0_BACKUP3_W2_SPEC>;
#[doc = "eFuse apb2otp block0 data register13."]
pub mod apb2otp_blk0_backup3_w2;
#[doc = "APB2OTP_BLK0_BACKUP3_W3 (r) register accessor: eFuse apb2otp block0 data register14.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup3_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup3_w3`] module"]
pub type APB2OTP_BLK0_BACKUP3_W3 =
    crate::Reg<apb2otp_blk0_backup3_w3::APB2OTP_BLK0_BACKUP3_W3_SPEC>;
#[doc = "eFuse apb2otp block0 data register14."]
pub mod apb2otp_blk0_backup3_w3;
#[doc = "APB2OTP_BLK0_BACKUP3_W4 (r) register accessor: eFuse apb2otp block0 data register15.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup3_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup3_w4`] module"]
pub type APB2OTP_BLK0_BACKUP3_W4 =
    crate::Reg<apb2otp_blk0_backup3_w4::APB2OTP_BLK0_BACKUP3_W4_SPEC>;
#[doc = "eFuse apb2otp block0 data register15."]
pub mod apb2otp_blk0_backup3_w4;
#[doc = "APB2OTP_BLK0_BACKUP3_W5 (r) register accessor: eFuse apb2otp block0 data register16.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup3_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup3_w5`] module"]
pub type APB2OTP_BLK0_BACKUP3_W5 =
    crate::Reg<apb2otp_blk0_backup3_w5::APB2OTP_BLK0_BACKUP3_W5_SPEC>;
#[doc = "eFuse apb2otp block0 data register16."]
pub mod apb2otp_blk0_backup3_w5;
#[doc = "APB2OTP_BLK0_BACKUP4_W1 (r) register accessor: eFuse apb2otp block0 data register17.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup4_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup4_w1`] module"]
pub type APB2OTP_BLK0_BACKUP4_W1 =
    crate::Reg<apb2otp_blk0_backup4_w1::APB2OTP_BLK0_BACKUP4_W1_SPEC>;
#[doc = "eFuse apb2otp block0 data register17."]
pub mod apb2otp_blk0_backup4_w1;
#[doc = "APB2OTP_BLK0_BACKUP4_W2 (r) register accessor: eFuse apb2otp block0 data register18.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup4_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup4_w2`] module"]
pub type APB2OTP_BLK0_BACKUP4_W2 =
    crate::Reg<apb2otp_blk0_backup4_w2::APB2OTP_BLK0_BACKUP4_W2_SPEC>;
#[doc = "eFuse apb2otp block0 data register18."]
pub mod apb2otp_blk0_backup4_w2;
#[doc = "APB2OTP_BLK0_BACKUP4_W3 (r) register accessor: eFuse apb2otp block0 data register19.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup4_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup4_w3`] module"]
pub type APB2OTP_BLK0_BACKUP4_W3 =
    crate::Reg<apb2otp_blk0_backup4_w3::APB2OTP_BLK0_BACKUP4_W3_SPEC>;
#[doc = "eFuse apb2otp block0 data register19."]
pub mod apb2otp_blk0_backup4_w3;
#[doc = "APB2OTP_BLK0_BACKUP4_W4 (r) register accessor: eFuse apb2otp block0 data register20.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup4_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup4_w4`] module"]
pub type APB2OTP_BLK0_BACKUP4_W4 =
    crate::Reg<apb2otp_blk0_backup4_w4::APB2OTP_BLK0_BACKUP4_W4_SPEC>;
#[doc = "eFuse apb2otp block0 data register20."]
pub mod apb2otp_blk0_backup4_w4;
#[doc = "APB2OTP_BLK0_BACKUP4_W5 (r) register accessor: eFuse apb2otp block0 data register21.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup4_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk0_backup4_w5`] module"]
pub type APB2OTP_BLK0_BACKUP4_W5 =
    crate::Reg<apb2otp_blk0_backup4_w5::APB2OTP_BLK0_BACKUP4_W5_SPEC>;
#[doc = "eFuse apb2otp block0 data register21."]
pub mod apb2otp_blk0_backup4_w5;
#[doc = "APB2OTP_BLK1_W1 (r) register accessor: eFuse apb2otp block1 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w1`] module"]
pub type APB2OTP_BLK1_W1 = crate::Reg<apb2otp_blk1_w1::APB2OTP_BLK1_W1_SPEC>;
#[doc = "eFuse apb2otp block1 data register1."]
pub mod apb2otp_blk1_w1;
#[doc = "APB2OTP_BLK1_W2 (r) register accessor: eFuse apb2otp block1 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w2`] module"]
pub type APB2OTP_BLK1_W2 = crate::Reg<apb2otp_blk1_w2::APB2OTP_BLK1_W2_SPEC>;
#[doc = "eFuse apb2otp block1 data register2."]
pub mod apb2otp_blk1_w2;
#[doc = "APB2OTP_BLK1_W3 (r) register accessor: eFuse apb2otp block1 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w3`] module"]
pub type APB2OTP_BLK1_W3 = crate::Reg<apb2otp_blk1_w3::APB2OTP_BLK1_W3_SPEC>;
#[doc = "eFuse apb2otp block1 data register3."]
pub mod apb2otp_blk1_w3;
#[doc = "APB2OTP_BLK1_W4 (r) register accessor: eFuse apb2otp block1 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w4`] module"]
pub type APB2OTP_BLK1_W4 = crate::Reg<apb2otp_blk1_w4::APB2OTP_BLK1_W4_SPEC>;
#[doc = "eFuse apb2otp block1 data register4."]
pub mod apb2otp_blk1_w4;
#[doc = "APB2OTP_BLK1_W5 (r) register accessor: eFuse apb2otp block1 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w5`] module"]
pub type APB2OTP_BLK1_W5 = crate::Reg<apb2otp_blk1_w5::APB2OTP_BLK1_W5_SPEC>;
#[doc = "eFuse apb2otp block1 data register5."]
pub mod apb2otp_blk1_w5;
#[doc = "APB2OTP_BLK1_W6 (r) register accessor: eFuse apb2otp block1 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w6`] module"]
pub type APB2OTP_BLK1_W6 = crate::Reg<apb2otp_blk1_w6::APB2OTP_BLK1_W6_SPEC>;
#[doc = "eFuse apb2otp block1 data register6."]
pub mod apb2otp_blk1_w6;
#[doc = "APB2OTP_BLK1_W7 (r) register accessor: eFuse apb2otp block1 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w7`] module"]
pub type APB2OTP_BLK1_W7 = crate::Reg<apb2otp_blk1_w7::APB2OTP_BLK1_W7_SPEC>;
#[doc = "eFuse apb2otp block1 data register7."]
pub mod apb2otp_blk1_w7;
#[doc = "APB2OTP_BLK1_W8 (r) register accessor: eFuse apb2otp block1 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w8`] module"]
pub type APB2OTP_BLK1_W8 = crate::Reg<apb2otp_blk1_w8::APB2OTP_BLK1_W8_SPEC>;
#[doc = "eFuse apb2otp block1 data register8."]
pub mod apb2otp_blk1_w8;
#[doc = "APB2OTP_BLK1_W9 (r) register accessor: eFuse apb2otp block1 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk1_w9`] module"]
pub type APB2OTP_BLK1_W9 = crate::Reg<apb2otp_blk1_w9::APB2OTP_BLK1_W9_SPEC>;
#[doc = "eFuse apb2otp block1 data register9."]
pub mod apb2otp_blk1_w9;
#[doc = "APB2OTP_BLK2_W1 (r) register accessor: eFuse apb2otp block2 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w1`] module"]
pub type APB2OTP_BLK2_W1 = crate::Reg<apb2otp_blk2_w1::APB2OTP_BLK2_W1_SPEC>;
#[doc = "eFuse apb2otp block2 data register1."]
pub mod apb2otp_blk2_w1;
#[doc = "APB2OTP_BLK2_W2 (r) register accessor: eFuse apb2otp block2 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w2`] module"]
pub type APB2OTP_BLK2_W2 = crate::Reg<apb2otp_blk2_w2::APB2OTP_BLK2_W2_SPEC>;
#[doc = "eFuse apb2otp block2 data register2."]
pub mod apb2otp_blk2_w2;
#[doc = "APB2OTP_BLK2_W3 (r) register accessor: eFuse apb2otp block2 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w3`] module"]
pub type APB2OTP_BLK2_W3 = crate::Reg<apb2otp_blk2_w3::APB2OTP_BLK2_W3_SPEC>;
#[doc = "eFuse apb2otp block2 data register3."]
pub mod apb2otp_blk2_w3;
#[doc = "APB2OTP_BLK2_W4 (r) register accessor: eFuse apb2otp block2 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w4`] module"]
pub type APB2OTP_BLK2_W4 = crate::Reg<apb2otp_blk2_w4::APB2OTP_BLK2_W4_SPEC>;
#[doc = "eFuse apb2otp block2 data register4."]
pub mod apb2otp_blk2_w4;
#[doc = "APB2OTP_BLK2_W5 (r) register accessor: eFuse apb2otp block2 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w5`] module"]
pub type APB2OTP_BLK2_W5 = crate::Reg<apb2otp_blk2_w5::APB2OTP_BLK2_W5_SPEC>;
#[doc = "eFuse apb2otp block2 data register5."]
pub mod apb2otp_blk2_w5;
#[doc = "APB2OTP_BLK2_W6 (r) register accessor: eFuse apb2otp block2 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w6`] module"]
pub type APB2OTP_BLK2_W6 = crate::Reg<apb2otp_blk2_w6::APB2OTP_BLK2_W6_SPEC>;
#[doc = "eFuse apb2otp block2 data register6."]
pub mod apb2otp_blk2_w6;
#[doc = "APB2OTP_BLK2_W7 (r) register accessor: eFuse apb2otp block2 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w7`] module"]
pub type APB2OTP_BLK2_W7 = crate::Reg<apb2otp_blk2_w7::APB2OTP_BLK2_W7_SPEC>;
#[doc = "eFuse apb2otp block2 data register7."]
pub mod apb2otp_blk2_w7;
#[doc = "APB2OTP_BLK2_W8 (r) register accessor: eFuse apb2otp block2 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w8`] module"]
pub type APB2OTP_BLK2_W8 = crate::Reg<apb2otp_blk2_w8::APB2OTP_BLK2_W8_SPEC>;
#[doc = "eFuse apb2otp block2 data register8."]
pub mod apb2otp_blk2_w8;
#[doc = "APB2OTP_BLK2_W9 (r) register accessor: eFuse apb2otp block2 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w9`] module"]
pub type APB2OTP_BLK2_W9 = crate::Reg<apb2otp_blk2_w9::APB2OTP_BLK2_W9_SPEC>;
#[doc = "eFuse apb2otp block2 data register9."]
pub mod apb2otp_blk2_w9;
#[doc = "APB2OTP_BLK2_W10 (r) register accessor: eFuse apb2otp block2 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w10`] module"]
pub type APB2OTP_BLK2_W10 = crate::Reg<apb2otp_blk2_w10::APB2OTP_BLK2_W10_SPEC>;
#[doc = "eFuse apb2otp block2 data register10."]
pub mod apb2otp_blk2_w10;
#[doc = "APB2OTP_BLK2_W11 (r) register accessor: eFuse apb2otp block2 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk2_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk2_w11`] module"]
pub type APB2OTP_BLK2_W11 = crate::Reg<apb2otp_blk2_w11::APB2OTP_BLK2_W11_SPEC>;
#[doc = "eFuse apb2otp block2 data register11."]
pub mod apb2otp_blk2_w11;
#[doc = "APB2OTP_BLK3_W1 (r) register accessor: eFuse apb2otp block3 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w1`] module"]
pub type APB2OTP_BLK3_W1 = crate::Reg<apb2otp_blk3_w1::APB2OTP_BLK3_W1_SPEC>;
#[doc = "eFuse apb2otp block3 data register1."]
pub mod apb2otp_blk3_w1;
#[doc = "APB2OTP_BLK3_W2 (r) register accessor: eFuse apb2otp block3 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w2`] module"]
pub type APB2OTP_BLK3_W2 = crate::Reg<apb2otp_blk3_w2::APB2OTP_BLK3_W2_SPEC>;
#[doc = "eFuse apb2otp block3 data register2."]
pub mod apb2otp_blk3_w2;
#[doc = "APB2OTP_BLK3_W3 (r) register accessor: eFuse apb2otp block3 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w3`] module"]
pub type APB2OTP_BLK3_W3 = crate::Reg<apb2otp_blk3_w3::APB2OTP_BLK3_W3_SPEC>;
#[doc = "eFuse apb2otp block3 data register3."]
pub mod apb2otp_blk3_w3;
#[doc = "APB2OTP_BLK3_W4 (r) register accessor: eFuse apb2otp block3 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w4`] module"]
pub type APB2OTP_BLK3_W4 = crate::Reg<apb2otp_blk3_w4::APB2OTP_BLK3_W4_SPEC>;
#[doc = "eFuse apb2otp block3 data register4."]
pub mod apb2otp_blk3_w4;
#[doc = "APB2OTP_BLK3_W5 (r) register accessor: eFuse apb2otp block3 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w5`] module"]
pub type APB2OTP_BLK3_W5 = crate::Reg<apb2otp_blk3_w5::APB2OTP_BLK3_W5_SPEC>;
#[doc = "eFuse apb2otp block3 data register5."]
pub mod apb2otp_blk3_w5;
#[doc = "APB2OTP_BLK3_W6 (r) register accessor: eFuse apb2otp block3 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w6`] module"]
pub type APB2OTP_BLK3_W6 = crate::Reg<apb2otp_blk3_w6::APB2OTP_BLK3_W6_SPEC>;
#[doc = "eFuse apb2otp block3 data register6."]
pub mod apb2otp_blk3_w6;
#[doc = "APB2OTP_BLK3_W7 (r) register accessor: eFuse apb2otp block3 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w7`] module"]
pub type APB2OTP_BLK3_W7 = crate::Reg<apb2otp_blk3_w7::APB2OTP_BLK3_W7_SPEC>;
#[doc = "eFuse apb2otp block3 data register7."]
pub mod apb2otp_blk3_w7;
#[doc = "APB2OTP_BLK3_W8 (r) register accessor: eFuse apb2otp block3 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w8`] module"]
pub type APB2OTP_BLK3_W8 = crate::Reg<apb2otp_blk3_w8::APB2OTP_BLK3_W8_SPEC>;
#[doc = "eFuse apb2otp block3 data register8."]
pub mod apb2otp_blk3_w8;
#[doc = "APB2OTP_BLK3_W9 (r) register accessor: eFuse apb2otp block3 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w9`] module"]
pub type APB2OTP_BLK3_W9 = crate::Reg<apb2otp_blk3_w9::APB2OTP_BLK3_W9_SPEC>;
#[doc = "eFuse apb2otp block3 data register9."]
pub mod apb2otp_blk3_w9;
#[doc = "APB2OTP_BLK3_W10 (r) register accessor: eFuse apb2otp block3 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w10`] module"]
pub type APB2OTP_BLK3_W10 = crate::Reg<apb2otp_blk3_w10::APB2OTP_BLK3_W10_SPEC>;
#[doc = "eFuse apb2otp block3 data register10."]
pub mod apb2otp_blk3_w10;
#[doc = "APB2OTP_BLK3_W11 (r) register accessor: eFuse apb2otp block3 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk3_w11`] module"]
pub type APB2OTP_BLK3_W11 = crate::Reg<apb2otp_blk3_w11::APB2OTP_BLK3_W11_SPEC>;
#[doc = "eFuse apb2otp block3 data register11."]
pub mod apb2otp_blk3_w11;
#[doc = "APB2OTP_BLK4_W1 (r) register accessor: eFuse apb2otp BLOCK7 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w1`] module"]
pub type APB2OTP_BLK4_W1 = crate::Reg<apb2otp_blk4_w1::APB2OTP_BLK4_W1_SPEC>;
#[doc = "eFuse apb2otp BLOCK7 data register1."]
pub mod apb2otp_blk4_w1;
#[doc = "APB2OTP_BLK4_W2 (r) register accessor: eFuse apb2otp block4 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w2`] module"]
pub type APB2OTP_BLK4_W2 = crate::Reg<apb2otp_blk4_w2::APB2OTP_BLK4_W2_SPEC>;
#[doc = "eFuse apb2otp block4 data register2."]
pub mod apb2otp_blk4_w2;
#[doc = "APB2OTP_BLK4_W3 (r) register accessor: eFuse apb2otp block4 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w3`] module"]
pub type APB2OTP_BLK4_W3 = crate::Reg<apb2otp_blk4_w3::APB2OTP_BLK4_W3_SPEC>;
#[doc = "eFuse apb2otp block4 data register3."]
pub mod apb2otp_blk4_w3;
#[doc = "APB2OTP_BLK4_W4 (r) register accessor: eFuse apb2otp block4 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w4`] module"]
pub type APB2OTP_BLK4_W4 = crate::Reg<apb2otp_blk4_w4::APB2OTP_BLK4_W4_SPEC>;
#[doc = "eFuse apb2otp block4 data register4."]
pub mod apb2otp_blk4_w4;
#[doc = "APB2OTP_BLK4_W5 (r) register accessor: eFuse apb2otp block4 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w5`] module"]
pub type APB2OTP_BLK4_W5 = crate::Reg<apb2otp_blk4_w5::APB2OTP_BLK4_W5_SPEC>;
#[doc = "eFuse apb2otp block4 data register5."]
pub mod apb2otp_blk4_w5;
#[doc = "APB2OTP_BLK4_W6 (r) register accessor: eFuse apb2otp block4 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w6`] module"]
pub type APB2OTP_BLK4_W6 = crate::Reg<apb2otp_blk4_w6::APB2OTP_BLK4_W6_SPEC>;
#[doc = "eFuse apb2otp block4 data register6."]
pub mod apb2otp_blk4_w6;
#[doc = "APB2OTP_BLK4_W7 (r) register accessor: eFuse apb2otp block4 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w7`] module"]
pub type APB2OTP_BLK4_W7 = crate::Reg<apb2otp_blk4_w7::APB2OTP_BLK4_W7_SPEC>;
#[doc = "eFuse apb2otp block4 data register7."]
pub mod apb2otp_blk4_w7;
#[doc = "APB2OTP_BLK4_W8 (r) register accessor: eFuse apb2otp block4 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w8`] module"]
pub type APB2OTP_BLK4_W8 = crate::Reg<apb2otp_blk4_w8::APB2OTP_BLK4_W8_SPEC>;
#[doc = "eFuse apb2otp block4 data register8."]
pub mod apb2otp_blk4_w8;
#[doc = "APB2OTP_BLK4_W9 (r) register accessor: eFuse apb2otp block4 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w9`] module"]
pub type APB2OTP_BLK4_W9 = crate::Reg<apb2otp_blk4_w9::APB2OTP_BLK4_W9_SPEC>;
#[doc = "eFuse apb2otp block4 data register9."]
pub mod apb2otp_blk4_w9;
#[doc = "APB2OTP_BLK4_W10 (r) register accessor: eFuse apb2otp block4 data registe10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w10`] module"]
pub type APB2OTP_BLK4_W10 = crate::Reg<apb2otp_blk4_w10::APB2OTP_BLK4_W10_SPEC>;
#[doc = "eFuse apb2otp block4 data registe10."]
pub mod apb2otp_blk4_w10;
#[doc = "APB2OTP_BLK4_W11 (r) register accessor: eFuse apb2otp block4 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk4_w11`] module"]
pub type APB2OTP_BLK4_W11 = crate::Reg<apb2otp_blk4_w11::APB2OTP_BLK4_W11_SPEC>;
#[doc = "eFuse apb2otp block4 data register11."]
pub mod apb2otp_blk4_w11;
#[doc = "APB2OTP_BLK5_W1 (r) register accessor: eFuse apb2otp block5 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w1`] module"]
pub type APB2OTP_BLK5_W1 = crate::Reg<apb2otp_blk5_w1::APB2OTP_BLK5_W1_SPEC>;
#[doc = "eFuse apb2otp block5 data register1."]
pub mod apb2otp_blk5_w1;
#[doc = "APB2OTP_BLK5_W2 (r) register accessor: eFuse apb2otp block5 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w2`] module"]
pub type APB2OTP_BLK5_W2 = crate::Reg<apb2otp_blk5_w2::APB2OTP_BLK5_W2_SPEC>;
#[doc = "eFuse apb2otp block5 data register2."]
pub mod apb2otp_blk5_w2;
#[doc = "APB2OTP_BLK5_W3 (r) register accessor: eFuse apb2otp block5 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w3`] module"]
pub type APB2OTP_BLK5_W3 = crate::Reg<apb2otp_blk5_w3::APB2OTP_BLK5_W3_SPEC>;
#[doc = "eFuse apb2otp block5 data register3."]
pub mod apb2otp_blk5_w3;
#[doc = "APB2OTP_BLK5_W4 (r) register accessor: eFuse apb2otp block5 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w4`] module"]
pub type APB2OTP_BLK5_W4 = crate::Reg<apb2otp_blk5_w4::APB2OTP_BLK5_W4_SPEC>;
#[doc = "eFuse apb2otp block5 data register4."]
pub mod apb2otp_blk5_w4;
#[doc = "APB2OTP_BLK5_W5 (r) register accessor: eFuse apb2otp block5 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w5`] module"]
pub type APB2OTP_BLK5_W5 = crate::Reg<apb2otp_blk5_w5::APB2OTP_BLK5_W5_SPEC>;
#[doc = "eFuse apb2otp block5 data register5."]
pub mod apb2otp_blk5_w5;
#[doc = "APB2OTP_BLK5_W6 (r) register accessor: eFuse apb2otp block5 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w6`] module"]
pub type APB2OTP_BLK5_W6 = crate::Reg<apb2otp_blk5_w6::APB2OTP_BLK5_W6_SPEC>;
#[doc = "eFuse apb2otp block5 data register6."]
pub mod apb2otp_blk5_w6;
#[doc = "APB2OTP_BLK5_W7 (r) register accessor: eFuse apb2otp block5 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w7`] module"]
pub type APB2OTP_BLK5_W7 = crate::Reg<apb2otp_blk5_w7::APB2OTP_BLK5_W7_SPEC>;
#[doc = "eFuse apb2otp block5 data register7."]
pub mod apb2otp_blk5_w7;
#[doc = "APB2OTP_BLK5_W8 (r) register accessor: eFuse apb2otp block5 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w8`] module"]
pub type APB2OTP_BLK5_W8 = crate::Reg<apb2otp_blk5_w8::APB2OTP_BLK5_W8_SPEC>;
#[doc = "eFuse apb2otp block5 data register8."]
pub mod apb2otp_blk5_w8;
#[doc = "APB2OTP_BLK5_W9 (r) register accessor: eFuse apb2otp block5 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w9`] module"]
pub type APB2OTP_BLK5_W9 = crate::Reg<apb2otp_blk5_w9::APB2OTP_BLK5_W9_SPEC>;
#[doc = "eFuse apb2otp block5 data register9."]
pub mod apb2otp_blk5_w9;
#[doc = "APB2OTP_BLK5_W10 (r) register accessor: eFuse apb2otp block5 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w10`] module"]
pub type APB2OTP_BLK5_W10 = crate::Reg<apb2otp_blk5_w10::APB2OTP_BLK5_W10_SPEC>;
#[doc = "eFuse apb2otp block5 data register10."]
pub mod apb2otp_blk5_w10;
#[doc = "APB2OTP_BLK5_W11 (r) register accessor: eFuse apb2otp block5 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk5_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk5_w11`] module"]
pub type APB2OTP_BLK5_W11 = crate::Reg<apb2otp_blk5_w11::APB2OTP_BLK5_W11_SPEC>;
#[doc = "eFuse apb2otp block5 data register11."]
pub mod apb2otp_blk5_w11;
#[doc = "APB2OTP_BLK6_W1 (r) register accessor: eFuse apb2otp block6 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w1`] module"]
pub type APB2OTP_BLK6_W1 = crate::Reg<apb2otp_blk6_w1::APB2OTP_BLK6_W1_SPEC>;
#[doc = "eFuse apb2otp block6 data register1."]
pub mod apb2otp_blk6_w1;
#[doc = "APB2OTP_BLK6_W2 (r) register accessor: eFuse apb2otp block6 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w2`] module"]
pub type APB2OTP_BLK6_W2 = crate::Reg<apb2otp_blk6_w2::APB2OTP_BLK6_W2_SPEC>;
#[doc = "eFuse apb2otp block6 data register2."]
pub mod apb2otp_blk6_w2;
#[doc = "APB2OTP_BLK6_W3 (r) register accessor: eFuse apb2otp block6 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w3`] module"]
pub type APB2OTP_BLK6_W3 = crate::Reg<apb2otp_blk6_w3::APB2OTP_BLK6_W3_SPEC>;
#[doc = "eFuse apb2otp block6 data register3."]
pub mod apb2otp_blk6_w3;
#[doc = "APB2OTP_BLK6_W4 (r) register accessor: eFuse apb2otp block6 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w4`] module"]
pub type APB2OTP_BLK6_W4 = crate::Reg<apb2otp_blk6_w4::APB2OTP_BLK6_W4_SPEC>;
#[doc = "eFuse apb2otp block6 data register4."]
pub mod apb2otp_blk6_w4;
#[doc = "APB2OTP_BLK6_W5 (r) register accessor: eFuse apb2otp block6 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w5`] module"]
pub type APB2OTP_BLK6_W5 = crate::Reg<apb2otp_blk6_w5::APB2OTP_BLK6_W5_SPEC>;
#[doc = "eFuse apb2otp block6 data register5."]
pub mod apb2otp_blk6_w5;
#[doc = "APB2OTP_BLK6_W6 (r) register accessor: eFuse apb2otp block6 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w6`] module"]
pub type APB2OTP_BLK6_W6 = crate::Reg<apb2otp_blk6_w6::APB2OTP_BLK6_W6_SPEC>;
#[doc = "eFuse apb2otp block6 data register6."]
pub mod apb2otp_blk6_w6;
#[doc = "APB2OTP_BLK6_W7 (r) register accessor: eFuse apb2otp block6 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w7`] module"]
pub type APB2OTP_BLK6_W7 = crate::Reg<apb2otp_blk6_w7::APB2OTP_BLK6_W7_SPEC>;
#[doc = "eFuse apb2otp block6 data register7."]
pub mod apb2otp_blk6_w7;
#[doc = "APB2OTP_BLK6_W8 (r) register accessor: eFuse apb2otp block6 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w8`] module"]
pub type APB2OTP_BLK6_W8 = crate::Reg<apb2otp_blk6_w8::APB2OTP_BLK6_W8_SPEC>;
#[doc = "eFuse apb2otp block6 data register8."]
pub mod apb2otp_blk6_w8;
#[doc = "APB2OTP_BLK6_W9 (r) register accessor: eFuse apb2otp block6 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w9`] module"]
pub type APB2OTP_BLK6_W9 = crate::Reg<apb2otp_blk6_w9::APB2OTP_BLK6_W9_SPEC>;
#[doc = "eFuse apb2otp block6 data register9."]
pub mod apb2otp_blk6_w9;
#[doc = "APB2OTP_BLK6_W10 (r) register accessor: eFuse apb2otp block6 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w10`] module"]
pub type APB2OTP_BLK6_W10 = crate::Reg<apb2otp_blk6_w10::APB2OTP_BLK6_W10_SPEC>;
#[doc = "eFuse apb2otp block6 data register10."]
pub mod apb2otp_blk6_w10;
#[doc = "APB2OTP_BLK6_W11 (r) register accessor: eFuse apb2otp block6 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk6_w11`] module"]
pub type APB2OTP_BLK6_W11 = crate::Reg<apb2otp_blk6_w11::APB2OTP_BLK6_W11_SPEC>;
#[doc = "eFuse apb2otp block6 data register11."]
pub mod apb2otp_blk6_w11;
#[doc = "APB2OTP_BLK7_W1 (r) register accessor: eFuse apb2otp block7 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w1`] module"]
pub type APB2OTP_BLK7_W1 = crate::Reg<apb2otp_blk7_w1::APB2OTP_BLK7_W1_SPEC>;
#[doc = "eFuse apb2otp block7 data register1."]
pub mod apb2otp_blk7_w1;
#[doc = "APB2OTP_BLK7_W2 (r) register accessor: eFuse apb2otp block7 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w2`] module"]
pub type APB2OTP_BLK7_W2 = crate::Reg<apb2otp_blk7_w2::APB2OTP_BLK7_W2_SPEC>;
#[doc = "eFuse apb2otp block7 data register2."]
pub mod apb2otp_blk7_w2;
#[doc = "APB2OTP_BLK7_W3 (r) register accessor: eFuse apb2otp block7 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w3`] module"]
pub type APB2OTP_BLK7_W3 = crate::Reg<apb2otp_blk7_w3::APB2OTP_BLK7_W3_SPEC>;
#[doc = "eFuse apb2otp block7 data register3."]
pub mod apb2otp_blk7_w3;
#[doc = "APB2OTP_BLK7_W4 (r) register accessor: eFuse apb2otp block7 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w4`] module"]
pub type APB2OTP_BLK7_W4 = crate::Reg<apb2otp_blk7_w4::APB2OTP_BLK7_W4_SPEC>;
#[doc = "eFuse apb2otp block7 data register4."]
pub mod apb2otp_blk7_w4;
#[doc = "APB2OTP_BLK7_W5 (r) register accessor: eFuse apb2otp block7 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w5`] module"]
pub type APB2OTP_BLK7_W5 = crate::Reg<apb2otp_blk7_w5::APB2OTP_BLK7_W5_SPEC>;
#[doc = "eFuse apb2otp block7 data register5."]
pub mod apb2otp_blk7_w5;
#[doc = "APB2OTP_BLK7_W6 (r) register accessor: eFuse apb2otp block7 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w6`] module"]
pub type APB2OTP_BLK7_W6 = crate::Reg<apb2otp_blk7_w6::APB2OTP_BLK7_W6_SPEC>;
#[doc = "eFuse apb2otp block7 data register6."]
pub mod apb2otp_blk7_w6;
#[doc = "APB2OTP_BLK7_W7 (r) register accessor: eFuse apb2otp block7 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w7`] module"]
pub type APB2OTP_BLK7_W7 = crate::Reg<apb2otp_blk7_w7::APB2OTP_BLK7_W7_SPEC>;
#[doc = "eFuse apb2otp block7 data register7."]
pub mod apb2otp_blk7_w7;
#[doc = "APB2OTP_BLK7_W8 (r) register accessor: eFuse apb2otp block7 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w8`] module"]
pub type APB2OTP_BLK7_W8 = crate::Reg<apb2otp_blk7_w8::APB2OTP_BLK7_W8_SPEC>;
#[doc = "eFuse apb2otp block7 data register8."]
pub mod apb2otp_blk7_w8;
#[doc = "APB2OTP_BLK7_W9 (r) register accessor: eFuse apb2otp block7 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w9`] module"]
pub type APB2OTP_BLK7_W9 = crate::Reg<apb2otp_blk7_w9::APB2OTP_BLK7_W9_SPEC>;
#[doc = "eFuse apb2otp block7 data register9."]
pub mod apb2otp_blk7_w9;
#[doc = "APB2OTP_BLK7_W10 (r) register accessor: eFuse apb2otp block7 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w10`] module"]
pub type APB2OTP_BLK7_W10 = crate::Reg<apb2otp_blk7_w10::APB2OTP_BLK7_W10_SPEC>;
#[doc = "eFuse apb2otp block7 data register10."]
pub mod apb2otp_blk7_w10;
#[doc = "APB2OTP_BLK7_W11 (r) register accessor: eFuse apb2otp block7 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk7_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk7_w11`] module"]
pub type APB2OTP_BLK7_W11 = crate::Reg<apb2otp_blk7_w11::APB2OTP_BLK7_W11_SPEC>;
#[doc = "eFuse apb2otp block7 data register11."]
pub mod apb2otp_blk7_w11;
#[doc = "APB2OTP_BLK8_W1 (r) register accessor: eFuse apb2otp block8 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w1`] module"]
pub type APB2OTP_BLK8_W1 = crate::Reg<apb2otp_blk8_w1::APB2OTP_BLK8_W1_SPEC>;
#[doc = "eFuse apb2otp block8 data register1."]
pub mod apb2otp_blk8_w1;
#[doc = "APB2OTP_BLK8_W2 (r) register accessor: eFuse apb2otp block8 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w2`] module"]
pub type APB2OTP_BLK8_W2 = crate::Reg<apb2otp_blk8_w2::APB2OTP_BLK8_W2_SPEC>;
#[doc = "eFuse apb2otp block8 data register2."]
pub mod apb2otp_blk8_w2;
#[doc = "APB2OTP_BLK8_W3 (r) register accessor: eFuse apb2otp block8 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w3`] module"]
pub type APB2OTP_BLK8_W3 = crate::Reg<apb2otp_blk8_w3::APB2OTP_BLK8_W3_SPEC>;
#[doc = "eFuse apb2otp block8 data register3."]
pub mod apb2otp_blk8_w3;
#[doc = "APB2OTP_BLK8_W4 (r) register accessor: eFuse apb2otp block8 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w4`] module"]
pub type APB2OTP_BLK8_W4 = crate::Reg<apb2otp_blk8_w4::APB2OTP_BLK8_W4_SPEC>;
#[doc = "eFuse apb2otp block8 data register4."]
pub mod apb2otp_blk8_w4;
#[doc = "APB2OTP_BLK8_W5 (r) register accessor: eFuse apb2otp block8 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w5`] module"]
pub type APB2OTP_BLK8_W5 = crate::Reg<apb2otp_blk8_w5::APB2OTP_BLK8_W5_SPEC>;
#[doc = "eFuse apb2otp block8 data register5."]
pub mod apb2otp_blk8_w5;
#[doc = "APB2OTP_BLK8_W6 (r) register accessor: eFuse apb2otp block8 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w6`] module"]
pub type APB2OTP_BLK8_W6 = crate::Reg<apb2otp_blk8_w6::APB2OTP_BLK8_W6_SPEC>;
#[doc = "eFuse apb2otp block8 data register6."]
pub mod apb2otp_blk8_w6;
#[doc = "APB2OTP_BLK8_W7 (r) register accessor: eFuse apb2otp block8 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w7`] module"]
pub type APB2OTP_BLK8_W7 = crate::Reg<apb2otp_blk8_w7::APB2OTP_BLK8_W7_SPEC>;
#[doc = "eFuse apb2otp block8 data register7."]
pub mod apb2otp_blk8_w7;
#[doc = "APB2OTP_BLK8_W8 (r) register accessor: eFuse apb2otp block8 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w8`] module"]
pub type APB2OTP_BLK8_W8 = crate::Reg<apb2otp_blk8_w8::APB2OTP_BLK8_W8_SPEC>;
#[doc = "eFuse apb2otp block8 data register8."]
pub mod apb2otp_blk8_w8;
#[doc = "APB2OTP_BLK8_W9 (r) register accessor: eFuse apb2otp block8 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w9`] module"]
pub type APB2OTP_BLK8_W9 = crate::Reg<apb2otp_blk8_w9::APB2OTP_BLK8_W9_SPEC>;
#[doc = "eFuse apb2otp block8 data register9."]
pub mod apb2otp_blk8_w9;
#[doc = "APB2OTP_BLK8_W10 (r) register accessor: eFuse apb2otp block8 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w10`] module"]
pub type APB2OTP_BLK8_W10 = crate::Reg<apb2otp_blk8_w10::APB2OTP_BLK8_W10_SPEC>;
#[doc = "eFuse apb2otp block8 data register10."]
pub mod apb2otp_blk8_w10;
#[doc = "APB2OTP_BLK8_W11 (r) register accessor: eFuse apb2otp block8 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk8_w11`] module"]
pub type APB2OTP_BLK8_W11 = crate::Reg<apb2otp_blk8_w11::APB2OTP_BLK8_W11_SPEC>;
#[doc = "eFuse apb2otp block8 data register11."]
pub mod apb2otp_blk8_w11;
#[doc = "APB2OTP_BLK9_W1 (r) register accessor: eFuse apb2otp block9 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w1`] module"]
pub type APB2OTP_BLK9_W1 = crate::Reg<apb2otp_blk9_w1::APB2OTP_BLK9_W1_SPEC>;
#[doc = "eFuse apb2otp block9 data register1."]
pub mod apb2otp_blk9_w1;
#[doc = "APB2OTP_BLK9_W2 (r) register accessor: eFuse apb2otp block9 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w2`] module"]
pub type APB2OTP_BLK9_W2 = crate::Reg<apb2otp_blk9_w2::APB2OTP_BLK9_W2_SPEC>;
#[doc = "eFuse apb2otp block9 data register2."]
pub mod apb2otp_blk9_w2;
#[doc = "APB2OTP_BLK9_W3 (r) register accessor: eFuse apb2otp block9 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w3`] module"]
pub type APB2OTP_BLK9_W3 = crate::Reg<apb2otp_blk9_w3::APB2OTP_BLK9_W3_SPEC>;
#[doc = "eFuse apb2otp block9 data register3."]
pub mod apb2otp_blk9_w3;
#[doc = "APB2OTP_BLK9_W4 (r) register accessor: eFuse apb2otp block9 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w4`] module"]
pub type APB2OTP_BLK9_W4 = crate::Reg<apb2otp_blk9_w4::APB2OTP_BLK9_W4_SPEC>;
#[doc = "eFuse apb2otp block9 data register4."]
pub mod apb2otp_blk9_w4;
#[doc = "APB2OTP_BLK9_W5 (r) register accessor: eFuse apb2otp block9 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w5`] module"]
pub type APB2OTP_BLK9_W5 = crate::Reg<apb2otp_blk9_w5::APB2OTP_BLK9_W5_SPEC>;
#[doc = "eFuse apb2otp block9 data register5."]
pub mod apb2otp_blk9_w5;
#[doc = "APB2OTP_BLK9_W6 (r) register accessor: eFuse apb2otp block9 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w6`] module"]
pub type APB2OTP_BLK9_W6 = crate::Reg<apb2otp_blk9_w6::APB2OTP_BLK9_W6_SPEC>;
#[doc = "eFuse apb2otp block9 data register6."]
pub mod apb2otp_blk9_w6;
#[doc = "APB2OTP_BLK9_W7 (r) register accessor: eFuse apb2otp block9 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w7`] module"]
pub type APB2OTP_BLK9_W7 = crate::Reg<apb2otp_blk9_w7::APB2OTP_BLK9_W7_SPEC>;
#[doc = "eFuse apb2otp block9 data register7."]
pub mod apb2otp_blk9_w7;
#[doc = "APB2OTP_BLK9_W8 (r) register accessor: eFuse apb2otp block9 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w8`] module"]
pub type APB2OTP_BLK9_W8 = crate::Reg<apb2otp_blk9_w8::APB2OTP_BLK9_W8_SPEC>;
#[doc = "eFuse apb2otp block9 data register8."]
pub mod apb2otp_blk9_w8;
#[doc = "APB2OTP_BLK9_W9 (r) register accessor: eFuse apb2otp block9 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w9`] module"]
pub type APB2OTP_BLK9_W9 = crate::Reg<apb2otp_blk9_w9::APB2OTP_BLK9_W9_SPEC>;
#[doc = "eFuse apb2otp block9 data register9."]
pub mod apb2otp_blk9_w9;
#[doc = "APB2OTP_BLK9_W10 (r) register accessor: eFuse apb2otp block9 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w10`] module"]
pub type APB2OTP_BLK9_W10 = crate::Reg<apb2otp_blk9_w10::APB2OTP_BLK9_W10_SPEC>;
#[doc = "eFuse apb2otp block9 data register10."]
pub mod apb2otp_blk9_w10;
#[doc = "APB2OTP_BLK9_W11 (r) register accessor: eFuse apb2otp block9 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk9_w11`] module"]
pub type APB2OTP_BLK9_W11 = crate::Reg<apb2otp_blk9_w11::APB2OTP_BLK9_W11_SPEC>;
#[doc = "eFuse apb2otp block9 data register11."]
pub mod apb2otp_blk9_w11;
#[doc = "APB2OTP_BLK10_W1 (r) register accessor: eFuse apb2otp block10 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w1`] module"]
pub type APB2OTP_BLK10_W1 = crate::Reg<apb2otp_blk10_w1::APB2OTP_BLK10_W1_SPEC>;
#[doc = "eFuse apb2otp block10 data register1."]
pub mod apb2otp_blk10_w1;
#[doc = "APB2OTP_BLK10_W2 (r) register accessor: eFuse apb2otp block10 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w2`] module"]
pub type APB2OTP_BLK10_W2 = crate::Reg<apb2otp_blk10_w2::APB2OTP_BLK10_W2_SPEC>;
#[doc = "eFuse apb2otp block10 data register2."]
pub mod apb2otp_blk10_w2;
#[doc = "APB2OTP_BLK10_W3 (r) register accessor: eFuse apb2otp block10 data register3.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w3`] module"]
pub type APB2OTP_BLK10_W3 = crate::Reg<apb2otp_blk10_w3::APB2OTP_BLK10_W3_SPEC>;
#[doc = "eFuse apb2otp block10 data register3."]
pub mod apb2otp_blk10_w3;
#[doc = "APB2OTP_BLK10_W4 (r) register accessor: eFuse apb2otp block10 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w4`] module"]
pub type APB2OTP_BLK10_W4 = crate::Reg<apb2otp_blk10_w4::APB2OTP_BLK10_W4_SPEC>;
#[doc = "eFuse apb2otp block10 data register4."]
pub mod apb2otp_blk10_w4;
#[doc = "APB2OTP_BLK10_W5 (r) register accessor: eFuse apb2otp block10 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w5`] module"]
pub type APB2OTP_BLK10_W5 = crate::Reg<apb2otp_blk10_w5::APB2OTP_BLK10_W5_SPEC>;
#[doc = "eFuse apb2otp block10 data register5."]
pub mod apb2otp_blk10_w5;
#[doc = "APB2OTP_BLK10_W6 (r) register accessor: eFuse apb2otp block10 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w6`] module"]
pub type APB2OTP_BLK10_W6 = crate::Reg<apb2otp_blk10_w6::APB2OTP_BLK10_W6_SPEC>;
#[doc = "eFuse apb2otp block10 data register6."]
pub mod apb2otp_blk10_w6;
#[doc = "APB2OTP_BLK10_W7 (r) register accessor: eFuse apb2otp block10 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w7`] module"]
pub type APB2OTP_BLK10_W7 = crate::Reg<apb2otp_blk10_w7::APB2OTP_BLK10_W7_SPEC>;
#[doc = "eFuse apb2otp block10 data register7."]
pub mod apb2otp_blk10_w7;
#[doc = "APB2OTP_BLK10_W8 (r) register accessor: eFuse apb2otp block10 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w8`] module"]
pub type APB2OTP_BLK10_W8 = crate::Reg<apb2otp_blk10_w8::APB2OTP_BLK10_W8_SPEC>;
#[doc = "eFuse apb2otp block10 data register8."]
pub mod apb2otp_blk10_w8;
#[doc = "APB2OTP_BLK10_W9 (r) register accessor: eFuse apb2otp block10 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w9`] module"]
pub type APB2OTP_BLK10_W9 = crate::Reg<apb2otp_blk10_w9::APB2OTP_BLK10_W9_SPEC>;
#[doc = "eFuse apb2otp block10 data register9."]
pub mod apb2otp_blk10_w9;
#[doc = "APB2OTP_BLK10_W10 (r) register accessor: eFuse apb2otp block10 data register10.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w10`] module"]
pub type APB2OTP_BLK10_W10 = crate::Reg<apb2otp_blk10_w10::APB2OTP_BLK10_W10_SPEC>;
#[doc = "eFuse apb2otp block10 data register10."]
pub mod apb2otp_blk10_w10;
#[doc = "APB2OTP_BLK10_W11 (r) register accessor: eFuse apb2otp block10 data register11.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk10_w11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_blk10_w11`] module"]
pub type APB2OTP_BLK10_W11 = crate::Reg<apb2otp_blk10_w11::APB2OTP_BLK10_W11_SPEC>;
#[doc = "eFuse apb2otp block10 data register11."]
pub mod apb2otp_blk10_w11;
#[doc = "APB2OTP_EN (rw) register accessor: eFuse apb2otp enable configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2otp_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2otp_en`] module"]
pub type APB2OTP_EN = crate::Reg<apb2otp_en::APB2OTP_EN_SPEC>;
#[doc = "eFuse apb2otp enable configuration register."]
pub mod apb2otp_en;
