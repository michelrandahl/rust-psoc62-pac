#[doc = "Register `SW_RST_R` reader"]
pub type R = crate::R<SW_RST_R_SPEC>;
#[doc = "Register `SW_RST_R` writer"]
pub type W = crate::W<SW_RST_R_SPEC>;
#[doc = "Field `SW_RST_ALL` reader - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SW_RST_ALL_R = crate::BitReader;
#[doc = "Field `SW_RST_ALL` writer - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SW_RST_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RST_CMD` reader - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SW_RST_CMD_R = crate::BitReader;
#[doc = "Field `SW_RST_CMD` writer - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SW_RST_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RST_DAT` reader - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SW_RST_DAT_R = crate::BitReader;
#[doc = "Field `SW_RST_DAT` writer - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SW_RST_DAT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_all(&self) -> SW_RST_ALL_R {
        SW_RST_ALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_cmd(&self) -> SW_RST_CMD_R {
        SW_RST_CMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_dat(&self) -> SW_RST_DAT_R {
        SW_RST_DAT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_all(&mut self) -> SW_RST_ALL_W<SW_RST_R_SPEC> {
        SW_RST_ALL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_cmd(&mut self) -> SW_RST_CMD_W<SW_RST_R_SPEC> {
        SW_RST_CMD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_dat(&mut self) -> SW_RST_DAT_W<SW_RST_R_SPEC> {
        SW_RST_DAT_W::new(self, 2)
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
#[doc = "Software Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_rst_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_rst_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_RST_R_SPEC;
impl crate::RegisterSpec for SW_RST_R_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sw_rst_r::R`](R) reader structure"]
impl crate::Readable for SW_RST_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_rst_r::W`](W) writer structure"]
impl crate::Writable for SW_RST_R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_RST_R to value 0"]
impl crate::Resettable for SW_RST_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
