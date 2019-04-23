// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of d3dcommon.h
use shared::basetsd::SIZE_T;
use shared::minwindef::{LPCVOID, LPVOID, UINT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCSTR};
ENUM!{enum D3D_DRIVER_TYPE {
    D3D_DRIVER_TYPE_UNKNOWN,
    D3D_DRIVER_TYPE_HARDWARE,
    D3D_DRIVER_TYPE_REFERENCE,
    D3D_DRIVER_TYPE_NULL,
    D3D_DRIVER_TYPE_SOFTWARE,
    D3D_DRIVER_TYPE_WARP,
}}
ENUM!{enum D3D_FEATURE_LEVEL {
    D3D_FEATURE_LEVEL_9_1 = 0x9100,
    D3D_FEATURE_LEVEL_9_2 = 0x9200,
    D3D_FEATURE_LEVEL_9_3 = 0x9300,
    D3D_FEATURE_LEVEL_10_0 = 0xa000,
    D3D_FEATURE_LEVEL_10_1 = 0xa100,
    D3D_FEATURE_LEVEL_11_0 = 0xb000,
    D3D_FEATURE_LEVEL_11_1 = 0xb100,
    D3D_FEATURE_LEVEL_12_0 = 0xc000,
    D3D_FEATURE_LEVEL_12_1 = 0xc100,
}}
ENUM!{enum D3D_PRIMITIVE_TOPOLOGY {
    D3D_PRIMITIVE_TOPOLOGY_UNDEFINED = 0,
    D3D_PRIMITIVE_TOPOLOGY_POINTLIST = 1,
    D3D_PRIMITIVE_TOPOLOGY_LINELIST = 2,
    D3D_PRIMITIVE_TOPOLOGY_LINESTRIP = 3,
    D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST = 4,
    D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP = 5,
    D3D_PRIMITIVE_TOPOLOGY_LINELIST_ADJ = 10,
    D3D_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ = 11,
    D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ = 12,
    D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ = 13,
    D3D_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST = 33,
    D3D_PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST = 34,
    D3D_PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST = 35,
    D3D_PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST = 36,
    D3D_PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST = 37,
    D3D_PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST = 38,
    D3D_PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST = 39,
    D3D_PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST = 40,
    D3D_PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST = 41,
    D3D_PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST = 42,
    D3D_PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST = 43,
    D3D_PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST = 44,
    D3D_PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST = 45,
    D3D_PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST = 46,
    D3D_PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST = 47,
    D3D_PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST = 48,
    D3D_PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST = 49,
    D3D_PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST = 50,
    D3D_PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST = 51,
    D3D_PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST = 52,
    D3D_PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST = 53,
    D3D_PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST = 54,
    D3D_PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST = 55,
    D3D_PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST = 56,
    D3D_PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST = 57,
    D3D_PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST = 58,
    D3D_PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST = 59,
    D3D_PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST = 60,
    D3D_PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST = 61,
    D3D_PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST = 62,
    D3D_PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST = 63,
    D3D_PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST = 64,
}}
pub const D3D10_PRIMITIVE_TOPOLOGY_UNDEFINED: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_UNDEFINED;
pub const D3D10_PRIMITIVE_TOPOLOGY_POINTLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_POINTLIST;
pub const D3D10_PRIMITIVE_TOPOLOGY_LINELIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_LINELIST;
pub const D3D10_PRIMITIVE_TOPOLOGY_LINESTRIP: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_LINESTRIP;
pub const D3D10_PRIMITIVE_TOPOLOGY_TRIANGLELIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
pub const D3D10_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP;
pub const D3D10_PRIMITIVE_TOPOLOGY_LINELIST_ADJ: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_LINELIST_ADJ;
pub const D3D10_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ;
pub const D3D10_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ;
pub const D3D10_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ;
pub const D3D11_PRIMITIVE_TOPOLOGY_UNDEFINED: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_UNDEFINED;
pub const D3D11_PRIMITIVE_TOPOLOGY_POINTLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_POINTLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_LINELIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_LINELIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_LINESTRIP: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_LINESTRIP;
pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP;
pub const D3D11_PRIMITIVE_TOPOLOGY_LINELIST_ADJ: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_LINELIST_ADJ;
pub const D3D11_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ;
pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ;
pub const D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ;
pub const D3D11_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST;
pub const D3D11_PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST: D3D_PRIMITIVE_TOPOLOGY =
    D3D_PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST;
