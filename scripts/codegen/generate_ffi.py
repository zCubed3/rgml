#!/bin/python

#
# Generates FFI bindings for various types
#

from enum import Enum
import datetime


class FFIMethodConvention(Enum):
    INSTANCE = 0
    INSTANCE_INSTANCE = 1
    OPERATOR = 2
    CUSTOM = 3


class FFIMethod:
    rust_name: str = ""
    ffi_name: str = ""
    convention: FFIMethodConvention = FFIMethodConvention.INSTANCE
    input_type: str = ""
    output_type: str = ""

    input_alt: str = ""
    input_alt_use: bool = False
    call_alt: str = ""
    call_alt_use: bool = False

    def __init__(self, rust_name: str, ffi_name: str, calling_type: FFIMethodConvention, input_type: str,
                 output_type: str):
        self.rust_name = rust_name
        self.ffi_name = ffi_name
        self.convention = calling_type
        self.input_type = input_type
        self.output_type = output_type

    def set_input_alt(self, alt: str):
        self.input_alt = alt
        self.input_alt_use = True

        return self

    def set_call_alt(self, alt: str):
        self.call_alt = alt
        self.call_alt_use = True

        return self

    def get_inputs(self) -> str:
        if self.input_alt_use:
            return self.input_alt.format(RUST_NAME=self.rust_name)

        if self.convention == FFIMethodConvention.INSTANCE:
            return f"i: %s" % self.input_type
        elif self.convention == FFIMethodConvention.INSTANCE_INSTANCE:
            return f"lhs: %s, rhs: %s" % (self.input_type, self.input_type)
        elif self.convention == FFIMethodConvention.OPERATOR:
            return f"lhs: %s, rhs: %s" % (self.input_type, self.input_type)

    def get_call(self) -> str:
        if self.call_alt_use:
            return self.call_alt.format(RUST_NAME=self.rust_name)

        if self.convention == FFIMethodConvention.INSTANCE:
            return f"i.%s()" % self.rust_name
        elif self.convention == FFIMethodConvention.INSTANCE_INSTANCE:
            return f"lhs.%s(rhs)" % self.rust_name
        elif self.convention == FFIMethodConvention.OPERATOR:
            return f"lhs %s rhs" % self.rust_name

    def emit_func(self, prefix: str = "") -> str:
        return f'#[no_mangle]\npub extern "C" fn %s%s(%s) -> %s {{\n\treturn %s;\n}}\n' % \
               (prefix, self.ffi_name, self.get_inputs(), self.output_type, self.get_call())


class FFIBind:
    rust_name: str = ""
    ffi_name: str = ""

    def __init__(self, rust_name: str, ffi_name: str, methods: list[FFIMethod]):
        self.rust_name = rust_name
        self.ffi_name = ffi_name
        self.methods = methods


