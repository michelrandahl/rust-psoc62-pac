#[doc = "Register `INTR_HOST_EP_SET` reader"]
pub type R = crate::R<INTR_HOST_EP_SET_SPEC>;
#[doc = "Register `INTR_HOST_EP_SET` writer"]
pub type W = crate::W<INTR_HOST_EP_SET_SPEC>;
#[doc = "Field `EP1DRQS` reader - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
pub type EP1DRQS_R = crate::BitReader;
#[doc = "Field `EP1DRQS` writer - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
pub type EP1DRQS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1SPKS` reader - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
pub type EP1SPKS_R = crate::BitReader;
#[doc = "Field `EP1SPKS` writer - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
pub type EP1SPKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2DRQS` reader - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
pub type EP2DRQS_R = crate::BitReader;
#[doc = "Field `EP2DRQS` writer - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
pub type EP2DRQS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2SPKS` reader - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
pub type EP2SPKS_R = crate::BitReader;
#[doc = "Field `EP2SPKS` writer - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
pub type EP2SPKS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn ep1drqs(&self) -> EP1DRQS_R {
        EP1DRQS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
    #[inline(always)]
    pub fn ep1spks(&self) -> EP1SPKS_R {
        EP1SPKS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn ep2drqs(&self) -> EP2DRQS_R {
        EP2DRQS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
    #[inline(always)]
    pub fn ep2spks(&self) -> EP2SPKS_R {
        EP2SPKS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ep1drqs(&mut self) -> EP1DRQS_W<INTR_HOST_EP_SET_SPEC> {
        EP1DRQS_W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ep1spks(&mut self) -> EP1SPKS_W<INTR_HOST_EP_SET_SPEC> {
        EP1SPKS_W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ep2drqs(&mut self) -> EP2DRQS_W<INTR_HOST_EP_SET_SPEC> {
        EP2DRQS_W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ep2spks(&mut self) -> EP2SPKS_W<INTR_HOST_EP_SET_SPEC> {
        EP2SPKS_W::new(self, 5)
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
#[doc = "Interrupt USB Host Endpoint Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_host_ep_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_host_ep_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_HOST_EP_SET_SPEC;
impl crate::RegisterSpec for INTR_HOST_EP_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_host_ep_set::R`](R) reader structure"]
impl crate::Readable for INTR_HOST_EP_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_host_ep_set::W`](W) writer structure"]
impl crate::Writable for INTR_HOST_EP_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_HOST_EP_SET to value 0"]
impl crate::Resettable for INTR_HOST_EP_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