ENUM!{enum D3D_PRIMITIVE {
    D3D_PRIMITIVE_UNDEFINED = 0,
    D3D_PRIMITIVE_POINT = 1,
    D3D_PRIMITIVE_LINE = 2,
    D3D_PRIMITIVE_TRIANGLE = 3,
    D3D_PRIMITIVE_LINE_ADJ = 6,
    D3D_PRIMITIVE_TRIANGLE_ADJ = 7,
    D3D_PRIMITIVE_1_CONTROL_POINT_PATCH = 8,
    D3D_PRIMITIVE_2_CONTROL_POINT_PATCH = 9,
    D3D_PRIMITIVE_3_CONTROL_POINT_PATCH = 10,
    D3D_PRIMITIVE_4_CONTROL_POINT_PATCH = 11,
    D3D_PRIMITIVE_5_CONTROL_POINT_PATCH = 12,
    D3D_PRIMITIVE_6_CONTROL_POINT_PATCH = 13,
    D3D_PRIMITIVE_7_CONTROL_POINT_PATCH = 14,
    D3D_PRIMITIVE_8_CONTROL_POINT_PATCH = 15,
    D3D_PRIMITIVE_9_CONTROL_POINT_PATCH = 16,
    D3D_PRIMITIVE_10_CONTROL_POINT_PATCH = 17,
    D3D_PRIMITIVE_11_CONTROL_POINT_PATCH = 18,
    D3D_PRIMITIVE_12_CONTROL_POINT_PATCH = 19,
    D3D_PRIMITIVE_13_CONTROL_POINT_PATCH = 20,
    D3D_PRIMITIVE_14_CONTROL_POINT_PATCH = 21,
    D3D_PRIMITIVE_15_CONTROL_POINT_PATCH = 22,
    D3D_PRIMITIVE_16_CONTROL_POINT_PATCH = 23,
    D3D_PRIMITIVE_17_CONTROL_POINT_PATCH = 24,
    D3D_PRIMITIVE_18_CONTROL_POINT_PATCH = 25,
    D3D_PRIMITIVE_19_CONTROL_POINT_PATCH = 26,
    D3D_PRIMITIVE_20_CONTROL_POINT_PATCH = 28,
    D3D_PRIMITIVE_21_CONTROL_POINT_PATCH = 29,
    D3D_PRIMITIVE_22_CONTROL_POINT_PATCH = 30,
    D3D_PRIMITIVE_23_CONTROL_POINT_PATCH = 31,
    D3D_PRIMITIVE_24_CONTROL_POINT_PATCH = 32,
    D3D_PRIMITIVE_25_CONTROL_POINT_PATCH = 33,
    D3D_PRIMITIVE_26_CONTROL_POINT_PATCH = 34,
    D3D_PRIMITIVE_27_CONTROL_POINT_PATCH = 35,
    D3D_PRIMITIVE_28_CONTROL_POINT_PATCH = 36,
    D3D_PRIMITIVE_29_CONTROL_POINT_PATCH = 37,
    D3D_PRIMITIVE_30_CONTROL_POINT_PATCH = 38,
    D3D_PRIMITIVE_31_CONTROL_POINT_PATCH = 39,
    D3D_PRIMITIVE_32_CONTROL_POINT_PATCH = 40,
}}
pub const D3D10_PRIMITIVE_UNDEFINED: D3D_PRIMITIVE = D3D_PRIMITIVE_UNDEFINED;
pub const D3D10_PRIMITIVE_POINT: D3D_PRIMITIVE = D3D_PRIMITIVE_POINT;
pub const D3D10_PRIMITIVE_LINE: D3D_PRIMITIVE = D3D_PRIMITIVE_LINE;
pub const D3D10_PRIMITIVE_TRIANGLE: D3D_PRIMITIVE = D3D_PRIMITIVE_TRIANGLE;
pub const D3D10_PRIMITIVE_LINE_ADJ: D3D_PRIMITIVE = D3D_PRIMITIVE_LINE_ADJ;
pub const D3D10_PRIMITIVE_TRIANGLE_ADJ: D3D_PRIMITIVE = D3D_PRIMITIVE_TRIANGLE_ADJ;
pub const D3D11_PRIMITIVE_UNDEFINED: D3D_PRIMITIVE = D3D_PRIMITIVE_UNDEFINED;
pub const D3D11_PRIMITIVE_POINT: D3D_PRIMITIVE = D3D_PRIMITIVE_POINT;
pub const D3D11_PRIMITIVE_LINE: D3D_PRIMITIVE = D3D_PRIMITIVE_LINE;
pub const D3D11_PRIMITIVE_TRIANGLE: D3D_PRIMITIVE = D3D_PRIMITIVE_TRIANGLE;
pub const D3D11_PRIMITIVE_LINE_ADJ: D3D_PRIMITIVE = D3D_PRIMITIVE_LINE_ADJ;
pub const D3D11_PRIMITIVE_TRIANGLE_ADJ: D3D_PRIMITIVE = D3D_PRIMITIVE_TRIANGLE_ADJ;
pub const D3D11_PRIMITIVE_1_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_1_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_2_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_2_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_3_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_3_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_4_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_4_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_5_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_5_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_6_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_6_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_7_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_7_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_8_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_8_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_9_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_9_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_10_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_10_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_11_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_11_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_12_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_12_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_13_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_13_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_14_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_14_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_15_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_15_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_16_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_16_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_17_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_17_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_18_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_18_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_19_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_19_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_20_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_20_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_21_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_21_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_22_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_22_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_23_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_23_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_24_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_24_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_25_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_25_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_26_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_26_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_27_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_27_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_28_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_28_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_29_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_29_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_30_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_30_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_31_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_31_CONTROL_POINT_PATCH;
pub const D3D11_PRIMITIVE_32_CONTROL_POINT_PATCH: D3D_PRIMITIVE =
    D3D_PRIMITIVE_32_CONTROL_POINT_PATCH;
ENUM!{enum D3D_SRV_DIMENSION {
    D3D_SRV_DIMENSION_UNKNOWN = 0,
    D3D_SRV_DIMENSION_BUFFER = 1,
    D3D_SRV_DIMENSION_TEXTURE1D = 2,
    D3D_SRV_DIMENSION_TEXTURE1DARRAY = 3,
    D3D_SRV_DIMENSION_TEXTURE2D = 4,
    D3D_SRV_DIMENSION_TEXTURE2DARRAY = 5,
    D3D_SRV_DIMENSION_TEXTURE2DMS = 6,
    D3D_SRV_DIMENSION_TEXTURE2DMSARRAY = 7,
    D3D_SRV_DIMENSION_TEXTURE3D = 8,
    D3D_SRV_DIMENSION_TEXTURECUBE = 9,
    D3D_SRV_DIMENSION_TEXTURECUBEARRAY = 10,
    D3D_SRV_DIMENSION_BUFFEREX = 11,
}}
pub const D3D10_SRV_DIMENSION_UNKNOWN: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_UNKNOWN;
pub const D3D10_SRV_DIMENSION_BUFFER: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_BUFFER;
pub const D3D10_SRV_DIMENSION_TEXTURE1D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE1D;
pub const D3D10_SRV_DIMENSION_TEXTURE1DARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE1DARRAY;
pub const D3D10_SRV_DIMENSION_TEXTURE2D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE2D;
pub const D3D10_SRV_DIMENSION_TEXTURE2DARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE2DARRAY;
pub const D3D10_SRV_DIMENSION_TEXTURE2DMS: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE2DMS;
pub const D3D10_SRV_DIMENSION_TEXTURE2DMSARRAY: D3D_SRV_DIMENSION =
    D3D_SRV_DIMENSION_TEXTURE2DMSARRAY;
