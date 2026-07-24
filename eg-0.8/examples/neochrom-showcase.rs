//! STM32N6 NeoChrom GPU hardware acceleration showcase for `embedded-graphics`.
//!
//! Demonstrates hardware-accelerated 2D drawing primitives, affine transformations
//! (rotation around pivot, sub-rectangle scaling, 3D perspective quad warping),
//! Porter-Duff alpha blending, global opacity, hardware scissors clipping, and A8 font glyph blitting.

use embassy_stm32_neochrom::{BlendMode, FrameBuffer, NeoChrom};
use embedded_graphics::neochrom::NeochromDisplay;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};

fn main() -> Result<(), embassy_stm32_neochrom::Error> {
    // Initialize NeoChrom GPU driver (stub HAL in test mode)
    let mut gpu = NeoChrom::new().unwrap();

    // Allocate 800x480 RGBA8888 destination framebuffer and 128x128 sprite buffer
    let fb: FrameBuffer<800, 480, { 800 * 480 }> = FrameBuffer::new();
    let sprite_fb: FrameBuffer<128, 128, { 128 * 128 }> = FrameBuffer::new();

    // Create GPU-accelerated DrawTarget wrapper
    let mut display = NeochromDisplay::new(&mut gpu, &fb);

    // 1. Hardware Screen Clear (1 command list)
    display.clear(Rgb888::new(15, 23, 42))?; // Dark slate blue

    // 2. Hardware Primitives & Standard DrawTarget Integration
    // Filled rectangle background card
    Rectangle::new(Point::new(40, 40), Size::new(720, 400))
        .into_styled(PrimitiveStyle::with_fill(Rgb888::new(30, 41, 59)))
        .draw(&mut display)?;

    // GPU-accelerated rounded card
    display.fill_rounded_rect(60, 60, 320, 180, 20, Rgb888::new(51, 65, 85))?;

    // GPU-accelerated circle (gauge display)
    display.fill_circle(220, 150, 60, Rgb888::new(14, 165, 233))?;

    // GPU-accelerated triangle (vector accent)
    display.fill_triangle(420, 220, 480, 100, 540, 220, Rgb888::new(244, 63, 94))?;

    // GPU-accelerated lines
    display.draw_line(60, 260, 380, 260, Rgb888::new(148, 163, 184))?;

    // 3. Hardware Affine Transformations
    // Rotate sprite around custom center & pivot point (e.g. gauge needle at 45 degrees)
    display.blit_rotate_pivot(&sprite_fb, 220.0, 150.0, 64.0, 12.0, 45.0)?;

    // Crop sub-rectangle from texture atlas and scale to destination rect
    display.blit_subrect_fit(&sprite_fb, 420, 260, 160, 120, 0, 0, 64, 64)?;

    // 3D perspective quad warping (2.5D page flip transition)
    display.blit_quad_fit(
        &sprite_fb,
        600.0, 60.0,   // Top-left
        740.0, 80.0,   // Top-right
        720.0, 220.0,  // Bottom-right
        610.0, 200.0,  // Bottom-left
    )?;

    // 4. Hardware Alpha Blending & Global Opacity
    // Set 50% opacity translucent modal overlay
    display.set_const_color(Rgb888::WHITE, 128);
    display.set_blend_fill(BlendMode::Simple);
    display.fill_rounded_rect(200, 300, 400, 120, 16, Rgb888::new(15, 23, 42))?;

    // Reset global opacity back to 100% and enable Additive blending for glow effect
    display.set_const_color(Rgb888::WHITE, 255);
    display.set_blend_fill(BlendMode::Add);
    display.fill_circle(400, 360, 30, Rgb888::new(168, 85, 247))?;
    display.set_blend_fill(BlendMode::Simple); // Restore simple blend

    // 5. Hardware Scissors Clipping Rectangle
    // Restrict drawing to a 300x80 scroll window
    display.set_clip(250, 320, 300, 80);
    display.fill_solid(&Rectangle::new(Point::new(200, 300), Size::new(400, 120)), Rgb888::new(34, 197, 94))?;
    display.reset_clip(); // Reset clipping to full screen

    Ok(())
}
