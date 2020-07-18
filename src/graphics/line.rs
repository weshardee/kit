use crate::graphics::*;
use crate::math::*;
use crate::*;
use std::mem::size_of;

pub fn draw_line(ctx: &mut Ctx, a: V3, b: V3, color: V4) {
    // TODO there's probably a more idiomatic Rust way to do this, or a library I can use...
    let i = ctx.gl.lines.count;
    ctx.gl.lines.count += 1;
    ctx.gl.lines.e[i] = DrawLine {
        point_a: v3(a.x, a.y, 0.0),
        color_a: color,
        point_b: v3(b.x, b.y, 0.0),
        color_b: color,
    };
}

pub fn init(ctx: &mut Ctx) {
    // TODO common primitives (line, point, maybe others?) could share the same shaders
    let (vs_src, fs_src) = match sg_api() {
        SgApi::OpenGL33 => (
            include_str!("line.vert.glsl"),
            include_str!("line.frag.glsl"),
        ),
        SgApi::Metal => (include_str!("line.vs.metal"), include_str!("line.fs.metal")),
        _ => panic!(),
    };
    let pipeline = sg_make_pipeline(&SgPipelineDesc {
        // index_type: SgIndexType::UInt32,
        primitive_type: SgPrimitiveType::Lines, // TODO replace with line strip?
        shader: sg_make_shader(&SgShaderDesc {
            vs: SgShaderStageDesc {
                source: Some(vs_src),
                uniform_blocks: vec![std_uniform_block()],
                ..Default::default()
            },
            fs: SgShaderStageDesc {
                source: Some(fs_src),
                ..Default::default()
            },
            attrs: vec![],
        }),
        layout: SgLayoutDesc {
            attrs: vec![
                SgVertexAttrDesc {
                    // name : "in_position",
                    format: SgVertexFormat::Float3,
                    ..Default::default()
                },
                SgVertexAttrDesc {
                    // name : "in_color",
                    format: SgVertexFormat::Float4,
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        // depth_stencil: SgDepthStencilState {
        //     depth_compare_func: SgCompareFunc::LessEqual,
        //     depth_write_enabled: true,
        //     ..Default::default()
        // },
        // blend: SgBlendState {
        //     enabled: true,
        //     color_format: SgPixelFormat::RGBA8,
        //     depth_format: SgPixelFormat::Depth,
        //     dst_factor_rgb: SgBlendFactor::OneMinusSrcAlpha,
        //     ..Default::default()
        // },
        ..Default::default()
    });
    let bindings = SgBindings {
        vertex_buffers: vec![sg_make_buffer::<()>(
            None,
            &SgBufferDesc {
                size: BYTES_LINES,
                usage: SgUsage::Stream,
                ..Default::default()
            },
        )],
        ..Default::default()
    };
    ctx.gl.lines.shape = GlShape { bindings, pipeline };
}

pub fn present(ctx: &mut Ctx) {
    sg_update_buffer(
        ctx.gl.lines.shape.bindings.vertex_buffers[0],
        &ctx.gl.lines.e,
        (ctx.gl.lines.count * size_of::<DrawLine>()) as i32,
    );
    sg_apply_pipeline(ctx.gl.lines.shape.pipeline);
    sg_apply_bindings(&ctx.gl.lines.shape.bindings);
    sg_apply_uniforms(
        SgShaderStage::Vertex,
        0,
        &ctx.gl.view_proj,
        size_of::<M4>() as i32,
    );
    sg_draw(0, (ctx.gl.lines.count * 2) as i32, 1);
}