pub const D3D10_SRV_DIMENSION_TEXTURE3D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE3D;
pub const D3D10_SRV_DIMENSION_TEXTURECUBE: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURECUBE;
pub const D3D10_1_SRV_DIMENSION_UNKNOWN: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_UNKNOWN;
pub const D3D10_1_SRV_DIMENSION_BUFFER: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_BUFFER;
pub const D3D10_1_SRV_DIMENSION_TEXTURE1D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE1D;
pub const D3D10_1_SRV_DIMENSION_TEXTURE1DARRAY: D3D_SRV_DIMENSION =
    D3D_SRV_DIMENSION_TEXTURE1DARRAY;
pub const D3D10_1_SRV_DIMENSION_TEXTURE2D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE2D;
pub const D3D10_1_SRV_DIMENSION_TEXTURE2DARRAY: D3D_SRV_DIMENSION =
    D3D_SRV_DIMENSION_TEXTURE2DARRAY;
pub const D3D10_1_SRV_DIMENSION_TEXTURE2DMS: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE2DMS;
pub const D3D10_1_SRV_DIMENSION_TEXTURE2DMSARRAY: D3D_SRV_DIMENSION =
    D3D_SRV_DIMENSION_TEXTURE2DMSARRAY;
pub const D3D10_1_SRV_DIMENSION_TEXTURE3D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE3D;
pub const D3D10_1_SRV_DIMENSION_TEXTURECUBE: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURECUBE;
pub const D3D10_1_SRV_DIMENSION_TEXTURECUBEARRAY: D3D_SRV_DIMENSION =
    D3D_SRV_DIMENSION_TEXTURECUBEARRAY;
pub const D3D11_SRV_DIMENSION_UNKNOWN: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_UNKNOWN;
pub const D3D11_SRV_DIMENSION_BUFFER: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_BUFFER;
pub const D3D11_SRV_DIMENSION_TEXTURE1D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE1D;
pub const D3D11_SRV_DIMENSION_TEXTURE1DARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE1DARRAY;
pub const D3D11_SRV_DIMENSION_TEXTURE2D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE2D;
pub const D3D11_SRV_DIMENSION_TEXTURE2DARRAY: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE2DARRAY;
pub const D3D11_SRV_DIMENSION_TEXTURE2DMS: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE2DMS;
pub const D3D11_SRV_DIMENSION_TEXTURE2DMSARRAY: D3D_SRV_DIMENSION =
    D3D_SRV_DIMENSION_TEXTURE2DMSARRAY;
pub const D3D11_SRV_DIMENSION_TEXTURE3D: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURE3D;
pub const D3D11_SRV_DIMENSION_TEXTURECUBE: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_TEXTURECUBE;
pub const D3D11_SRV_DIMENSION_TEXTURECUBEARRAY: D3D_SRV_DIMENSION =
    D3D_SRV_DIMENSION_TEXTURECUBEARRAY;
pub const D3D11_SRV_DIMENSION_BUFFEREX: D3D_SRV_DIMENSION = D3D_SRV_DIMENSION_BUFFEREX;
STRUCT!{#[debug] struct D3D_SHADER_MACRO {
    Name: LPCSTR,
    Definition: LPCSTR,
}}
pub type LPD3D_SHADER_MACRO = *mut D3D_SHADER_MACRO;
DEFINE_GUID!{IID_ID3D10Blob,
    0x8ba5fb08, 0x5195, 0x40e2, 0xac, 0x58, 0x0d, 0x98, 0x9c, 0x3a, 0x01, 0x02}
