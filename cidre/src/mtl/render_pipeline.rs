use std::ops::{Index, IndexMut};

use crate::{arc, define_mtl, define_obj_type, ns, objc};

use super::{argument::Argument, Function, PixelFormat};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum BlendFactor {
    Zero = 0,
    One = 1,
    SourceColor = 2,
    OneMinusSourceColor = 3,
    SourceAlpha = 4,
    OneMinusSourceAlpha = 5,
    DestinationColor = 6,
    OneMinusDestinationColor = 7,
    DestinationAlpha = 8,
    OneMinusDestinationAlpha = 9,
    SourceAlphaSaturated = 10,
    BlendColor = 11,
    OneMinusBlendColor = 12,
    BlendAlpha = 13,
    OneMinusBlendAlpha = 14,
    Source1Color = 15,
    OneMinusSource1Color = 16,
    Source1Alpha = 17,
    OneMinusSource1Alpha = 18,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum BlendOperation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum ColorWriteMask {
    None = 0,
    Red = 0x1 << 3,
    Green = 0x1 << 2,
    Blue = 0x1 << 1,
    Alpha = 0x1 << 0,
    All = 0xf,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum PrimitiveTopologyClass {
    Unspecified = 0,
    Point = 1,
    Line = 2,
    Triangle = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TessellationPartitionMode {
    Pow2 = 0,
    Integer = 1,
    FractionalOdd = 2,
    FractionalEven = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TessellationFactorStepFunction {
    Constant = 0,
    PerPatch = 1,
    PerInstance = 2,
    PerPatchAndPerInstance = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TessellationFactorFormat {
    MTLTessellationFactorFormatHalf = 0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TessellationControlPointIndexType {
    None = 0,
    U16 = 1,
    U32 = 2,
}

define_obj_type!(ColorAttachmentDescriptor(ns::Id));

impl ColorAttachmentDescriptor {
    #[objc::msg_send(pixelFormat)]
    pub fn pixel_format(&self) -> PixelFormat;

    #[objc::msg_send(setPixelFormat:)]
    pub fn set_pixel_format(&mut self, value: PixelFormat);

    #[objc::msg_send(isBlendingEnabled)]
    pub fn blening_enabled(&self) -> bool;

    #[objc::msg_send(setBlendingEnabled:)]
    pub fn set_blening_enabled(&mut self, value: bool);

    #[objc::msg_send(sourceRGBBlendFactor)]
    pub fn src_rgb_blend_factor(&self) -> BlendFactor;

    #[objc::msg_send(setSourceRGBBlendFactor:)]
    pub fn set_src_rgb_blend_factor(&mut self, value: BlendFactor);

    #[objc::msg_send(destinationRGBBlendFactor)]
    pub fn dest_rgb_blend_factor(&self) -> BlendFactor;

    #[objc::msg_send(setDestinationRGBBlendFactor:)]
    pub fn set_dest_rgb_blend_factor(&mut self, value: BlendFactor);

    #[objc::msg_send(rgbBlendOperation)]
    pub fn rgb_blend_operation(&self) -> BlendOperation;

    #[objc::msg_send(setRgbBlendOperation:)]
    pub fn set_rgb_blend_operation(&mut self, value: BlendOperation);

    #[objc::msg_send(sourceAlphaBlendFactor)]
    pub fn src_alpha_blend_factor(&self) -> BlendFactor;

    #[objc::msg_send(setSourceAlphaBlendFactor:)]
    pub fn set_src_alpha_blend_factor(&mut self, value: BlendFactor);

    #[objc::msg_send(destinationAlphaBlendFactor)]
    pub fn dest_alpha_blend_factor(&self) -> BlendFactor;

    #[objc::msg_send(setDestinationAlphaBlendFactor:)]
    pub fn set_dest_alpha_blend_factor(&mut self, value: BlendFactor);

    #[objc::msg_send(alphaBlendOperation)]
    pub fn alpha_blend_operation(&self) -> BlendOperation;

    #[objc::msg_send(setAlphaBlendOperation:)]
    pub fn alpha_rgb_blend_operation(&mut self, value: BlendOperation);

    #[objc::msg_send(writeMask)]
    pub fn write_mask(&self) -> ColorWriteMask;

    #[objc::msg_send(setWriteMask:)]
    pub fn set_write_mask(&mut self, value: ColorWriteMask);
}

define_obj_type!(Reflection(ns::Id));

impl Reflection {
    #[objc::msg_send(vertexArguments)]
    pub fn vertex_arguments(&self) -> Option<&ns::Array<Argument>>;

    #[objc::msg_send(fragmentArguments)]
    pub fn fragment_arguments(&self) -> Option<&ns::Array<Argument>>;

    #[objc::msg_send(tileArguments)]
    pub fn tile_arguments(&self) -> Option<&ns::Array<Argument>>;
}

define_obj_type!(Descriptor(ns::Id));

impl Descriptor {
    define_mtl!(reset);

    /// ```no_run
    /// use cidre::{cf, mtl};
    ///
    /// let mut desc = mtl::RenderPipelineDescriptor::new();
    ///
    /// assert!(desc.vertex_function().is_none());
    /// assert!(desc.fragment_function().is_none());
    ///
    /// desc.reset();
    /// ```
    pub fn new() -> arc::R<Descriptor> {
        unsafe { MTLRenderPipelineDescriptor_new() }
    }

    #[objc::msg_send(vertexFunction)]
    pub fn vertex_function(&self) -> Option<&Function>;

    #[objc::msg_send(setVertexFunction:)]
    pub fn set_vertex_function(&mut self, value: Option<&Function>);

    #[inline]
    pub fn set_vertex_fn(&mut self, value: &Function) {
        self.set_vertex_function(Some(value))
    }

    #[objc::msg_send(fragmentFunction)]
    pub fn fragment_function(&self) -> Option<&Function>;

    #[objc::msg_send(setFragmentFunction:)]
    pub fn set_fragment_function(&mut self, value: Option<&Function>);

    #[inline]
    pub fn set_fragment_fn(&mut self, value: &Function) {
        self.set_fragment_function(Some(value))
    }

    #[objc::msg_send(colorAttachments)]
    pub fn color_attachments(&self) -> &ColorAttachmentDescriptorArray;

    #[objc::msg_send(colorAttachments)]
    pub fn color_attachments_mut(&mut self) -> &mut ColorAttachmentDescriptorArray;

    #[objc::msg_send(rasterSampleCount)]
    pub fn raster_sample_count(&self) -> usize;

    #[objc::msg_send(setRasterSampleCount:)]
    pub fn set_raster_sample_count(&mut self, value: usize);
}
#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLRenderPipelineDescriptor_new() -> arc::R<Descriptor>;
}

define_obj_type!(FunctionsDescriptor(ns::Id));

define_obj_type!(State(ns::Id));

impl State {
    define_mtl!(device, label, gpu_resouce_id);

    #[objc::msg_send(maxTotalThreadsPerThreadgroup)]
    pub fn max_total_threads_per_threadgroup(&self) -> usize;

    #[objc::msg_send(threadgroupSizeMatchesTileSize)]
    pub fn threadgourp_size_matches_tile_size(&self) -> bool;

    #[objc::msg_send(imageblockSampleLength)]
    pub fn imageblock_sample_length(&self) -> usize;
}

define_obj_type!(ColorAttachmentDescriptorArray(ns::Id));

impl ColorAttachmentDescriptorArray {
    #[inline]
    pub fn get_at(&self, index: usize) -> &ColorAttachmentDescriptor {
        unsafe {
            MTLRenderPipelineColorAttachmentDescriptorArray_rsel_objectAtIndexedSubscript(
                self, index,
            )
        }
    }

    #[inline]
    pub fn get_mut_at(&mut self, index: usize) -> &mut ColorAttachmentDescriptor {
        unsafe {
            MTLRenderPipelineColorAttachmentDescriptorArray_rsel_objectAtIndexedSubscript(
                self, index,
            )
        }
    }

    #[inline]
    pub fn set_at(&mut self, index: usize, value: &ColorAttachmentDescriptor) {
        unsafe {
            MTLRenderPipelineColorAttachmentDescriptorArray_wsel_setObjectAtIndexedSubscript(
                self,
                Some(value),
                index,
            )
        }
    }

    #[inline]
    pub fn reset_at(&mut self, index: usize) {
        unsafe {
            MTLRenderPipelineColorAttachmentDescriptorArray_wsel_setObjectAtIndexedSubscript(
                self, None, index,
            )
        }
    }
}

impl Index<usize> for ColorAttachmentDescriptorArray {
    type Output = ColorAttachmentDescriptor;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get_at(index)
    }
}

impl IndexMut<usize> for ColorAttachmentDescriptorArray {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut_at(index)
    }
}

extern "C" {

    fn MTLRenderPipelineColorAttachmentDescriptorArray_rsel_objectAtIndexedSubscript(
        id: &ns::Id,
        index: usize,
    ) -> &mut ColorAttachmentDescriptor;

    fn MTLRenderPipelineColorAttachmentDescriptorArray_wsel_setObjectAtIndexedSubscript(
        id: &mut ns::Id,
        value: Option<&ColorAttachmentDescriptor>,
        index: usize,
    );
}

define_obj_type!(TileRenderPipelineColorAttachmentDescriptor(ns::Id));
define_obj_type!(TileRenderPipelineColorAttachmentDescriptorArray(ns::Id));

define_obj_type!(TileRenderPipelineDescriptor(ns::Id));

impl TileRenderPipelineDescriptor {
    define_mtl!(reset);
}

define_obj_type!(MeshRenderPipelineDescriptor(ns::Id));

impl MeshRenderPipelineDescriptor {
    define_mtl!(reset, label, set_label);
}

#[cfg(test)]
mod tests {
    use crate::mtl;
    #[test]
    fn basics() {
        let mut desc = mtl::RenderPipelineDescriptor::new();

        assert!(desc.vertex_function().is_none());
        assert!(desc.fragment_function().is_none());

        desc.reset();
    }
}
