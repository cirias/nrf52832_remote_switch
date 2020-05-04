#include "motors.h"

void motor_init(motor_t * p_motor, const motor_init_t * p_motor_init)
{
    p_motor->in1 = p_motor_init->in1;
    p_motor->in2 = p_motor_init->in2;
    p_motor->pwm = p_motor_init->pwm;
}

void motor_enable(const motor_t * p_motor)
{
    nrf_gpio_cfg_output(p_motor->in1);
    nrf_gpio_cfg_output(p_motor->in2);
    nrf_gpio_cfg_output(p_motor->pwm);
}

void motor_disable(const motor_t * p_motor)
{
    nrf_gpio_cfg_default(p_motor->pwm);
    nrf_gpio_cfg_default(p_motor->in1);
    nrf_gpio_cfg_default(p_motor->in2);
}

void motor_forward(const motor_t * p_motor)
{
    nrf_gpio_pin_write(p_motor->in1, 1);
    nrf_gpio_pin_write(p_motor->in2, 0);
    nrf_gpio_pin_write(p_motor->pwm, 1);
}

void motor_backward(const motor_t * p_motor)
{
    nrf_gpio_pin_write(p_motor->in1, 0);
    nrf_gpio_pin_write(p_motor->in2, 1);
    nrf_gpio_pin_write(p_motor->pwm, 1);
}

void motor_off(const motor_t * p_motor)
{
    nrf_gpio_pin_write(p_motor->pwm, 0);
}
