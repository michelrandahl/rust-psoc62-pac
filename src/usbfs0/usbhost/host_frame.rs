#[doc = "Register `HOST_FRAME` reader"]
pub type R = crate::R<HOST_FRAME_SPEC>;
#[doc = "Register `HOST_FRAME` writer"]
pub type W = crate::W<HOST_FRAME_SPEC>;
#[doc = "Field `FRAME` reader - These bits are used to specify a frame number of SOF. Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
pub type FRAME_R = crate::FieldReader<u16>;
#[doc = "Field `FRAME` writer - These bits are used to specify a frame number of SOF. Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
pub type FRAME_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - These bits are used to specify a frame number of SOF. Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - These bits are used to specify a frame number of SOF. Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FRAME_W<HOST_FRAME_SPEC> {
        FRAME_W::new(self, 0)
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
#[doc = "Host Frame Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_frame::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_frame::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_FRAME_SPEC;
impl crate::RegisterSpec for HOST_FRAME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_frame::R`](R) reader structure"]
impl crate::Readable for HOST_FRAME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_frame::W`](W) writer structure"]
impl crate::Writable for HOST_FRAME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_FRAME to value 0"]
impl crate::Resettable for HOST_FRAME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
