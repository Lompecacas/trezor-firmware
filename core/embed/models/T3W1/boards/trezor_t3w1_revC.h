#pragma once

#define VDD_1V8 1
#define USE_SMPS 1

#define BTN_POWER_PIN GPIO_PIN_5
#define BTN_POWER_PORT GPIOE
#define BTN_POWER_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE
#define BTN_EXTI_INTERRUPT_GPIOSEL EXTI_GPIOE
#define BTN_EXTI_INTERRUPT_LINE EXTI_LINE_5
#define BTN_EXTI_INTERRUPT_PIN GPIO_PIN_5
#define BTN_EXTI_INTERRUPT_NUM EXTI5_IRQn
#define BTN_EXTI_INTERRUPT_HANDLER EXTI5_IRQHandler

#define DISPLAY_COLOR_MODE DMA2D_OUTPUT_ARGB8888
#define DISPLAY_PANEL_LX250A2401A
#define DISPLAY_GFXMMU 1
#define DISPLAY_RESET_PIN GPIO_PIN_2
#define DISPLAY_RESET_PORT GPIOE
#define DISPLAY_RESET_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE
#define DISPLAY_PWREN_PIN GPIO_PIN_1
#define DISPLAY_PWREN_PORT GPIOG
#define DISPLAY_PWREN_CLK_ENA __HAL_RCC_GPIOG_CLK_ENABLE

#define TPS61062_ILED_PIN GPIO_PIN_3
#define TPS61062_ILED_PORT GPIOE
#define TPS61062_ILED_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE
#define TPS61062_EN_PIN GPIO_PIN_9
#define TPS61062_EN_PORT GPIOB
#define TPS61062_EN_CLK_ENA __HAL_RCC_GPIOB_CLK_ENABLE

#define NPM1300_I2C_INSTANCE 0
#define NPM1300_INT_PIN GPIO_PIN_13
#define NPM1300_INT_PORT GPIOH
#define NPM1300_INT_PIN_CLK_ENA __HAL_RCC_GPIOH_CLK_ENABLE
#define NPM1300_EXTI_INTERRUPT_GPIOSEL EXTI_GPIOH
#define NPM1300_EXTI_INTERRUPT_LINE EXTI_LINE_13
#define NPM1300_EXTI_INTERRUPT_NUM EXTI13_IRQn
#define NPM1300_EXTI_INTERRUPT_HANDLER EXTI13_IRQHandler

#define STWLC38_I2C_INSTANCE 1
#define STWLC38_INT_PIN GPIO_PIN_15
#define STWLC38_INT_PORT GPIOG
#define STWLC38_INT_PIN_CLK_ENA __HAL_RCC_GPIOG_CLK_ENABLE
#define STWLC38_EXTI_INTERRUPT_GPIOSEL EXTI_GPIOG
#define STWLC38_EXTI_INTERRUPT_LINE EXTI_LINE_15
#define STWLC38_EXTI_INTERRUPT_NUM EXTI15_IRQn
#define STWLC38_EXTI_INTERRUPT_HANDLER EXTI15_IRQHandler
#define STWLC38_ENB_PIN GPIO_PIN_3
#define STWLC38_ENB_PORT GPIOD
#define STWLC38_ENB_PIN_CLK_ENA __HAL_RCC_GPIOD_CLK_ENABLE

#define I2C_COUNT 5

