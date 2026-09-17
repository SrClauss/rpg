use image::{DynamicImage, ImageOutputFormat, Rgba};
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut};
use rusttype::{Font, Scale};
use std::error::Error;
use std::io::Cursor;

/// Recebe os bytes da imagem PNG original gerada pelo Gemini, a largura e altura do grid hexagonal em células (q, r),
/// e desenha a sobreposição (overlay) gráfica dos hexágonos sobre a imagem juntamente com o rótulo de coordenadas (q, r).
/// Retorna os bytes em formato PNG da imagem gradeada.
pub fn draw_hex_grid_overlay(
    image_bytes: &[u8],
    width_hexes: usize,
    height_hexes: usize,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut img = image::load_from_memory(image_bytes)?.to_rgba8();
    let (img_w, img_h) = img.dimensions();

    let cols = width_hexes.max(1) as f32;
    let rows = height_hexes.max(1) as f32;

    // Raio do hexágono derivado do tamanho da imagem
    let hex_radius = (img_w as f32 / (cols * 1.7320508 + 0.8660254))
        .min(img_h as f32 / (rows * 1.5 + 0.5));

    let draw_width_hexes = ((img_w as f32 / (1.7320508 * hex_radius)).ceil() as usize) + 2;
    let draw_height_hexes = ((img_h as f32 / (1.5 * hex_radius)).ceil() as usize) + 2;

    let grid_color = Rgba([255, 255, 255, 160]); // Linha branca semi-transparente
    let text_color = Rgba([255, 255, 0, 230]);   // Texto amarelo legível

    // Tenta carregar uma fonte do sistema para desenhar os rótulos de coordenadas
    let font_bytes = std::fs::read("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")
        .or_else(|_| std::fs::read("/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf"))
        .ok();
    let font = font_bytes.as_ref().and_then(|bytes| Font::try_from_bytes(bytes));

    let scale = Scale::uniform((hex_radius * 0.4).max(10.0));

    for r in 0..draw_height_hexes {
        for q in 0..draw_width_hexes {
            // Centro do hexágono em coordenadas da tela (pointy-topped)
            let offset_x = if r % 2 != 0 { 0.8660254 * hex_radius } else { 0.0 };
            let center_x = (q as f32 * 1.7320508 * hex_radius) + offset_x + (1.7320508 * hex_radius / 2.0);
            let center_y = (r as f32 * 1.5 * hex_radius) + hex_radius;

            // Calcula os 6 vértices do hexágono
            let mut vertices = Vec::with_capacity(6);
            for i in 0..6 {
                let angle_rad = (60.0 * i as f32 - 30.0).to_radians();
                let vx = center_x + hex_radius * angle_rad.cos();
                let vy = center_y + hex_radius * angle_rad.sin();
                vertices.push((vx as f32, vy as f32));
            }

            // Desenha as 6 arestas do hexágono
            for i in 0..6 {
                let (x1, y1) = vertices[i];
                let (x2, y2) = vertices[(i + 1) % 6];

                draw_line_segment_mut(
                    &mut img,
                    (x1 as f32, y1 as f32),
                    (x2 as f32, y2 as f32),
                    grid_color,
                );
            }

            // Desenha o rótulo da coordenada (q, r) no centro do hexágono se a fonte estiver disponível
            if let Some(font_ref) = &font {
                let label = format!("{},{}", q, r);
                let text_x = (center_x - (label.len() as f32 * scale.x * 0.28)) as i32;
                let text_y = (center_y - (scale.y * 0.5)) as i32;

                if text_x >= 0 && text_y >= 0 && (text_x as u32) < img_w && (text_y as u32) < img_h {
                    draw_text_mut(
                        &mut img,
                        text_color,
                        text_x,
                        text_y,
                        scale,
                        font_ref,
                        &label,
                    );
                }
            }
        }
    }

    // Codifica a imagem modificada para PNG em memória
    let mut buffer = Vec::new();
    let dynamic_img = DynamicImage::ImageRgba8(img);
    dynamic_img.write_to(&mut Cursor::new(&mut buffer), ImageOutputFormat::Png)?;

    Ok(buffer)
}


