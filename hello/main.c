/* Copyright (c) Microsoft Corporation. All rights reserved.
   Licensed under the MIT License. */


#include <errno.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>
#include <signal.h>

#include <curl/curl.h>

#include <assert.h>

// applibs_versions.h defines the API struct versions to use for applibs APIs.
#include "applibs_versions.h"
#include <applibs/log.h>
#include <applibs/gpio.h>
#include <applibs/networking.h>
#include <applibs/storage.h>

#include "rust_hello.h"
#include <hw/mt3620_rdb.h>
#include <hw/sample_hardware.h>
#include "main.h"

static volatile sig_atomic_t terminationRequired = false;

/// <summary>
///     Signal handler for termination requests. This handler must be async-signal-safe.
/// </summary>
static void TerminationHandler(int signalNumber)
{
	// Don't use Log_Debug here, as it is not guaranteed to be async-signal-safe.
	terminationRequired = true;
}



/// <summary>
///     Main entry point for this sample.
/// </summary>
int main(int argc, char *argv[])
{
	int green = GPIO_OpenAsOutput(MT3620_RDB_LED1_GREEN, GPIO_OutputMode_PushPull, GPIO_Value_High);
	if (green < 0) {
		Log_Debug(
			"Error opening green GPIO: %s (%d). Check that app_manifest.json includes the GPIO used.\n",
			strerror(errno), errno);
		return -1;
	}

	int red = GPIO_OpenAsOutput(MT3620_RDB_LED1_RED, GPIO_OutputMode_PushPull, GPIO_Value_High);
	if (red < 0) {
		Log_Debug(
			"Error opening red GPIO: %s (%d). Check that app_manifest.json includes the GPIO used.\n",
			strerror(errno), errno);
		return -1;
	}

	blink(green);
	blink(red);
	blink(green);
	Log_Debug("Rust for Azure Sphere via C embedded proof of concept.\n");
	
	hello_world();
	blink(red);
	float keygen = 0;
	float sign = 0;
	int iters = 100;
	for (size_t i = 0; i < iters; i++)
	{
		Log_Debug("i: %d\n", i); 
		keygen += run_keygen_test();
		blink(green);
		sign += run_sign_test();
		blink(red);
	}
	
	Log_Debug("Application exiting... keygen: %.7f s sign: %.7f s \n", keygen/iters, sign/iters);
	blink(green);
	blink(red);
	return 0;
}

void blink(int fd)
{
	const struct timespec sleepTime = { 0, 100 * 1000 * 1000 }; // 0.1 sec
	GPIO_SetValue(fd, GPIO_Value_High);
	nanosleep(&sleepTime, NULL);
	GPIO_SetValue(fd, GPIO_Value_Low);
}
