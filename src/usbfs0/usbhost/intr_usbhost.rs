#[doc = "Register `INTR_USBHOST` reader"]
pub type R = crate::R<INTR_USBHOST_SPEC>;
#[doc = "Register `INTR_USBHOST` writer"]
pub type W = crate::W<INTR_USBHOST_SPEC>;
#[doc = "Field `SOFIRQ` reader - If this bit is set to '1', it means that SOF token sending is started. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not issue an interrupt request by starting a SOF token. '1' : Issues an interrupt request by starting a SOF token. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type SOFIRQ_R = crate::BitReader;
#[doc = "Field `SOFIRQ` writer - If this bit is set to '1', it means that SOF token sending is started. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not issue an interrupt request by starting a SOF token. '1' : Issues an interrupt request by starting a SOF token. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type SOFIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRQ` reader - If this bit is set to '1', it means that a device disconnection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device disconnection. '1' : Issues an interrupt request by detecting a device disconnection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type DIRQ_R = crate::BitReader;
#[doc = "Field `DIRQ` writer - If this bit is set to '1', it means that a device disconnection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device disconnection. '1' : Issues an interrupt request by detecting a device disconnection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type DIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNNIRQ` reader - If this bit is set to '1', it means that a device connection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device connection. '1' : Issues an interrupt request by detecting a device connection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type CNNIRQ_R = crate::BitReader;
#[doc = "Field `CNNIRQ` writer - If this bit is set to '1', it means that a device connection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device connection. '1' : Issues an interrupt request by detecting a device connection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type CNNIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPIRQ` reader - If this bit is set to '1', it means that a token is completed. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by token completion. '1' : Issues an interrupt request by token completion. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit is not set to '1' even if the TCAN bit of the Interrupt USBHost Register (INTR_USBHOST) changes to '1'. - Take the following steps when this bit is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
pub type CMPIRQ_R = crate::BitReader;
#[doc = "Field `CMPIRQ` writer - If this bit is set to '1', it means that a token is completed. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by token completion. '1' : Issues an interrupt request by token completion. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit is not set to '1' even if the TCAN bit of the Interrupt USBHost Register (INTR_USBHOST) changes to '1'. - Take the following steps when this bit is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
pub type CMPIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URIRQ` reader - If this bit is set to '1', it means that USB bus resetting is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by USB bus resetting. '1' : Issues an interrupt request by USB bus resetting. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type URIRQ_R = crate::BitReader;
#[doc = "Field `URIRQ` writer - If this bit is set to '1', it means that USB bus resetting is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by USB bus resetting. '1' : Issues an interrupt request by USB bus resetting. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type URIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKIRQ` reader - If this bit is set to '1', it means that remote Wake-up is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by restart. '1' : Issues an interrupt request by restart. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type RWKIRQ_R = crate::BitReader;
#[doc = "Field `RWKIRQ` writer - If this bit is set to '1', it means that remote Wake-up is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by restart. '1' : Issues an interrupt request by restart. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type RWKIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_6` reader - N/A"]
pub type RSVD_6_R = crate::BitReader;
#[doc = "Field `RSVD_6` writer - N/A"]
pub type RSVD_6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCAN` reader - If this bit is set to '1', it means that token sending is canceled based on the setting of the CANCEL bit of Host Control 2 Register (HOST_CTL2). When this bit is '0', it means that token sending is not canceled. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not cancel token sending. '1' : Cancels token sending. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type TCAN_R = crate::BitReader;
#[doc = "Field `TCAN` writer - If this bit is set to '1', it means that token sending is canceled based on the setting of the CANCEL bit of Host Control 2 Register (HOST_CTL2). When this bit is '0', it means that token sending is not canceled. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not cancel token sending. '1' : Cancels token sending. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type TCAN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If this bit is set to '1', it means that SOF token sending is started. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not issue an interrupt request by starting a SOF token. '1' : Issues an interrupt request by starting a SOF token. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn sofirq(&self) -> SOFIRQ_R {
        SOFIRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this bit is set to '1', it means that a device disconnection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device disconnection. '1' : Issues an interrupt request by detecting a device disconnection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn dirq(&self) -> DIRQ_R {
        DIRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a device connection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device connection. '1' : Issues an interrupt request by detecting a device connection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn cnnirq(&self) -> CNNIRQ_R {
        CNNIRQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that a token is completed. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by token completion. '1' : Issues an interrupt request by token completion. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit is not set to '1' even if the TCAN bit of the Interrupt USBHost Register (INTR_USBHOST) changes to '1'. - Take the following steps when this bit is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    pub fn cmpirq(&self) -> CMPIRQ_R {
        CMPIRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that USB bus resetting is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by USB bus resetting. '1' : Issues an interrupt request by USB bus resetting. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn urirq(&self) -> URIRQ_R {
        URIRQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that remote Wake-up is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by restart. '1' : Issues an interrupt request by restart. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn rwkirq(&self) -> RWKIRQ_R {
        RWKIRQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that token sending is canceled based on the setting of the CANCEL bit of Host Control 2 Register (HOST_CTL2). When this bit is '0', it means that token sending is not canceled. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not cancel token sending. '1' : Cancels token sending. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tcan(&self) -> TCAN_R {
        TCAN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set to '1', it means that SOF token sending is started. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not issue an interrupt request by starting a SOF token. '1' : Issues an interrupt request by starting a SOF token. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn sofirq(&mut self) -> SOFIRQ_W<INTR_USBHOST_SPEC> {
        SOFIRQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - If this bit is set to '1', it means that a device disconnection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device disconnection. '1' : Issues an interrupt request by detecting a device disconnection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn dirq(&mut self) -> DIRQ_W<INTR_USBHOST_SPEC> {
        DIRQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a device connection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device connection. '1' : Issues an interrupt request by detecting a device connection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn cnnirq(&mut self) -> CNNIRQ_W<INTR_USBHOST_SPEC> {
        CNNIRQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that a token is completed. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by token completion. '1' : Issues an interrupt request by token completion. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit is not set to '1' even if the TCAN bit of the Interrupt USBHost Register (INTR_USBHOST) changes to '1'. - Take the following steps when this bit is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn cmpirq(&mut self) -> CMPIRQ_W<INTR_USBHOST_SPEC> {
        CMPIRQ_W::new(self, 3)
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that USB bus resetting is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by USB bus resetting. '1' : Issues an interrupt request by USB bus resetting. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn urirq(&mut self) -> URIRQ_W<INTR_USBHOST_SPEC> {
        URIRQ_W::new(self, 4)
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that remote Wake-up is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by restart. '1' : Issues an interrupt request by restart. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn rwkirq(&mut self) -> RWKIRQ_W<INTR_USBHOST_SPEC> {
        RWKIRQ_W::new(self, 5)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_6(&mut self) -> RSVD_6_W<INTR_USBHOST_SPEC> {
        RSVD_6_W::new(self, 6)
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that token sending is canceled based on the setting of the CANCEL bit of Host Control 2 Register (HOST_CTL2). When this bit is '0', it means that token sending is not canceled. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not cancel token sending. '1' : Cancels token sending. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn tcan(&mut self) -> TCAN_W<INTR_USBHOST_SPEC> {
        TCAN_W::new(self, 7)
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
#[doc = "Interrupt USB Host Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_usbhost::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_usbhost::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_USBHOST_SPEC;
impl crate::RegisterSpec for INTR_USBHOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_usbhost::R`](R) reader structure"]
impl crate::Readable for INTR_USBHOST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_usbhost::W`](W) writer structure"]
impl crate::Writable for INTR_USBHOST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_USBHOST to value 0"]
impl crate::Resettable for INTR_USBHOST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
