#[doc = "Register `HOST_CTRL1_R` reader"]
pub type R = crate::R<HOST_CTRL1_R_SPEC>;
#[doc = "Register `HOST_CTRL1_R` writer"]
pub type W = crate::W<HOST_CTRL1_R_SPEC>;
#[doc = "Field `LED_CTRL` reader - LED Control This bit is used to caution the user not to remove the card while the SD card is being accessed. The value is reflected on the led_ctrl ouput. Values: - 0x0 (OFF): LED off - 0x1 (ON): LED on"]
pub type LED_CTRL_R = crate::BitReader;
#[doc = "Field `LED_CTRL` writer - LED Control This bit is used to caution the user not to remove the card while the SD card is being accessed. The value is reflected on the led_ctrl ouput. Values: - 0x0 (OFF): LED off - 0x1 (ON): LED on"]
pub type LED_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAT_XFER_WIDTH` reader - Data Transfer Width For SD/eMMC mode,this bit selects the data transfer width of the Host Controller. The Host Driver sets it to match the data width of the SD/eMMC card. Values: - 0x1 (FOUR_BIT): 4-bit mode - 0x0 (ONE_BIT): 1-bit mode"]
pub type DAT_XFER_WIDTH_R = crate::BitReader;
#[doc = "Field `DAT_XFER_WIDTH` writer - Data Transfer Width For SD/eMMC mode,this bit selects the data transfer width of the Host Controller. The Host Driver sets it to match the data width of the SD/eMMC card. Values: - 0x1 (FOUR_BIT): 4-bit mode - 0x0 (ONE_BIT): 1-bit mode"]
pub type DAT_XFER_WIDTH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGH_SPEED_EN` reader - High Speed Enable (SD/eMMC Mode only) Before setting this bit, the Host Driver checks the High Speed Support in the Capabilities register. Note: SDHC always outputs the sd_cmd_out and sd_dat_out lines at the rising edge of card clock irrespective of this bit. Values: - 0x1 (HIGH_SPEED): High Speed mode - 0x0 (NORMAL_SPEED): Normal Speed mode"]
pub type HIGH_SPEED_EN_R = crate::BitReader;
#[doc = "Field `HIGH_SPEED_EN` writer - High Speed Enable (SD/eMMC Mode only) Before setting this bit, the Host Driver checks the High Speed Support in the Capabilities register. Note: SDHC always outputs the sd_cmd_out and sd_dat_out lines at the rising edge of card clock irrespective of this bit. Values: - 0x1 (HIGH_SPEED): High Speed mode - 0x0 (NORMAL_SPEED): Normal Speed mode"]
pub type HIGH_SPEED_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_SEL` reader - N/A"]
pub type DMA_SEL_R = crate::FieldReader;
#[doc = "Field `DMA_SEL` writer - N/A"]
pub type DMA_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXT_DAT_XFER` reader - Extended Data Transfer Width This bit controls 8-bit bus width mode of embedded device. Values: - 0x1 (EIGHT_BIT): 8-bit Bus Width - 0x0 (DEFAULT): Bus Width is selected by the Data Transfer Width"]
pub type EXT_DAT_XFER_R = crate::BitReader;
#[doc = "Field `EXT_DAT_XFER` writer - Extended Data Transfer Width This bit controls 8-bit bus width mode of embedded device. Values: - 0x1 (EIGHT_BIT): 8-bit Bus Width - 0x0 (DEFAULT): Bus Width is selected by the Data Transfer Width"]
pub type EXT_DAT_XFER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_DETECT_TEST_LVL` reader - Card Detect Test Level This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates whether a card inserted or not. Values: - 0x1 (CARD_INSERTED): Card Inserted - 0x0 (No_CARD): No Card"]
pub type CARD_DETECT_TEST_LVL_R = crate::BitReader;
#[doc = "Field `CARD_DETECT_TEST_LVL` writer - Card Detect Test Level This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates whether a card inserted or not. Values: - 0x1 (CARD_INSERTED): Card Inserted - 0x0 (No_CARD): No Card"]
pub type CARD_DETECT_TEST_LVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_DETECT_SIG_SEL` reader - Card Detect Signal Selection This bit selects a source for card detection. When the source for the card detection is switched, the interrupt must be disabled during the switching period. Values: - 0x1 (CARD_DT_TEST_LEVEL): Card Detect Test Level is selected (for test purpose) - 0x0 (card_detect_n): card_detect_n signal is selected (for normal use)"]
pub type CARD_DETECT_SIG_SEL_R = crate::BitReader;
#[doc = "Field `CARD_DETECT_SIG_SEL` writer - Card Detect Signal Selection This bit selects a source for card detection. When the source for the card detection is switched, the interrupt must be disabled during the switching period. Values: - 0x1 (CARD_DT_TEST_LEVEL): Card Detect Test Level is selected (for test purpose) - 0x0 (card_detect_n): card_detect_n signal is selected (for normal use)"]
pub type CARD_DETECT_SIG_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LED Control This bit is used to caution the user not to remove the card while the SD card is being accessed. The value is reflected on the led_ctrl ouput. Values: - 0x0 (OFF): LED off - 0x1 (ON): LED on"]
    #[inline(always)]
    pub fn led_ctrl(&self) -> LED_CTRL_R {
        LED_CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Transfer Width For SD/eMMC mode,this bit selects the data transfer width of the Host Controller. The Host Driver sets it to match the data width of the SD/eMMC card. Values: - 0x1 (FOUR_BIT): 4-bit mode - 0x0 (ONE_BIT): 1-bit mode"]
    #[inline(always)]
    pub fn dat_xfer_width(&self) -> DAT_XFER_WIDTH_R {
        DAT_XFER_WIDTH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable (SD/eMMC Mode only) Before setting this bit, the Host Driver checks the High Speed Support in the Capabilities register. Note: SDHC always outputs the sd_cmd_out and sd_dat_out lines at the rising edge of card clock irrespective of this bit. Values: - 0x1 (HIGH_SPEED): High Speed mode - 0x0 (NORMAL_SPEED): Normal Speed mode"]
    #[inline(always)]
    pub fn high_speed_en(&self) -> HIGH_SPEED_EN_R {
        HIGH_SPEED_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - N/A"]
    #[inline(always)]
    pub fn dma_sel(&self) -> DMA_SEL_R {
        DMA_SEL_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width This bit controls 8-bit bus width mode of embedded device. Values: - 0x1 (EIGHT_BIT): 8-bit Bus Width - 0x0 (DEFAULT): Bus Width is selected by the Data Transfer Width"]
    #[inline(always)]
    pub fn ext_dat_xfer(&self) -> EXT_DAT_XFER_R {
        EXT_DAT_XFER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Detect Test Level This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates whether a card inserted or not. Values: - 0x1 (CARD_INSERTED): Card Inserted - 0x0 (No_CARD): No Card"]
    #[inline(always)]
    pub fn card_detect_test_lvl(&self) -> CARD_DETECT_TEST_LVL_R {
        CARD_DETECT_TEST_LVL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection This bit selects a source for card detection. When the source for the card detection is switched, the interrupt must be disabled during the switching period. Values: - 0x1 (CARD_DT_TEST_LEVEL): Card Detect Test Level is selected (for test purpose) - 0x0 (card_detect_n): card_detect_n signal is selected (for normal use)"]
    #[inline(always)]
    pub fn card_detect_sig_sel(&self) -> CARD_DETECT_SIG_SEL_R {
        CARD_DETECT_SIG_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control This bit is used to caution the user not to remove the card while the SD card is being accessed. The value is reflected on the led_ctrl ouput. Values: - 0x0 (OFF): LED off - 0x1 (ON): LED on"]
    #[inline(always)]
    #[must_use]
    pub fn led_ctrl(&mut self) -> LED_CTRL_W<HOST_CTRL1_R_SPEC> {
        LED_CTRL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data Transfer Width For SD/eMMC mode,this bit selects the data transfer width of the Host Controller. The Host Driver sets it to match the data width of the SD/eMMC card. Values: - 0x1 (FOUR_BIT): 4-bit mode - 0x0 (ONE_BIT): 1-bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn dat_xfer_width(&mut self) -> DAT_XFER_WIDTH_W<HOST_CTRL1_R_SPEC> {
        DAT_XFER_WIDTH_W::new(self, 1)
    }
    #[doc = "Bit 2 - High Speed Enable (SD/eMMC Mode only) Before setting this bit, the Host Driver checks the High Speed Support in the Capabilities register. Note: SDHC always outputs the sd_cmd_out and sd_dat_out lines at the rising edge of card clock irrespective of this bit. Values: - 0x1 (HIGH_SPEED): High Speed mode - 0x0 (NORMAL_SPEED): Normal Speed mode"]
    #[inline(always)]
    #[must_use]
    pub fn high_speed_en(&mut self) -> HIGH_SPEED_EN_W<HOST_CTRL1_R_SPEC> {
        HIGH_SPEED_EN_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn dma_sel(&mut self) -> DMA_SEL_W<HOST_CTRL1_R_SPEC> {
        DMA_SEL_W::new(self, 3)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width This bit controls 8-bit bus width mode of embedded device. Values: - 0x1 (EIGHT_BIT): 8-bit Bus Width - 0x0 (DEFAULT): Bus Width is selected by the Data Transfer Width"]
    #[inline(always)]
    #[must_use]
    pub fn ext_dat_xfer(&mut self) -> EXT_DAT_XFER_W<HOST_CTRL1_R_SPEC> {
        EXT_DAT_XFER_W::new(self, 5)
    }
    #[doc = "Bit 6 - Card Detect Test Level This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates whether a card inserted or not. Values: - 0x1 (CARD_INSERTED): Card Inserted - 0x0 (No_CARD): No Card"]
    #[inline(always)]
    #[must_use]
    pub fn card_detect_test_lvl(&mut self) -> CARD_DETECT_TEST_LVL_W<HOST_CTRL1_R_SPEC> {
        CARD_DETECT_TEST_LVL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection This bit selects a source for card detection. When the source for the card detection is switched, the interrupt must be disabled during the switching period. Values: - 0x1 (CARD_DT_TEST_LEVEL): Card Detect Test Level is selected (for test purpose) - 0x0 (card_detect_n): card_detect_n signal is selected (for normal use)"]
    #[inline(always)]
    #[must_use]
    pub fn card_detect_sig_sel(&mut self) -> CARD_DETECT_SIG_SEL_W<HOST_CTRL1_R_SPEC> {
        CARD_DETECT_SIG_SEL_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctrl1_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctrl1_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_CTRL1_R_SPEC;
impl crate::RegisterSpec for HOST_CTRL1_R_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`host_ctrl1_r::R`](R) reader structure"]
impl crate::Readable for HOST_CTRL1_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_ctrl1_r::W`](W) writer structure"]
impl crate::Writable for HOST_CTRL1_R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_CTRL1_R to value 0"]
impl crate::Resettable for HOST_CTRL1_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
