package handler

// #include "handler.h"
import "C"

import "unsafe"

type HandlerResponse struct {
  Status uint32
}

type HandlerRequest struct {
  Headers []string
  Params []string
}

// handler
var handler Handler = nil
func SetHandler(i Handler) {
  handler = i
}
type Handler interface {
  Handle(req HandlerRequest) HandlerResponse 
}
//export handler_handle
func HandlerHandle(req *C.handler_request_t, ret *C.handler_response_t) {
  defer C.handler_request_free(req)
  var lift_req HandlerRequest
  var lift_req_Headers []string
  lift_req_Headers = make([]string, req.headers.len)
  if req.headers.len > 0 {
    for lift_req_Headers_i := 0; lift_req_Headers_i < int(req.headers.len); lift_req_Headers_i++ {
      var empty_lift_req_Headers C.handler_string_t
      lift_req_Headers_ptr := *(*C.handler_string_t)(unsafe.Pointer(uintptr(unsafe.Pointer(req.headers.ptr)) +
      uintptr(lift_req_Headers_i)*unsafe.Sizeof(empty_lift_req_Headers)))
      var list_lift_req_Headers string
      list_lift_req_Headers = C.GoStringN(lift_req_Headers_ptr.ptr, C.int(lift_req_Headers_ptr.len))
      lift_req_Headers[lift_req_Headers_i] = list_lift_req_Headers
    }
  }
  lift_req.Headers = lift_req_Headers
  var lift_req_Params []string
  lift_req_Params = make([]string, req.params.len)
  if req.params.len > 0 {
    for lift_req_Params_i := 0; lift_req_Params_i < int(req.params.len); lift_req_Params_i++ {
      var empty_lift_req_Params C.handler_string_t
      lift_req_Params_ptr := *(*C.handler_string_t)(unsafe.Pointer(uintptr(unsafe.Pointer(req.params.ptr)) +
      uintptr(lift_req_Params_i)*unsafe.Sizeof(empty_lift_req_Params)))
      var list_lift_req_Params string
      list_lift_req_Params = C.GoStringN(lift_req_Params_ptr.ptr, C.int(lift_req_Params_ptr.len))
      lift_req_Params[lift_req_Params_i] = list_lift_req_Params
    }
  }
  lift_req.Params = lift_req_Params
  result := handler.Handle(lift_req)
  var lower_result C.handler_response_t
  lower_result_status := C.uint32_t(result.Status)
  lower_result.status = lower_result_status
  *ret = lower_result
  
}