RIDL!{#[uuid(0x8ba5fb08, 0x5195, 0x40e2, 0xac, 0x58, 0xd, 0x98, 0x9c, 0x3a, 0x1, 0x2)]
interface ID3D10Blob(ID3D10BlobVtbl): IUnknown(IUnknownVtbl) {
    fn GetBufferPointer() -> LPVOID,
    fn GetBufferSize() -> SIZE_T,
}}
pub type LPD3D10BLOB = *mut ID3D10Blob;
pub type ID3DBlob = ID3D10Blob;
pub type LPD3DBLOB = *mut ID3DBlob;
ENUM!{enum D3D_INCLUDE_TYPE {
    D3D_INCLUDE_LOCAL = 0,
    D3D_INCLUDE_SYSTEM,
}}
pub const D3D10_INCLUDE_LOCAL: D3D_INCLUDE_TYPE = D3D_INCLUDE_LOCAL;
pub const D3D10_INCLUDE_SYSTEM: D3D_INCLUDE_TYPE = D3D_INCLUDE_SYSTEM;
RIDL!{#[uuid(0x00000000, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00)]
interface ID3DInclude(ID3DIncludeVtbl) {
    fn Open(
        IncludeType: D3D_INCLUDE_TYPE,
        pFileName: LPCSTR,
        pParentData: LPCVOID,
        ppData: *mut LPCVOID,
        pBytes: *mut UINT,
    ) -> HRESULT,
    fn Close(
        pData: LPCVOID,
    ) -> HRESULT,
}}
pub type LPD3DINCLUDE = *mut ID3DInclude;
ENUM!{enum D3D_SHADER_VARIABLE_CLASS {
    D3D_SVC_SCALAR = 0,
    D3D_SVC_VECTOR,
    D3D_SVC_MATRIX_ROWS,
    D3D_SVC_MATRIX_COLUMNS,
    D3D_SVC_OBJECT,
    D3D_SVC_STRUCT,
    D3D_SVC_INTERFACE_CLASS,
    D3D_SVC_INTERFACE_POINTER,
}}
pub const D3D10_SVC_SCALAR: D3D_SHADER_VARIABLE_CLASS = D3D_SVC_SCALAR;
pub const D3D10_SVC_VECTOR: D3D_SHADER_VARIABLE_CLASS = D3D_SVC_VECTOR;
pub const D3D10_SVC_MATRIX_ROWS: D3D_SHADER_VARIABLE_CLASS = D3D_SVC_MATRIX_ROWS;
pub const D3D10_SVC_MATRIX_COLUMNS: D3D_SHADER_VARIABLE_CLASS = D3D_SVC_MATRIX_COLUMNS;
pub const D3D10_SVC_OBJECT: D3D_SHADER_VARIABLE_CLASS = D3D_SVC_OBJECT;
pub const D3D10_SVC_STRUCT: D3D_SHADER_VARIABLE_CLASS = D3D_SVC_STRUCT;
pub const D3D11_SVC_INTERFACE_CLASS: D3D_SHADER_VARIABLE_CLASS = D3D_SVC_INTERFACE_CLASS;
pub const D3D11_SVC_INTERFACE_POINTER: D3D_SHADER_VARIABLE_CLASS = D3D_SVC_INTERFACE_POINTER;
ENUM!{enum D3D_SHADER_VARIABLE_FLAGS {
    D3D_SVF_USERPACKED = 1,
    D3D_SVF_USED = 2,
    D3D_SVF_INTERFACE_POINTER = 4,
    D3D_SVF_INTERFACE_PARAMETER = 8,
}}
pub const D3D10_SVF_USERPACKED: D3D_SHADER_VARIABLE_FLAGS = D3D_SVF_USERPACKED;
pub const D3D10_SVF_USED: D3D_SHADER_VARIABLE_FLAGS = D3D_SVF_USED;
pub const D3D11_SVF_INTERFACE_POINTER: D3D_SHADER_VARIABLE_FLAGS = D3D_SVF_INTERFACE_POINTER;
pub const D3D11_SVF_INTERFACE_PARAMETER: D3D_SHADER_VARIABLE_FLAGS = D3D_SVF_INTERFACE_PARAMETER;
ENUM!{enum D3D_SHADER_VARIABLE_TYPE {
    D3D_SVT_VOID = 0,
    D3D_SVT_BOOL = 1,
    D3D_SVT_INT = 2,
    D3D_SVT_FLOAT = 3,
    D3D_SVT_STRING = 4,
    D3D_SVT_TEXTURE = 5,
    D3D_SVT_TEXTURE1D = 6,
    D3D_SVT_TEXTURE2D = 7,
    D3D_SVT_TEXTURE3D = 8,
    D3D_SVT_TEXTURECUBE = 9,
    D3D_SVT_SAMPLER = 10,
    D3D_SVT_SAMPLER1D = 11,
    D3D_SVT_SAMPLER2D = 12,
    D3D_SVT_SAMPLER3D = 13,
    D3D_SVT_SAMPLERCUBE = 14,
    D3D_SVT_PIXELSHADER = 15,
    D3D_SVT_VERTEXSHADER = 16,
    D3D_SVT_PIXELFRAGMENT = 17,
    D3D_SVT_VERTEXFRAGMENT = 18,
    D3D_SVT_UINT = 19,
    D3D_SVT_UINT8 = 20,
    D3D_SVT_GEOMETRYSHADER = 21,
    D3D_SVT_RASTERIZER = 22,
    D3D_SVT_DEPTHSTENCIL = 23,
    D3D_SVT_BLEND = 24,
    D3D_SVT_BUFFER = 25,
    D3D_SVT_CBUFFER = 26,
    D3D_SVT_TBUFFER = 27,
    D3D_SVT_TEXTURE1DARRAY = 28,
    D3D_SVT_TEXTURE2DARRAY = 29,
    D3D_SVT_RENDERTARGETVIEW = 30,
    D3D_SVT_DEPTHSTENCILVIEW = 31,
    D3D_SVT_TEXTURE2DMS = 32,
    D3D_SVT_TEXTURE2DMSARRAY = 33,
    D3D_SVT_TEXTURECUBEARRAY = 34,
    D3D_SVT_HULLSHADER = 35,
    D3D_SVT_DOMAINSHADER = 36,
    D3D_SVT_INTERFACE_POINTER = 37,
    D3D_SVT_COMPUTESHADER = 38,
    D3D_SVT_DOUBLE = 39,
    D3D_SVT_RWTEXTURE1D = 40,
    D3D_SVT_RWTEXTURE1DARRAY = 41,
    D3D_SVT_RWTEXTURE2D = 42,
    D3D_SVT_RWTEXTURE2DARRAY = 43,
    D3D_SVT_RWTEXTURE3D = 44,
    D3D_SVT_RWBUFFER = 45,
    D3D_SVT_BYTEADDRESS_BUFFER = 46,
    D3D_SVT_RWBYTEADDRESS_BUFFER = 47,
    D3D_SVT_STRUCTURED_BUFFER = 48,
    D3D_SVT_RWSTRUCTURED_BUFFER = 49,
    D3D_SVT_APPEND_STRUCTURED_BUFFER = 50,
    D3D_SVT_CONSUME_STRUCTURED_BUFFER = 51,
    D3D_SVT_MIN8FLOAT = 52,
    D3D_SVT_MIN10FLOAT = 53,
    D3D_SVT_MIN16FLOAT = 54,
    D3D_SVT_MIN12INT = 55,
    D3D_SVT_MIN16INT = 56,
    D3D_SVT_MIN16UINT = 57,
}}
pub const D3D10_SVT_VOID: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_VOID;
pub const D3D10_SVT_BOOL: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_BOOL;
pub const D3D10_SVT_INT: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_INT;
pub const D3D10_SVT_FLOAT: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_FLOAT;
pub const D3D10_SVT_STRING: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_STRING;
pub const D3D10_SVT_TEXTURE: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_TEXTURE;
pub const D3D10_SVT_TEXTURE1D: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_TEXTURE1D;
pub const D3D10_SVT_TEXTURE2D: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_TEXTURE2D;
pub const D3D10_SVT_TEXTURE3D: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_TEXTURE3D;
pub const D3D10_SVT_TEXTURECUBE: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_TEXTURECUBE;
pub const D3D10_SVT_SAMPLER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_SAMPLER;
pub const D3D10_SVT_SAMPLER1D: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_SAMPLER1D;
pub const D3D10_SVT_SAMPLER2D: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_SAMPLER2D;
pub const D3D10_SVT_SAMPLER3D: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_SAMPLER3D;
pub const D3D10_SVT_SAMPLERCUBE: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_SAMPLERCUBE;
pub const D3D10_SVT_PIXELSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_PIXELSHADER;
pub const D3D10_SVT_VERTEXSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_VERTEXSHADER;
pub const D3D10_SVT_PIXELFRAGMENT: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_PIXELFRAGMENT;
pub const D3D10_SVT_VERTEXFRAGMENT: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_VERTEXFRAGMENT;
pub const D3D10_SVT_UINT: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_UINT;
pub const D3D10_SVT_UINT8: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_UINT8;
pub const D3D10_SVT_GEOMETRYSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_GEOMETRYSHADER;
pub const D3D10_SVT_RASTERIZER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_RASTERIZER;
pub const D3D10_SVT_DEPTHSTENCIL: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_DEPTHSTENCIL;
pub const D3D10_SVT_BLEND: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_BLEND;
pub const D3D10_SVT_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_BUFFER;
pub const D3D10_SVT_CBUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_CBUFFER;
pub const D3D10_SVT_TBUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_TBUFFER;
pub const D3D10_SVT_TEXTURE1DARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_TEXTURE1DARRAY;
pub const D3D10_SVT_TEXTURE2DARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_TEXTURE2DARRAY;
pub const D3D10_SVT_RENDERTARGETVIEW: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_RENDERTARGETVIEW;
pub const D3D10_SVT_DEPTHSTENCILVIEW: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_DEPTHSTENCILVIEW;
pub const D3D10_SVT_TEXTURE2DMS: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_TEXTURE2DMS;
pub const D3D10_SVT_TEXTURE2DMSARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_TEXTURE2DMSARRAY;
pub const D3D10_SVT_TEXTURECUBEARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_TEXTURECUBEARRAY;
pub const D3D11_SVT_HULLSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_HULLSHADER;
pub const D3D11_SVT_DOMAINSHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_DOMAINSHADER;
pub const D3D11_SVT_INTERFACE_POINTER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_INTERFACE_POINTER;
pub const D3D11_SVT_COMPUTESHADER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_COMPUTESHADER;
pub const D3D11_SVT_DOUBLE: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_DOUBLE;
pub const D3D11_SVT_RWTEXTURE1D: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_RWTEXTURE1D;
pub const D3D11_SVT_RWTEXTURE1DARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_RWTEXTURE1DARRAY;
pub const D3D11_SVT_RWTEXTURE2D: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_RWTEXTURE2D;
pub const D3D11_SVT_RWTEXTURE2DARRAY: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_RWTEXTURE2DARRAY;
pub const D3D11_SVT_RWTEXTURE3D: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_RWTEXTURE3D;
pub const D3D11_SVT_RWBUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_RWBUFFER;
pub const D3D11_SVT_BYTEADDRESS_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_BYTEADDRESS_BUFFER;
pub const D3D11_SVT_RWBYTEADDRESS_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_RWBYTEADDRESS_BUFFER;
pub const D3D11_SVT_STRUCTURED_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_STRUCTURED_BUFFER;
pub const D3D11_SVT_RWSTRUCTURED_BUFFER: D3D_SHADER_VARIABLE_TYPE = D3D_SVT_RWSTRUCTURED_BUFFER;
pub const D3D11_SVT_APPEND_STRUCTURED_BUFFER: D3D_SHADER_VARIABLE_TYPE =
    D3D_SVT_APPEND_STRUCTURED_BUFFER;
