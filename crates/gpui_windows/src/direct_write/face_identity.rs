use super::{DirectWriteTextSystem, RendererContext, StringIndexConverter};
use anyhow::Result;
use gpui::{FontId, GlyphId, PlatformTextSystem, RenderGlyphParams, point, px};
use std::mem::ManuallyDrop;
use windows::Win32::Graphics::DirectWrite::*;
use windows::core::{IUnknown, Interface, w};

// Exercise the renderer callback and the same glyph-run builder used by both raster paths.
fn draw_face(
    system: &DirectWriteTextSystem,
    face: &IDWriteFontFace3,
) -> Result<(FontId, IUnknown)> {
    let mut state = system.state.write();
    let factory = &system.components.factory;
    let layout = unsafe {
        let format = factory.CreateTextFormat(
            w!("Arial"),
            &state.system_font_collection,
            DWRITE_FONT_WEIGHT_NORMAL,
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            14.0,
            &system.components.locale,
        )?;
        factory.CreateTextLayout(&[u16::from(b'A')], &format, 64., 64.)?
    };
    let mut glyph_id = 0;
    unsafe { face.GetGlyphIndices(&u32::from('A'), 1, &mut glyph_id)? };
    assert_ne!(glyph_id, 0);
    let face: &IDWriteFontFace = face;
    let run = DWRITE_GLYPH_RUN {
        // The caller owns the COM reference throughout this synchronous callback.
        fontFace: ManuallyDrop::new(Some(unsafe { std::ptr::read(face) })),
        fontEmSize: 14.0,
        glyphCount: 1,
        glyphIndices: &glyph_id,
        glyphAdvances: &8.0,
        glyphOffsets: &DWRITE_GLYPH_OFFSET::default(),
        ..Default::default()
    };
    let description = DWRITE_GLYPH_RUN_DESCRIPTION {
        string: w!("A"),
        stringLength: 1,
        clusterMap: &0,
        ..Default::default()
    };
    let mut runs = Vec::new();
    let mut context = RendererContext {
        text_system: &mut state,
        components: &system.components,
        index_converter: StringIndexConverter::new("A"),
        runs: &mut runs,
        layout: &layout,
        width: 0.0,
    };
    unsafe {
        system.components.text_renderer.0.DrawGlyphRun(
            Some((&raw mut context).cast()),
            0.0,
            0.0,
            DWRITE_MEASURING_MODE_NATURAL,
            &run,
            &description,
            None,
        )?;
    }
    assert_eq!(runs.len(), 1);
    let font_id = runs[0].font_id;
    assert_eq!(runs[0].glyphs[0].id, GlyphId(u32::from(glyph_id)));
    let params = RenderGlyphParams {
        font_id,
        glyph_id: runs[0].glyphs[0].id,
        font_size: px(14.),
        subpixel_variant: point(0, 0),
        scale_factor: 1.,
        is_emoji: false,
        subpixel_rendering: false,
        dilation: 0,
    };
    let raster_face = state.with_glyph_run(&params, |run, _| {
        run.fontFace.as_ref().unwrap().cast::<IUnknown>()
    })?;
    Ok((font_id, raster_face))
}

#[test]
fn shaped_run_keeps_simulated_face() -> Result<()> {
    let system = DirectWriteTextSystem::new()?;
    let base_id = system.font_id(&gpui::font("Arial"))?;
    let simulated = unsafe {
        system.state.read().fonts[base_id.0]
            .font_face
            .GetFontFaceReference()?
            .CreateFontFaceWithSimulations(DWRITE_FONT_SIMULATIONS_OBLIQUE)?
    };
    assert_eq!(
        unsafe { simulated.GetSimulations() },
        DWRITE_FONT_SIMULATIONS_OBLIQUE
    );
    let (id, raster_face) = draw_face(&system, &simulated)?;
    assert_eq!(raster_face, simulated.cast::<IUnknown>()?);
    assert_ne!(id, base_id);
    assert_eq!(draw_face(&system, &simulated)?.0, id);
    Ok(())
}

#[test]
fn variable_weights_reach_rasterizer() -> Result<()> {
    let system = DirectWriteTextSystem::new()?;
    if system.components.factory.cast::<IDWriteFactory6>().is_err() {
        return Ok(());
    }
    let base = system.font_id(&gpui::font("Bahnschrift"))?;
    let face = system.state.read().fonts[base.0]
        .font_face
        .cast::<IDWriteFontFace5>()?;
    let resource = unsafe { face.GetFontResource()? };
    let mut ids = Vec::new();
    for weight in [550., 650.] {
        let axis = DWRITE_FONT_AXIS_VALUE {
            axisTag: DWRITE_FONT_AXIS_TAG_WEIGHT,
            value: weight,
        };
        let face = unsafe { resource.CreateFontFace(DWRITE_FONT_SIMULATIONS_NONE, &[axis])? };
        let mut axes = vec![
            DWRITE_FONT_AXIS_VALUE::default();
            unsafe { face.GetFontAxisValueCount() } as usize
        ];
        unsafe { face.GetFontAxisValues(&mut axes)? };
        assert!(axes.contains(&axis));
        let (id, raster_face) = draw_face(&system, &face.cast()?)?;
        assert_eq!(raster_face, face.cast::<IUnknown>()?);
        ids.push(id);
    }
    assert_ne!(ids[0], ids[1]);
    Ok(())
}
