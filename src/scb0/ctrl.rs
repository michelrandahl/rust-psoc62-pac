#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `OVS` reader - N/A"]
pub type OVS_R = crate::FieldReader;
#[doc = "Field `OVS` writer - N/A"]
pub type OVS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EC_AM_MODE` reader - This field specifies the clocking for the address matching (I2C slave) or slave selection detection logic (SPI slave) '0': Internally clocked mode '1': Externally clocked mode In internally clocked mode the address detection(and slave selection detection) is done by clk_scb, and thus won't be done in deep sleep power mode as clk_scb isn't active. In externally clocked mode the address detection is done by the I2C/SPI interface clock. This allows for the device to be awoken on I2C salve address match and SPI slave select assertion. The clocking for the rest of the logic is determined by CTRL.EC_OP_MODE. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field must be '0'."]
pub type EC_AM_MODE_R = crate::BitReader;
#[doc = "Field `EC_AM_MODE` writer - This field specifies the clocking for the address matching (I2C slave) or slave selection detection logic (SPI slave) '0': Internally clocked mode '1': Externally clocked mode In internally clocked mode the address detection(and slave selection detection) is done by clk_scb, and thus won't be done in deep sleep power mode as clk_scb isn't active. In externally clocked mode the address detection is done by the I2C/SPI interface clock. This allows for the device to be awoken on I2C salve address match and SPI slave select assertion. The clocking for the rest of the logic is determined by CTRL.EC_OP_MODE. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field must be '0'."]
pub type EC_AM_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EC_OP_MODE` reader - This field specifies the clocking for the SCB block after the address phase '0': Internally clocked mode '1': externally clocked mode In internally clocked mode, the serial interface protocols run off the clk_scb. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field must be '0'."]
pub type EC_OP_MODE_R = crate::BitReader;
#[doc = "Field `EC_OP_MODE` writer - This field specifies the clocking for the SCB block after the address phase '0': Internally clocked mode '1': externally clocked mode In internally clocked mode, the serial interface protocols run off the clk_scb. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field must be '0'."]
pub type EC_OP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_MODE` reader - This field determines if EZ mode is enabled or disabled for the SCB block '0': EZ Mode Disabled '1': EZ Mode Enabled In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode can only be used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The external master should use continuous data frames; i.e. data frames not seperated by slave deselection. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field must be '0'."]
pub type EZ_MODE_R = crate::BitReader;
#[doc = "Field `EZ_MODE` writer - This field determines if EZ mode is enabled or disabled for the SCB block '0': EZ Mode Disabled '1': EZ Mode Enabled In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode can only be used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The external master should use continuous data frames; i.e. data frames not seperated by slave deselection. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field must be '0'."]
pub type EZ_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTE_MODE` reader - N/A"]
pub type BYTE_MODE_R = crate::BitReader;
#[doc = "Field `BYTE_MODE` writer - N/A"]
pub type BYTE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_RESP_MODE` reader - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1'). In CMD_RESP mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a write memory data element or a read memory data element. The difference from EZ mode is that the address is written by the CPU, not the interface master. CMD_RESP mode can only be used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The external master should use continuous data frames; i.e. data frames not seperated by slave deselection. In CMD_RESP mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field must be '0'."]
pub type CMD_RESP_MODE_R = crate::BitReader;
#[doc = "Field `CMD_RESP_MODE` writer - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1'). In CMD_RESP mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a write memory data element or a read memory data element. The difference from EZ mode is that the address is written by the CPU, not the interface master. CMD_RESP mode can only be used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The external master should use continuous data frames; i.e. data frames not seperated by slave deselection. In CMD_RESP mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field must be '0'."]
pub type CMD_RESP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_ACCEPT` reader - Determines whether a received matching address is accepted in the RX FIFO:. '0': Matching address does not go in RX FIFO '1': Match address does go in RX FIFO In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when this bit is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO. In SPI mode this field must be '0'"]
pub type ADDR_ACCEPT_R = crate::BitReader;
#[doc = "Field `ADDR_ACCEPT` writer - Determines whether a received matching address is accepted in the RX FIFO:. '0': Matching address does not go in RX FIFO '1': Match address does go in RX FIFO In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when this bit is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO. In SPI mode this field must be '0'"]
pub type ADDR_ACCEPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK` reader - Only used in externally clocked mode. If the externally clocked logic and the CPU access the EZ memory at the same time this bit determines whether a CPU access should block and result in bus wait states '0': Do not block, but ingore a write and return 0xffff:ffff for a read '1': Block, resulting in CPU wait states. If BLOCK is '0' and the accesses collide, CPU read operations return 0xffff:ffff and CPU write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of the INTR_TX and INTR_RX registers."]
pub type BLOCK_R = crate::BitReader;
#[doc = "Field `BLOCK` writer - Only used in externally clocked mode. If the externally clocked logic and the CPU access the EZ memory at the same time this bit determines whether a CPU access should block and result in bus wait states '0': Do not block, but ingore a write and return 0xffff:ffff for a read '1': Block, resulting in CPU wait states. If BLOCK is '0' and the accesses collide, CPU read operations return 0xffff:ffff and CPU write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of the INTR_TX and INTR_RX registers."]
pub type BLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - N/A"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "N/A\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: N/A"]
    I2C = 0,
    #[doc = "1: N/A"]
    SPI = 1,
    #[doc = "2: N/A"]
    UART = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::I2C),
            1 => Some(MODE_A::SPI),
            2 => Some(MODE_A::UART),
            _ => None,
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == MODE_A::I2C
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == MODE_A::SPI
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == MODE_A::UART
    }
}
#[doc = "Field `MODE` writer - N/A"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "N/A"]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::I2C)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::SPI)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn uart(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::UART)
    }
}
#[doc = "Field `ENABLED` reader - 0': Block Disabled '1': Block Enabled The proper order in which to initialize the SCB is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable SCB, select the specific operation mode and oversampling factor. When the SCB is enabled, no control information should be changed. Changes must be made AFTER disabling the SCB, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the SCB is re-enabled. Note that disabling the SCB will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - 0': Block Disabled '1': Block Enabled The proper order in which to initialize the SCB is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable SCB, select the specific operation mode and oversampling factor. When the SCB is enabled, no control information should be changed. Changes must be made AFTER disabling the SCB, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the SCB is re-enabled. Note that disabling the SCB will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - This field specifies the clocking for the address matching (I2C slave) or slave selection detection logic (SPI slave) '0': Internally clocked mode '1': Externally clocked mode In internally clocked mode the address detection(and slave selection detection) is done by clk_scb, and thus won't be done in deep sleep power mode as clk_scb isn't active. In externally clocked mode the address detection is done by the I2C/SPI interface clock. This allows for the device to be awoken on I2C salve address match and SPI slave select assertion. The clocking for the rest of the logic is determined by CTRL.EC_OP_MODE. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field must be '0'."]
    #[inline(always)]
    pub fn ec_am_mode(&self) -> EC_AM_MODE_R {
        EC_AM_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This field specifies the clocking for the SCB block after the address phase '0': Internally clocked mode '1': externally clocked mode In internally clocked mode, the serial interface protocols run off the clk_scb. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field must be '0'."]
    #[inline(always)]
    pub fn ec_op_mode(&self) -> EC_OP_MODE_R {
        EC_OP_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This field determines if EZ mode is enabled or disabled for the SCB block '0': EZ Mode Disabled '1': EZ Mode Enabled In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode can only be used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The external master should use continuous data frames; i.e. data frames not seperated by slave deselection. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field must be '0'."]
    #[inline(always)]
    pub fn ez_mode(&self) -> EZ_MODE_R {
        EZ_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn byte_mode(&self) -> BYTE_MODE_R {
        BYTE_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1'). In CMD_RESP mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a write memory data element or a read memory data element. The difference from EZ mode is that the address is written by the CPU, not the interface master. CMD_RESP mode can only be used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The external master should use continuous data frames; i.e. data frames not seperated by slave deselection. In CMD_RESP mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field must be '0'."]
    #[inline(always)]
    pub fn cmd_resp_mode(&self) -> CMD_RESP_MODE_R {
        CMD_RESP_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO:. '0': Matching address does not go in RX FIFO '1': Match address does go in RX FIFO In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when this bit is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO. In SPI mode this field must be '0'"]
    #[inline(always)]
    pub fn addr_accept(&self) -> ADDR_ACCEPT_R {
        ADDR_ACCEPT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Only used in externally clocked mode. If the externally clocked logic and the CPU access the EZ memory at the same time this bit determines whether a CPU access should block and result in bus wait states '0': Do not block, but ingore a write and return 0xffff:ffff for a read '1': Block, resulting in CPU wait states. If BLOCK is '0' and the accesses collide, CPU read operations return 0xffff:ffff and CPU write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of the INTR_TX and INTR_RX registers."]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 31 - 0': Block Disabled '1': Block Enabled The proper order in which to initialize the SCB is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable SCB, select the specific operation mode and oversampling factor. When the SCB is enabled, no control information should be changed. Changes must be made AFTER disabling the SCB, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the SCB is re-enabled. Note that disabling the SCB will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ovs(&mut self) -> OVS_W<CTRL_SPEC> {
        OVS_W::new(self, 0)
    }
    #[doc = "Bit 8 - This field specifies the clocking for the address matching (I2C slave) or slave selection detection logic (SPI slave) '0': Internally clocked mode '1': Externally clocked mode In internally clocked mode the address detection(and slave selection detection) is done by clk_scb, and thus won't be done in deep sleep power mode as clk_scb isn't active. In externally clocked mode the address detection is done by the I2C/SPI interface clock. This allows for the device to be awoken on I2C salve address match and SPI slave select assertion. The clocking for the rest of the logic is determined by CTRL.EC_OP_MODE. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field must be '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ec_am_mode(&mut self) -> EC_AM_MODE_W<CTRL_SPEC> {
        EC_AM_MODE_W::new(self, 8)
    }
    #[doc = "Bit 9 - This field specifies the clocking for the SCB block after the address phase '0': Internally clocked mode '1': externally clocked mode In internally clocked mode, the serial interface protocols run off the clk_scb. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field must be '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ec_op_mode(&mut self) -> EC_OP_MODE_W<CTRL_SPEC> {
        EC_OP_MODE_W::new(self, 9)
    }
    #[doc = "Bit 10 - This field determines if EZ mode is enabled or disabled for the SCB block '0': EZ Mode Disabled '1': EZ Mode Enabled In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode can only be used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The external master should use continuous data frames; i.e. data frames not seperated by slave deselection. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field must be '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ez_mode(&mut self) -> EZ_MODE_W<CTRL_SPEC> {
        EZ_MODE_W::new(self, 10)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn byte_mode(&mut self) -> BYTE_MODE_W<CTRL_SPEC> {
        BYTE_MODE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1'). In CMD_RESP mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a write memory data element or a read memory data element. The difference from EZ mode is that the address is written by the CPU, not the interface master. CMD_RESP mode can only be used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The external master should use continuous data frames; i.e. data frames not seperated by slave deselection. In CMD_RESP mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field must be '0'."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_resp_mode(&mut self) -> CMD_RESP_MODE_W<CTRL_SPEC> {
        CMD_RESP_MODE_W::new(self, 12)
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO:. '0': Matching address does not go in RX FIFO '1': Match address does go in RX FIFO In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when this bit is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO. In SPI mode this field must be '0'"]
    #[inline(always)]
    #[must_use]
    pub fn addr_accept(&mut self) -> ADDR_ACCEPT_W<CTRL_SPEC> {
        ADDR_ACCEPT_W::new(self, 16)
    }
    #[doc = "Bit 17 - Only used in externally clocked mode. If the externally clocked logic and the CPU access the EZ memory at the same time this bit determines whether a CPU access should block and result in bus wait states '0': Do not block, but ingore a write and return 0xffff:ffff for a read '1': Block, resulting in CPU wait states. If BLOCK is '0' and the accesses collide, CPU read operations return 0xffff:ffff and CPU write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of the INTR_TX and INTR_RX registers."]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BLOCK_W<CTRL_SPEC> {
        BLOCK_W::new(self, 17)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRL_SPEC> {
        MODE_W::new(self, 24)
    }
    #[doc = "Bit 31 - 0': Block Disabled '1': Block Enabled The proper order in which to initialize the SCB is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable SCB, select the specific operation mode and oversampling factor. When the SCB is enabled, no control information should be changed. Changes must be made AFTER disabling the SCB, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the SCB is re-enabled. Note that disabling the SCB will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<CTRL_SPEC> {
        ENABLED_W::new(self, 31)
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
#[doc = "Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0300_000f"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_000f;
}
