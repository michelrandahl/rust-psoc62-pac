#[doc = "Register `BOOT_CTRL_R` reader"]
pub type R = crate::R<BOOT_CTRL_R_SPEC>;
#[doc = "Register `BOOT_CTRL_R` writer"]
pub type W = crate::W<BOOT_CTRL_R_SPEC>;
#[doc = "Field `MAN_BOOT_EN` reader - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
pub type MAN_BOOT_EN_R = crate::BitReader;
#[doc = "Field `MAN_BOOT_EN` writer - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
pub type MAN_BOOT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALIDATE_BOOT` writer - Validate Mandatory Boot Enable bit This bit is used to validate the MAN_BOOT_EN bit. Values: - 0x1 (TRUE): Validate Mandatory boot enable bit - 0x0 (FALSE): Ignore Mandatory boot Enable bit"]
pub type VALIDATE_BOOT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_ACK_ENABLE` reader - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
pub type BOOT_ACK_ENABLE_R = crate::BitReader;
#[doc = "Field `BOOT_ACK_ENABLE` writer - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
pub type BOOT_ACK_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_TOUT_CNT` reader - N/A"]
pub type BOOT_TOUT_CNT_R = crate::FieldReader;
#[doc = "Field `BOOT_TOUT_CNT` writer - N/A"]
pub type BOOT_TOUT_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
    #[inline(always)]
    pub fn man_boot_en(&self) -> MAN_BOOT_EN_R {
        MAN_BOOT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
    #[inline(always)]
    pub fn boot_ack_enable(&self) -> BOOT_ACK_ENABLE_R {
        BOOT_ACK_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - N/A"]
    #[inline(always)]
    pub fn boot_tout_cnt(&self) -> BOOT_TOUT_CNT_R {
        BOOT_TOUT_CNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDHC clears this bit after the boot transfer is completed or terminated. Values: - 0x1 (MAN_BOOT_EN): Mandatory boot enable - 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
    #[inline(always)]
    #[must_use]
    pub fn man_boot_en(&mut self) -> MAN_BOOT_EN_W<BOOT_CTRL_R_SPEC> {
        MAN_BOOT_EN_W::new(self, 0)
    }
    #[doc = "Bit 7 - Validate Mandatory Boot Enable bit This bit is used to validate the MAN_BOOT_EN bit. Values: - 0x1 (TRUE): Validate Mandatory boot enable bit - 0x0 (FALSE): Ignore Mandatory boot Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn validate_boot(&mut self) -> VALIDATE_BOOT_W<BOOT_CTRL_R_SPEC> {
        VALIDATE_BOOT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Boot Acknowledge Enable When this bit set, SDHC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: - 0x1 (TRUE): Boot Ack enable - 0x0 (FALSE): Boot Ack disable"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack_enable(&mut self) -> BOOT_ACK_ENABLE_W<BOOT_CTRL_R_SPEC> {
        BOOT_ACK_ENABLE_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn boot_tout_cnt(&mut self) -> BOOT_TOUT_CNT_W<BOOT_CTRL_R_SPEC> {
        BOOT_TOUT_CNT_W::new(self, 12)
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
#[doc = "eMMC Boot Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_ctrl_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_ctrl_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT_CTRL_R_SPEC;
impl crate::RegisterSpec for BOOT_CTRL_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`boot_ctrl_r::R`](R) reader structure"]
impl crate::Readable for BOOT_CTRL_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boot_ctrl_r::W`](W) writer structure"]
impl crate::Writable for BOOT_CTRL_R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOT_CTRL_R to value 0"]
impl crate::Resettable for BOOT_CTRL_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
