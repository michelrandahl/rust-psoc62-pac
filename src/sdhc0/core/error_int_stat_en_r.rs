#[doc = "Register `ERROR_INT_STAT_EN_R` reader"]
pub type R = crate::R<ERROR_INT_STAT_EN_R_SPEC>;
#[doc = "Register `ERROR_INT_STAT_EN_R` writer"]
pub type W = crate::W<ERROR_INT_STAT_EN_R_SPEC>;
#[doc = "Field `CMD_TOUT_ERR_STAT_EN` reader - Command Timeout Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_TOUT_ERR_STAT_EN_R = crate::BitReader;
#[doc = "Field `CMD_TOUT_ERR_STAT_EN` writer - Command Timeout Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_TOUT_ERR_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC_ERR_STAT_EN` reader - ommand CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_CRC_ERR_STAT_EN_R = crate::BitReader;
#[doc = "Field `CMD_CRC_ERR_STAT_EN` writer - ommand CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_CRC_ERR_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_END_BIT_ERR_STAT_EN` reader - Command End Bit Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_END_BIT_ERR_STAT_EN_R = crate::BitReader;
#[doc = "Field `CMD_END_BIT_ERR_STAT_EN` writer - Command End Bit Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_END_BIT_ERR_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_IDX_ERR_STAT_EN` reader - Command Index Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_IDX_ERR_STAT_EN_R = crate::BitReader;
#[doc = "Field `CMD_IDX_ERR_STAT_EN` writer - Command Index Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CMD_IDX_ERR_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TOUT_ERR_STAT_EN` reader - Data Timeout Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DATA_TOUT_ERR_STAT_EN_R = crate::BitReader;
#[doc = "Field `DATA_TOUT_ERR_STAT_EN` writer - Data Timeout Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DATA_TOUT_ERR_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_CRC_ERR_STAT_EN` reader - Data CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DATA_CRC_ERR_STAT_EN_R = crate::BitReader;
#[doc = "Field `DATA_CRC_ERR_STAT_EN` writer - Data CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DATA_CRC_ERR_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_END_BIT_ERR_STAT_EN` reader - Data End Bit Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DATA_END_BIT_ERR_STAT_EN_R = crate::BitReader;
#[doc = "Field `DATA_END_BIT_ERR_STAT_EN` writer - Data End Bit Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DATA_END_BIT_ERR_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUR_LMT_ERR_STAT_EN` reader - Current Limit Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CUR_LMT_ERR_STAT_EN_R = crate::BitReader;
#[doc = "Field `CUR_LMT_ERR_STAT_EN` writer - Current Limit Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CUR_LMT_ERR_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD_ERR_STAT_EN` reader - Auto CMD Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type AUTO_CMD_ERR_STAT_EN_R = crate::BitReader;
#[doc = "Field `AUTO_CMD_ERR_STAT_EN` writer - Auto CMD Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type AUTO_CMD_ERR_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA_ERR_STAT_EN` reader - ADMA Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type ADMA_ERR_STAT_EN_R = crate::BitReader;
#[doc = "Field `ADMA_ERR_STAT_EN` writer - ADMA Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type ADMA_ERR_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNING_ERR_STAT_EN` reader - Tuning Error Status Enable (UHS-I Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type TUNING_ERR_STAT_EN_R = crate::BitReader;
#[doc = "Field `TUNING_ERR_STAT_EN` writer - Tuning Error Status Enable (UHS-I Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type TUNING_ERR_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_ERR_STAT_EN` reader - Response Error Status Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type RESP_ERR_STAT_EN_R = crate::BitReader;
#[doc = "Field `RESP_ERR_STAT_EN` writer - Response Error Status Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type RESP_ERR_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_ACK_ERR_STAT_EN` reader - Boot Acknowledgment Error (eMMC Mode only) Setting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BOOT_ACK_ERR_STAT_EN_R = crate::BitReader;
#[doc = "Field `BOOT_ACK_ERR_STAT_EN` writer - Boot Acknowledgment Error (eMMC Mode only) Setting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BOOT_ACK_ERR_STAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR_ERR_STAT_EN1` reader - N/A"]
pub type VENDOR_ERR_STAT_EN1_R = crate::BitReader;
#[doc = "Field `VENDOR_ERR_STAT_EN1` writer - N/A"]
pub type VENDOR_ERR_STAT_EN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR_ERR_STAT_EN2` reader - N/A"]
pub type VENDOR_ERR_STAT_EN2_R = crate::BitReader;
#[doc = "Field `VENDOR_ERR_STAT_EN2` writer - N/A"]
pub type VENDOR_ERR_STAT_EN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR_ERR_STAT_EN3` reader - N/A"]
pub type VENDOR_ERR_STAT_EN3_R = crate::BitReader;
#[doc = "Field `VENDOR_ERR_STAT_EN3` writer - N/A"]
pub type VENDOR_ERR_STAT_EN3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Timeout Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_tout_err_stat_en(&self) -> CMD_TOUT_ERR_STAT_EN_R {
        CMD_TOUT_ERR_STAT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ommand CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_crc_err_stat_en(&self) -> CMD_CRC_ERR_STAT_EN_R {
        CMD_CRC_ERR_STAT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_end_bit_err_stat_en(&self) -> CMD_END_BIT_ERR_STAT_EN_R {
        CMD_END_BIT_ERR_STAT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_idx_err_stat_en(&self) -> CMD_IDX_ERR_STAT_EN_R {
        CMD_IDX_ERR_STAT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_tout_err_stat_en(&self) -> DATA_TOUT_ERR_STAT_EN_R {
        DATA_TOUT_ERR_STAT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_crc_err_stat_en(&self) -> DATA_CRC_ERR_STAT_EN_R {
        DATA_CRC_ERR_STAT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_end_bit_err_stat_en(&self) -> DATA_END_BIT_ERR_STAT_EN_R {
        DATA_END_BIT_ERR_STAT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cur_lmt_err_stat_en(&self) -> CUR_LMT_ERR_STAT_EN_R {
        CUR_LMT_ERR_STAT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn auto_cmd_err_stat_en(&self) -> AUTO_CMD_ERR_STAT_EN_R {
        AUTO_CMD_ERR_STAT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADMA Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn adma_err_stat_en(&self) -> ADMA_ERR_STAT_EN_R {
        ADMA_ERR_STAT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tuning Error Status Enable (UHS-I Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn tuning_err_stat_en(&self) -> TUNING_ERR_STAT_EN_R {
        TUNING_ERR_STAT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Response Error Status Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn resp_err_stat_en(&self) -> RESP_ERR_STAT_EN_R {
        RESP_ERR_STAT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Boot Acknowledgment Error (eMMC Mode only) Setting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn boot_ack_err_stat_en(&self) -> BOOT_ACK_ERR_STAT_EN_R {
        BOOT_ACK_ERR_STAT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn vendor_err_stat_en1(&self) -> VENDOR_ERR_STAT_EN1_R {
        VENDOR_ERR_STAT_EN1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn vendor_err_stat_en2(&self) -> VENDOR_ERR_STAT_EN2_R {
        VENDOR_ERR_STAT_EN2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn vendor_err_stat_en3(&self) -> VENDOR_ERR_STAT_EN3_R {
        VENDOR_ERR_STAT_EN3_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_tout_err_stat_en(&mut self) -> CMD_TOUT_ERR_STAT_EN_W<ERROR_INT_STAT_EN_R_SPEC> {
        CMD_TOUT_ERR_STAT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ommand CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_err_stat_en(&mut self) -> CMD_CRC_ERR_STAT_EN_W<ERROR_INT_STAT_EN_R_SPEC> {
        CMD_CRC_ERR_STAT_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Command End Bit Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_end_bit_err_stat_en(
        &mut self,
    ) -> CMD_END_BIT_ERR_STAT_EN_W<ERROR_INT_STAT_EN_R_SPEC> {
        CMD_END_BIT_ERR_STAT_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Command Index Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_idx_err_stat_en(&mut self) -> CMD_IDX_ERR_STAT_EN_W<ERROR_INT_STAT_EN_R_SPEC> {
        CMD_IDX_ERR_STAT_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Data Timeout Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_tout_err_stat_en(&mut self) -> DATA_TOUT_ERR_STAT_EN_W<ERROR_INT_STAT_EN_R_SPEC> {
        DATA_TOUT_ERR_STAT_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Data CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_crc_err_stat_en(&mut self) -> DATA_CRC_ERR_STAT_EN_W<ERROR_INT_STAT_EN_R_SPEC> {
        DATA_CRC_ERR_STAT_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Data End Bit Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_end_bit_err_stat_en(
        &mut self,
    ) -> DATA_END_BIT_ERR_STAT_EN_W<ERROR_INT_STAT_EN_R_SPEC> {
        DATA_END_BIT_ERR_STAT_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Current Limit Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cur_lmt_err_stat_en(&mut self) -> CUR_LMT_ERR_STAT_EN_W<ERROR_INT_STAT_EN_R_SPEC> {
        CUR_LMT_ERR_STAT_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Auto CMD Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_err_stat_en(&mut self) -> AUTO_CMD_ERR_STAT_EN_W<ERROR_INT_STAT_EN_R_SPEC> {
        AUTO_CMD_ERR_STAT_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - ADMA Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn adma_err_stat_en(&mut self) -> ADMA_ERR_STAT_EN_W<ERROR_INT_STAT_EN_R_SPEC> {
        ADMA_ERR_STAT_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Tuning Error Status Enable (UHS-I Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_err_stat_en(&mut self) -> TUNING_ERR_STAT_EN_W<ERROR_INT_STAT_EN_R_SPEC> {
        TUNING_ERR_STAT_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Response Error Status Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn resp_err_stat_en(&mut self) -> RESP_ERR_STAT_EN_W<ERROR_INT_STAT_EN_R_SPEC> {
        RESP_ERR_STAT_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Boot Acknowledgment Error (eMMC Mode only) Setting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack_err_stat_en(&mut self) -> BOOT_ACK_ERR_STAT_EN_W<ERROR_INT_STAT_EN_R_SPEC> {
        BOOT_ACK_ERR_STAT_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_err_stat_en1(&mut self) -> VENDOR_ERR_STAT_EN1_W<ERROR_INT_STAT_EN_R_SPEC> {
        VENDOR_ERR_STAT_EN1_W::new(self, 13)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_err_stat_en2(&mut self) -> VENDOR_ERR_STAT_EN2_W<ERROR_INT_STAT_EN_R_SPEC> {
        VENDOR_ERR_STAT_EN2_W::new(self, 14)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_err_stat_en3(&mut self) -> VENDOR_ERR_STAT_EN3_W<ERROR_INT_STAT_EN_R_SPEC> {
        VENDOR_ERR_STAT_EN3_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Error Interrupt Status Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_int_stat_en_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error_int_stat_en_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERROR_INT_STAT_EN_R_SPEC;
impl crate::RegisterSpec for ERROR_INT_STAT_EN_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`error_int_stat_en_r::R`](R) reader structure"]
impl crate::Readable for ERROR_INT_STAT_EN_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`error_int_stat_en_r::W`](W) writer structure"]
impl crate::Writable for ERROR_INT_STAT_EN_R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERROR_INT_STAT_EN_R to value 0"]
impl crate::Resettable for ERROR_INT_STAT_EN_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
