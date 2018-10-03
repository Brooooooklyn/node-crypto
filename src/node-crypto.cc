#include <node.h>
#include <iostream>

using namespace v8;
using namespace std;

extern "C" char* Node_Crypto_Get_V8_String_Unicode_Content(v8::Local<v8::Value> obj) {
  v8::String::Utf8Value str(obj);
  return *str;
}
