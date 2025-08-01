#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `INDEX` reader - Command index."]
pub type INDEX_R = crate::FieldReader;
#[doc = "Field `INDEX` writer - Command index."]
pub type INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESPONSE_EXPECT` reader - 0: No response expected from card; 1: Response expected from card."]
pub type RESPONSE_EXPECT_R = crate::BitReader;
#[doc = "Field `RESPONSE_EXPECT` writer - 0: No response expected from card; 1: Response expected from card."]
pub type RESPONSE_EXPECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPONSE_LENGTH` reader - 0: Short response expected from card; 1: Long response expected from card."]
pub type RESPONSE_LENGTH_R = crate::BitReader;
#[doc = "Field `RESPONSE_LENGTH` writer - 0: Short response expected from card; 1: Long response expected from card."]
pub type RESPONSE_LENGTH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHECK_RESPONSE_CRC` reader - 0: Do not check; 1: Check response CRC. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
pub type CHECK_RESPONSE_CRC_R = crate::BitReader;
#[doc = "Field `CHECK_RESPONSE_CRC` writer - 0: Do not check; 1: Check response CRC. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
pub type CHECK_RESPONSE_CRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_EXPECTED` reader - 0: No data transfer expected; 1: Data transfer expected."]
pub type DATA_EXPECTED_R = crate::BitReader;
#[doc = "Field `DATA_EXPECTED` writer - 0: No data transfer expected; 1: Data transfer expected."]
pub type DATA_EXPECTED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_WRITE` reader - 0: Read from card; 1: Write to card. Don't care if no data is expected from card."]
pub type READ_WRITE_R = crate::BitReader;
#[doc = "Field `READ_WRITE` writer - 0: Read from card; 1: Write to card. Don't care if no data is expected from card."]
pub type READ_WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER_MODE` reader - Block data transfer command; 1: Stream data transfer command. Don't care if no data expected."]
pub type TRANSFER_MODE_R = crate::BitReader;
#[doc = "Field `TRANSFER_MODE` writer - Block data transfer command; 1: Stream data transfer command. Don't care if no data expected."]
pub type TRANSFER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_AUTO_STOP` reader - 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer."]
pub type SEND_AUTO_STOP_R = crate::BitReader;
#[doc = "Field `SEND_AUTO_STOP` writer - 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer."]
pub type SEND_AUTO_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT_PRVDATA_COMPLETE` reader - 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command. The SDHOST_WAIT_PRVDATA_COMPLETE] = 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command."]
pub type WAIT_PRVDATA_COMPLETE_R = crate::BitReader;
#[doc = "Field `WAIT_PRVDATA_COMPLETE` writer - 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command. The SDHOST_WAIT_PRVDATA_COMPLETE] = 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command."]
pub type WAIT_PRVDATA_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_ABORT_CMD` reader - 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state."]
pub type STOP_ABORT_CMD_R = crate::BitReader;
#[doc = "Field `STOP_ABORT_CMD` writer - 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state."]
pub type STOP_ABORT_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_INITIALIZATION` reader - 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command. After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card."]
pub type SEND_INITIALIZATION_R = crate::BitReader;
#[doc = "Field `SEND_INITIALIZATION` writer - 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command. After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card."]
pub type SEND_INITIALIZATION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_NUMBER` reader - Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported."]
pub type CARD_NUMBER_R = crate::FieldReader;
#[doc = "Field `CARD_NUMBER` writer - Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported."]
pub type CARD_NUMBER_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` reader - 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain. Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
pub type UPDATE_CLOCK_REGISTERS_ONLY_R = crate::BitReader;
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` writer - 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain. Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
pub type UPDATE_CLOCK_REGISTERS_ONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_CEATA_DEVICE` reader - Read access flag. 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device; 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device."]
pub type READ_CEATA_DEVICE_R = crate::BitReader;
#[doc = "Field `READ_CEATA_DEVICE` writer - Read access flag. 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device; 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device."]
pub type READ_CEATA_DEVICE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCS_EXPECTED` reader - Expected Command Completion Signal (CCS) configuration. 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device; 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
pub type CCS_EXPECTED_R = crate::BitReader;
#[doc = "Field `CCS_EXPECTED` writer - Expected Command Completion Signal (CCS) configuration. 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device; 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
pub type CCS_EXPECTED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_HOLE` reader - Use Hold Register. 0: CMD and DATA sent to card bypassing HOLD Register; 1: CMD and DATA sent to card through the HOLD Register."]
pub type USE_HOLE_R = crate::BitReader;
#[doc = "Field `USE_HOLE` writer - Use Hold Register. 0: CMD and DATA sent to card bypassing HOLD Register; 1: CMD and DATA sent to card through the HOLD Register."]
pub type USE_HOLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_CMD` reader - Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register."]
pub type START_CMD_R = crate::BitReader;
#[doc = "Field `START_CMD` writer - Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register."]
pub type START_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 0: No response expected from card; 1: Response expected from card."]
    #[inline(always)]
    pub fn response_expect(&self) -> RESPONSE_EXPECT_R {
        RESPONSE_EXPECT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: Short response expected from card; 1: Long response expected from card."]
    #[inline(always)]
    pub fn response_length(&self) -> RESPONSE_LENGTH_R {
        RESPONSE_LENGTH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: Do not check; 1: Check response CRC. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
    #[inline(always)]
    pub fn check_response_crc(&self) -> CHECK_RESPONSE_CRC_R {
        CHECK_RESPONSE_CRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: No data transfer expected; 1: Data transfer expected."]
    #[inline(always)]
    pub fn data_expected(&self) -> DATA_EXPECTED_R {
        DATA_EXPECTED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0: Read from card; 1: Write to card. Don't care if no data is expected from card."]
    #[inline(always)]
    pub fn read_write(&self) -> READ_WRITE_R {
        READ_WRITE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Block data transfer command; 1: Stream data transfer command. Don't care if no data expected."]
    #[inline(always)]
    pub fn transfer_mode(&self) -> TRANSFER_MODE_R {
        TRANSFER_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer."]
    #[inline(always)]
    pub fn send_auto_stop(&self) -> SEND_AUTO_STOP_R {
        SEND_AUTO_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command. The SDHOST_WAIT_PRVDATA_COMPLETE] = 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command."]
    #[inline(always)]
    pub fn wait_prvdata_complete(&self) -> WAIT_PRVDATA_COMPLETE_R {
        WAIT_PRVDATA_COMPLETE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state."]
    #[inline(always)]
    pub fn stop_abort_cmd(&self) -> STOP_ABORT_CMD_R {
        STOP_ABORT_CMD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command. After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card."]
    #[inline(always)]
    pub fn send_initialization(&self) -> SEND_INITIALIZATION_R {
        SEND_INITIALIZATION_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported."]
    #[inline(always)]
    pub fn card_number(&self) -> CARD_NUMBER_R {
        CARD_NUMBER_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain. Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
    #[inline(always)]
    pub fn update_clock_registers_only(&self) -> UPDATE_CLOCK_REGISTERS_ONLY_R {
        UPDATE_CLOCK_REGISTERS_ONLY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Read access flag. 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device; 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device."]
    #[inline(always)]
    pub fn read_ceata_device(&self) -> READ_CEATA_DEVICE_R {
        READ_CEATA_DEVICE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Expected Command Completion Signal (CCS) configuration. 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device; 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
    #[inline(always)]
    pub fn ccs_expected(&self) -> CCS_EXPECTED_R {
        CCS_EXPECTED_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - Use Hold Register. 0: CMD and DATA sent to card bypassing HOLD Register; 1: CMD and DATA sent to card through the HOLD Register."]
    #[inline(always)]
    pub fn use_hole(&self) -> USE_HOLE_R {
        USE_HOLE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register."]
    #[inline(always)]
    pub fn start_cmd(&self) -> START_CMD_R {
        START_CMD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("index", &self.index())
            .field("response_expect", &self.response_expect())
            .field("response_length", &self.response_length())
            .field("check_response_crc", &self.check_response_crc())
            .field("data_expected", &self.data_expected())
            .field("read_write", &self.read_write())
            .field("transfer_mode", &self.transfer_mode())
            .field("send_auto_stop", &self.send_auto_stop())
            .field("wait_prvdata_complete", &self.wait_prvdata_complete())
            .field("stop_abort_cmd", &self.stop_abort_cmd())
            .field("send_initialization", &self.send_initialization())
            .field("card_number", &self.card_number())
            .field(
                "update_clock_registers_only",
                &self.update_clock_registers_only(),
            )
            .field("read_ceata_device", &self.read_ceata_device())
            .field("ccs_expected", &self.ccs_expected())
            .field("use_hole", &self.use_hole())
            .field("start_cmd", &self.start_cmd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn index(&mut self) -> INDEX_W<CMD_SPEC> {
        INDEX_W::new(self, 0)
    }
    #[doc = "Bit 6 - 0: No response expected from card; 1: Response expected from card."]
    #[inline(always)]
    pub fn response_expect(&mut self) -> RESPONSE_EXPECT_W<CMD_SPEC> {
        RESPONSE_EXPECT_W::new(self, 6)
    }
    #[doc = "Bit 7 - 0: Short response expected from card; 1: Long response expected from card."]
    #[inline(always)]
    pub fn response_length(&mut self) -> RESPONSE_LENGTH_W<CMD_SPEC> {
        RESPONSE_LENGTH_W::new(self, 7)
    }
    #[doc = "Bit 8 - 0: Do not check; 1: Check response CRC. Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller."]
    #[inline(always)]
    pub fn check_response_crc(&mut self) -> CHECK_RESPONSE_CRC_W<CMD_SPEC> {
        CHECK_RESPONSE_CRC_W::new(self, 8)
    }
    #[doc = "Bit 9 - 0: No data transfer expected; 1: Data transfer expected."]
    #[inline(always)]
    pub fn data_expected(&mut self) -> DATA_EXPECTED_W<CMD_SPEC> {
        DATA_EXPECTED_W::new(self, 9)
    }
    #[doc = "Bit 10 - 0: Read from card; 1: Write to card. Don't care if no data is expected from card."]
    #[inline(always)]
    pub fn read_write(&mut self) -> READ_WRITE_W<CMD_SPEC> {
        READ_WRITE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Block data transfer command; 1: Stream data transfer command. Don't care if no data expected."]
    #[inline(always)]
    pub fn transfer_mode(&mut self) -> TRANSFER_MODE_W<CMD_SPEC> {
        TRANSFER_MODE_W::new(self, 11)
    }
    #[doc = "Bit 12 - 0: No stop command is sent at the end of data transfer; 1: Send stop command at the end of data transfer."]
    #[inline(always)]
    pub fn send_auto_stop(&mut self) -> SEND_AUTO_STOP_W<CMD_SPEC> {
        SEND_AUTO_STOP_W::new(self, 12)
    }
    #[doc = "Bit 13 - 0: Send command at once, even if previous data transfer has not completed; 1: Wait for previous data transfer to complete before sending Command. The SDHOST_WAIT_PRVDATA_COMPLETE] = 0 option is typically used to query status of card during data transfer or to stop current data transfer. SDHOST_CARD_NUMBERr should be same as in previous command."]
    #[inline(always)]
    pub fn wait_prvdata_complete(&mut self) -> WAIT_PRVDATA_COMPLETE_W<CMD_SPEC> {
        WAIT_PRVDATA_COMPLETE_W::new(self, 13)
    }
    #[doc = "Bit 14 - 0: Neither stop nor abort command can stop current data transfer. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0; 1: Stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state."]
    #[inline(always)]
    pub fn stop_abort_cmd(&mut self) -> STOP_ABORT_CMD_W<CMD_SPEC> {
        STOP_ABORT_CMD_W::new(self, 14)
    }
    #[doc = "Bit 15 - 0: Do not send initialization sequence (80 clocks of 1) before sending this command; 1: Send initialization sequence before sending this command. After powered on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card."]
    #[inline(always)]
    pub fn send_initialization(&mut self) -> SEND_INITIALIZATION_W<CMD_SPEC> {
        SEND_INITIALIZATION_W::new(self, 15)
    }
    #[doc = "Bits 16:20 - Card number in use. Represents physical slot number of card being accessed. In SD-only mode, up to two cards are supported."]
    #[inline(always)]
    pub fn card_number(&mut self) -> CARD_NUMBER_W<CMD_SPEC> {
        CARD_NUMBER_W::new(self, 16)
    }
    #[doc = "Bit 21 - 0: Normal command sequence; 1: Do not send commands, just update clock register value into card clock domain. Following register values are transferred into card clock domain: CLKDIV, CLRSRC, and CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode). This is provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when sdhost_update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, and BYTCNT. CIU uses new register values for new command sequence to card(s). When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
    #[inline(always)]
    pub fn update_clock_registers_only(&mut self) -> UPDATE_CLOCK_REGISTERS_ONLY_W<CMD_SPEC> {
        UPDATE_CLOCK_REGISTERS_ONLY_W::new(self, 21)
    }
    #[doc = "Bit 22 - Read access flag. 0: Host is not performing read access (RW_REG or RW_BLK)towards CE-ATA device; 1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device. Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. SD/MMC should not indicate read data timeout while waiting for data from CE-ATA device."]
    #[inline(always)]
    pub fn read_ceata_device(&mut self) -> READ_CEATA_DEVICE_W<CMD_SPEC> {
        READ_CEATA_DEVICE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Expected Command Completion Signal (CCS) configuration. 0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device; 1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE-ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. SD/MMC sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
    #[inline(always)]
    pub fn ccs_expected(&mut self) -> CCS_EXPECTED_W<CMD_SPEC> {
        CCS_EXPECTED_W::new(self, 23)
    }
    #[doc = "Bit 29 - Use Hold Register. 0: CMD and DATA sent to card bypassing HOLD Register; 1: CMD and DATA sent to card through the HOLD Register."]
    #[inline(always)]
    pub fn use_hole(&mut self) -> USE_HOLE_W<CMD_SPEC> {
        USE_HOLE_W::new(self, 29)
    }
    #[doc = "Bit 31 - Start command. Once command is served by the CIU, this bit is automatically cleared. When this bit is set, host should not attempt to write to any command registers. If a write is attempted, hardware lock error is set in raw interrupt register. Once command is sent and a response is received from SD_MMC_CEATA cards, Command Done bit is set in the raw interrupt Register."]
    #[inline(always)]
    pub fn start_cmd(&mut self) -> START_CMD_W<CMD_SPEC> {
        START_CMD_W::new(self, 31)
    }
}
#[doc = "Command and boot configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0x2000_0000"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0x2000_0000;
}
