#[doc = "Register `HOST_CTL0` reader"]
pub type R = crate::R<HOST_CTL0_SPEC>;
#[doc = "Register `HOST_CTL0` writer"]
pub type W = crate::W<HOST_CTL0_SPEC>;
#[doc = "Field `HOST` reader - This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The operation mode does not transition to the required one immediately after it was changed using this bit. Read this bit to check that the operation mode has changed. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'.. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
pub type HOST_R = crate::BitReader;
#[doc = "Field `HOST` writer - This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The operation mode does not transition to the required one immediately after it was changed using this bit. Read this bit to check that the operation mode has changed. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'.. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
pub type HOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn' affect the USB Device."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn' affect the USB Device."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The operation mode does not transition to the required one immediately after it was changed using this bit. Read this bit to check that the operation mode has changed. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'.. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
    #[inline(always)]
    pub fn host(&self) -> HOST_R {
        HOST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn' affect the USB Device."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The operation mode does not transition to the required one immediately after it was changed using this bit. Read this bit to check that the operation mode has changed. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'.. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn host(&mut self) -> HOST_W<HOST_CTL0_SPEC> {
        HOST_W::new(self, 0)
    }
    #[doc = "Bit 31 - This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn' affect the USB Device."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<HOST_CTL0_SPEC> {
        ENABLE_W::new(self, 31)
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
#[doc = "Host Control 0 Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_CTL0_SPEC;
impl crate::RegisterSpec for HOST_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ctl0::R`](R) reader structure"]
impl crate::Readable for HOST_CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_ctl0::W`](W) writer structure"]
impl crate::Writable for HOST_CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_CTL0 to value 0"]
impl crate::Resettable for HOST_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
