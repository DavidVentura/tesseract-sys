#ifndef CUSTOM_CAPI_H_
#define CUSTOM_CAPI_H_

#include <tesseract/capi.h>

#ifdef __cplusplus
extern "C" {
#endif

TESS_API BOOL TessResultIteratorIsAtBeginningOf(const TessResultIterator *handle,
                                                TessPageIteratorLevel level);
TESS_API BOOL TessResultIteratorIsAtFinalElement(const TessResultIterator *handle,
                                                 TessPageIteratorLevel level,
                                                 TessPageIteratorLevel element);

#ifdef __cplusplus
}
#endif

#endif // CUSTOM_CAPI_H_