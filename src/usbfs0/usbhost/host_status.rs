#[doc = "Register `HOST_STATUS` reader"]
pub type R = crate::R<HOST_STATUS_SPEC>;
#[doc = "Register `HOST_STATUS` writer"]
pub type W = crate::W<HOST_STATUS_SPEC>;
#[doc = "Field `CSTAT` reader - When this bit is '1', it means that the device is connected. When this bit is '0', it means that the device is disconnected. '0' : Device is disconnected. '1' : Device is connected. Notes: - This bit is initialized if the RST bit of the Host Control 1 Register (Host_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1)."]
pub type CSTAT_R = crate::BitReader;
#[doc = "Field `TMODE` reader - If this bit is '1', it means that the device is connected in the full-speed mode. When this bit is '0', it means that the device is connected in the low-speed mode. This bit is valid when the CSTAT bit of the Host Status Register (HOST_STATUS) is '1'. '0' : Low-speed. '1' : Full-speed. Notes: - This bit is initialized if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1)."]
pub type TMODE_R = crate::BitReader;
#[doc = "Field `SUSP` reader - If this bit is set to '1', the USB Host is placed into the suspend state. If this bit is set to '0' while it is '1' or the USB bus is placed into the k-state mode, the suspend state is released, and the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. Set to '1' : Suspend. Set '0' while this bit is '1' : Resume. Others : Holds the status. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - If this bit is set to '1', this bit must not be set to '1' until the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. - Do not set this bit to '1' while the USB is active (during USB bus resetting, data transfer, or SOF timer running). - If the value of this bit is changed, it is not immediately reflected on the state of the USB bus. To check whether or not the state is updated, read this bit."]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `SUSP` writer - If this bit is set to '1', the USB Host is placed into the suspend state. If this bit is set to '0' while it is '1' or the USB bus is placed into the k-state mode, the suspend state is released, and the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. Set to '1' : Suspend. Set '0' while this bit is '1' : Resume. Others : Holds the status. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - If this bit is set to '1', this bit must not be set to '1' until the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. - Do not set this bit to '1' while the USB is active (during USB bus resetting, data transfer, or SOF timer running). - If the value of this bit is changed, it is not immediately reflected on the state of the USB bus. To check whether or not the state is updated, read this bit."]
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFBUSY` reader - When a SOF token is sent using the Host Token Endpoint Register (HOST_TOKEN), this bit is set to '1', which means that the SOF timer is active. When this bit is '0', it means that the SOF timer is under suspension. To stop the active SOF timer, write '0' to this bit. However, if this bit is written with '1', its value is ignored. '0' : The SOF timer is stopped. '1' : The SOF timer is active. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - The SOF timer does not stop immediately after this bit has been set to '0' to stop the SOF timer. To check whether or not the SOF timer is stopped, read this bit."]
pub type SOFBUSY_R = crate::BitReader;
#[doc = "Field `SOFBUSY` writer - When a SOF token is sent using the Host Token Endpoint Register (HOST_TOKEN), this bit is set to '1', which means that the SOF timer is active. When this bit is '0', it means that the SOF timer is under suspension. To stop the active SOF timer, write '0' to this bit. However, if this bit is written with '1', its value is ignored. '0' : The SOF timer is stopped. '1' : The SOF timer is active. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - The SOF timer does not stop immediately after this bit has been set to '0' to stop the SOF timer. To check whether or not the SOF timer is stopped, read this bit."]
pub type SOFBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URST` reader - When this bit is set to '1', the USB bus is reset. This bit continues set to '1' during USB bus resetting, and changes to '0' when USB bus resetting is ended. If this bit is set to '0', no processing is performed."]
pub type URST_R = crate::BitReader;
#[doc = "Field `URST` writer - When this bit is set to '1', the USB bus is reset. This bit continues set to '1' during USB bus resetting, and changes to '0' when USB bus resetting is ended. If this bit is set to '0', no processing is performed."]
pub type URST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_5` reader - N/A"]
pub type RSVD_5_R = crate::BitReader;
#[doc = "Field `RSTBUSY` reader - This bit shows that USB Host is being reset internally. If the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1', this bit is set to '1'. If the RST bit of Host Control 1 Register (HOST_CTL1) is set to '0', this bit is set to '0'. '0' : USB Host isn't being reset. '1' : USB Host is being reset. Notes: - If this bit is '1', the token must't be executed. - This bit isn't set to '0' or '1' immediately evne if the RST bit of Host Control 1 Register (HOST_CTL1) is set to '0' or '1'."]
pub type RSTBUSY_R = crate::BitReader;
#[doc = "Field `CLKSEL_ST` reader - This bit shows whether it is full-speed or not. If the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is set to '1', this bit is set to '1'. '0' : Low speed '1' : Full speed Note: - If this bit is different from the CLKSEL bit, The execution of the token and bus reset must be waited until the match. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1)."]
pub type CLKSEL_ST_R = crate::BitReader;
#[doc = "Field `HOST_ST` reader - This bit shows whether it is USB Host mode. If the HOST bit of the Host Control Register (HOST_CTL0) is set to '1', this bit is set to '1'. '0' : USB Device '1' : USB Host Notes: - If this bit is different from the CLKSEL bit, The execution of the token must be waited until the match. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1)."]
pub type HOST_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When this bit is '1', it means that the device is connected. When this bit is '0', it means that the device is disconnected. '0' : Device is disconnected. '1' : Device is connected. Notes: - This bit is initialized if the RST bit of the Host Control 1 Register (Host_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1)."]
    #[inline(always)]
    pub fn cstat(&self) -> CSTAT_R {
        CSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this bit is '1', it means that the device is connected in the full-speed mode. When this bit is '0', it means that the device is connected in the low-speed mode. This bit is valid when the CSTAT bit of the Host Status Register (HOST_STATUS) is '1'. '0' : Low-speed. '1' : Full-speed. Notes: - This bit is initialized if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1)."]
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this bit is set to '1', the USB Host is placed into the suspend state. If this bit is set to '0' while it is '1' or the USB bus is placed into the k-state mode, the suspend state is released, and the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. Set to '1' : Suspend. Set '0' while this bit is '1' : Resume. Others : Holds the status. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - If this bit is set to '1', this bit must not be set to '1' until the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. - Do not set this bit to '1' while the USB is active (during USB bus resetting, data transfer, or SOF timer running). - If the value of this bit is changed, it is not immediately reflected on the state of the USB bus. To check whether or not the state is updated, read this bit."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When a SOF token is sent using the Host Token Endpoint Register (HOST_TOKEN), this bit is set to '1', which means that the SOF timer is active. When this bit is '0', it means that the SOF timer is under suspension. To stop the active SOF timer, write '0' to this bit. However, if this bit is written with '1', its value is ignored. '0' : The SOF timer is stopped. '1' : The SOF timer is active. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - The SOF timer does not stop immediately after this bit has been set to '0' to stop the SOF timer. To check whether or not the SOF timer is stopped, read this bit."]
    #[inline(always)]
    pub fn sofbusy(&self) -> SOFBUSY_R {
        SOFBUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When this bit is set to '1', the USB bus is reset. This bit continues set to '1' during USB bus resetting, and changes to '0' when USB bus resetting is ended. If this bit is set to '0', no processing is performed."]
    #[inline(always)]
    pub fn urst(&self) -> URST_R {
        URST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn rsvd_5(&self) -> RSVD_5_R {
        RSVD_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit shows that USB Host is being reset internally. If the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1', this bit is set to '1'. If the RST bit of Host Control 1 Register (HOST_CTL1) is set to '0', this bit is set to '0'. '0' : USB Host isn't being reset. '1' : USB Host is being reset. Notes: - If this bit is '1', the token must't be executed. - This bit isn't set to '0' or '1' immediately evne if the RST bit of Host Control 1 Register (HOST_CTL1) is set to '0' or '1'."]
    #[inline(always)]
    pub fn rstbusy(&self) -> RSTBUSY_R {
        RSTBUSY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit shows whether it is full-speed or not. If the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is set to '1', this bit is set to '1'. '0' : Low speed '1' : Full speed Note: - If this bit is different from the CLKSEL bit, The execution of the token and bus reset must be waited until the match. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1)."]
    #[inline(always)]
    pub fn clksel_st(&self) -> CLKSEL_ST_R {
        CLKSEL_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit shows whether it is USB Host mode. If the HOST bit of the Host Control Register (HOST_CTL0) is set to '1', this bit is set to '1'. '0' : USB Device '1' : USB Host Notes: - If this bit is different from the CLKSEL bit, The execution of the token must be waited until the match. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1)."]
    #[inline(always)]
    pub fn host_st(&self) -> HOST_ST_R {
        HOST_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - If this bit is set to '1', the USB Host is placed into the suspend state. If this bit is set to '0' while it is '1' or the USB bus is placed into the k-state mode, the suspend state is released, and the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. Set to '1' : Suspend. Set '0' while this bit is '1' : Resume. Others : Holds the status. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - If this bit is set to '1', this bit must not be set to '1' until the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. - Do not set this bit to '1' while the USB is active (during USB bus resetting, data transfer, or SOF timer running). - If the value of this bit is changed, it is not immediately reflected on the state of the USB bus. To check whether or not the state is updated, read this bit."]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<HOST_STATUS_SPEC> {
        SUSP_W::new(self, 2)
    }
    #[doc = "Bit 3 - When a SOF token is sent using the Host Token Endpoint Register (HOST_TOKEN), this bit is set to '1', which means that the SOF timer is active. When this bit is '0', it means that the SOF timer is under suspension. To stop the active SOF timer, write '0' to this bit. However, if this bit is written with '1', its value is ignored. '0' : The SOF timer is stopped. '1' : The SOF timer is active. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - The SOF timer does not stop immediately after this bit has been set to '0' to stop the SOF timer. To check whether or not the SOF timer is stopped, read this bit."]
    #[inline(always)]
    #[must_use]
    pub fn sofbusy(&mut self) -> SOFBUSY_W<HOST_STATUS_SPEC> {
        SOFBUSY_W::new(self, 3)
    }
    #[doc = "Bit 4 - When this bit is set to '1', the USB bus is reset. This bit continues set to '1' during USB bus resetting, and changes to '0' when USB bus resetting is ended. If this bit is set to '0', no processing is performed."]
    #[inline(always)]
    #[must_use]
    pub fn urst(&mut self) -> URST_W<HOST_STATUS_SPEC> {
        URST_W::new(self, 4)
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
#[doc = "Host Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_STATUS_SPEC;
impl crate::RegisterSpec for HOST_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_status::R`](R) reader structure"]
impl crate::Readable for HOST_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_status::W`](W) writer structure"]
impl crate::Writable for HOST_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_STATUS to value 0xc2"]
impl crate::Resettable for HOST_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0xc2;
}
