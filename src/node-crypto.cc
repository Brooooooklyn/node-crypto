#include <node.h>
#include <iostream>
#include <nan.h>

using namespace v8;
using namespace std;
using namespace Nan;

typedef struct StringStruct {
  char* content;
  int length;
} StringStruct;

extern "C" StringStruct Node_Crypto_Get_V8_String_Unicode_Content(v8::Local<v8::Value> obj) {
  auto isolate = v8::Isolate::GetCurrent();
  v8::String::Utf8Value str(isolate, obj);
  StringStruct result = { *str, str.length() };
  return result;
}
