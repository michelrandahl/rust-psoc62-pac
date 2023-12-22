#[doc = "Register `ARB_EP4_CFG` reader"]
pub type R = crate::R<ARB_EP4_CFG_SPEC>;
#[doc = "Register `ARB_EP4_CFG` writer"]
pub type W = crate::W<ARB_EP4_CFG_SPEC>;
#[doc = "Field `IN_DATA_RDY` reader - Indication that Endpoint Packet Data is Ready in Main memory"]
pub type IN_DATA_RDY_R = crate::BitReader;
#[doc = "Field `IN_DATA_RDY` writer - Indication that Endpoint Packet Data is Ready in Main memory"]
pub type IN_DATA_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_REQ` reader - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
pub type DMA_REQ_R = crate::BitReader;
#[doc = "Field `DMA_REQ` writer - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
pub type DMA_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_BYPASS` reader - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
pub type CRC_BYPASS_R = crate::BitReader<CRC_BYPASS_A>;
#[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC_BYPASS_A {
    #[doc = "0: No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    CRC_NORMAL = 0,
    #[doc = "1: CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    CRC_BYPASS = 1,
}
impl From<CRC_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl CRC_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRC_BYPASS_A {
        match self.bits {
            false => CRC_BYPASS_A::CRC_NORMAL,
            true => CRC_BYPASS_A::CRC_BYPASS,
        }
    }
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    #[inline(always)]
    pub fn is_crc_normal(&self) -> bool {
        *self == CRC_BYPASS_A::CRC_NORMAL
    }
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    #[inline(always)]
    pub fn is_crc_bypass(&self) -> bool {
        *self == CRC_BYPASS_A::CRC_BYPASS
    }
}
#[doc = "Field `CRC_BYPASS` writer - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
pub type CRC_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG, CRC_BYPASS_A>;
impl<'a, REG> CRC_BYPASS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    #[inline(always)]
    pub fn crc_normal(self) -> &'a mut crate::W<REG> {
        self.variant(CRC_BYPASS_A::CRC_NORMAL)
    }
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    #[inline(always)]
    pub fn crc_bypass(self) -> &'a mut crate::W<REG> {
        self.variant(CRC_BYPASS_A::CRC_BYPASS)
    }
}
#[doc = "Field `RESET_PTR` reader - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
pub type RESET_PTR_R = crate::BitReader<RESET_PTR_A>;
#[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_PTR_A {
    #[doc = "0: Do not Reset Pointer; Krypton Backward compatibility mode"]
    RESET_KRYPTON = 0,
    #[doc = "1: Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    RESET_NORMAL = 1,
}
impl From<RESET_PTR_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_PTR_A) -> Self {
        variant as u8 != 0
    }
}
impl RESET_PTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESET_PTR_A {
        match self.bits {
            false => RESET_PTR_A::RESET_KRYPTON,
            true => RESET_PTR_A::RESET_NORMAL,
        }
    }
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    #[inline(always)]
    pub fn is_reset_krypton(&self) -> bool {
        *self == RESET_PTR_A::RESET_KRYPTON
    }
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    #[inline(always)]
    pub fn is_reset_normal(&self) -> bool {
        *self == RESET_PTR_A::RESET_NORMAL
    }
}
#[doc = "Field `RESET_PTR` writer - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
pub type RESET_PTR_W<'a, REG> = crate::BitWriter<'a, REG, RESET_PTR_A>;
impl<'a, REG> RESET_PTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    #[inline(always)]
    pub fn reset_krypton(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_PTR_A::RESET_KRYPTON)
    }
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    #[inline(always)]
    pub fn reset_normal(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_PTR_A::RESET_NORMAL)
    }
}
impl R {
    #[doc = "Bit 0 - Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn in_data_rdy(&self) -> IN_DATA_RDY_R {
        IN_DATA_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn dma_req(&self) -> DMA_REQ_R {
        DMA_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn crc_bypass(&self) -> CRC_BYPASS_R {
        CRC_BYPASS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn reset_ptr(&self) -> RESET_PTR_R {
        RESET_PTR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    #[must_use]
    pub fn in_data_rdy(&mut self) -> IN_DATA_RDY_W<ARB_EP4_CFG_SPEC> {
        IN_DATA_RDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    #[must_use]
    pub fn dma_req(&mut self) -> DMA_REQ_W<ARB_EP4_CFG_SPEC> {
        DMA_REQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    #[must_use]
    pub fn crc_bypass(&mut self) -> CRC_BYPASS_W<ARB_EP4_CFG_SPEC> {
        CRC_BYPASS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    #[must_use]
    pub fn reset_ptr(&mut self) -> RESET_PTR_W<ARB_EP4_CFG_SPEC> {
        RESET_PTR_W::new(self, 3)
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
#[doc = "Endpoint Configuration Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_ep4_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_ep4_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_EP4_CFG_SPEC;
impl crate::RegisterSpec for ARB_EP4_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_ep4_cfg::R`](R) reader structure"]
impl crate::Readable for ARB_EP4_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_ep4_cfg::W`](W) writer structure"]
impl crate::Writable for ARB_EP4_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARB_EP4_CFG to value 0"]
impl crate::Resettable for ARB_EP4_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
