#[doc = "Register `BLOCKCOUNT_R` reader"]
pub type R = crate::R<BLOCKCOUNT_R_SPEC>;
#[doc = "Register `BLOCKCOUNT_R` writer"]
pub type W = crate::W<BLOCKCOUNT_R_SPEC>;
#[doc = "Field `BLOCK_CNT` reader - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
pub type BLOCK_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_CNT` writer - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
pub type BLOCK_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
    #[inline(always)]
    pub fn block_cnt(&self) -> BLOCK_CNT_R {
        BLOCK_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
    #[inline(always)]
    #[must_use]
    pub fn block_cnt(&mut self) -> BLOCK_CNT_W<BLOCKCOUNT_R_SPEC> {
        BLOCK_CNT_W::new(self, 0)
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
#[doc = "16-bit Block Count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blockcount_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blockcount_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLOCKCOUNT_R_SPEC;
impl crate::RegisterSpec for BLOCKCOUNT_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`blockcount_r::R`](R) reader structure"]
impl crate::Readable for BLOCKCOUNT_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blockcount_r::W`](W) writer structure"]
impl crate::Writable for BLOCKCOUNT_R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLOCKCOUNT_R to value 0"]
impl crate::Resettable for BLOCKCOUNT_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