pub const D3D11_SVT_CONSUME_STRUCTURED_BUFFER: D3D_SHADER_VARIABLE_TYPE =
    D3D_SVT_CONSUME_STRUCTURED_BUFFER;
ENUM!{enum D3D_SHADER_INPUT_FLAGS {
    D3D_SIF_USERPACKED = 0x1,
    D3D_SIF_COMPARISON_SAMPLER = 0x2,
    D3D_SIF_TEXTURE_COMPONENT_0 = 0x4,
    D3D_SIF_TEXTURE_COMPONENT_1 = 0x8,
    D3D_SIF_TEXTURE_COMPONENTS = 0xc,
    D3D_SIF_UNUSED = 0x10,
}}
pub const D3D10_SIF_USERPACKED: D3D_SHADER_INPUT_FLAGS = D3D_SIF_USERPACKED;
pub const D3D10_SIF_COMPARISON_SAMPLER: D3D_SHADER_INPUT_FLAGS = D3D_SIF_COMPARISON_SAMPLER;
pub const D3D10_SIF_TEXTURE_COMPONENT_0: D3D_SHADER_INPUT_FLAGS = D3D_SIF_TEXTURE_COMPONENT_0;
pub const D3D10_SIF_TEXTURE_COMPONENT_1: D3D_SHADER_INPUT_FLAGS = D3D_SIF_TEXTURE_COMPONENT_1;
pub const D3D10_SIF_TEXTURE_COMPONENTS: D3D_SHADER_INPUT_FLAGS = D3D_SIF_TEXTURE_COMPONENTS;
ENUM!{enum D3D_SHADER_INPUT_TYPE {
    D3D_SIT_CBUFFER,
    D3D_SIT_TBUFFER,
    D3D_SIT_TEXTURE,
    D3D_SIT_SAMPLER,
    D3D_SIT_UAV_RWTYPED,
    D3D_SIT_STRUCTURED,
    D3D_SIT_UAV_RWSTRUCTURED,
    D3D_SIT_BYTEADDRESS,
    D3D_SIT_UAV_RWBYTEADDRESS,
    D3D_SIT_UAV_APPEND_STRUCTURED,
    D3D_SIT_UAV_CONSUME_STRUCTURED,
    D3D_SIT_UAV_RWSTRUCTURED_WITH_COUNTER,
}}
pub const D3D10_SIT_CBUFFER: D3D_SHADER_INPUT_TYPE = D3D_SIT_CBUFFER;
pub const D3D10_SIT_TBUFFER: D3D_SHADER_INPUT_TYPE = D3D_SIT_TBUFFER;
pub const D3D10_SIT_TEXTURE: D3D_SHADER_INPUT_TYPE = D3D_SIT_TEXTURE;
pub const D3D10_SIT_SAMPLER: D3D_SHADER_INPUT_TYPE = D3D_SIT_SAMPLER;
pub const D3D11_SIT_UAV_RWTYPED: D3D_SHADER_INPUT_TYPE = D3D_SIT_UAV_RWTYPED;
pub const D3D11_SIT_STRUCTURED: D3D_SHADER_INPUT_TYPE = D3D_SIT_STRUCTURED;
pub const D3D11_SIT_UAV_RWSTRUCTURED: D3D_SHADER_INPUT_TYPE = D3D_SIT_UAV_RWSTRUCTURED;
pub const D3D11_SIT_BYTEADDRESS: D3D_SHADER_INPUT_TYPE = D3D_SIT_BYTEADDRESS;
pub const D3D11_SIT_UAV_RWBYTEADDRESS: D3D_SHADER_INPUT_TYPE = D3D_SIT_UAV_RWBYTEADDRESS;
pub const D3D11_SIT_UAV_APPEND_STRUCTURED: D3D_SHADER_INPUT_TYPE = D3D_SIT_UAV_APPEND_STRUCTURED;
pub const D3D11_SIT_UAV_CONSUME_STRUCTURED: D3D_SHADER_INPUT_TYPE = D3D_SIT_UAV_CONSUME_STRUCTURED;
pub const D3D11_SIT_UAV_RWSTRUCTURED_WITH_COUNTER: D3D_SHADER_INPUT_TYPE =
    D3D_SIT_UAV_RWSTRUCTURED_WITH_COUNTER;
