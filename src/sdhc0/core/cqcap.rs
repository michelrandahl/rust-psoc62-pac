#[doc = "Register `CQCAP` reader"]
pub type R = crate::R<CQCAP_SPEC>;
#[doc = "Field `ITCFVAL` reader - Internal Timer Clock Frequency Value (ITCFVAL) This field scales the frequency of the timer clock provided by ITCFMUL. The Final clock frequency of actual timer clock is calculated as ITCFVAL* ITCFMUL."]
pub type ITCFVAL_R = crate::FieldReader<u16>;
#[doc = "Field `ITCFMUL` reader - N/A"]
pub type ITCFMUL_R = crate::FieldReader;
#[doc = "Field `CRYPTO_SUPPORT` reader - Crypto Support This bit indicates whether the Host Controller supports cryptographic operations. Values: - 0x0 (FALSE): Crypto not Supported - 0x1 (TRUE): Crypto Supported"]
pub type CRYPTO_SUPPORT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - Internal Timer Clock Frequency Value (ITCFVAL) This field scales the frequency of the timer clock provided by ITCFMUL. The Final clock frequency of actual timer clock is calculated as ITCFVAL* ITCFMUL."]
    #[inline(always)]
    pub fn itcfval(&self) -> ITCFVAL_R {
        ITCFVAL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15 - N/A"]
    #[inline(always)]
    pub fn itcfmul(&self) -> ITCFMUL_R {
        ITCFMUL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Crypto Support This bit indicates whether the Host Controller supports cryptographic operations. Values: - 0x0 (FALSE): Crypto not Supported - 0x1 (TRUE): Crypto Supported"]
    #[inline(always)]
    pub fn crypto_support(&self) -> CRYPTO_SUPPORT_R {
        CRYPTO_SUPPORT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Command Queuing Capabilities register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcap::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQCAP_SPEC;
impl crate::RegisterSpec for CQCAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcap::R`](R) reader structure"]
impl crate::Readable for CQCAP_SPEC {}
#[doc = "`reset()` method sets CQCAP to value 0x30c8"]
impl crate::Resettable for CQCAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x30c8;
}
