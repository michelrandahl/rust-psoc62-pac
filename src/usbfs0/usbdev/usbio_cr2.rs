#[doc = "Register `USBIO_CR2` reader"]
pub type R = crate::R<USBIO_CR2_SPEC>;
#[doc = "Register `USBIO_CR2` writer"]
pub type W = crate::W<USBIO_CR2_SPEC>;
#[doc = "Field `RSVD_5_0` reader - N/A"]
pub type RSVD_5_0_R = crate::FieldReader;
#[doc = "Field `TEST_PKT` reader - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
pub type TEST_PKT_R = crate::BitReader;
#[doc = "Field `TEST_PKT` writer - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
pub type TEST_PKT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_7` reader - N/A"]
pub type RSVD_7_R = crate::BitReader;
#[doc = "Field `RSVD_7` writer - N/A"]
pub type RSVD_7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - N/A"]
    #[inline(always)]
    pub fn rsvd_5_0(&self) -> RSVD_5_0_R {
        RSVD_5_0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
    #[inline(always)]
    pub fn test_pkt(&self) -> TEST_PKT_R {
        TEST_PKT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn rsvd_7(&self) -> RSVD_7_R {
        RSVD_7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
    #[inline(always)]
    #[must_use]
    pub fn test_pkt(&mut self) -> TEST_PKT_W<USBIO_CR2_SPEC> {
        TEST_PKT_W::new(self, 6)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_7(&mut self) -> RSVD_7_W<USBIO_CR2_SPEC> {
        RSVD_7_W::new(self, 7)
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
#[doc = "USBIO control 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbio_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbio_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBIO_CR2_SPEC;
impl crate::RegisterSpec for USBIO_CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbio_cr2::R`](R) reader structure"]
impl crate::Readable for USBIO_CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbio_cr2::W`](W) writer structure"]
impl crate::Writable for USBIO_CR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBIO_CR2 to value 0"]
impl crate::Resettable for USBIO_CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