ENUM!{enum D3D_SHADER_CBUFFER_FLAGS {
    D3D_CBF_USERPACKED = 1,
}}
pub const D3D10_CBF_USERPACKED: D3D_SHADER_CBUFFER_FLAGS = D3D_CBF_USERPACKED;
ENUM!{enum D3D_CBUFFER_TYPE {
    D3D_CT_CBUFFER,
    D3D_CT_TBUFFER,
    D3D_CT_INTERFACE_POINTERS,
    D3D_CT_RESOURCE_BIND_INFO,
}}
pub const D3D10_CT_CBUFFER: D3D_CBUFFER_TYPE = D3D_CT_CBUFFER;
pub const D3D10_CT_TBUFFER: D3D_CBUFFER_TYPE = D3D_CT_TBUFFER;
pub const D3D11_CT_CBUFFER: D3D_CBUFFER_TYPE = D3D_CT_CBUFFER;
pub const D3D11_CT_TBUFFER: D3D_CBUFFER_TYPE = D3D_CT_TBUFFER;
pub const D3D11_CT_INTERFACE_POINTERS: D3D_CBUFFER_TYPE = D3D_CT_INTERFACE_POINTERS;
pub const D3D11_CT_RESOURCE_BIND_INFO: D3D_CBUFFER_TYPE = D3D_CT_RESOURCE_BIND_INFO;
ENUM!{enum D3D_NAME {
    D3D_NAME_UNDEFINED = 0,
    D3D_NAME_POSITION = 1,
    D3D_NAME_CLIP_DISTANCE = 2,
    D3D_NAME_CULL_DISTANCE = 3,
    D3D_NAME_RENDER_TARGET_ARRAY_INDEX = 4,
    D3D_NAME_VIEWPORT_ARRAY_INDEX = 5,
    D3D_NAME_VERTEX_ID = 6,
    D3D_NAME_PRIMITIVE_ID = 7,
    D3D_NAME_INSTANCE_ID = 8,
    D3D_NAME_IS_FRONT_FACE = 9,
    D3D_NAME_SAMPLE_INDEX = 10,
    D3D_NAME_FINAL_QUAD_EDGE_TESSFACTOR = 11,
    D3D_NAME_FINAL_QUAD_INSIDE_TESSFACTOR = 12,
    D3D_NAME_FINAL_TRI_EDGE_TESSFACTOR = 13,
    D3D_NAME_FINAL_TRI_INSIDE_TESSFACTOR = 14,
    D3D_NAME_FINAL_LINE_DETAIL_TESSFACTOR = 15,
    D3D_NAME_FINAL_LINE_DENSITY_TESSFACTOR = 16,
    D3D_NAME_TARGET = 64,
    D3D_NAME_DEPTH = 65,
    D3D_NAME_COVERAGE = 66,
    D3D_NAME_DEPTH_GREATER_EQUAL = 67,
    D3D_NAME_DEPTH_LESS_EQUAL = 68,
}}
pub const D3D10_NAME_UNDEFINED: D3D_NAME = D3D_NAME_UNDEFINED;
pub const D3D10_NAME_POSITION: D3D_NAME = D3D_NAME_POSITION;
pub const D3D10_NAME_CLIP_DISTANCE: D3D_NAME = D3D_NAME_CLIP_DISTANCE;
pub const D3D10_NAME_CULL_DISTANCE: D3D_NAME = D3D_NAME_CULL_DISTANCE;
pub const D3D10_NAME_RENDER_TARGET_ARRAY_INDEX: D3D_NAME = D3D_NAME_RENDER_TARGET_ARRAY_INDEX;
pub const D3D10_NAME_VIEWPORT_ARRAY_INDEX: D3D_NAME = D3D_NAME_VIEWPORT_ARRAY_INDEX;
pub const D3D10_NAME_VERTEX_ID: D3D_NAME = D3D_NAME_VERTEX_ID;
pub const D3D10_NAME_PRIMITIVE_ID: D3D_NAME = D3D_NAME_PRIMITIVE_ID;
pub const D3D10_NAME_INSTANCE_ID: D3D_NAME = D3D_NAME_INSTANCE_ID;
pub const D3D10_NAME_IS_FRONT_FACE: D3D_NAME = D3D_NAME_IS_FRONT_FACE;
pub const D3D10_NAME_SAMPLE_INDEX: D3D_NAME = D3D_NAME_SAMPLE_INDEX;
pub const D3D10_NAME_TARGET: D3D_NAME = D3D_NAME_TARGET;
pub const D3D10_NAME_DEPTH: D3D_NAME = D3D_NAME_DEPTH;
pub const D3D10_NAME_COVERAGE: D3D_NAME = D3D_NAME_COVERAGE;
pub const D3D11_NAME_FINAL_QUAD_EDGE_TESSFACTOR: D3D_NAME = D3D_NAME_FINAL_QUAD_EDGE_TESSFACTOR;
pub const D3D11_NAME_FINAL_QUAD_INSIDE_TESSFACTOR: D3D_NAME
    = D3D_NAME_FINAL_QUAD_INSIDE_TESSFACTOR;
