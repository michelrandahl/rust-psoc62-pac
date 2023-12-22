#[doc = "Register `INTR_HOST_EP` reader"]
pub type R = crate::R<INTR_HOST_EP_SPEC>;
#[doc = "Register `INTR_HOST_EP` writer"]
pub type W = crate::W<INTR_HOST_EP_SPEC>;
#[doc = "Field `EP1DRQ` reader - This bit indicates that the EP1 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
pub type EP1DRQ_R = crate::BitReader;
#[doc = "Field `EP1DRQ` writer - This bit indicates that the EP1 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
pub type EP1DRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1SPK` reader - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 1 Control Register (HOST_EP1_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
pub type EP1SPK_R = crate::BitReader;
#[doc = "Field `EP1SPK` writer - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 1 Control Register (HOST_EP1_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
pub type EP1SPK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2DRQ` reader - This bit indicates that the EP2 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
pub type EP2DRQ_R = crate::BitReader;
#[doc = "Field `EP2DRQ` writer - This bit indicates that the EP2 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
pub type EP2DRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2SPK` reader - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 2 Control Register (HOST_EP2_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
pub type EP2SPK_R = crate::BitReader;
#[doc = "Field `EP2SPK` writer - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 2 Control Register (HOST_EP2_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
pub type EP2SPK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - This bit indicates that the EP1 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    pub fn ep1drq(&self) -> EP1DRQ_R {
        EP1DRQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 1 Control Register (HOST_EP1_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    pub fn ep1spk(&self) -> EP1SPK_R {
        EP1SPK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates that the EP2 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    pub fn ep2drq(&self) -> EP2DRQ_R {
        EP2DRQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 2 Control Register (HOST_EP2_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    pub fn ep2spk(&self) -> EP2SPK_R {
        EP2SPK_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit indicates that the EP1 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    #[must_use]
    pub fn ep1drq(&mut self) -> EP1DRQ_W<INTR_HOST_EP_SPEC> {
        EP1DRQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 1 Control Register (HOST_EP1_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep1spk(&mut self) -> EP1SPK_W<INTR_HOST_EP_SPEC> {
        EP1SPK_W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit indicates that the EP2 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    #[must_use]
    pub fn ep2drq(&mut self) -> EP2DRQ_W<INTR_HOST_EP_SPEC> {
        EP2DRQ_W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 2 Control Register (HOST_EP2_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep2spk(&mut self) -> EP2SPK_W<INTR_HOST_EP_SPEC> {
        EP2SPK_W::new(self, 5)
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
#[doc = "Interrupt USB Host Endpoint Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_host_ep::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_host_ep::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_HOST_EP_SPEC;
impl crate::RegisterSpec for INTR_HOST_EP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_host_ep::R`](R) reader structure"]
impl crate::Readable for INTR_HOST_EP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_host_ep::W`](W) writer structure"]
impl crate::Writable for INTR_HOST_EP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_HOST_EP to value 0"]
impl crate::Resettable for INTR_HOST_EP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
