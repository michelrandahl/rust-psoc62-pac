#[doc = "Register `AP_CTL` reader"]
pub type R = crate::R<AP_CTL_SPEC>;
#[doc = "Register `AP_CTL` writer"]
pub type W = crate::W<AP_CTL_SPEC>;
#[doc = "Field `CM0_ENABLE` reader - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
pub type CM0_ENABLE_R = crate::BitReader;
#[doc = "Field `CM0_ENABLE` writer - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
pub type CM0_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM4_ENABLE` reader - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
pub type CM4_ENABLE_R = crate::BitReader;
#[doc = "Field `CM4_ENABLE` writer - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
pub type CM4_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_ENABLE` reader - Enables the system AP interface: '0': Disabled. '1': Enabled."]
pub type SYS_ENABLE_R = crate::BitReader;
#[doc = "Field `SYS_ENABLE` writer - Enables the system AP interface: '0': Disabled. '1': Enabled."]
pub type SYS_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM0_DISABLE` reader - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
pub type CM0_DISABLE_R = crate::BitReader;
#[doc = "Field `CM0_DISABLE` writer - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
pub type CM0_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM4_DISABLE` reader - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
pub type CM4_DISABLE_R = crate::BitReader;
#[doc = "Field `CM4_DISABLE` writer - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
pub type CM4_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_DISABLE` reader - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
pub type SYS_DISABLE_R = crate::BitReader;
#[doc = "Field `SYS_DISABLE` writer - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
pub type SYS_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm0_enable(&self) -> CM0_ENABLE_R {
        CM0_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm4_enable(&self) -> CM4_ENABLE_R {
        CM4_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the system AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn sys_enable(&self) -> SYS_ENABLE_R {
        SYS_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm0_disable(&self) -> CM0_DISABLE_R {
        CM0_DISABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm4_disable(&self) -> CM4_DISABLE_R {
        CM4_DISABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
    #[inline(always)]
    pub fn sys_disable(&self) -> SYS_DISABLE_R {
        SYS_DISABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cm0_enable(&mut self) -> CM0_ENABLE_W<AP_CTL_SPEC> {
        CM0_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cm4_enable(&mut self) -> CM4_ENABLE_W<AP_CTL_SPEC> {
        CM4_ENABLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables the system AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn sys_enable(&mut self) -> SYS_ENABLE_W<AP_CTL_SPEC> {
        SYS_ENABLE_W::new(self, 2)
    }
    #[doc = "Bit 16 - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn cm0_disable(&mut self) -> CM0_DISABLE_W<AP_CTL_SPEC> {
        CM0_DISABLE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn cm4_disable(&mut self) -> CM4_DISABLE_W<AP_CTL_SPEC> {
        CM4_DISABLE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn sys_disable(&mut self) -> SYS_DISABLE_W<AP_CTL_SPEC> {
        SYS_DISABLE_W::new(self, 18)
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
#[doc = "Access port control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ap_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ap_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AP_CTL_SPEC;
impl crate::RegisterSpec for AP_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ap_ctl::R`](R) reader structure"]
impl crate::Readable for AP_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ap_ctl::W`](W) writer structure"]
impl crate::Writable for AP_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AP_CTL to value 0"]
impl crate::Resettable for AP_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