#define I2C_INSTANCE_0 I2C1
#define I2C_INSTANCE_0_CLK_EN __HAL_RCC_I2C1_CLK_ENABLE
#define I2C_INSTANCE_0_CLK_DIS __HAL_RCC_I2C1_CLK_DISABLE
#define I2C_INSTANCE_0_PIN_AF GPIO_AF4_I2C1
#define I2C_INSTANCE_0_SDA_PORT GPIOG
#define I2C_INSTANCE_0_SDA_PIN GPIO_PIN_13
#define I2C_INSTANCE_0_SDA_CLK_EN __HAL_RCC_GPIOG_CLK_ENABLE
#define I2C_INSTANCE_0_SCL_PORT GPIOG
#define I2C_INSTANCE_0_SCL_PIN GPIO_PIN_14
#define I2C_INSTANCE_0_SCL_CLK_EN __HAL_RCC_GPIOG_CLK_ENABLE
#define I2C_INSTANCE_0_RESET_REG &RCC->APB1RSTR1
#define I2C_INSTANCE_0_RESET_BIT RCC_APB1RSTR1_I2C1RST
#define I2C_INSTANCE_0_EV_IRQHandler I2C1_EV_IRQHandler
#define I2C_INSTANCE_0_ER_IRQHandler I2C1_ER_IRQHandler
#define I2C_INSTANCE_0_EV_IRQn I2C1_EV_IRQn
#define I2C_INSTANCE_0_ER_IRQn I2C1_ER_IRQn
#define I2C_INSTANCE_0_GUARD_TIME 0

#define I2C_INSTANCE_1 I2C2
#define I2C_INSTANCE_1_CLK_EN __HAL_RCC_I2C2_CLK_ENABLE
#define I2C_INSTANCE_1_CLK_DIS __HAL_RCC_I2C2_CLK_DISABLE
#define I2C_INSTANCE_1_PIN_AF GPIO_AF4_I2C2
#define I2C_INSTANCE_1_SDA_PORT GPIOF
#define I2C_INSTANCE_1_SDA_PIN GPIO_PIN_0
#define I2C_INSTANCE_1_SDA_CLK_EN __HAL_RCC_GPIOF_CLK_ENABLE
#define I2C_INSTANCE_1_SCL_PORT GPIOF
#define I2C_INSTANCE_1_SCL_PIN GPIO_PIN_1
#define I2C_INSTANCE_1_SCL_CLK_EN __HAL_RCC_GPIOF_CLK_ENABLE
#define I2C_INSTANCE_1_RESET_REG &RCC->APB1RSTR1
#define I2C_INSTANCE_1_RESET_BIT RCC_APB1RSTR1_I2C2RST
#define I2C_INSTANCE_1_EV_IRQHandler I2C2_EV_IRQHandler
#define I2C_INSTANCE_1_ER_IRQHandler I2C2_ER_IRQHandler
#define I2C_INSTANCE_1_EV_IRQn I2C2_EV_IRQn
#define I2C_INSTANCE_1_ER_IRQn I2C2_ER_IRQn
#define I2C_INSTANCE_1_GUARD_TIME 0

#define I2C_INSTANCE_2 I2C3
#define I2C_INSTANCE_2_CLK_EN __HAL_RCC_I2C3_CLK_ENABLE
#define I2C_INSTANCE_2_CLK_DIS __HAL_RCC_I2C3_CLK_DISABLE
#define I2C_INSTANCE_2_PIN_AF GPIO_AF4_I2C3
#define I2C_INSTANCE_2_SDA_PORT GPIOC
#define I2C_INSTANCE_2_SDA_PIN GPIO_PIN_1
#define I2C_INSTANCE_2_SDA_CLK_EN __HAL_RCC_GPIOC_CLK_ENABLE
#define I2C_INSTANCE_2_SCL_PORT GPIOC
#define I2C_INSTANCE_2_SCL_PIN GPIO_PIN_0
#define I2C_INSTANCE_2_SCL_CLK_EN __HAL_RCC_GPIOC_CLK_ENABLE
#define I2C_INSTANCE_2_RESET_REG &RCC->APB3RSTR
#define I2C_INSTANCE_2_RESET_BIT RCC_APB3RSTR_I2C3RST
#define I2C_INSTANCE_2_EV_IRQHandler I2C3_EV_IRQHandler
#define I2C_INSTANCE_2_ER_IRQHandler I2C3_ER_IRQHandler
#define I2C_INSTANCE_2_EV_IRQn I2C3_EV_IRQn
#define I2C_INSTANCE_2_ER_IRQn I2C3_ER_IRQn
#define I2C_INSTANCE_2_GUARD_TIME 0