pub const D3D11_NAME_FINAL_TRI_EDGE_TESSFACTOR: D3D_NAME = D3D_NAME_FINAL_TRI_EDGE_TESSFACTOR;
pub const D3D11_NAME_FINAL_TRI_INSIDE_TESSFACTOR: D3D_NAME = D3D_NAME_FINAL_TRI_INSIDE_TESSFACTOR;
pub const D3D11_NAME_FINAL_LINE_DETAIL_TESSFACTOR: D3D_NAME
    = D3D_NAME_FINAL_LINE_DETAIL_TESSFACTOR;
pub const D3D11_NAME_FINAL_LINE_DENSITY_TESSFACTOR: D3D_NAME
    = D3D_NAME_FINAL_LINE_DENSITY_TESSFACTOR;
pub const D3D11_NAME_DEPTH_GREATER_EQUAL: D3D_NAME = D3D_NAME_DEPTH_GREATER_EQUAL;
pub const D3D11_NAME_DEPTH_LESS_EQUAL: D3D_NAME = D3D_NAME_DEPTH_LESS_EQUAL;
ENUM!{enum D3D_RESOURCE_RETURN_TYPE {
    D3D_RETURN_TYPE_UNORM = 1,
    D3D_RETURN_TYPE_SNORM = 2,
    D3D_RETURN_TYPE_SINT = 3,
    D3D_RETURN_TYPE_UINT = 4,
    D3D_RETURN_TYPE_FLOAT = 5,
    D3D_RETURN_TYPE_MIXED = 6,
    D3D_RETURN_TYPE_DOUBLE = 7,
    D3D_RETURN_TYPE_CONTINUED = 8,
}}
pub const D3D10_RETURN_TYPE_UNORM: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_UNORM;
pub const D3D10_RETURN_TYPE_SNORM: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_SNORM;
pub const D3D10_RETURN_TYPE_SINT: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_SINT;
pub const D3D10_RETURN_TYPE_UINT: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_UINT;
pub const D3D10_RETURN_TYPE_FLOAT: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_FLOAT;
pub const D3D10_RETURN_TYPE_MIXED: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_MIXED;
pub const D3D11_RETURN_TYPE_UNORM: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_UNORM;
pub const D3D11_RETURN_TYPE_SNORM: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_SNORM;
pub const D3D11_RETURN_TYPE_SINT: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_SINT;
pub const D3D11_RETURN_TYPE_UINT: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_UINT;
pub const D3D11_RETURN_TYPE_FLOAT: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_FLOAT;
pub const D3D11_RETURN_TYPE_MIXED: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_MIXED;
pub const D3D11_RETURN_TYPE_DOUBLE: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_DOUBLE;
pub const D3D11_RETURN_TYPE_CONTINUED: D3D_RESOURCE_RETURN_TYPE = D3D_RETURN_TYPE_CONTINUED;
ENUM!{enum D3D_REGISTER_COMPONENT_TYPE {
    D3D_REGISTER_COMPONENT_UNKNOWN = 0,
    D3D_REGISTER_COMPONENT_UINT32 = 1,
    D3D_REGISTER_COMPONENT_SINT32 = 2,
    D3D_REGISTER_COMPONENT_FLOAT32 = 3,
}}
pub const D3D10_REGISTER_COMPONENT_UNKNOWN: D3D_REGISTER_COMPONENT_TYPE =
    D3D_REGISTER_COMPONENT_UNKNOWN;
pub const D3D10_REGISTER_COMPONENT_UINT32: D3D_REGISTER_COMPONENT_TYPE =
    D3D_REGISTER_COMPONENT_UINT32;
pub const D3D10_REGISTER_COMPONENT_SINT32: D3D_REGISTER_COMPONENT_TYPE =
    D3D_REGISTER_COMPONENT_SINT32;
pub const D3D10_REGISTER_COMPONENT_FLOAT32: D3D_REGISTER_COMPONENT_TYPE =
    D3D_REGISTER_COMPONENT_FLOAT32;
