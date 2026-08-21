use crate::maths::{Float2, Float3, Float4};
use bytemuck::cast_slice;
use glam::f32::Mat4;
use glow::{ARRAY_BUFFER, ATOMIC_COUNTER_BUFFER, BYTE, CLAMP_TO_EDGE, COPY_READ_BUFFER, COPY_WRITE_BUFFER, Context, DISPATCH_INDIRECT_BUFFER, DRAW_INDIRECT_BUFFER, DYNAMIC_DRAW, DYNAMIC_READ, ELEMENT_ARRAY_BUFFER, HasContext, LINEAR, LINEAR_MIPMAP_LINEAR, NativeBuffer, NativeProgram, NativeTexture, NativeUniformLocation, NativeVertexArray, PARAMETER_BUFFER, PIXEL_PACK_BUFFER, PIXEL_UNPACK_BUFFER, QUERY_BUFFER, RGBA, SHADER_STORAGE_BUFFER, SHORT, STATIC_COPY, STATIC_DRAW, STATIC_READ, STREAM_COPY, STREAM_DRAW, STREAM_READ, TEXTURE_2D, TEXTURE_BASE_LEVEL, TEXTURE_BUFFER, TEXTURE_MAG_FILTER, TEXTURE_MAX_LEVEL, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T, TEXTURE0, TRANSFORM_FEEDBACK_BUFFER, UNIFORM_BUFFER, UNSIGNED_BYTE, UNSIGNED_SHORT, INT, UNSIGNED_INT, FLOAT, DOUBLE, HALF_FLOAT, FIXED, UNSIGNED_INT_2_10_10_10_REV, UNSIGNED_INT_10F_11F_11F_REV, INT_2_10_10_10_REV};
use uuid::Uuid;

pub struct BufferObject {
    buffer: NativeBuffer,
    target: BufferTarget,
}

pub struct VertexBufferObject {
    buffer: NativeBuffer,
}

pub struct IndexBufferObject {
    buffer: NativeBuffer,
}

pub struct VertexArrayObject {
    vertex_array: NativeVertexArray,
    vertex_buffer: VertexBufferObject,
    index_buffer: IndexBufferObject,
}

impl BufferObject {
    pub fn create(gl: Context, target: BufferTarget) -> Self {
        unsafe {
            let buffer = gl.create_buffer().unwrap();
            BufferObject { buffer, target }
        }
    }

    pub fn bind(&self, gl: &Context) -> () {
        unsafe {
            gl.bind_buffer(self.target.to_u32(), Some(self.buffer));
        }
    }

    pub fn buffer_data(&self, gl: &Context, data: &[u8], usage: u32) -> () {
        unsafe {
            self.bind(gl);
            gl.buffer_data_u8_slice(self.target.to_u32(), data, usage)
        }
    }

    pub fn free(&self, gl: &Context) -> () {
        unsafe { gl.delete_buffer(self.buffer) }
    }
}

impl VertexArrayObject {
    pub fn create(
        gl: &Context,
        vertex_buffer: VertexBufferObject,
        index_buffer: IndexBufferObject,
    ) -> Self {
        unsafe {
            let vertex_array = gl.create_vertex_array().unwrap();

            Self {
                vertex_array,
                vertex_buffer,
                index_buffer,
            }
        }
    }

    pub fn bind(&self, gl: &Context) -> () {
        unsafe {
            gl.bind_vertex_array(Some(self.vertex_array));
        }
    }

    pub fn vertex_attribute(&self, gl: &Context, index: u32, pointer_type: VertexAttributePointerType, vertex_size: i32, offset: i32) {
        unsafe {
            let float_size = std::mem::size_of::<f32> as i32;

            gl.enable_vertex_array_attrib(self.vertex_array, index);

            gl.vertex_attrib_pointer_f32(
                index,
                float_size,
                pointer_type.to_u32(),
                false,
                vertex_size * float_size,
                offset * float_size,
            );
        }
    }
}

pub enum BufferTarget {
    ParameterBuffer = 0x80EE,
    ArrayBuffer = 0x8892,
    ElementArrayBuffer = 0x8893,
    PixelPackBuffer = 0x88EB,
    PixelUnpackBuffer = 0x88EC,
    UniformBuffer = 0x8A11,
    TextureBuffer = 0x8C2A,
    TransformFeedbackBuffer = 0x8C8E,
    CopyReadBuffer = 0x8F36,
    CopyWriteBuffer = 0x8F37,
    DrawIndirectBuffer = 0x8F3F,
    ShaderStorageBuffer = 0x90D2,
    DispatchIndirectBuffer = 0x90EE,
    QueryBuffer = 0x9192,
    AtomicCounterBuffer = 0x92C0,
}

pub enum BufferUsage {
    StreamDraw,
    StreamRead,
    StreamCopy,
    StaticDraw,
    StaticRead,
    StaticCopy,
    DynamicDraw,
    DynamicRead,
    DynamicCopy,
}

impl BufferUsage {
    pub fn to_u32(&self) -> u32 {
        match self {
            BufferUsage::StreamDraw => STREAM_DRAW,
            BufferUsage::StreamRead => STREAM_READ,
            BufferUsage::StreamCopy => STREAM_COPY,
            BufferUsage::StaticDraw => STATIC_DRAW,
            BufferUsage::StaticRead => STATIC_READ,
            BufferUsage::StaticCopy => STATIC_COPY,
            BufferUsage::DynamicDraw => DYNAMIC_DRAW,
            BufferUsage::DynamicRead => DYNAMIC_READ,
            BufferUsage::DynamicCopy => DYNAMIC_READ,
        }
    }
}