#define I2C_INSTANCE_3 I2C4
#define I2C_INSTANCE_3_CLK_EN __HAL_RCC_I2C4_CLK_ENABLE
#define I2C_INSTANCE_3_CLK_DIS __HAL_RCC_I2C4_CLK_DISABLE
#define I2C_INSTANCE_3_PIN_AF GPIO_AF4_I2C4
#define I2C_INSTANCE_3_SDA_PORT GPIOD
#define I2C_INSTANCE_3_SDA_PIN GPIO_PIN_13
#define I2C_INSTANCE_3_SDA_CLK_EN __HAL_RCC_GPIOD_CLK_ENABLE
#define I2C_INSTANCE_3_SCL_PORT GPIOD
#define I2C_INSTANCE_3_SCL_PIN GPIO_PIN_12
#define I2C_INSTANCE_3_SCL_CLK_EN __HAL_RCC_GPIOD_CLK_ENABLE
#define I2C_INSTANCE_3_RESET_REG &RCC->APB1RSTR2
#define I2C_INSTANCE_3_RESET_BIT RCC_APB1RSTR2_I2C4RST
#define I2C_INSTANCE_3_EV_IRQHandler I2C4_EV_IRQHandler
#define I2C_INSTANCE_3_ER_IRQHandler I2C4_ER_IRQHandler
#define I2C_INSTANCE_3_EV_IRQn I2C4_EV_IRQn
#define I2C_INSTANCE_3_ER_IRQn I2C4_ER_IRQn
#define I2C_INSTANCE_3_GUARD_TIME 50
#define I2C_INSTANCE_3_GTZC_PERIPH GTZC_PERIPH_I2C4

#define I2C_INSTANCE_4 I2C5
#define I2C_INSTANCE_4_CLK_EN __HAL_RCC_I2C5_CLK_ENABLE
#define I2C_INSTANCE_4_CLK_DIS __HAL_RCC_I2C5_CLK_DISABLE
#define I2C_INSTANCE_4_PIN_AF GPIO_AF4_I2C5
#define I2C_INSTANCE_4_SDA_PORT GPIOD
#define I2C_INSTANCE_4_SDA_PIN GPIO_PIN_0
#define I2C_INSTANCE_4_SDA_CLK_EN __HAL_RCC_GPIOD_CLK_ENABLE
#define I2C_INSTANCE_4_SCL_PORT GPIOD
#define I2C_INSTANCE_4_SCL_PIN GPIO_PIN_1
#define I2C_INSTANCE_4_SCL_CLK_EN __HAL_RCC_GPIOD_CLK_ENABLE
#define I2C_INSTANCE_4_RESET_REG &RCC->APB1RSTR2
#define I2C_INSTANCE_4_RESET_BIT RCC_APB1RSTR2_I2C5RST
#define I2C_INSTANCE_4_EV_IRQHandler I2C5_EV_IRQHandler
#define I2C_INSTANCE_4_ER_IRQHandler I2C5_ER_IRQHandler
#define I2C_INSTANCE_4_EV_IRQn I2C5_EV_IRQn
#define I2C_INSTANCE_4_ER_IRQn I2C5_ER_IRQn
#define I2C_INSTANCE_4_GUARD_TIME 0

#define TOUCH_PANEL_LX250A2410A 1
#define TOUCH_SENSITIVITY 0x40
#define TOUCH_WAKEUP_WORKAROUND 1
#define TOUCH_I2C_INSTANCE 4
#define TOUCH_INT_PORT GPIOC
#define TOUCH_INT_PIN GPIO_PIN_3

#define DRV2625_I2C_INSTANCE 2
#define HAPTIC_ACTUATOR "actuators/ld0625bc.h"
#define DRV2625_TRIG_PIN GPIO_PIN_2
#define DRV2625_TRIG_PORT GPIOA
#define DRV2625_TRIG_CLK_ENA __HAL_RCC_GPIOA_CLK_ENABLE
#define DRV2625_TRIG_AF GPIO_AF14_TIM15
#define DRV2625_TRIG_TIM TIM15
#define DRV2625_TRIG_TIM_CLK_ENA __HAL_RCC_TIM15_CLK_ENABLE
#define DRV2625_TRIG_TIM_CLK_DIS __HAL_RCC_TIM15_CLK_DISABLE
#define DRV2625_TRIG_TIM_FORCE_RESET __HAL_RCC_TIM15_FORCE_RESET
#define DRV2625_TRIG_TIM_RELEASE_RESET __HAL_RCC_TIM15_RELEASE_RESET
#define DRV2625_RESET_PIN GPIO_PIN_3
#define DRV2625_RESET_PORT GPIOA
#define DRV2625_RESET_CLK_ENA __HAL_RCC_GPIOA_CLK_ENABLE

