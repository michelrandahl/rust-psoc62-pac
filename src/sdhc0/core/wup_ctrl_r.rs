#[doc = "Register `WUP_CTRL_R` reader"]
pub type R = crate::R<WUP_CTRL_R_SPEC>;
#[doc = "Register `WUP_CTRL_R` writer"]
pub type W = crate::W<WUP_CTRL_R_SPEC>;
#[doc = "Field `WUP_CARD_INT` reader - Wakeup Event Enable on SDIO Card Interrupt (through DAT\\[1\\]). This bit enables wakeup event through an SDIO Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type WUP_CARD_INT_R = crate::BitReader;
#[doc = "Field `WUP_CARD_INT` writer - Wakeup Event Enable on SDIO Card Interrupt (through DAT\\[1\\]). This bit enables wakeup event through an SDIO Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type WUP_CARD_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP_CARD_INSERT` reader - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type WUP_CARD_INSERT_R = crate::BitReader;
#[doc = "Field `WUP_CARD_INSERT` writer - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type WUP_CARD_INSERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUP_CARD_REMOVAL` reader - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type WUP_CARD_REMOVAL_R = crate::BitReader;
#[doc = "Field `WUP_CARD_REMOVAL` writer - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type WUP_CARD_REMOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup Event Enable on SDIO Card Interrupt (through DAT\\[1\\]). This bit enables wakeup event through an SDIO Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn wup_card_int(&self) -> WUP_CARD_INT_R {
        WUP_CARD_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn wup_card_insert(&self) -> WUP_CARD_INSERT_R {
        WUP_CARD_INSERT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn wup_card_removal(&self) -> WUP_CARD_REMOVAL_R {
        WUP_CARD_REMOVAL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Event Enable on SDIO Card Interrupt (through DAT\\[1\\]). This bit enables wakeup event through an SDIO Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wup_card_int(&mut self) -> WUP_CARD_INT_W<WUP_CTRL_R_SPEC> {
        WUP_CARD_INT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wup_card_insert(&mut self) -> WUP_CARD_INSERT_W<WUP_CTRL_R_SPEC> {
        WUP_CARD_INSERT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wup_card_removal(&mut self) -> WUP_CARD_REMOVAL_W<WUP_CTRL_R_SPEC> {
        WUP_CARD_REMOVAL_W::new(self, 2)
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
#[doc = "Wakeup Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wup_ctrl_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wup_ctrl_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WUP_CTRL_R_SPEC;
impl crate::RegisterSpec for WUP_CTRL_R_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wup_ctrl_r::R`](R) reader structure"]
impl crate::Readable for WUP_CTRL_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wup_ctrl_r::W`](W) writer structure"]
impl crate::Writable for WUP_CTRL_R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUP_CTRL_R to value 0"]
impl crate::Resettable for WUP_CTRL_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
