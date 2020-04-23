#ifndef BOARD_CUSTOM_H
#define BOARD_CUSTOM_H

#ifdef __cplusplus
extern "C" {
#endif

#include "nrf_gpio.h"

// LED definitions
#define LEDS_NUMBER    2

#define LED_1          17
#define LED_2          18

#define LEDS_ACTIVE_STATE 0

#define LEDS_LIST { LED_1, LED_2 }

// #define LEDS_INV_MASK  LEDS_MASK

#define BSP_LED_0      LED_1
#define BSP_LED_1      LED_2

#define BUTTONS_NUMBER 2

#define BUTTON_1       14
#define BUTTON_2       13
#define BUTTON_PULL    NRF_GPIO_PIN_PULLUP

#define BUTTONS_ACTIVE_STATE 0

#define BUTTONS_LIST { BUTTON_1, BUTTON_2 }

#define BSP_BUTTON_0   BUTTON_1
#define BSP_BUTTON_1   BUTTON_2

// #define BSP_SELF_PINRESET_PIN NRF_GPIO_PIN_MAP(0,19)

// #define HWFC           true

#ifdef __cplusplus
}
#endif

#endif // BOARD_CUSTOM_H
