#[doc = "Register `HOST_ERR` reader"]
pub type R = crate::R<HOST_ERR_SPEC>;
#[doc = "Register `HOST_ERR` writer"]
pub type W = crate::W<HOST_ERR_SPEC>;
#[doc = "Field `HS` reader - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. HS bits change values '11' under the following condition. However, if HS bits are written except the following conditions, the values are ignored. - HS bits indicate values except '11' and write the value '11' to HS bits. Note: This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type HS_R = crate::FieldReader<HS_A>;
#[doc = "These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. HS bits change values '11' under the following condition. However, if HS bits are written except the following conditions, the values are ignored. - HS bits indicate values except '11' and write the value '11' to HS bits. Note: This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HS_A {
    #[doc = "0: Acknowledge Packet"]
    ACK = 0,
    #[doc = "1: Non-Acknowledge Packet"]
    NAK = 1,
    #[doc = "2: Stall Packet"]
    STALL = 2,
    #[doc = "3: Null Packet"]
    NULL = 3,
}
impl From<HS_A> for u8 {
    #[inline(always)]
    fn from(variant: HS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HS_A {
    type Ux = u8;
}
impl HS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HS_A {
        match self.bits {
            0 => HS_A::ACK,
            1 => HS_A::NAK,
            2 => HS_A::STALL,
            3 => HS_A::NULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Acknowledge Packet"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == HS_A::ACK
    }
    #[doc = "Non-Acknowledge Packet"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == HS_A::NAK
    }
    #[doc = "Stall Packet"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == HS_A::STALL
    }
    #[doc = "Null Packet"]
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        *self == HS_A::NULL
    }
}
#[doc = "Field `HS` writer - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. HS bits change values '11' under the following condition. However, if HS bits are written except the following conditions, the values are ignored. - HS bits indicate values except '11' and write the value '11' to HS bits. Note: This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type HS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HS_A>;
impl<'a, REG> HS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Acknowledge Packet"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(HS_A::ACK)
    }
    #[doc = "Non-Acknowledge Packet"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut crate::W<REG> {
        self.variant(HS_A::NAK)
    }
    #[doc = "Stall Packet"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut crate::W<REG> {
        self.variant(HS_A::STALL)
    }
    #[doc = "Null Packet"]
    #[inline(always)]
    pub fn null(self) -> &'a mut crate::W<REG> {
        self.variant(HS_A::NULL)
    }
}
#[doc = "Field `STUFF` reader - If this bit is set to '1', it means that a bit stuffing error is detected. When this bit is '0', it means that no stuffing error is detected. If a stuffing error is detected, bit5 (Timeout) of this register is also set to '1'. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No stuffing error. '1' : Stuffing error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type STUFF_R = crate::BitReader;
#[doc = "Field `STUFF` writer - If this bit is set to '1', it means that a bit stuffing error is detected. When this bit is '0', it means that no stuffing error is detected. If a stuffing error is detected, bit5 (Timeout) of this register is also set to '1'. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No stuffing error. '1' : Stuffing error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type STUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGERR` reader - If this bit is set to '1', it means that the data of this bit does not match the value of the received toggle data. When this bit is '0', it means that no toggle error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No toggle error. '1' : Toggle error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type TGERR_R = crate::BitReader;
#[doc = "Field `TGERR` writer - If this bit is set to '1', it means that the data of this bit does not match the value of the received toggle data. When this bit is '0', it means that no toggle error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No toggle error. '1' : Toggle error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type TGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no CRC error is detected. If a CRC error is detected, bit5 (Timeout) of this register is also set to '1'. When this bit is '0', it means that no CRC error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No CRC error. '1' : CRC error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type CRC_R = crate::BitReader;
#[doc = "Field `CRC` writer - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no CRC error is detected. If a CRC error is detected, bit5 (Timeout) of this register is also set to '1'. When this bit is '0', it means that no CRC error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No CRC error. '1' : CRC error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type CRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUT` reader - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. When this bit is '0', it means that no error occurs. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No timeout. '1' : Timeout occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type TOUT_R = crate::BitReader;
#[doc = "Field `TOUT` writer - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. When this bit is '0', it means that no error occurs. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No timeout. '1' : Timeout occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type TOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RERR` reader - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (Timeout) of this register is also set to '1'. When this bit is '0', it means that no error occurs. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No receive error. '1' : Maximum packet receive error. - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type RERR_R = crate::BitReader;
#[doc = "Field `RERR` writer - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (Timeout) of this register is also set to '1'. When this bit is '0', it means that no error occurs. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No receive error. '1' : Maximum packet receive error. - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type RERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTSOF` reader - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that no lost SOF error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Sends SOF. '1' : SOF sending error. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type LSTSOF_R = crate::BitReader;
#[doc = "Field `LSTSOF` writer - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that no lost SOF error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Sends SOF. '1' : SOF sending error. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type LSTSOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. HS bits change values '11' under the following condition. However, if HS bits are written except the following conditions, the values are ignored. - HS bits indicate values except '11' and write the value '11' to HS bits. Note: This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a bit stuffing error is detected. When this bit is '0', it means that no stuffing error is detected. If a stuffing error is detected, bit5 (Timeout) of this register is also set to '1'. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No stuffing error. '1' : Stuffing error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn stuff(&self) -> STUFF_R {
        STUFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that the data of this bit does not match the value of the received toggle data. When this bit is '0', it means that no toggle error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No toggle error. '1' : Toggle error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tgerr(&self) -> TGERR_R {
        TGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no CRC error is detected. If a CRC error is detected, bit5 (Timeout) of this register is also set to '1'. When this bit is '0', it means that no CRC error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No CRC error. '1' : CRC error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. When this bit is '0', it means that no error occurs. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No timeout. '1' : Timeout occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (Timeout) of this register is also set to '1'. When this bit is '0', it means that no error occurs. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No receive error. '1' : Maximum packet receive error. - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn rerr(&self) -> RERR_R {
        RERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that no lost SOF error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Sends SOF. '1' : SOF sending error. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn lstsof(&self) -> LSTSOF_R {
        LSTSOF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. HS bits change values '11' under the following condition. However, if HS bits are written except the following conditions, the values are ignored. - HS bits indicate values except '11' and write the value '11' to HS bits. Note: This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn hs(&mut self) -> HS_W<HOST_ERR_SPEC> {
        HS_W::new(self, 0)
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a bit stuffing error is detected. When this bit is '0', it means that no stuffing error is detected. If a stuffing error is detected, bit5 (Timeout) of this register is also set to '1'. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No stuffing error. '1' : Stuffing error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn stuff(&mut self) -> STUFF_W<HOST_ERR_SPEC> {
        STUFF_W::new(self, 2)
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that the data of this bit does not match the value of the received toggle data. When this bit is '0', it means that no toggle error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No toggle error. '1' : Toggle error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn tgerr(&mut self) -> TGERR_W<HOST_ERR_SPEC> {
        TGERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no CRC error is detected. If a CRC error is detected, bit5 (Timeout) of this register is also set to '1'. When this bit is '0', it means that no CRC error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No CRC error. '1' : CRC error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<HOST_ERR_SPEC> {
        CRC_W::new(self, 4)
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. When this bit is '0', it means that no error occurs. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No timeout. '1' : Timeout occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> TOUT_W<HOST_ERR_SPEC> {
        TOUT_W::new(self, 5)
    }
    #[doc = "Bit 6 - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (Timeout) of this register is also set to '1'. When this bit is '0', it means that no error occurs. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No receive error. '1' : Maximum packet receive error. - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn rerr(&mut self) -> RERR_W<HOST_ERR_SPEC> {
        RERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that no lost SOF error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Sends SOF. '1' : SOF sending error. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn lstsof(&mut self) -> LSTSOF_W<HOST_ERR_SPEC> {
        LSTSOF_W::new(self, 7)
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
#[doc = "Host Error Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_err::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_err::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_ERR_SPEC;
impl crate::RegisterSpec for HOST_ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_err::R`](R) reader structure"]
impl crate::Readable for HOST_ERR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_err::W`](W) writer structure"]
impl crate::Writable for HOST_ERR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_ERR to value 0x03"]
impl crate::Resettable for HOST_ERR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