impl BufferTarget {
    pub fn to_u32(&self) -> u32 {
        match self {
            BufferTarget::ParameterBuffer => PARAMETER_BUFFER,
            BufferTarget::ArrayBuffer => ARRAY_BUFFER,
            BufferTarget::ElementArrayBuffer => ELEMENT_ARRAY_BUFFER,
            BufferTarget::PixelPackBuffer => PIXEL_PACK_BUFFER,
            BufferTarget::PixelUnpackBuffer => PIXEL_UNPACK_BUFFER,
            BufferTarget::UniformBuffer => UNIFORM_BUFFER,
            BufferTarget::TextureBuffer => TEXTURE_BUFFER,
            BufferTarget::TransformFeedbackBuffer => TRANSFORM_FEEDBACK_BUFFER,
            BufferTarget::CopyReadBuffer => COPY_READ_BUFFER,
            BufferTarget::CopyWriteBuffer => COPY_WRITE_BUFFER,
            BufferTarget::DrawIndirectBuffer => DRAW_INDIRECT_BUFFER,
            BufferTarget::ShaderStorageBuffer => SHADER_STORAGE_BUFFER,
            BufferTarget::DispatchIndirectBuffer => DISPATCH_INDIRECT_BUFFER,
            BufferTarget::QueryBuffer => QUERY_BUFFER,
            BufferTarget::AtomicCounterBuffer => ATOMIC_COUNTER_BUFFER,
        }
    }
}

impl VertexBufferObject {
    pub fn create(gl: Context, target: BufferTarget) -> Self {
        unsafe {
            let buffer = gl.create_buffer().unwrap();
            VertexBufferObject { buffer }
        }
    }

    pub fn bind(&self, gl: &Context) -> () {
        unsafe {
            gl.bind_buffer(BufferTarget::ArrayBuffer.to_u32(), Some(self.buffer));
        }
    }

    pub fn buffer_data(&self, gl: &Context, data: &[f32], usage: BufferUsage) -> () {
        unsafe {
            self.bind(gl);

            gl.buffer_data_u8_slice(
                BufferTarget::ArrayBuffer.to_u32(),
                cast_slice(&data),
                usage.to_u32(),
            )
        }
    }

    pub fn free(&self, gl: &Context) -> () {
        unsafe { gl.delete_buffer(self.buffer) }
    }
}

impl IndexBufferObject {
    pub fn create(gl: Context, target: BufferTarget) -> Self {
        unsafe {
            let buffer = gl.create_buffer().unwrap();
            IndexBufferObject { buffer }
        }
    }

    pub fn bind(&self, gl: &Context) -> () {
        unsafe {
            gl.bind_buffer(BufferTarget::ElementArrayBuffer.to_u32(), Some(self.buffer));
        }
    }

    pub fn buffer_data(&self, gl: &Context, data: &[u32], usage: BufferUsage) -> () {
        unsafe {
            self.bind(gl);

            gl.buffer_data_u8_slice(
                BufferTarget::ElementArrayBuffer.to_u32(),
                cast_slice(&data),
                usage.to_u32(),
            )
        }
    }

    pub fn free(&self, gl: &Context) -> () {
        unsafe { gl.delete_buffer(self.buffer) }
    }
}

pub enum VertexAttributePointerType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Float,
    Double,
    HalfFloat,
    Fixed,
    Int64Arb,
    Int64NV,
    UnsignedInt64Arb,
    UnsignedInt64NV,
    UnsignedInt2101010Rev,
    UnsignedInt2101010RevExt,
    UnsignedInt10f11f11fRev,
    Int2101010Rev,
}

impl VertexAttributePointerType {
    pub fn to_u32(&self) -> u32 {
        match self {
            VertexAttributePointerType::Byte => BYTE,
            VertexAttributePointerType::UnsignedByte => UNSIGNED_BYTE,
            VertexAttributePointerType::Short => SHORT,
            VertexAttributePointerType::UnsignedShort => UNSIGNED_SHORT,
            VertexAttributePointerType::Int => INT,
            VertexAttributePointerType::UnsignedInt => UNSIGNED_INT,
            VertexAttributePointerType::Float => FLOAT,
            VertexAttributePointerType::Double => DOUBLE,
            VertexAttributePointerType::HalfFloat => HALF_FLOAT,
            VertexAttributePointerType::Fixed => FIXED,
            VertexAttributePointerType::Int64Arb => INT, // TODO fix
            VertexAttributePointerType::Int64NV => INT, // TODO fix
            VertexAttributePointerType::UnsignedInt64Arb => UNSIGNED_INT, // TODO fix
            VertexAttributePointerType::UnsignedInt64NV => UNSIGNED_INT,
            VertexAttributePointerType::UnsignedInt2101010Rev => UNSIGNED_INT_2_10_10_10_REV,
            VertexAttributePointerType::UnsignedInt2101010RevExt => UNSIGNED_INT_2_10_10_10_REV,
            VertexAttributePointerType::UnsignedInt10f11f11fRev => UNSIGNED_INT_10F_11F_11F_REV,
            VertexAttributePointerType::Int2101010Rev => INT_2_10_10_10_REV
        }
    }
}
