#ifndef MOTORS_H__
#define MOTORS_H__

#include <stdint.h>

#include "nrf_gpio.h"

#define MOTOR_DEF(_name)                \
static motor_t _name;


typedef struct motor_s motor_t;

typedef struct
{
    uint32_t in1;
    uint32_t in2;
    uint32_t pwm;
} motor_init_t;

struct motor_s
{
    uint32_t in1;
    uint32_t in2;
    uint32_t pwm;
};

void motor_init(motor_t * p_motor, const motor_init_t * p_motor_init);

void motor_enable(const motor_t * p_motor);

void motor_disable(const motor_t * p_motor);

void motor_forward(const motor_t * p_motor);

void motor_backward(const motor_t * p_motor);

void motor_off(const motor_t * p_motor);

#endif // MOTORS_H__
