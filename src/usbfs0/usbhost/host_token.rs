#[doc = "Register `HOST_TOKEN` reader"]
pub type R = crate::R<HOST_TOKEN_SPEC>;
#[doc = "Register `HOST_TOKEN` writer"]
pub type W = crate::W<HOST_TOKEN_SPEC>;
#[doc = "Field `ENDPT` reader - These bits are used to specify an endpoint to send or receive data to or from the device. Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type ENDPT_R = crate::FieldReader;
#[doc = "Field `ENDPT` writer - These bits are used to specify an endpoint to send or receive data to or from the device. Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type ENDPT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TKNEN` reader - These bits send a token according to the settings. After operation has been ended, the TKNEN bit is set to '000', and the CMPIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The settings of the TGGL and ENDPT bits are ignored when sending a SOF token. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The PRE packet isn't supported. - Do not set '100' to the TKNEN bit when the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' - Change the USB to the USB Host before writing data to this bit. - When issuing a token again after the token interrupt flag (CMPIRQ) has been set to '1', wait for 3 cycles or more after a USB transfer clock (12 MHz in the full-speed mode, 1.5 MHz in the low-speed mode) was output, then write data to this bit. - Read the value of TKNEN bit if a new value is written in it .Continue writing in this bit until a retrieved value equals a new value written in. During this checking process, it is needed to prevent any interrupt. - Take the following steps when CMPIRQ bit of Interrupt USB Host Register (INTR_USBHOST) is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
pub type TKNEN_R = crate::FieldReader<TKNEN_A>;
#[doc = "These bits send a token according to the settings. After operation has been ended, the TKNEN bit is set to '000', and the CMPIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The settings of the TGGL and ENDPT bits are ignored when sending a SOF token. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The PRE packet isn't supported. - Do not set '100' to the TKNEN bit when the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' - Change the USB to the USB Host before writing data to this bit. - When issuing a token again after the token interrupt flag (CMPIRQ) has been set to '1', wait for 3 cycles or more after a USB transfer clock (12 MHz in the full-speed mode, 1.5 MHz in the low-speed mode) was output, then write data to this bit. - Read the value of TKNEN bit if a new value is written in it .Continue writing in this bit until a retrieved value equals a new value written in. During this checking process, it is needed to prevent any interrupt. - Take the following steps when CMPIRQ bit of Interrupt USB Host Register (INTR_USBHOST) is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TKNEN_A {
    #[doc = "0: Sends no data."]
    NONE = 0,
    #[doc = "1: Sends SETUP token."]
    SETUP = 1,
    #[doc = "2: Sends IN token."]
    IN = 2,
    #[doc = "3: Sends OUT token."]
    OUT = 3,
    #[doc = "4: Sends SOF token."]
    SOF = 4,
    #[doc = "5: Sends Isochronous IN."]
    ISO_IN = 5,
    #[doc = "6: Sends Isochronous OUT."]
    ISO_OUT = 6,
    #[doc = "7: N/A"]
    RSV = 7,
}
impl From<TKNEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TKNEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TKNEN_A {
    type Ux = u8;
}
impl TKNEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TKNEN_A {
        match self.bits {
            0 => TKNEN_A::NONE,
            1 => TKNEN_A::SETUP,
            2 => TKNEN_A::IN,
            3 => TKNEN_A::OUT,
            4 => TKNEN_A::SOF,
            5 => TKNEN_A::ISO_IN,
            6 => TKNEN_A::ISO_OUT,
            7 => TKNEN_A::RSV,
            _ => unreachable!(),
        }
    }
    #[doc = "Sends no data."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TKNEN_A::NONE
    }
    #[doc = "Sends SETUP token."]
    #[inline(always)]
    pub fn is_setup(&self) -> bool {
        *self == TKNEN_A::SETUP
    }
    #[doc = "Sends IN token."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == TKNEN_A::IN
    }
    #[doc = "Sends OUT token."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == TKNEN_A::OUT
    }
    #[doc = "Sends SOF token."]
    #[inline(always)]
    pub fn is_sof(&self) -> bool {
        *self == TKNEN_A::SOF
    }
    #[doc = "Sends Isochronous IN."]
    #[inline(always)]
    pub fn is_iso_in(&self) -> bool {
        *self == TKNEN_A::ISO_IN
    }
    #[doc = "Sends Isochronous OUT."]
    #[inline(always)]
    pub fn is_iso_out(&self) -> bool {
        *self == TKNEN_A::ISO_OUT
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_rsv(&self) -> bool {
        *self == TKNEN_A::RSV
    }
}
#[doc = "Field `TKNEN` writer - These bits send a token according to the settings. After operation has been ended, the TKNEN bit is set to '000', and the CMPIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The settings of the TGGL and ENDPT bits are ignored when sending a SOF token. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The PRE packet isn't supported. - Do not set '100' to the TKNEN bit when the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' - Change the USB to the USB Host before writing data to this bit. - When issuing a token again after the token interrupt flag (CMPIRQ) has been set to '1', wait for 3 cycles or more after a USB transfer clock (12 MHz in the full-speed mode, 1.5 MHz in the low-speed mode) was output, then write data to this bit. - Read the value of TKNEN bit if a new value is written in it .Continue writing in this bit until a retrieved value equals a new value written in. During this checking process, it is needed to prevent any interrupt. - Take the following steps when CMPIRQ bit of Interrupt USB Host Register (INTR_USBHOST) is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
pub type TKNEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, TKNEN_A>;
impl<'a, REG> TKNEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sends no data."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(TKNEN_A::NONE)
    }
    #[doc = "Sends SETUP token."]
    #[inline(always)]
    pub fn setup(self) -> &'a mut crate::W<REG> {
        self.variant(TKNEN_A::SETUP)
    }
    #[doc = "Sends IN token."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(TKNEN_A::IN)
    }
    #[doc = "Sends OUT token."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(TKNEN_A::OUT)
    }
    #[doc = "Sends SOF token."]
    #[inline(always)]
    pub fn sof(self) -> &'a mut crate::W<REG> {
        self.variant(TKNEN_A::SOF)
    }
    #[doc = "Sends Isochronous IN."]
    #[inline(always)]
    pub fn iso_in(self) -> &'a mut crate::W<REG> {
        self.variant(TKNEN_A::ISO_IN)
    }
    #[doc = "Sends Isochronous OUT."]
    #[inline(always)]
    pub fn iso_out(self) -> &'a mut crate::W<REG> {
        self.variant(TKNEN_A::ISO_OUT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsv(self) -> &'a mut crate::W<REG> {
        self.variant(TKNEN_A::RSV)
    }
}
#[doc = "Field `TGGL` reader - This bit is used to set toggle data. Toggle data is sent depending on the setting of this bit. When receiving toggle data, received toggle data is compared with the toggle data of this bit to verify whether or not an error occurs. '0' : DATA0 '1' : DATA1 Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Set this bit when the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN) is '000'."]
pub type TGGL_R = crate::BitReader;
#[doc = "Field `TGGL` writer - This bit is used to set toggle data. Toggle data is sent depending on the setting of this bit. When receiving toggle data, received toggle data is compared with the toggle data of this bit to verify whether or not an error occurs. '0' : DATA0 '1' : DATA1 Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Set this bit when the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN) is '000'."]
pub type TGGL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - These bits are used to specify an endpoint to send or receive data to or from the device. Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn endpt(&self) -> ENDPT_R {
        ENDPT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - These bits send a token according to the settings. After operation has been ended, the TKNEN bit is set to '000', and the CMPIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The settings of the TGGL and ENDPT bits are ignored when sending a SOF token. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The PRE packet isn't supported. - Do not set '100' to the TKNEN bit when the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' - Change the USB to the USB Host before writing data to this bit. - When issuing a token again after the token interrupt flag (CMPIRQ) has been set to '1', wait for 3 cycles or more after a USB transfer clock (12 MHz in the full-speed mode, 1.5 MHz in the low-speed mode) was output, then write data to this bit. - Read the value of TKNEN bit if a new value is written in it .Continue writing in this bit until a retrieved value equals a new value written in. During this checking process, it is needed to prevent any interrupt. - Take the following steps when CMPIRQ bit of Interrupt USB Host Register (INTR_USBHOST) is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    pub fn tknen(&self) -> TKNEN_R {
        TKNEN_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - This bit is used to set toggle data. Toggle data is sent depending on the setting of this bit. When receiving toggle data, received toggle data is compared with the toggle data of this bit to verify whether or not an error occurs. '0' : DATA0 '1' : DATA1 Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Set this bit when the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN) is '000'."]
    #[inline(always)]
    pub fn tggl(&self) -> TGGL_R {
        TGGL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - These bits are used to specify an endpoint to send or receive data to or from the device. Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn endpt(&mut self) -> ENDPT_W<HOST_TOKEN_SPEC> {
        ENDPT_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - These bits send a token according to the settings. After operation has been ended, the TKNEN bit is set to '000', and the CMPIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The settings of the TGGL and ENDPT bits are ignored when sending a SOF token. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - The PRE packet isn't supported. - Do not set '100' to the TKNEN bit when the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' - Change the USB to the USB Host before writing data to this bit. - When issuing a token again after the token interrupt flag (CMPIRQ) has been set to '1', wait for 3 cycles or more after a USB transfer clock (12 MHz in the full-speed mode, 1.5 MHz in the low-speed mode) was output, then write data to this bit. - Read the value of TKNEN bit if a new value is written in it .Continue writing in this bit until a retrieved value equals a new value written in. During this checking process, it is needed to prevent any interrupt. - Take the following steps when CMPIRQ bit of Interrupt USB Host Register (INTR_USBHOST) is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn tknen(&mut self) -> TKNEN_W<HOST_TOKEN_SPEC> {
        TKNEN_W::new(self, 4)
    }
    #[doc = "Bit 8 - This bit is used to set toggle data. Toggle data is sent depending on the setting of this bit. When receiving toggle data, received toggle data is compared with the toggle data of this bit to verify whether or not an error occurs. '0' : DATA0 '1' : DATA1 Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Set this bit when the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN) is '000'."]
    #[inline(always)]
    #[must_use]
    pub fn tggl(&mut self) -> TGGL_W<HOST_TOKEN_SPEC> {
        TGGL_W::new(self, 8)
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
#[doc = "Host Token Endpoint Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_token::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_token::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_TOKEN_SPEC;
impl crate::RegisterSpec for HOST_TOKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_token::R`](R) reader structure"]
impl crate::Readable for HOST_TOKEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_token::W`](W) writer structure"]
impl crate::Writable for HOST_TOKEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_TOKEN to value 0"]
impl crate::Resettable for HOST_TOKEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
