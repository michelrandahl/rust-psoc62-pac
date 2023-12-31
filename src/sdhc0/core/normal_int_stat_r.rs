#[doc = "Register `NORMAL_INT_STAT_R` reader"]
pub type R = crate::R<NORMAL_INT_STAT_R_SPEC>;
#[doc = "Register `NORMAL_INT_STAT_R` writer"]
pub type W = crate::W<NORMAL_INT_STAT_R_SPEC>;
#[doc = "Field `CMD_COMPLETE` reader - Command Complete In an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. Values: - 0x0 (FALSE): No command complete - 0x1 (TRUE): Command Complete"]
pub type CMD_COMPLETE_R = crate::BitReader;
#[doc = "Field `CMD_COMPLETE` writer - Command Complete In an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. Values: - 0x0 (FALSE): No command complete - 0x1 (TRUE): Command Complete"]
pub type CMD_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFER_COMPLETE` reader - Transfer Complete This bit is set when a read/write transfer and a command with status busy is completed. Values: - 0x0 (FALSE): Not complete - 0x1 (TRUE): Command execution is completed"]
pub type XFER_COMPLETE_R = crate::BitReader;
#[doc = "Field `XFER_COMPLETE` writer - Transfer Complete This bit is set when a read/write transfer and a command with status busy is completed. Values: - 0x0 (FALSE): Not complete - 0x1 (TRUE): Command execution is completed"]
pub type XFER_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGAP_EVENT` reader - Block Gap Event This bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request. Values: - 0x0 (FALSE): No Block Gap Event - 0x1 (TRUE): Transaction stopped at block gap"]
pub type BGAP_EVENT_R = crate::BitReader;
#[doc = "Field `BGAP_EVENT` writer - Block Gap Event This bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request. Values: - 0x0 (FALSE): No Block Gap Event - 0x1 (TRUE): Transaction stopped at block gap"]
pub type BGAP_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_INTERRUPT` reader - DMA Interrupt This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete. Values: - 0x0 (FALSE): No DMA Interrupt - 0x1 (TRUE): DMA Interrupt is generated"]
pub type DMA_INTERRUPT_R = crate::BitReader;
#[doc = "Field `DMA_INTERRUPT` writer - DMA Interrupt This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete. Values: - 0x0 (FALSE): No DMA Interrupt - 0x1 (TRUE): DMA Interrupt is generated"]
pub type DMA_INTERRUPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_WR_READY` reader - Buffer Write Ready This bit is set if the Buffer Write Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to write buffer - 0x1 (TRUE): Ready to write buffer"]
pub type BUF_WR_READY_R = crate::BitReader;
#[doc = "Field `BUF_WR_READY` writer - Buffer Write Ready This bit is set if the Buffer Write Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to write buffer - 0x1 (TRUE): Ready to write buffer"]
pub type BUF_WR_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_RD_READY` reader - Buffer Read Ready This bit is set if the Buffer Read Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to read buffer - 0x1 (TRUE): Ready to read buffer"]
pub type BUF_RD_READY_R = crate::BitReader;
#[doc = "Field `BUF_RD_READY` writer - Buffer Read Ready This bit is set if the Buffer Read Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to read buffer - 0x1 (TRUE): Ready to read buffer"]
pub type BUF_RD_READY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INSERTION` reader - Card Insertion This bit is set if the Card Inserted in the Present State register changes from 0 to 1. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Inserted"]
pub type CARD_INSERTION_R = crate::BitReader;
#[doc = "Field `CARD_INSERTION` writer - Card Insertion This bit is set if the Card Inserted in the Present State register changes from 0 to 1. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Inserted"]
pub type CARD_INSERTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_REMOVAL` reader - Card Removal This bit is set if the Card Inserted in the Present State register changes from 1 to 0. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Removed"]
pub type CARD_REMOVAL_R = crate::BitReader;
#[doc = "Field `CARD_REMOVAL` writer - Card Removal This bit is set if the Card Inserted in the Present State register changes from 1 to 0. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Removed"]
pub type CARD_REMOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INTERRUPT` reader - Card Interrupt This bit reflects the synchronized value of: - DAT\\[1\\]
Interrupt Input for SD Mode Values: - 0x0 (FALSE): No Card Interrupt - 0x1 (TRUE): Generate Card Interrupt"]
pub type CARD_INTERRUPT_R = crate::BitReader;
#[doc = "Field `FX_EVENT` reader - FX Event This status is set when R\\[14\\]
of response register is set to 1 and Response Type R1/R5 is set to 0 in Transfer Mode register. This interrupt is used with response check function. Values: - 0x0 (FALSE): No Event - 0x1 (TRUE): FX Event is detected"]
pub type FX_EVENT_R = crate::BitReader;
#[doc = "Field `CQE_EVENT` reader - Command Queuing Event This status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI's CQIS/CRNQIS register for more details. In UHS-II Mode, this bit is irrelevant. Values: - 0x0 (FALSE): No Event - 0x1 (TRUE): Command Queuing Event is detected"]
pub type CQE_EVENT_R = crate::BitReader;
#[doc = "Field `CQE_EVENT` writer - Command Queuing Event This status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI's CQIS/CRNQIS register for more details. In UHS-II Mode, this bit is irrelevant. Values: - 0x0 (FALSE): No Event - 0x1 (TRUE): Command Queuing Event is detected"]
pub type CQE_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_INTERRUPT` reader - Error Interrupt If any of the bits in the Error Interrupt Status register are set, then this bit is set. Values: - 0x0 (FALSE): No Error - 0x1 (TRUE): Error"]
pub type ERR_INTERRUPT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command Complete In an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. Values: - 0x0 (FALSE): No command complete - 0x1 (TRUE): Command Complete"]
    #[inline(always)]
    pub fn cmd_complete(&self) -> CMD_COMPLETE_R {
        CMD_COMPLETE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete This bit is set when a read/write transfer and a command with status busy is completed. Values: - 0x0 (FALSE): Not complete - 0x1 (TRUE): Command execution is completed"]
    #[inline(always)]
    pub fn xfer_complete(&self) -> XFER_COMPLETE_R {
        XFER_COMPLETE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event This bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request. Values: - 0x0 (FALSE): No Block Gap Event - 0x1 (TRUE): Transaction stopped at block gap"]
    #[inline(always)]
    pub fn bgap_event(&self) -> BGAP_EVENT_R {
        BGAP_EVENT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete. Values: - 0x0 (FALSE): No DMA Interrupt - 0x1 (TRUE): DMA Interrupt is generated"]
    #[inline(always)]
    pub fn dma_interrupt(&self) -> DMA_INTERRUPT_R {
        DMA_INTERRUPT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready This bit is set if the Buffer Write Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to write buffer - 0x1 (TRUE): Ready to write buffer"]
    #[inline(always)]
    pub fn buf_wr_ready(&self) -> BUF_WR_READY_R {
        BUF_WR_READY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready This bit is set if the Buffer Read Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to read buffer - 0x1 (TRUE): Ready to read buffer"]
    #[inline(always)]
    pub fn buf_rd_ready(&self) -> BUF_RD_READY_R {
        BUF_RD_READY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion This bit is set if the Card Inserted in the Present State register changes from 0 to 1. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Inserted"]
    #[inline(always)]
    pub fn card_insertion(&self) -> CARD_INSERTION_R {
        CARD_INSERTION_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal This bit is set if the Card Inserted in the Present State register changes from 1 to 0. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Removed"]
    #[inline(always)]
    pub fn card_removal(&self) -> CARD_REMOVAL_R {
        CARD_REMOVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt This bit reflects the synchronized value of: - DAT\\[1\\]
Interrupt Input for SD Mode Values: - 0x0 (FALSE): No Card Interrupt - 0x1 (TRUE): Generate Card Interrupt"]
    #[inline(always)]
    pub fn card_interrupt(&self) -> CARD_INTERRUPT_R {
        CARD_INTERRUPT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - FX Event This status is set when R\\[14\\]
of response register is set to 1 and Response Type R1/R5 is set to 0 in Transfer Mode register. This interrupt is used with response check function. Values: - 0x0 (FALSE): No Event - 0x1 (TRUE): FX Event is detected"]
    #[inline(always)]
    pub fn fx_event(&self) -> FX_EVENT_R {
        FX_EVENT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Command Queuing Event This status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI's CQIS/CRNQIS register for more details. In UHS-II Mode, this bit is irrelevant. Values: - 0x0 (FALSE): No Event - 0x1 (TRUE): Command Queuing Event is detected"]
    #[inline(always)]
    pub fn cqe_event(&self) -> CQE_EVENT_R {
        CQE_EVENT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt If any of the bits in the Error Interrupt Status register are set, then this bit is set. Values: - 0x0 (FALSE): No Error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn err_interrupt(&self) -> ERR_INTERRUPT_R {
        ERR_INTERRUPT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete In an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. Values: - 0x0 (FALSE): No command complete - 0x1 (TRUE): Command Complete"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete(&mut self) -> CMD_COMPLETE_W<NORMAL_INT_STAT_R_SPEC> {
        CMD_COMPLETE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete This bit is set when a read/write transfer and a command with status busy is completed. Values: - 0x0 (FALSE): Not complete - 0x1 (TRUE): Command execution is completed"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_complete(&mut self) -> XFER_COMPLETE_W<NORMAL_INT_STAT_R_SPEC> {
        XFER_COMPLETE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event This bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request. Values: - 0x0 (FALSE): No Block Gap Event - 0x1 (TRUE): Transaction stopped at block gap"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_event(&mut self) -> BGAP_EVENT_W<NORMAL_INT_STAT_R_SPEC> {
        BGAP_EVENT_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete. Values: - 0x0 (FALSE): No DMA Interrupt - 0x1 (TRUE): DMA Interrupt is generated"]
    #[inline(always)]
    #[must_use]
    pub fn dma_interrupt(&mut self) -> DMA_INTERRUPT_W<NORMAL_INT_STAT_R_SPEC> {
        DMA_INTERRUPT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready This bit is set if the Buffer Write Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to write buffer - 0x1 (TRUE): Ready to write buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf_wr_ready(&mut self) -> BUF_WR_READY_W<NORMAL_INT_STAT_R_SPEC> {
        BUF_WR_READY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready This bit is set if the Buffer Read Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to read buffer - 0x1 (TRUE): Ready to read buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf_rd_ready(&mut self) -> BUF_RD_READY_W<NORMAL_INT_STAT_R_SPEC> {
        BUF_RD_READY_W::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion This bit is set if the Card Inserted in the Present State register changes from 0 to 1. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Inserted"]
    #[inline(always)]
    #[must_use]
    pub fn card_insertion(&mut self) -> CARD_INSERTION_W<NORMAL_INT_STAT_R_SPEC> {
        CARD_INSERTION_W::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal This bit is set if the Card Inserted in the Present State register changes from 1 to 0. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Removed"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal(&mut self) -> CARD_REMOVAL_W<NORMAL_INT_STAT_R_SPEC> {
        CARD_REMOVAL_W::new(self, 7)
    }
    #[doc = "Bit 14 - Command Queuing Event This status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI's CQIS/CRNQIS register for more details. In UHS-II Mode, this bit is irrelevant. Values: - 0x0 (FALSE): No Event - 0x1 (TRUE): Command Queuing Event is detected"]
    #[inline(always)]
    #[must_use]
    pub fn cqe_event(&mut self) -> CQE_EVENT_W<NORMAL_INT_STAT_R_SPEC> {
        CQE_EVENT_W::new(self, 14)
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
#[doc = "Normal Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`normal_int_stat_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`normal_int_stat_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NORMAL_INT_STAT_R_SPEC;
impl crate::RegisterSpec for NORMAL_INT_STAT_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`normal_int_stat_r::R`](R) reader structure"]
impl crate::Readable for NORMAL_INT_STAT_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`normal_int_stat_r::W`](W) writer structure"]
impl crate::Writable for NORMAL_INT_STAT_R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NORMAL_INT_STAT_R to value 0"]
impl crate::Resettable for NORMAL_INT_STAT_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