#define OPTIGA_I2C_INSTANCE 3
#define OPTIGA_RST_PORT GPIOD
#define OPTIGA_RST_PIN GPIO_PIN_10
#define OPTIGA_RST_CLK_EN __HAL_RCC_GPIOD_CLK_ENABLE
#define OPTIGA_PWR_PORT GPIOD
#define OPTIGA_PWR_PIN GPIO_PIN_14
#define OPTIGA_PWR_CLK_EN __HAL_RCC_GPIOD_CLK_ENABLE

#define TROPIC01_PWR_PORT GPIOB
#define TROPIC01_PWR_PIN GPIO_PIN_11
#define TROPIC01_PWR_CLK_EN __HAL_RCC_GPIOB_CLK_ENABLE
#define TROPIC01_INT_PORT GPIOB
#define TROPIC01_INT_PIN GPIO_PIN_11
#define TROPIC01_INT_CLK_EN __HAL_RCC_GPIOB_CLK_ENABLE
#define TROPIC01_SPI SPI2
#define TROPIC01_SPI_GTZC_PERIPH GTZC_PERIPH_SPI2
#define TROPIC01_SPI_CLK_EN __HAL_RCC_SPI2_CLK_ENABLE
#define TROPIC01_SPI_CLK_DIS __HAL_RCC_SPI2_CLK_DISABLE
#define TROPIC01_SPI_FORCE_RESET __HAL_RCC_SPI2_FORCE_RESET
#define TROPIC01_SPI_RELEASE_RESET __HAL_RCC_SPI2_RELEASE_RESET
#define TROPIC01_SPI_PIN_AF GPIO_AF5_SPI2
#define TROPIC01_SPI_SCK_PORT GPIOB
#define TROPIC01_SPI_SCK_PIN GPIO_PIN_13
#define TROPIC01_SPI_SCK_EN __HAL_RCC_GPIOB_CLK_ENABLE
#define TROPIC01_SPI_MISO_PORT GPIOB
#define TROPIC01_SPI_MISO_PIN GPIO_PIN_14
#define TROPIC01_SPI_MISO_EN __HAL_RCC_GPIOB_CLK_ENABLE
#define TROPIC01_SPI_MOSI_PORT GPIOB
#define TROPIC01_SPI_MOSI_PIN GPIO_PIN_15
#define TROPIC01_SPI_MOSI_EN __HAL_RCC_GPIOB_CLK_ENABLE
#define TROPIC01_SPI_NSS_PORT GPIOI
#define TROPIC01_SPI_NSS_PIN GPIO_PIN_0
#define TROPIC01_SPI_NSS_EN __HAL_RCC_GPIOI_CLK_ENABLE

#define SBU_1_PIN GPIO_PIN_8
#define SBU_1_PORT GPIOC
#define SBU_1_CLK_ENA __HAL_RCC_GPIOC_CLK_ENABLE
#define SBU_2_PIN GPIO_PIN_9
#define SBU_2_PORT GPIOC
#define SBU_2_CLK_ENA __HAL_RCC_GPIOC_CLK_ENABLE

