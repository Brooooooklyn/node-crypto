#include <node.h>
#include <iostream>
#include <nan.h>

using namespace v8;
using namespace std;
using namespace Nan;

extern "C" char* Node_Crypto_Get_V8_String_Unicode_Content(v8::Local<v8::Value> obj) {
  v8::String::Utf8Value str(obj);
  return *str;
}

extern "C" bool Node_Crypto_New_Buffer_From_Contents(v8::Local<v8::Object> *out, char* data, uint32_t size) {
  Nan::MaybeLocal<v8::Object> maybe = Nan::NewBuffer(data, size);
  if (!maybe.ToLocal(out)) {
    return false;
  }
  return true;
}
