#ifndef BLE_CUS_H__
#define BLE_CUS_H__

#include <stdint.h>
#include <stdbool.h>
#include "ble.h"
#include "ble_srv_common.h"
#include "nrf_sdh_ble.h"

#ifdef __cplusplus
extern "C" {
#endif

/**@brief   Macro for defining a ble_cus instance.
 *
 * @param   _name   Name of the instance.
 * @hideinitializer
 */
#define BLE_CUS_DEF(_name)                                                     \
static ble_cus_t _name;                                                        \
NRF_SDH_BLE_OBSERVER(_name ## _obs,                                            \
                     BLE_HRS_BLE_OBSERVER_PRIO,                                \
                     ble_cus_on_ble_evt, &_name)



// CUSTOM_SERVICE_UUID_BASE afdd71a9-dd10-4821-bc08-95e9b0360190

#define CUSTOM_SERVICE_UUID_BASE {0x90, 0x01, 0x36, 0xB0, 0xE9, 0x95, 0x08, 0xBC, \
                                  0x21, 0x48, 0x10, 0xDD, 0xA9, 0x71, 0xDD, 0xAF}

#define CUSTOM_SERVICE_UUID       0x1400
#define CUSTOM_VALUE_CHAR_UUID    0x1401

// Forward declaration of the ble_cus_t type.
typedef struct ble_cus_s ble_cus_t;

/**@brief Custom Service event handler type. */
typedef void (*ble_cus_evt_handler_t) (uint16_t conn_handle, ble_cus_t * p_cus, uint8_t new_state);

/**@brief Battery Service init structure. This contains all options and data needed for
 *        initialization of the service.*/
typedef struct
{
    ble_cus_evt_handler_t         evt_handler; /**< Event handler to be called for handling events in the Custom Service. */
} ble_cus_init_t;

/**@brief Custom Service structure. This contains various status information for the service. */
struct ble_cus_s
{
    uint16_t                    service_handle;       /**< Handle of Custom Service (as provided by the BLE stack). */
    ble_gatts_char_handles_t    custom_value_char_handles; /**< Handles related to the Custom Value characteristic. */
    uint8_t                     uuid_type;            /**< UUID type for the Custom Service. */
    ble_cus_evt_handler_t       evt_handler;          /**< Event handler to be called for handling events in the Custom Service. */
};

/**@brief Function for initializing the Custom Service.
 *
 * @param[out]  p_cus       Custom Service structure. This structure will have to be supplied by
 *                          the application. It will be initialized by this function, and will later
 *                          be used to identify this particular service instance.
 * @param[in]   p_cus_init  Information needed to initialize the service.
 *
 * @return      NRF_SUCCESS on successful initialization of service, otherwise an error code.
 */
uint32_t ble_cus_init(ble_cus_t * p_cus, const ble_cus_init_t * p_cus_init);

/**@brief Function for handling the Application's BLE Stack events.
 *
 * @details Handles all events from the BLE stack of interest to the Battery Service.
 *
 * @note 
 *
 * @param[in]   p_cus      Custom Service structure.
 * @param[in]   p_ble_evt  Event received from the BLE stack.
 */
void ble_cus_on_ble_evt(ble_evt_t const * p_ble_evt, void * p_context);


#ifdef __cplusplus
}
#endif

#endif // BLE_CUS_H__