#define NRF_IN_RESERVED_PIN GPIO_PIN_7
#define NRF_IN_RESERVED_PORT GPIOE
#define NRF_IN_RESERVED_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE
#define NRF_OUT_RESET_PIN GPIO_PIN_0
#define NRF_OUT_RESET_PORT GPIOG
#define NRF_OUT_RESET_CLK_ENA __HAL_RCC_GPIOG_CLK_ENABLE
#define NRF_OUT_STAY_IN_BLD_PIN GPIO_PIN_15
#define NRF_OUT_STAY_IN_BLD_PORT GPIOE
#define NRF_OUT_STAY_IN_BLD_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE
#define NRF_OUT_SPI_READY_PIN GPIO_PIN_13
#define NRF_OUT_SPI_READY_PORT GPIOE
#define NRF_OUT_SPI_READY_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE
#define NRF_IN_SPI_REQUEST_PIN GPIO_PIN_11
#define NRF_IN_SPI_REQUEST_PORT GPIOE
#define NRF_IN_SPI_REQUEST_CLK_ENA __HAL_RCC_GPIOE_CLK_ENABLE
#define NRF_EXTI_INTERRUPT_GPIOSEL EXTI_GPIOE
#define NRF_EXTI_INTERRUPT_LINE EXTI_LINE_11
#define NRF_EXTI_INTERRUPT_PIN GPIO_PIN_11
#define NRF_EXTI_INTERRUPT_NUM EXTI11_IRQn
#define NRF_EXTI_INTERRUPT_HANDLER EXTI11_IRQHandler

#define NFC_SPI_INSTANCE SPI3
#define NFC_SPI_PIN_AF GPIO_AF6_SPI3
#define NFC_SPI_CLK_EN __HAL_RCC_SPI3_CLK_ENABLE
#define NFC_SPI_CLK_DIS __HAL_RCC_SPI3_CLK_DISABLE
#define NFC_SPI_FORCE_RESET __HAL_RCC_SPI3_FORCE_RESET
#define NFC_SPI_RELEASE_RESET __HAL_RCC_SPI3_RELEASE_RESET
#define NFC_SPI_MISO_PORT GPIOB
#define NFC_SPI_MISO_PIN GPIO_PIN_4
#define NFC_SPI_MISO_CLK_EN __HAL_RCC_GPIOB_CLK_ENABLE
#define NFC_SPI_MOSI_PORT GPIOB
#define NFC_SPI_MOSI_PIN GPIO_PIN_5
#define NFC_SPI_MOSI_CLK_EN __HAL_RCC_GPIOB_CLK_ENABLE
#define NFC_SPI_SCK_PORT GPIOG
#define NFC_SPI_SCK_PIN GPIO_PIN_9
#define NFC_SPI_SCK_CLK_EN __HAL_RCC_GPIOG_CLK_ENABLE
#define NFC_SPI_NSS_PORT GPIOG
#define NFC_SPI_NSS_PIN GPIO_PIN_12
#define NFC_SPI_NSS_CLK_EN __HAL_RCC_GPIOG_CLK_ENABLE

#define NFC_INT_PIN GPIO_PIN_10
#define NFC_INT_PORT GPIOG
#define NFC_INT_PIN_CLK_ENA __HAL_RCC_GPIOG_CLK_ENABLE
#define NFC_EXTI_INTERRUPT_GPIOSEL EXTI_GPIOG
#define NFC_EXTI_INTERRUPT_LINE EXTI_LINE_10
#define NFC_EXTI_INTERRUPT_NUM EXTI10_IRQn
#define NFC_EXTI_INTERRUPT_HANDLER EXTI10_IRQHandler

#define HW_REVISION_PUPD GPIO_PULLDOWN
#define HW_REVISION_0_PIN GPIO_PIN_1
#define HW_REVISION_0_PORT GPIOI
#define HW_REVISION_0_CLOCK_ENABLE() __HAL_RCC_GPIOI_CLK_ENABLE()
#define HW_REVISION_1_PIN GPIO_PIN_2
#define HW_REVISION_1_PORT GPIOI
#define HW_REVISION_1_CLOCK_ENABLE() __HAL_RCC_GPIOI_CLK_ENABLE()
#define HW_REVISION_2_PIN GPIO_PIN_3
#define HW_REVISION_2_PORT GPIOI
#define HW_REVISION_2_CLOCK_ENABLE() __HAL_RCC_GPIOI_CLK_ENABLE()

#define TAMPER_INPUT_2 1
