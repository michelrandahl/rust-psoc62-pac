#[doc = "Register `LPM_CTL` reader"]
pub type R = crate::R<LPM_CTL_SPEC>;
#[doc = "Register `LPM_CTL` writer"]
pub type W = crate::W<LPM_CTL_SPEC>;
#[doc = "Field `LPM_EN` reader - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
pub type LPM_EN_R = crate::BitReader;
#[doc = "Field `LPM_EN` writer - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
pub type LPM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_ACK_RESP` reader - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
pub type LPM_ACK_RESP_R = crate::BitReader;
#[doc = "Field `LPM_ACK_RESP` writer - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
pub type LPM_ACK_RESP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET_EN` reader - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
pub type NYET_EN_R = crate::BitReader;
#[doc = "Field `NYET_EN` writer - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
pub type NYET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUB_RESP` reader - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
pub type SUB_RESP_R = crate::BitReader;
#[doc = "Field `SUB_RESP` writer - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
pub type SUB_RESP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
    #[inline(always)]
    pub fn lpm_en(&self) -> LPM_EN_R {
        LPM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
    #[inline(always)]
    pub fn lpm_ack_resp(&self) -> LPM_ACK_RESP_R {
        LPM_ACK_RESP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
    #[inline(always)]
    pub fn nyet_en(&self) -> NYET_EN_R {
        NYET_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
    #[inline(always)]
    pub fn sub_resp(&self) -> SUB_RESP_R {
        SUB_RESP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_en(&mut self) -> LPM_EN_W<LPM_CTL_SPEC> {
        LPM_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_ack_resp(&mut self) -> LPM_ACK_RESP_W<LPM_CTL_SPEC> {
        LPM_ACK_RESP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
    #[inline(always)]
    #[must_use]
    pub fn nyet_en(&mut self) -> NYET_EN_W<LPM_CTL_SPEC> {
        NYET_EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
    #[inline(always)]
    #[must_use]
    pub fn sub_resp(&mut self) -> SUB_RESP_W<LPM_CTL_SPEC> {
        SUB_RESP_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LPM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpm_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpm_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPM_CTL_SPEC;
impl crate::RegisterSpec for LPM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpm_ctl::R`](R) reader structure"]
impl crate::Readable for LPM_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpm_ctl::W`](W) writer structure"]
impl crate::Writable for LPM_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPM_CTL to value 0"]
impl crate::Resettable for LPM_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
