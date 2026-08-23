use crate::maths::{Float2, Float3, Float4};
use bytemuck::cast_slice;
use glam::f32::Mat4;
use glow::{
    ARRAY_BUFFER, ATOMIC_COUNTER_BUFFER, BYTE, CLAMP_TO_EDGE, COPY_READ_BUFFER, COPY_WRITE_BUFFER,
    Context, DISPATCH_INDIRECT_BUFFER, DOUBLE, DRAW_INDIRECT_BUFFER, DYNAMIC_DRAW, DYNAMIC_READ,
    ELEMENT_ARRAY_BUFFER, FIXED, FLOAT, HALF_FLOAT, HasContext, INT, INT_2_10_10_10_REV, LINEAR,
    LINEAR_MIPMAP_LINEAR, NativeBuffer, NativeProgram, NativeTexture, NativeUniformLocation,
    NativeVertexArray, PARAMETER_BUFFER, PIXEL_PACK_BUFFER, PIXEL_UNPACK_BUFFER, QUERY_BUFFER,
    RGBA, SHADER_STORAGE_BUFFER, SHORT, STATIC_COPY, STATIC_DRAW, STATIC_READ, STREAM_COPY,
    STREAM_DRAW, STREAM_READ, TEXTURE_2D, TEXTURE_BASE_LEVEL, TEXTURE_BUFFER, TEXTURE_MAG_FILTER,
    TEXTURE_MAX_LEVEL, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T, TEXTURE0,
    TRANSFORM_FEEDBACK_BUFFER, UNIFORM_BUFFER, UNSIGNED_BYTE, UNSIGNED_INT,
    UNSIGNED_INT_2_10_10_10_REV, UNSIGNED_INT_10F_11F_11F_REV, UNSIGNED_SHORT,
};
use uuid::Uuid;
use crate::rendering::geometry::VertexCollection;

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

            println!(
                "VAO created with VBO={:?} IBO={:?}",
                vertex_buffer.buffer,
                index_buffer.buffer
            );

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
            // IMPORTANT:
            // Some drivers do NOT store VBO/IBO bindings inside the VAO.
            // If we don't re-bind the vertex + index buffers before each draw,
            // the second model's VAO will overwrite the first model's buffer state,
            // causing only the last model to render.
            // Rebinding here guarantees each VAO uses its own buffers correctly.
            self.vertex_buffer.bind(gl);
            self.index_buffer.bind(gl);
        }
    }

    pub fn buffer_data(&self, gl: &Context, vertices_collection: &VertexCollection, usage: BufferUsage) -> () {
        unsafe {
            gl.bind_vertex_array(Some(self.vertex_array));

            self.vertex_buffer.bind(gl);

            self.vertex_buffer.buffer_data(
                gl,
                vertices_collection.data_as_slice(),
                &usage
            );

            self.index_buffer.bind(gl);

            self.index_buffer.buffer_data(
                gl,
                vertices_collection.indices_as_slice(),
                &usage,
            );

            let mut offset = 0;
            let mut index = 0;

            let vertex_size = vertices_collection.vertex_size();

            let float_size = std::mem::size_of::<f32>() as i32;
            let vertex_size = vertices_collection.vertex_size(); // 3

            // TODO - uncomment
            // Hard‑wired position attribute at location 0
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                3,                          // vec3 position
                glow::FLOAT,
                false,
                12,   // 3 * 4 = 12
                0,                          // offset 0
            );

            /*
            for attribute in vertices_collection.get_layout_items() {
                if attribute.active {
                    self.set_vertex_attribute(
                        gl,
                        index,
                        attribute.count as i32,
                        VertexAttributePointerType::Float,
                        vertex_size,
                        offset,
                    );
                    index = index + 1;
                    offset = offset + attribute.count as i32;
                }
            }
            */

            gl.bind_vertex_array(None);
        }


    }

    pub fn set_vertex_attribute(
        &self,
        gl: &Context,
        index: u32,
        count: i32,
        pointer_type: VertexAttributePointerType,
        vertex_size: i32,
        offset: i32,
    ) {
        unsafe {
            let float_size = std::mem::size_of::<f32>() as i32;

            gl.bind_vertex_array(Some(self.vertex_array));
            self.vertex_buffer.bind(gl);
            self.index_buffer.bind(gl);


            gl.enable_vertex_attrib_array(index);


            gl.vertex_attrib_pointer_f32(
                index,
                count,
                pointer_type.to_u32(),
                false,
                vertex_size * float_size,
                offset * float_size,
            );

            println!(
                "attrib {}: count={}, stride={}, offset={}",
                index,
                count,
                vertex_size * float_size,
                offset * float_size
            );

            //let attrib = gl.ve get_vertex_attrib(index, glow::VERTEX_ATTRIB_ARRAY_POINTER);
            //println!("attrib {} pointer = {:?}", index, attrib);
        }
    }

    pub fn free(&self, gl: &Context) {
        unsafe {
            self.vertex_buffer.free(gl);
            self.index_buffer.free(gl);
            gl.delete_vertex_array(self.vertex_array)
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
    pub fn create(gl: &Context) -> Self {
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

    pub fn buffer_data(&self, gl: &Context, data: &[f32], usage: &BufferUsage) -> () {
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
    pub fn create(gl: &Context) -> Self {
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

    pub fn buffer_data(&self, gl: &Context, data: &[u32], usage: &BufferUsage) -> () {
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
            VertexAttributePointerType::Int64NV => INT,  // TODO fix
            VertexAttributePointerType::UnsignedInt64Arb => UNSIGNED_INT, // TODO fix
            VertexAttributePointerType::UnsignedInt64NV => UNSIGNED_INT,
            VertexAttributePointerType::UnsignedInt2101010Rev => UNSIGNED_INT_2_10_10_10_REV,
            VertexAttributePointerType::UnsignedInt2101010RevExt => UNSIGNED_INT_2_10_10_10_REV,
            VertexAttributePointerType::UnsignedInt10f11f11fRev => UNSIGNED_INT_10F_11F_11F_REV,
            VertexAttributePointerType::Int2101010Rev => INT_2_10_10_10_REV,
        }
    }
}

pub enum PrimitiveType {
    Points,
    Lines,
    LineLoop,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan,
    Quads,
    QuadsExt,
    LinesAdjacency,
    LinesAdjacencyArb,
    LinesAdjacencyExt,
    LineStripAdjacency,
    LineStripAdjacencyArb,
    LineStripAdjacencyExt,
    TrianglesAdjacency,
    TrianglesAdjacencyArb,
    TrianglesAdjacencyExt,
    TriangleStripAdjacency,
    TriangleStripAdjacencyArb,
    TriangleStripAdjacencyExt,
    Patches,
    PatchesExt,
}

impl PrimitiveType {
    pub fn to_u32(&self) -> u32 {
        match self {
            PrimitiveType::Points => glow::POINTS,
            PrimitiveType::Lines => glow::LINES,
            PrimitiveType::LineLoop => glow::LINE_LOOP,
            PrimitiveType::LineStrip => glow::LINE_STRIP,
            PrimitiveType::Triangles => glow::TRIANGLES,
            PrimitiveType::TriangleStrip => glow::TRIANGLE_STRIP,
            PrimitiveType::TriangleFan => glow::TRIANGLE_FAN,
            PrimitiveType::Quads => glow::QUADS,
            PrimitiveType::QuadsExt => glow::QUADS,
            PrimitiveType::LinesAdjacency => glow::LINES_ADJACENCY,
            PrimitiveType::LinesAdjacencyArb => glow::LINES_ADJACENCY,
            PrimitiveType::LinesAdjacencyExt => glow::LINES_ADJACENCY,
            PrimitiveType::LineStripAdjacency => glow::LINE_STRIP_ADJACENCY,
            PrimitiveType::LineStripAdjacencyArb => glow::LINE_STRIP_ADJACENCY,
            PrimitiveType::LineStripAdjacencyExt => glow::LINE_STRIP_ADJACENCY,
            PrimitiveType::TrianglesAdjacency => glow::TRIANGLES_ADJACENCY,
            PrimitiveType::TrianglesAdjacencyArb => glow::TRIANGLES_ADJACENCY,
            PrimitiveType::TrianglesAdjacencyExt => glow::TRIANGLES_ADJACENCY,
            PrimitiveType::TriangleStripAdjacency => glow::TRIANGLE_STRIP_ADJACENCY,
            PrimitiveType::TriangleStripAdjacencyArb => glow::TRIANGLE_STRIP_ADJACENCY,
            PrimitiveType::TriangleStripAdjacencyExt => glow::TRIANGLE_STRIP_ADJACENCY,
            PrimitiveType::Patches => glow::PATCHES,
            PrimitiveType::PatchesExt => glow::PATCHES,
        }
    }
}

pub enum DrawElementType {
    UnsignedByte,
    UnsignedShort,
    UnsignedInt,
}

impl DrawElementType {
    pub fn to_u32(&self) -> u32 {
        match self {
            DrawElementType::UnsignedByte => glow::UNSIGNED_BYTE,
            DrawElementType::UnsignedShort => glow::UNSIGNED_SHORT,
            DrawElementType::UnsignedInt => glow::UNSIGNED_INT,
        }
    }
}
