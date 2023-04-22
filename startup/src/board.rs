use core::panic::PanicInfo;
use stm32h7xx_hal::{device::Peripherals, prelude::*};
use stm32h7xx_hal::stm32::*;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[allow(non_snake_case)]
pub struct Board {
    /// Clock control and distribution
    /// returned after freezing clock config during startup
    pub ccdr: stm32h7xx_hal::rcc::Ccdr,

    #[doc = "AC"]
    pub AC: AC,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "ADC3"]
    pub ADC3: ADC3,
    #[doc = "ADC12_COMMON"]
    pub ADC12_COMMON: ADC12_COMMON,
    #[doc = "ADC3_COMMON"]
    pub ADC3_COMMON: ADC3_COMMON,
    #[doc = "AXI"]
    pub AXI: AXI,
    #[doc = "CAN_CCU"]
    pub CAN_CCU: CAN_CCU,
    #[doc = "CEC"]
    pub CEC: CEC,
    #[doc = "COMP1"]
    pub COMP1: COMP1,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "CRS"]
    pub CRS: CRS,
    #[doc = "CRYP"]
    pub CRYP: CRYP,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "DBGMCU"]
    pub DBGMCU: DBGMCU,
    #[doc = "DCMI"]
    pub DCMI: DCMI,
    #[doc = "DELAY_BLOCK_SDMMC1"]
    pub DELAY_BLOCK_SDMMC1: DELAY_BLOCK_SDMMC1,
    #[doc = "DELAY_BLOCK_SDMMC2"]
    pub DELAY_BLOCK_SDMMC2: DELAY_BLOCK_SDMMC2,
    #[doc = "DFSDM"]
    pub DFSDM: DFSDM,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "DMA2D"]
    pub DMA2D: DMA2D,
    #[doc = "DMAMUX1"]
    pub DMAMUX1: DMAMUX1,
    #[doc = "DMAMUX2"]
    pub DMAMUX2: DMAMUX2,
    #[doc = "DELAY_BLOCK_OCTOSPI1"]
    pub DELAY_BLOCK_OCTOSPI1: DELAY_BLOCK_OCTOSPI1,
    #[doc = "DELAY_BLOCK_OCTOSPI2"]
    pub DELAY_BLOCK_OCTOSPI2: DELAY_BLOCK_OCTOSPI2,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "ETHERNET_MAC"]
    pub ETHERNET_MAC: ETHERNET_MAC,
    #[doc = "FDCAN1"]
    pub FDCAN1: FDCAN1,
    #[doc = "FDCAN2"]
    pub FDCAN2: FDCAN2,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "FPU_CPACR"]
    pub FPU_CPACR: FPU_CPACR,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "GPIOG"]
    pub GPIOG: GPIOG,
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "GPIOJ"]
    pub GPIOJ: GPIOJ,
    #[doc = "GPIOK"]
    pub GPIOK: GPIOK,
    #[doc = "HSEM"]
    pub HSEM: HSEM,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "I2C4"]
    pub I2C4: I2C4,
    #[doc = "IWDG1"]
    pub IWDG1: IWDG1,
    #[doc = "LPTIM1"]
    pub LPTIM1: LPTIM1,
    #[doc = "LPTIM2"]
    pub LPTIM2: LPTIM2,
    #[doc = "LPTIM3"]
    pub LPTIM3: LPTIM3,
    #[doc = "LPTIM4"]
    pub LPTIM4: LPTIM4,
    #[doc = "LPTIM5"]
    pub LPTIM5: LPTIM5,
    #[doc = "LPUART1"]
    pub LPUART1: LPUART1,
    #[doc = "LTDC"]
    pub LTDC: LTDC,
    #[doc = "MDIOS"]
    pub MDIOS: MDIOS,
    #[doc = "MDMA"]
    pub MDMA: MDMA,
    #[doc = "NVIC_STIR"]
    pub NVIC_STIR: NVIC_STIR,
    #[doc = "OCTOSPI1"]
    pub OCTOSPI1: OCTOSPI1,
    #[doc = "OCTOSPI2"]
    pub OCTOSPI2: OCTOSPI2,
    #[doc = "OPAMP"]
    pub OPAMP: OPAMP,
    #[doc = "OTG1_HS_DEVICE"]
    pub OTG1_HS_DEVICE: OTG1_HS_DEVICE,
    #[doc = "OTG2_HS_DEVICE"]
    pub OTG2_HS_DEVICE: OTG2_HS_DEVICE,
    #[doc = "OTG1_HS_GLOBAL"]
    pub OTG1_HS_GLOBAL: OTG1_HS_GLOBAL,
    #[doc = "OTG1_HS_HOST"]
    pub OTG1_HS_HOST: OTG1_HS_HOST,
    #[doc = "OTG2_HS_HOST"]
    pub OTG2_HS_HOST: OTG2_HS_HOST,
    #[doc = "OTG1_HS_PWRCLK"]
    pub OTG1_HS_PWRCLK: OTG1_HS_PWRCLK,
    #[doc = "OTG2_HS_PWRCLK"]
    pub OTG2_HS_PWRCLK: OTG2_HS_PWRCLK,
    #[doc = "OCTOSPII_O_MANAGER"]
    pub OCTOSPII_O_MANAGER: OCTOSPII_O_MANAGER,
    #[doc = "PF"]
    pub PF: PF,
    // #[doc = "PWR"]
    // pub PWR: PWR,
    #[doc = "RAMECC1"]
    pub RAMECC1: RAMECC1,
    #[doc = "RAMECC2"]
    pub RAMECC2: RAMECC2,
    #[doc = "RAMECC3"]
    pub RAMECC3: RAMECC3,
    // #[doc = "RCC"]
    // pub RCC: RCC,  // Consumed during startup
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SAI1"]
    pub SAI1: SAI1,
    #[doc = "SAI4"]
    pub SAI4: SAI4,
    #[doc = "SCB_ACTRL"]
    pub SCB_ACTRL: SCB_ACTRL,
    #[doc = "SDMMC1"]
    pub SDMMC1: SDMMC1,
    #[doc = "SDMMC2"]
    pub SDMMC2: SDMMC2,
    #[doc = "SPDIFRX"]
    pub SPDIFRX: SPDIFRX,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "SPI4"]
    pub SPI4: SPI4,
    #[doc = "SPI5"]
    pub SPI5: SPI5,
    #[doc = "SPI6"]
    pub SPI6: SPI6,
    #[doc = "STK"]
    pub STK: STK,
    #[doc = "SWPMI"]
    pub SWPMI: SWPMI,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "TIM4"]
    pub TIM4: TIM4,
    #[doc = "TIM5"]
    pub TIM5: TIM5,
    #[doc = "TIM6"]
    pub TIM6: TIM6,
    #[doc = "TIM7"]
    pub TIM7: TIM7,
    #[doc = "TIM8"]
    pub TIM8: TIM8,
    #[doc = "TIM12"]
    pub TIM12: TIM12,
    #[doc = "TIM13"]
    pub TIM13: TIM13,
    #[doc = "TIM14"]
    pub TIM14: TIM14,
    #[doc = "TIM15"]
    pub TIM15: TIM15,
    #[doc = "TIM16"]
    pub TIM16: TIM16,
    #[doc = "TIM17"]
    pub TIM17: TIM17,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "UART5"]
    pub UART5: UART5,
    #[doc = "USART6"]
    pub USART6: USART6,
    #[doc = "UART7"]
    pub UART7: UART7,
    #[doc = "UART8"]
    pub UART8: UART8,
    #[doc = "VREFBUF"]
    pub VREFBUF: VREFBUF,
    #[doc = "WWDG1"]
    pub WWDG1: WWDG1,
    #[doc = "ADC2"]
    pub ADC2: ADC2,
    #[doc = "BDMA"]
    pub BDMA: BDMA,
    #[doc = "UART9"]
    pub UART9: UART9,
    #[doc = "USART10"]
    pub USART10: USART10,
    #[doc = "TIM23"]
    pub TIM23: TIM23,
    #[doc = "TIM24"]
    pub TIM24: TIM24,
    #[doc = "CORDIC"]
    pub CORDIC: CORDIC,
    #[doc = "FMAC"]
    pub FMAC: FMAC,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "ETHERNET_DMA"]
    pub ETHERNET_DMA: ETHERNET_DMA,
    #[doc = "ETHERNET_MTL"]
    pub ETHERNET_MTL: ETHERNET_MTL,
}

