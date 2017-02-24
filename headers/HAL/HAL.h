/*----------------------------------------------------------------------------*/
/* Copyright (c) FIRST 2013-2017. All Rights Reserved.                        */
/* Open Source Software - may be modified and shared by FRC teams. The code   */
/* must be accompanied by the FIRST BSD license file in the root directory of */
/* the project.                                                               */
/*----------------------------------------------------------------------------*/

#pragma once

#include <stdint.h>

#ifndef HAL_USE_LABVIEW

#include "Accelerometer.h"
#include "AnalogAccumulator.h"
#include "AnalogGyro.h"
#include "AnalogInput.h"
#include "AnalogOutput.h"
#include "AnalogTrigger.h"
#include "Compressor.h"
#include "Constants.h"
#include "Counter.h"
#include "DIO.h"
#include "DriverStation.h"
#include "Errors.h"
#include "I2C.h"
#include "Interrupts.h"
#include "Notifier.h"
#include "PDP.h"
#include "PWM.h"
#include "Ports.h"
#include "Power.h"
#include "Relay.h"
#include "SPI.h"
#include "SerialPort.h"
#include "Solenoid.h"

#endif  // HAL_USE_LABVIEW

// #include "FRC_NetworkCommunication/UsageReporting.h"
#include "Types.h"

enum HAL_RuntimeType { HAL_Athena, HAL_Mock };

#ifdef __cplusplus
extern "C" {
#endif

const char* HAL_GetErrorMessage(int32_t code);

int32_t HAL_GetFPGAVersion(int32_t* status);
int64_t HAL_GetFPGARevision(int32_t* status);

enum HAL_RuntimeType HAL_GetRuntimeType();
HAL_Bool HAL_GetFPGAButton(int32_t* status);

HAL_Bool HAL_GetSystemActive(int32_t* status);
HAL_Bool HAL_GetBrownedOut(int32_t* status);

void HAL_BaseInitialize(int32_t* status);

#ifndef HAL_USE_LABVIEW

HAL_PortHandle HAL_GetPort(int32_t channel);
HAL_PortHandle HAL_GetPortWithModule(int32_t module, int32_t channel);

uint64_t HAL_GetFPGATime(int32_t* status);

int32_t HAL_Initialize(int32_t mode);

// ifdef's definition is to allow for default parameters in C++.
#ifdef __cplusplus
int64_t HAL_Report(int32_t resource, int32_t instanceNumber,
                   int32_t context = 0, const char* feature = nullptr);
#else
int64_t HAL_Report(int32_t resource, int32_t instanceNumber, int32_t context,
                   const char* feature);
#endif

#endif  // HAL_USE_LABVIEW
#ifdef __cplusplus
}
#endif
