#[doc = "Register `HOST_EOF` reader"]
pub type R = crate::R<HOST_EOF_SPEC>;
#[doc = "Register `HOST_EOF` writer"]
pub type W = crate::W<HOST_EOF_SPEC>;
#[doc = "Field `EOF` reader - These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type EOF_R = crate::FieldReader<u16>;
#[doc = "Field `EOF` writer - These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type EOF_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn eof(&self) -> EOF_R {
        EOF_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn eof(&mut self) -> EOF_W<HOST_EOF_SPEC> {
        EOF_W::new(self, 0)
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
#[doc = "Host EOF Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_eof::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_eof::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_EOF_SPEC;
impl crate::RegisterSpec for HOST_EOF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_eof::R`](R) reader structure"]
impl crate::Readable for HOST_EOF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_eof::W`](W) writer structure"]
impl crate::Writable for HOST_EOF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_EOF to value 0"]
impl crate::Resettable for HOST_EOF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
