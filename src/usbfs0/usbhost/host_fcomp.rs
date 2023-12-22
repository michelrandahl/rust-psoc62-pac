#[doc = "Register `HOST_FCOMP` reader"]
pub type R = crate::R<HOST_FCOMP_SPEC>;
#[doc = "Register `HOST_FCOMP` writer"]
pub type W = crate::W<HOST_FCOMP_SPEC>;
#[doc = "Field `FRAMECOMP` reader - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type FRAMECOMP_R = crate::FieldReader;
#[doc = "Field `FRAMECOMP` writer - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type FRAMECOMP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn framecomp(&self) -> FRAMECOMP_R {
        FRAMECOMP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn framecomp(&mut self) -> FRAMECOMP_W<HOST_FCOMP_SPEC> {
        FRAMECOMP_W::new(self, 0)
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
#[doc = "Host SOF Interrupt Frame Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_fcomp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_fcomp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_FCOMP_SPEC;
impl crate::RegisterSpec for HOST_FCOMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_fcomp::R`](R) reader structure"]
impl crate::Readable for HOST_FCOMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_fcomp::W`](W) writer structure"]
impl crate::Writable for HOST_FCOMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_FCOMP to value 0"]
impl crate::Resettable for HOST_FCOMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
