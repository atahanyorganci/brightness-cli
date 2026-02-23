#ifndef WRAPPER_DISPLAY_SERVICES_H
#define WRAPPER_DISPLAY_SERVICES_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * DisplayServices private framework (macOS).
 * Path: /System/Library/PrivateFrameworks/DisplayServices.framework/DisplayServices
 *
 * Returns 0 on success, non-zero on failure.
 */
int DisplayServicesGetBrightness(int displayID, float *brightness);
int DisplayServicesSetBrightness(int displayID, float brightness);

#ifdef __cplusplus
}
#endif

#endif /* WRAPPER_DISPLAY_SERVICES_H */