bindings: list[FFIBind] = [
    # Single precision vectors
    FFIBind("Vector2F32", "vec2", [
        FFIMethod("+", "add", FFIMethodConvention.OPERATOR, "Vector2F32", "Vector2F32"),
        FFIMethod("-", "sub", FFIMethodConvention.OPERATOR, "Vector2F32", "Vector2F32"),
        FFIMethod("*", "mul", FFIMethodConvention.OPERATOR, "Vector2F32", "Vector2F32"),
        FFIMethod("/", "div", FFIMethodConvention.OPERATOR, "Vector2F32", "Vector2F32"),
        FFIMethod("normalize", "normalize", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("abs", "abs", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("floor", "floor", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("ceil", "ceil", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("round", "round", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("saturate", "saturate", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("sqrt", "sqrt", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("sin", "sin", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("cos", "cos", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("tan", "tan", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("sign", "sign", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("fract", "fract", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("sum", "sum", FFIMethodConvention.INSTANCE, "Vector2F32", "f32"),
        FFIMethod("magnitude", "magnitude", FFIMethodConvention.INSTANCE, "Vector2F32", "f32"),
        FFIMethod("magnitude_sqr", "magnitude_sqr", FFIMethodConvention.INSTANCE, "Vector2F32", "f32"),
        FFIMethod("min", "min", FFIMethodConvention.INSTANCE_INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("max", "max", FFIMethodConvention.INSTANCE_INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("pow", "pow", FFIMethodConvention.INSTANCE_INSTANCE, "Vector2F32", "Vector2F32"),
        FFIMethod("dot", "dot", FFIMethodConvention.INSTANCE_INSTANCE, "Vector2F32", "f32"),
        FFIMethod("lerp", "lerp", FFIMethodConvention.CUSTOM, "Vector2F32", "Vector2F32")
            .set_input_alt("from: Vector2F32, to: Vector2F32, alpha: f32")
            .set_call_alt("from.lerp(to, alpha)"),
        FFIMethod("new", "new", FFIMethodConvention.CUSTOM, "Vector2F32", "Vector2F32")
            .set_input_alt("x: f32, y: f32")
            .set_call_alt("Vector2F32::new(x, y)"),
        FFIMethod("scalar", "scalar", FFIMethodConvention.CUSTOM, "Vector2F32", "Vector2F32")
            .set_input_alt("x: f32")
            .set_call_alt("Vector2F32::from_scalar(x)"),
        FFIMethod("default", "default", FFIMethodConvention.CUSTOM, "Vector2F32", "Vector2F32")
            .set_input_alt("")
            .set_call_alt("Vector2F32::default()"),
        FFIMethod("from", "from_vec3", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector2F32")
            .set_call_alt("Vector2F32::from(i)"),
        FFIMethod("from", "from_vec4", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector2F32")
            .set_call_alt("Vector2F32::from(i)"),
    ]),
    FFIBind("Vector3F32", "vec3", [
        FFIMethod("+", "add", FFIMethodConvention.OPERATOR, "Vector3F32", "Vector3F32"),
        FFIMethod("-", "sub", FFIMethodConvention.OPERATOR, "Vector3F32", "Vector3F32"),
        FFIMethod("*", "mul", FFIMethodConvention.OPERATOR, "Vector3F32", "Vector3F32"),
        FFIMethod("/", "div", FFIMethodConvention.OPERATOR, "Vector3F32", "Vector3F32"),
        FFIMethod("normalize", "normalize", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("abs", "abs", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("floor", "floor", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("ceil", "ceil", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("round", "round", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("saturate", "saturate", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("sqrt", "sqrt", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("sin", "sin", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("cos", "cos", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("tan", "tan", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("sign", "sign", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("fract", "fract", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("sum", "sum", FFIMethodConvention.INSTANCE, "Vector3F32", "f32"),
        FFIMethod("magnitude", "magnitude", FFIMethodConvention.INSTANCE, "Vector3F32", "f32"),
        FFIMethod("magnitude_sqr", "magnitude_sqr", FFIMethodConvention.INSTANCE, "Vector3F32", "f32"),
        FFIMethod("min", "min", FFIMethodConvention.INSTANCE_INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("max", "max", FFIMethodConvention.INSTANCE_INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("pow", "pow", FFIMethodConvention.INSTANCE_INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("cross", "cross", FFIMethodConvention.INSTANCE_INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("reflect", "reflect", FFIMethodConvention.INSTANCE_INSTANCE, "Vector3F32", "Vector3F32"),
        FFIMethod("dot", "dot", FFIMethodConvention.INSTANCE_INSTANCE, "Vector3F32", "f32"),
        FFIMethod("lerp", "lerp", FFIMethodConvention.CUSTOM, "Vector3F32", "Vector3F32")
            .set_input_alt("from: Vector3F32, to: Vector3F32, alpha: f32")
            .set_call_alt("from.lerp(to, alpha)"),
        FFIMethod("new", "new", FFIMethodConvention.CUSTOM, "Vector3F32", "Vector3F32")
            .set_input_alt("x: f32, y: f32, z: f32")
            .set_call_alt("Vector3F32::new(x, y, z)"),
        FFIMethod("scalar", "scalar", FFIMethodConvention.CUSTOM, "Vector3F32", "Vector3F32")
            .set_input_alt("x: f32")
            .set_call_alt("Vector3F32::from_scalar(x)"),
        FFIMethod("default", "default", FFIMethodConvention.CUSTOM, "Vector3F32", "Vector3F32")
            .set_input_alt("")
            .set_call_alt("Vector3F32::default()"),
        FFIMethod("from", "from_vec2", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector3F32")
            .set_call_alt("Vector3F32::from(i)"),
        FFIMethod("from", "from_vec4", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector3F32")
            .set_call_alt("Vector3F32::from(i)"),
    ]),
    FFIBind("Vector4F32", "vec4", [
        FFIMethod("+", "add", FFIMethodConvention.OPERATOR, "Vector4F32", "Vector4F32"),
        FFIMethod("-", "sub", FFIMethodConvention.OPERATOR, "Vector4F32", "Vector4F32"),
        FFIMethod("*", "mul", FFIMethodConvention.OPERATOR, "Vector4F32", "Vector4F32"),
        FFIMethod("/", "div", FFIMethodConvention.OPERATOR, "Vector4F32", "Vector4F32"),
        FFIMethod("*", "mul_mat4", FFIMethodConvention.OPERATOR, "Vector4F32", "Vector4F32")
            .set_input_alt("lhs: Vector4F32, rhs: Matrix4x4F32"),
        FFIMethod("normalize", "normalize", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("abs", "abs", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("floor", "floor", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("ceil", "ceil", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("round", "round", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("saturate", "saturate", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("sqrt", "sqrt", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("sin", "sin", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("cos", "cos", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("tan", "tan", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("sign", "sign", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("fract", "fract", FFIMethodConvention.INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("sum", "sum", FFIMethodConvention.INSTANCE, "Vector4F32", "f32"),
        FFIMethod("magnitude", "magnitude", FFIMethodConvention.INSTANCE, "Vector4F32", "f32"),
        FFIMethod("magnitude_sqr", "magnitude_sqr", FFIMethodConvention.INSTANCE, "Vector4F32", "f32"),
        FFIMethod("min", "min", FFIMethodConvention.INSTANCE_INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("max", "max", FFIMethodConvention.INSTANCE_INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("pow", "pow", FFIMethodConvention.INSTANCE_INSTANCE, "Vector4F32", "Vector4F32"),
        FFIMethod("dot", "dot", FFIMethodConvention.INSTANCE_INSTANCE, "Vector4F32", "f32"),
        FFIMethod("lerp", "lerp", FFIMethodConvention.CUSTOM, "Vector4F32", "Vector4F32")
            .set_input_alt("from: Vector4F32, to: Vector4F32, alpha: f32")
            .set_call_alt("from.lerp(to, alpha)"),
        FFIMethod("identity", "identity", FFIMethodConvention.CUSTOM, "Vector4F32", "Vector4F32")
            .set_input_alt("")
            .set_call_alt("Vector4F32::identity()"),
        FFIMethod("new", "new", FFIMethodConvention.CUSTOM, "Vector4F32", "Vector4F32")
            .set_input_alt("x: f32, y: f32, z: f32, w: f32")
            .set_call_alt("Vector4F32::new(x, y, z, w)"),
        FFIMethod("scalar", "scalar", FFIMethodConvention.CUSTOM, "Vector4F32", "Vector4F32")
            .set_input_alt("x: f32")
            .set_call_alt("Vector4F32::from_scalar(x)"),
        FFIMethod("default", "default", FFIMethodConvention.CUSTOM, "Vector4F32", "Vector4F32")
            .set_input_alt("")
            .set_call_alt("Vector4F32::default()"),
        FFIMethod("from", "from_vec2", FFIMethodConvention.INSTANCE, "Vector2F32", "Vector4F32")
            .set_call_alt("Vector4F32::from(i)"),
        FFIMethod("from", "from_vec3", FFIMethodConvention.INSTANCE, "Vector3F32", "Vector4F32")
            .set_call_alt("Vector4F32::from(i)"),
    ]),

    # Double precision vectors
    FFIBind("Vector2F64", "dvec2", [
        FFIMethod("+", "add", FFIMethodConvention.OPERATOR, "Vector2F64", "Vector2F64"),
        FFIMethod("-", "sub", FFIMethodConvention.OPERATOR, "Vector2F64", "Vector2F64"),
        FFIMethod("*", "mul", FFIMethodConvention.OPERATOR, "Vector2F64", "Vector2F64"),
        FFIMethod("/", "div", FFIMethodConvention.OPERATOR, "Vector2F64", "Vector2F64"),
        FFIMethod("normalize", "normalize", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("abs", "abs", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("floor", "floor", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("ceil", "ceil", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("round", "round", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("saturate", "saturate", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("sqrt", "sqrt", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("sin", "sin", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("cos", "cos", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("tan", "tan", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("sign", "sign", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("fract", "fract", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("sum", "sum", FFIMethodConvention.INSTANCE, "Vector2F64", "f64"),
        FFIMethod("magnitude", "magnitude", FFIMethodConvention.INSTANCE, "Vector2F64", "f64"),
        FFIMethod("magnitude_sqr", "magnitude_sqr", FFIMethodConvention.INSTANCE, "Vector2F64", "f64"),
        FFIMethod("min", "min", FFIMethodConvention.INSTANCE_INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("max", "max", FFIMethodConvention.INSTANCE_INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("pow", "pow", FFIMethodConvention.INSTANCE_INSTANCE, "Vector2F64", "Vector2F64"),
        FFIMethod("dot", "dot", FFIMethodConvention.INSTANCE_INSTANCE, "Vector2F64", "f64"),
        FFIMethod("lerp", "lerp", FFIMethodConvention.CUSTOM, "Vector2F64", "Vector2F64")
            .set_input_alt("from: Vector2F64, to: Vector2F64, alpha: f64")
            .set_call_alt("from.lerp(to, alpha)"),
        FFIMethod("new", "new", FFIMethodConvention.CUSTOM, "Vector2F64", "Vector2F64")
            .set_input_alt("x: f64, y: f64")
            .set_call_alt("Vector2F64::new(x, y)"),
        FFIMethod("scalar", "scalar", FFIMethodConvention.CUSTOM, "Vector2F64", "Vector2F64")
            .set_input_alt("x: f64")
            .set_call_alt("Vector2F64::from_scalar(x)"),
        FFIMethod("default", "default", FFIMethodConvention.CUSTOM, "Vector2F64", "Vector2F64")
            .set_input_alt("")
            .set_call_alt("Vector2F64::default()"),
        FFIMethod("from", "from_vec3", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector2F64")
            .set_call_alt("Vector2F64::from(i)"),
        FFIMethod("from", "from_vec4", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector2F64")
            .set_call_alt("Vector2F64::from(i)"),
    ]),
    FFIBind("Vector3F64", "dvec3", [
        FFIMethod("+", "add", FFIMethodConvention.OPERATOR, "Vector3F64", "Vector3F64"),
        FFIMethod("-", "sub", FFIMethodConvention.OPERATOR, "Vector3F64", "Vector3F64"),
        FFIMethod("*", "mul", FFIMethodConvention.OPERATOR, "Vector3F64", "Vector3F64"),
        FFIMethod("/", "div", FFIMethodConvention.OPERATOR, "Vector3F64", "Vector3F64"),
        FFIMethod("normalize", "normalize", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("abs", "abs", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("floor", "floor", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("ceil", "ceil", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("round", "round", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("saturate", "saturate", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("sqrt", "sqrt", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("sin", "sin", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("cos", "cos", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("tan", "tan", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("sign", "sign", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("fract", "fract", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("sum", "sum", FFIMethodConvention.INSTANCE, "Vector3F64", "f64"),
        FFIMethod("magnitude", "magnitude", FFIMethodConvention.INSTANCE, "Vector3F64", "f64"),
        FFIMethod("magnitude_sqr", "magnitude_sqr", FFIMethodConvention.INSTANCE, "Vector3F64", "f64"),
        FFIMethod("min", "min", FFIMethodConvention.INSTANCE_INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("max", "max", FFIMethodConvention.INSTANCE_INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("pow", "pow", FFIMethodConvention.INSTANCE_INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("cross", "cross", FFIMethodConvention.INSTANCE_INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("reflect", "reflect", FFIMethodConvention.INSTANCE_INSTANCE, "Vector3F64", "Vector3F64"),
        FFIMethod("dot", "dot", FFIMethodConvention.INSTANCE_INSTANCE, "Vector3F64", "f64"),
        FFIMethod("lerp", "lerp", FFIMethodConvention.CUSTOM, "Vector3F64", "Vector3F64")
            .set_input_alt("from: Vector3F64, to: Vector3F64, alpha: f64")
            .set_call_alt("from.lerp(to, alpha)"),
        FFIMethod("new", "new", FFIMethodConvention.CUSTOM, "Vector3F64", "Vector3F64")
            .set_input_alt("x: f64, y: f64, z: f64")
            .set_call_alt("Vector3F64::new(x, y, z)"),
        FFIMethod("scalar", "scalar", FFIMethodConvention.CUSTOM, "Vector3F64", "Vector3F64")
            .set_input_alt("x: f64")
            .set_call_alt("Vector3F64::from_scalar(x)"),
        FFIMethod("default", "default", FFIMethodConvention.CUSTOM, "Vector3F64", "Vector3F64")
            .set_input_alt("")
            .set_call_alt("Vector3F64::default()"),
        FFIMethod("from", "from_vec2", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector3F64")
            .set_call_alt("Vector3F64::from(i)"),
        FFIMethod("from", "from_vec4", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector3F64")
            .set_call_alt("Vector3F64::from(i)"),
    ]),
    FFIBind("Vector4F64", "dvec4", [
        FFIMethod("+", "add", FFIMethodConvention.OPERATOR, "Vector4F64", "Vector4F64"),
        FFIMethod("-", "sub", FFIMethodConvention.OPERATOR, "Vector4F64", "Vector4F64"),
        FFIMethod("*", "mul", FFIMethodConvention.OPERATOR, "Vector4F64", "Vector4F64"),
        FFIMethod("/", "div", FFIMethodConvention.OPERATOR, "Vector4F64", "Vector4F64"),
        FFIMethod("*", "mul_mat4", FFIMethodConvention.OPERATOR, "Vector4F64", "Vector4F64")
            .set_input_alt("lhs: Vector4F64, rhs: Matrix4x4F64"),
        FFIMethod("normalize", "normalize", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("abs", "abs", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("floor", "floor", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("ceil", "ceil", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("round", "round", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("saturate", "saturate", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("sqrt", "sqrt", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("sin", "sin", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("cos", "cos", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("tan", "tan", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("sign", "sign", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("fract", "fract", FFIMethodConvention.INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("sum", "sum", FFIMethodConvention.INSTANCE, "Vector4F64", "f64"),
        FFIMethod("magnitude", "magnitude", FFIMethodConvention.INSTANCE, "Vector4F64", "f64"),
        FFIMethod("magnitude_sqr", "magnitude_sqr", FFIMethodConvention.INSTANCE, "Vector4F64", "f64"),
        FFIMethod("min", "min", FFIMethodConvention.INSTANCE_INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("max", "max", FFIMethodConvention.INSTANCE_INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("pow", "pow", FFIMethodConvention.INSTANCE_INSTANCE, "Vector4F64", "Vector4F64"),
        FFIMethod("dot", "dot", FFIMethodConvention.INSTANCE_INSTANCE, "Vector4F64", "f64"),
        FFIMethod("lerp", "lerp", FFIMethodConvention.CUSTOM, "Vector4F64", "Vector4F64")
            .set_input_alt("from: Vector4F64, to: Vector4F64, alpha: f64")
            .set_call_alt("from.lerp(to, alpha)"),
        FFIMethod("identity", "identity", FFIMethodConvention.CUSTOM, "Vector4F64", "Vector4F64")
            .set_input_alt("")
            .set_call_alt("Vector4F64::identity()"),
        FFIMethod("new", "new", FFIMethodConvention.CUSTOM, "Vector4F64", "Vector4F64")
            .set_input_alt("x: f64, y: f64, z: f64, w: f64")
            .set_call_alt("Vector4F64::new(x, y, z, w)"),
        FFIMethod("scalar", "scalar", FFIMethodConvention.CUSTOM, "Vector4F64", "Vector4F64")
            .set_input_alt("x: f64")
            .set_call_alt("Vector4F64::from_scalar(x)"),
        FFIMethod("default", "default", FFIMethodConvention.CUSTOM, "Vector4F64", "Vector4F64")
            .set_input_alt("")
            .set_call_alt("Vector4F64::default()"),
        FFIMethod("from", "from_vec2", FFIMethodConvention.INSTANCE, "Vector2F64", "Vector4F64")
            .set_call_alt("Vector4F64::from(i)"),
        FFIMethod("from", "from_vec3", FFIMethodConvention.INSTANCE, "Vector3F64", "Vector4F64")
            .set_call_alt("Vector4F64::from(i)"),
    ]),

    # Single precision matrices
    FFIBind("Matrix4x4F32", "mat4", [
        FFIMethod("*", "mul", FFIMethodConvention.OPERATOR, "Matrix4x4F32", "Matrix4x4F32"),
        FFIMethod("*", "mul_vec4", FFIMethodConvention.OPERATOR, "Matrix4x4F32", "Vector4F32")
            .set_input_alt("lhs: Matrix4x4F32, rhs: Vector4F32"),
        FFIMethod("inverse", "inverse", FFIMethodConvention.INSTANCE, "Matrix4x4F32", "Matrix4x4F32"),
        FFIMethod("transpose", "transpose", FFIMethodConvention.INSTANCE, "Matrix4x4F32", "Matrix4x4F32"),
        FFIMethod("translation", "translation", FFIMethodConvention.INSTANCE, "Matrix4x4F32", "Matrix4x4F32")
            .set_input_alt("vector: Vector3F32")
            .set_call_alt("Matrix4x4F32::translation(vector)"),
        FFIMethod("rotation", "rotation", FFIMethodConvention.INSTANCE, "Matrix4x4F32", "Matrix4x4F32")
            .set_input_alt("euler: Vector3F32")
            .set_call_alt("Matrix4x4F32::rotation(euler)"),
        FFIMethod("rotate_x", "rotate_x", FFIMethodConvention.INSTANCE, "Matrix4x4F32", "Matrix4x4F32")
            .set_input_alt("degrees: f32")
            .set_call_alt("Matrix4x4F32::rotate_x(degrees)"),
        FFIMethod("rotate_y", "rotate_y", FFIMethodConvention.INSTANCE, "Matrix4x4F32", "Matrix4x4F32")
            .set_input_alt("degrees: f32")
            .set_call_alt("Matrix4x4F32::rotate_y(degrees)"),
        FFIMethod("rotate_z", "rotate_z", FFIMethodConvention.INSTANCE, "Matrix4x4F32", "Matrix4x4F32")
            .set_input_alt("degrees: f32")
            .set_call_alt("Matrix4x4F32::rotate_z(degrees)"),
        FFIMethod("perspective", "perspective", FFIMethodConvention.INSTANCE, "Matrix4x4F32", "Matrix4x4F32")
            .set_input_alt("fov_y: f32, aspect: f32, z_near: f32, z_far: f32")
            .set_call_alt("Matrix4x4F32::perspective(fov_y, aspect, z_near, z_far)"),
        FFIMethod("default", "default", FFIMethodConvention.CUSTOM, "Matrix4x4F32", "Matrix4x4F32")
            .set_input_alt("")
            .set_call_alt("Matrix4x4F32::default()"),
        FFIMethod("identity", "identity", FFIMethodConvention.CUSTOM, "Matrix4x4F32", "Matrix4x4F32")
            .set_input_alt("")
            .set_call_alt("Matrix4x4F32::identity()"),
    ]),

    # Double precision matrices
    FFIBind("Matrix4x4F64", "dmat4", [
        FFIMethod("*", "mul", FFIMethodConvention.OPERATOR, "Matrix4x4F64", "Matrix4x4F64"),
        FFIMethod("*", "mul_vec4", FFIMethodConvention.OPERATOR, "Matrix4x4F64", "Vector4F64")
            .set_input_alt("lhs: Matrix4x4F64, rhs: Vector4F64"),
        FFIMethod("inverse", "inverse", FFIMethodConvention.INSTANCE, "Matrix4x4F64", "Matrix4x4F64"),
        FFIMethod("transpose", "transpose", FFIMethodConvention.INSTANCE, "Matrix4x4F64", "Matrix4x4F64"),
        FFIMethod("translation", "translation", FFIMethodConvention.INSTANCE, "Matrix4x4F64", "Matrix4x4F64")
            .set_input_alt("vector: Vector3F64")
            .set_call_alt("Matrix4x4F64::translation(vector)"),
        FFIMethod("rotation", "rotation", FFIMethodConvention.INSTANCE, "Matrix4x4F64", "Matrix4x4F64")
            .set_input_alt("euler: Vector3F64")
            .set_call_alt("Matrix4x4F64::rotation(euler)"),
        FFIMethod("rotate_x", "rotate_x", FFIMethodConvention.INSTANCE, "Matrix4x4F64", "Matrix4x4F64")
            .set_input_alt("degrees: f64")
            .set_call_alt("Matrix4x4F64::rotate_x(degrees)"),
        FFIMethod("rotate_y", "rotate_y", FFIMethodConvention.INSTANCE, "Matrix4x4F64", "Matrix4x4F64")
            .set_input_alt("degrees: f64")
            .set_call_alt("Matrix4x4F64::rotate_y(degrees)"),
        FFIMethod("rotate_z", "rotate_z", FFIMethodConvention.INSTANCE, "Matrix4x4F64", "Matrix4x4F64")
            .set_input_alt("degrees: f64")
            .set_call_alt("Matrix4x4F64::rotate_z(degrees)"),
        FFIMethod("perspective", "perspective", FFIMethodConvention.INSTANCE, "Matrix4x4F64", "Matrix4x4F64")
            .set_input_alt("fov_y: f64, aspect: f64, z_near: f64, z_far: f64")
            .set_call_alt("Matrix4x4F64::perspective(fov_y, aspect, z_near, z_far)"),
        FFIMethod("default", "default", FFIMethodConvention.CUSTOM, "Matrix4x4F64", "Matrix4x4F64")
            .set_input_alt("")
            .set_call_alt("Matrix4x4F64::default()"),
        FFIMethod("identity", "identity", FFIMethodConvention.CUSTOM, "Matrix4x4F64", "Matrix4x4F64")
            .set_input_alt("")
            .set_call_alt("Matrix4x4F64::identity()"),
    ]),
]

prefix = '''
#![allow(clippy::needless_return)]

use crate::prelude::*;

'''

postfix = ''

print("Generating Rust -> C bindings...\n")
with open("src/ffi.rs", "w") as file:
    file.write("//\n")
    file.write(f"// Generated at {datetime.datetime.utcnow()} by generate_ffi.py\n")
    file.write("//\n")
    file.write(prefix)

    for bind in bindings:
        file.write("//\n")
        file.write(f"// %s\n" % bind.rust_name)
        file.write("//\n")

        print(bind.rust_name)

        last = bind.methods[-1]
        for method in bind.methods:
            file.write(method.emit_func(bind.ffi_name + "_"))
            file.write("\n")

            tree = "├ "

            if method == last:
                tree = "└ "

            print(f"{tree}%s -> %s" % (method.rust_name, bind.ffi_name + "_" + method.ffi_name))

        print("")

    file.write(postfix)


def make_csharp_type(raw_type: str) -> str:
    raw_type = raw_type.replace("F32", "").replace("f32", "float")

    if raw_type.endswith("F64"):
        raw_type = "D" + raw_type.replace("F64", "")

    raw_type = raw_type.replace("F64", "").replace("f64", "double")
    return raw_type


print("Generating C -> C# bindings...\n")
with open("bindings/dotnet/Internal.cs", "w") as file:
    file.write("//\n")
    file.write(f"// Generated at {datetime.datetime.utcnow()} by generate_ffi.py\n")
    file.write("//\n\n")

    file.write("using System.Runtime.InteropServices;\n\n")

    file.write("namespace RGML.Internal {\n")

    for bind in bindings:
        print(bind.rust_name)

        file.write(f"\tinternal class {bind.rust_name} {{\n")

        skeleton = "\t\t/*\n"
        for method in bind.methods:
            file.write('\t\t[DllImport("rgml")]\n')

            csharp_output = make_csharp_type(method.output_type)
            csharp_raw_name = bind.ffi_name + "_" + method.ffi_name
            csharp_name = method.ffi_name.capitalize()
            csharp_input = method.get_inputs()

            csharp_lhs = csharp_input
            csharp_rhs = csharp_input

            # We need to go from Rust typing to normal typing
            # x: f32 -> float x
            csharp_this = ""
            if csharp_input:
                input_tokens = method.get_inputs().split(",")

                csharp_input = ""
                csharp_lhs = ""
                csharp_rhs = ""

                last = input_tokens[-1]
                first = input_tokens[0]

                lhs_self_str = ""
                rhs_self_str = ""
                self_type = make_csharp_type(bind.rust_name)

                for token in input_tokens:
                    hint = token.split(":")

                    arg = hint[0].strip()
                    type = make_csharp_type(hint[1].strip())

                    csharp_input += type + " " + arg
                    csharp_lhs += type + " " + arg
                    csharp_rhs += arg

                    if token != last:
                        csharp_input += ", "
                        csharp_lhs += ", "
                        csharp_rhs += ", "

                    if token == first and type == self_type:
                        lhs_self_str = csharp_lhs
                        rhs_self_str = csharp_rhs
                        csharp_this = "this"

                csharp_lhs = csharp_lhs.replace(lhs_self_str, "")
                csharp_rhs = csharp_rhs.replace(rhs_self_str, "")

            pad = ", "

            if not csharp_lhs or not csharp_this:
                pad = ""

            file.write(f"\t\tinternal static extern {csharp_output} {csharp_raw_name}({csharp_input});\n\n")

            skeleton += f"\t\tpublic {csharp_output} {csharp_name}({csharp_lhs}) =>"
            skeleton += f" {bind.rust_name}.{csharp_raw_name}({csharp_this}{pad}{csharp_rhs});\n"

        file.write("\t\t// Skeleton Bindings (these are placeholders, they may need to be manually fixed)\n")
        file.write(f"{skeleton}\t\t*/\n")

        file.write(f"\t}}\n")

    file.write("}\n")