ENUM!{enum D3D_TESSELLATOR_DOMAIN {
    D3D_TESSELLATOR_DOMAIN_UNDEFINED,
    D3D_TESSELLATOR_DOMAIN_ISOLINE,
    D3D_TESSELLATOR_DOMAIN_TRI,
    D3D_TESSELLATOR_DOMAIN_QUAD,
}}
pub const D3D11_TESSELLATOR_DOMAIN_UNDEFINED: D3D_TESSELLATOR_DOMAIN =
    D3D_TESSELLATOR_DOMAIN_UNDEFINED;
pub const D3D11_TESSELLATOR_DOMAIN_ISOLINE: D3D_TESSELLATOR_DOMAIN =
    D3D_TESSELLATOR_DOMAIN_ISOLINE;
pub const D3D11_TESSELLATOR_DOMAIN_TRI: D3D_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN_TRI;
pub const D3D11_TESSELLATOR_DOMAIN_QUAD: D3D_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN_QUAD;
ENUM!{enum D3D_TESSELLATOR_PARTITIONING {
    D3D_TESSELLATOR_PARTITIONING_UNDEFINED,
    D3D_TESSELLATOR_PARTITIONING_INTEGER,
    D3D_TESSELLATOR_PARTITIONING_POW2,
    D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_ODD,
    D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_EVEN,
}}
pub const D3D11_TESSELLATOR_PARTITIONING_UNDEFINED: D3D_TESSELLATOR_PARTITIONING =
    D3D_TESSELLATOR_PARTITIONING_UNDEFINED;
pub const D3D11_TESSELLATOR_PARTITIONING_INTEGER: D3D_TESSELLATOR_PARTITIONING =
    D3D_TESSELLATOR_PARTITIONING_INTEGER;
pub const D3D11_TESSELLATOR_PARTITIONING_POW2: D3D_TESSELLATOR_PARTITIONING =
    D3D_TESSELLATOR_PARTITIONING_POW2;
pub const D3D11_TESSELLATOR_PARTITIONING_FRACTIONAL_ODD: D3D_TESSELLATOR_PARTITIONING =
    D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_ODD;
pub const D3D11_TESSELLATOR_PARTITIONING_FRACTIONAL_EVEN: D3D_TESSELLATOR_PARTITIONING =
    D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_EVEN;
ENUM!{enum D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    D3D_TESSELLATOR_OUTPUT_UNDEFINED,
    D3D_TESSELLATOR_OUTPUT_POINT,
    D3D_TESSELLATOR_OUTPUT_LINE,
    D3D_TESSELLATOR_OUTPUT_TRIANGLE_CW,
    D3D_TESSELLATOR_OUTPUT_TRIANGLE_CCW,
}}
pub const D3D11_TESSELLATOR_OUTPUT_UNDEFINED: D3D_TESSELLATOR_OUTPUT_PRIMITIVE =
    D3D_TESSELLATOR_OUTPUT_UNDEFINED;
pub const D3D11_TESSELLATOR_OUTPUT_POINT: D3D_TESSELLATOR_OUTPUT_PRIMITIVE =
    D3D_TESSELLATOR_OUTPUT_POINT;
pub const D3D11_TESSELLATOR_OUTPUT_LINE: D3D_TESSELLATOR_OUTPUT_PRIMITIVE =
    D3D_TESSELLATOR_OUTPUT_LINE;
pub const D3D11_TESSELLATOR_OUTPUT_TRIANGLE_CW: D3D_TESSELLATOR_OUTPUT_PRIMITIVE =
    D3D_TESSELLATOR_OUTPUT_TRIANGLE_CW;
pub const D3D11_TESSELLATOR_OUTPUT_TRIANGLE_CCW: D3D_TESSELLATOR_OUTPUT_PRIMITIVE =
    D3D_TESSELLATOR_OUTPUT_TRIANGLE_CCW;
ENUM!{enum D3D_MIN_PRECISION {
    D3D_MIN_PRECISION_DEFAULT,
    D3D_MIN_PRECISION_FLOAT_16,
    D3D_MIN_PRECISION_FLOAT_2_8,
    D3D_MIN_PRECISION_RESERVED,
    D3D_MIN_PRECISION_SINT_16,
    D3D_MIN_PRECISION_UINT_16,
    D3D_MIN_PRECISION_ANY_16 = 0xf0,
    D3D_MIN_PRECISION_ANY_10 = 0xf1,
}}
ENUM!{enum D3D_INTERPOLATION_MODE {
    D3D_INTERPOLATION_UNDEFINED,
    D3D_INTERPOLATION_CONSTANT,
    D3D_INTERPOLATION_LINEAR,
    D3D_INTERPOLATION_LINEAR_CENTROID,
    D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE,
    D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE_CENTROID,
    D3D_INTERPOLATION_LINEAR_SAMPLE,
    D3D_INTERPOLATION_LINEAR_NOPERSPECTIVE_SAMPLE,
}}
ENUM!{enum D3D_PARAMETER_FLAGS {
    D3D_PF_NONE = 0,
    D3D_PF_IN = 0x1,
    D3D_PF_OUT = 0x2,
}}
DEFINE_GUID!{WKPDID_D3DDebugObjectName,
    0x429b8c22, 0x9188, 0x4b0c, 0x87, 0x42, 0xac, 0xb0, 0xbf, 0x85, 0xc2, 0x00}
DEFINE_GUID!{WKPDID_D3DDebugObjectNameW,
    0x4cca5fd8, 0x921f, 0x42c8, 0x85, 0x66, 0x70, 0xca, 0xf2, 0xa9, 0xb7, 0x41}
DEFINE_GUID!{WKPDID_CommentStringW,
    0xd0149dc0, 0x90e8, 0x4ec8, 0x81, 0x44, 0xe9, 0x00, 0xad, 0x26, 0x6b, 0xb2}