impl Board {
    pub fn new() -> Self {
        // Get access to the device specific peripherals from the peripheral access crate
        let dp: Peripherals = stm32h7xx_hal::stm32::Peripherals::take().unwrap();

        // Take ownership over the RCC devices and convert them into the corresponding HAL structs
        let rcc: stm32h7xx_hal::rcc::Rcc = dp.RCC.constrain();

        let pwr: stm32h7xx_hal::pwr::Pwr = dp.PWR.constrain();
        let pwrcfg: stm32h7xx_hal::pwr::PowerConfiguration = pwr.freeze();

        // Freeze the configuration of all the clocks in the system and
        // retrieve the Core Clock Distribution and Reset (CCDR) object
        let ccdr: stm32h7xx_hal::rcc::Ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

        Board {
            ccdr: ccdr,
            AC: dp.AC,
            ADC1: dp.ADC1,
            ADC3: dp.ADC3,
            ADC12_COMMON: dp.ADC12_COMMON,
            ADC3_COMMON: dp.ADC3_COMMON,
            AXI: dp.AXI,
            CAN_CCU: dp.CAN_CCU,
            CEC: dp.CEC,
            COMP1: dp.COMP1,
            CRC: dp.CRC,
            CRS: dp.CRS,
            CRYP: dp.CRYP,
            DAC: dp.DAC,
            DBGMCU: dp.DBGMCU,
            DCMI: dp.DCMI,
            DELAY_BLOCK_SDMMC1: dp.DELAY_BLOCK_SDMMC1,
            DELAY_BLOCK_SDMMC2: dp.DELAY_BLOCK_SDMMC2,
            DFSDM: dp.DFSDM,
            DMA1: dp.DMA1,
            DMA2D: dp.DMA2D,
            DMAMUX1: dp.DMAMUX1,
            DMAMUX2: dp.DMAMUX2,
            DELAY_BLOCK_OCTOSPI1: dp.DELAY_BLOCK_OCTOSPI1,
            DELAY_BLOCK_OCTOSPI2: dp.DELAY_BLOCK_OCTOSPI2,
            EXTI: dp.EXTI,
            ETHERNET_MAC: dp.ETHERNET_MAC,
            FDCAN1: dp.FDCAN1,
            FDCAN2: dp.FDCAN2,
            FMAC: dp.FMAC,
            FMC: dp.FMC,
            FPU_CPACR: dp.FPU_CPACR,
            FLASH: dp.FLASH,
            GPIOA: dp.GPIOA,
            GPIOB: dp.GPIOB,
            GPIOC: dp.GPIOC,
            GPIOD: dp.GPIOD,
            GPIOE: dp.GPIOE,
            GPIOF: dp.GPIOF,
            GPIOG: dp.GPIOG,
            GPIOH: dp.GPIOH,
            GPIOJ: dp.GPIOJ,
            GPIOK: dp.GPIOK,
            HSEM: dp.HSEM,
            I2C1: dp.I2C1,
            I2C2: dp.I2C2,
            I2C3: dp.I2C3,
            I2C4: dp.I2C4,
            IWDG1: dp.IWDG1,
            LPTIM1: dp.LPTIM1,
            LPTIM2: dp.LPTIM2,
            LPTIM3: dp.LPTIM3,
            LPTIM4: dp.LPTIM4,
            LPTIM5: dp.LPTIM5,
            LPUART1: dp.LPUART1,
            LTDC: dp.LTDC,
            MDIOS: dp.MDIOS,
            MDMA: dp.MDMA,
            NVIC_STIR: dp.NVIC_STIR,
            OCTOSPI1: dp.OCTOSPI1,
            OCTOSPI2: dp.OCTOSPI2,
            OPAMP: dp.OPAMP,
            OTG1_HS_DEVICE: dp.OTG1_HS_DEVICE,
            OTG2_HS_DEVICE: dp.OTG2_HS_DEVICE,
            OTG1_HS_GLOBAL: dp.OTG1_HS_GLOBAL,
            OTG1_HS_HOST: dp.OTG1_HS_HOST,
            OTG2_HS_HOST: dp.OTG2_HS_HOST,
            OTG1_HS_PWRCLK: dp.OTG1_HS_PWRCLK,
            OTG2_HS_PWRCLK: dp.OTG2_HS_PWRCLK,
            OCTOSPII_O_MANAGER: dp.OCTOSPII_O_MANAGER,
            PF: dp.PF,
            RAMECC1: dp.RAMECC1,
            RAMECC2: dp.RAMECC2,
            RAMECC3: dp.RAMECC3,
            RNG: dp.RNG,
            RTC: dp.RTC,
            SAI1: dp.SAI1,
            SAI4: dp.SAI4,
            SCB_ACTRL: dp.SCB_ACTRL,
            SDMMC1: dp.SDMMC1,
            SDMMC2: dp.SDMMC2,
            SPDIFRX: dp.SPDIFRX,
            SPI1: dp.SPI1,
            SPI2: dp.SPI2,
            SPI3: dp.SPI3,
            SPI4: dp.SPI4,
            SPI5: dp.SPI5,
            SPI6: dp.SPI6,
            STK: dp.STK,
            SWPMI: dp.SWPMI,
            SYSCFG: dp.SYSCFG,
            TIM1: dp.TIM1,
            TIM12: dp.TIM12,
            TIM2: dp.TIM2,
            TIM3: dp.TIM3,
            TIM4: dp.TIM4,
            TIM5: dp.TIM5,
            TIM6: dp.TIM6,
            TIM7: dp.TIM7,
            TIM8: dp.TIM8,
            TIM13: dp.TIM13,
            TIM14: dp.TIM14,
            TIM15: dp.TIM15,
            TIM16: dp.TIM16,
            TIM17: dp.TIM17,
            TIM23: dp.TIM23,
            TIM24: dp.TIM24,
            USART1: dp.USART1,
            USART2: dp.USART2,
            USART3: dp.USART3,
            UART4: dp.UART4,
            UART5: dp.UART5,
            USART6: dp.USART6,
            UART7: dp.UART7,
            UART8: dp.UART8,
            UART9: dp.UART9,
            USART10: dp.USART10,
            VREFBUF: dp.VREFBUF,
            WWDG1: dp.WWDG1,
            ADC2: dp.ADC2,
            BDMA: dp.BDMA,
            CORDIC: dp.CORDIC,
            DMA2: dp.DMA2,
            ETHERNET_DMA: dp.ETHERNET_DMA,
            ETHERNET_MTL: dp.ETHERNET_MTL,
        }
    }
}
