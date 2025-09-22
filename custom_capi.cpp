#include "custom_capi.h"
#include <tesseract/resultiterator.h>

BOOL TessResultIteratorIsAtBeginningOf(const TessResultIterator *handle,
                                       TessPageIteratorLevel level) {
  return static_cast<int>(handle->IsAtBeginningOf(level));
}

BOOL TessResultIteratorIsAtFinalElement(const TessResultIterator *handle,
                                        TessPageIteratorLevel level,
                                        TessPageIteratorLevel element) {
  return static_cast<int>(handle->IsAtFinalElement(level, element));
}