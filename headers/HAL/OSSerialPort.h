/*----------------------------------------------------------------------------*/
/* Copyright (c) FIRST 2016-2017. All Rights Reserved.                        */
/* Open Source Software - may be modified and shared by FRC teams. The code   */
/* must be accompanied by the FIRST BSD license file in the root directory of */
/* the project.                                                               */
/*----------------------------------------------------------------------------*/

#pragma once

#include <stdint.h>

#include "SerialPort.h"

#ifdef __cplusplus
extern "C" {
#endif

void HAL_InitializeOSSerialPort(enum HAL_SerialPort port, int32_t* status);
void HAL_SetOSSerialBaudRate(enum HAL_SerialPort port, int32_t baud,
                             int32_t* status);
void HAL_SetOSSerialDataBits(enum HAL_SerialPort port, int32_t bits,
                             int32_t* status);
void HAL_SetOSSerialParity(enum HAL_SerialPort port, int32_t parity,
                           int32_t* status);
void HAL_SetOSSerialStopBits(enum HAL_SerialPort port, int32_t stopBits,
                             int32_t* status);
void HAL_SetOSSerialWriteMode(enum HAL_SerialPort port, int32_t mode,
                              int32_t* status);
void HAL_SetOSSerialFlowControl(enum HAL_SerialPort port, int32_t flow,
                                int32_t* status);
void HAL_SetOSSerialTimeout(enum HAL_SerialPort port, double timeout,
                            int32_t* status);
void HAL_EnableOSSerialTermination(enum HAL_SerialPort port, char terminator,
                                   int32_t* status);
void HAL_DisableOSSerialTermination(enum HAL_SerialPort port, int32_t* status);
void HAL_SetOSSerialReadBufferSize(enum HAL_SerialPort port, int32_t size,
                                   int32_t* status);
void HAL_SetOSSerialWriteBufferSize(enum HAL_SerialPort port, int32_t size,
                                    int32_t* status);
int32_t HAL_GetOSSerialBytesReceived(enum HAL_SerialPort port, int32_t* status);
int32_t HAL_ReadOSSerial(enum HAL_SerialPort port, char* buffer, int32_t count,
                         int32_t* status);
int32_t HAL_WriteOSSerial(enum HAL_SerialPort port, const char* buffer,
                          int32_t count, int32_t* status);
void HAL_FlushOSSerial(enum HAL_SerialPort port, int32_t* status);
void HAL_ClearOSSerial(enum HAL_SerialPort port, int32_t* status);
void HAL_CloseOSSerial(enum HAL_SerialPort port, int32_t* status);
#ifdef __cplusplus
}
#endif
