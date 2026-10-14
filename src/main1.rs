use macroquad::prelude::*;

struct Player {
    x: i16,
    y: i16,
    score: i16,
}

struct Sweet {
    x: i16,
    y: i16,
}

const SQUARES: i16 = 15;
#[macroquad::main("MyGame")]
async fn main() {
    let color_bg = Color::from_hex(0x578a34);
    let color_header = Color::from_hex(0x4a752c);
    let color_case_1 = Color::from_hex(0xaad751);
    let color_case_2 = Color::from_hex(0xa2d149);
    let apple: Texture2D = load_texture("src/apple.png").await.unwrap();
    const HEADER_HEIGHT: f32 = 40.;
    let mut player = Player {
        x: 0,
        y: 0,
        score: 0,
    };
    let mut sweets = Vec::new();
    let total_sweets = 10;

    for _ in 0..total_sweets {
        sweets.push(Sweet {
            x: rand::gen_range(0, SQUARES),
            y: rand::gen_range(0, SQUARES),
        });
    }

    loop {
        // logique de déplacement du joueur
        if is_key_pressed(KeyCode::Right) && player.x < SQUARES - 1 {
            player.x += 1;
        }
        if is_key_pressed(KeyCode::Left) && player.x > 0 {
            player.x -= 1;
        }
        if is_key_pressed(KeyCode::Down) && player.y < SQUARES - 1 {
            player.y += 1;
        }
        if is_key_pressed(KeyCode::Up) && player.y > 0 {
            player.y -= 1;
        }

        clear_background(color_bg);

        let available_height = screen_height() - HEADER_HEIGHT;
        let game_size = screen_width().min(available_height) - 20.; // -20 pour de la marge
        let offset_x = (screen_width() - game_size) / 2.;
        let offset_y = HEADER_HEIGHT + (available_height - game_size) / 2.;
        let sq_size = game_size / SQUARES as f32;

        // construction du header
        draw_rectangle(0.0, 0.0, screen_width(), HEADER_HEIGHT, color_header);
        draw_texture(&apple, 0., 0., WHITE);
        draw_text(
            &format!("{}", player.score),
            40.0,
            HEADER_HEIGHT / 2.0 + 5.,
            20.0,
            WHITE,
        );

        // construction du stage
        for row in 0..SQUARES {
            for col in 0..SQUARES {
                // alternance des couleurs
                let color = if (row + col) % 2 == 0 {
                    color_case_1
                } else {
                    color_case_2
                };
                draw_rectangle(
                    offset_x + col as f32 * sq_size,
                    offset_y + row as f32 * sq_size,
                    sq_size,
                    sq_size,
                    color,
                );
            }
        }

        // --- LOGIQUE DE COLLISION AVEC LES BONBONS ---
        // On garde seulement les bonbons qui ne sont pas à la position du joueur
        let initial_count = sweets.len();
        sweets.retain(|s| s.x != player.x || s.y != player.y);

        // Si la taille a diminué, c'est qu'un bonbon a été ramassé
        if sweets.len() < initial_count {
            player.score += 1;
        }

        // --- CONDITION DE FIN ---
        if sweets.is_empty() {
            // Le jeu s'arrête
            draw_text(
                "GAME OVER - TOUS LES BONBONS SONT RAMASSÉS",
                50.0,
                50.0,
                30.0,
                RED,
            );
            // Ici, plus tard, le serveur lancera une nouvelle partie [cite: 29]
        }

        // construction du joueur
        draw_rectangle(
            offset_x + player.x as f32 * sq_size + 5.,
            offset_y + player.y as f32 * sq_size + 5.,
            sq_size - 10.0,
            sq_size - 10.0,
            BLUE,
        );

        let anim_scale = 1.0 + (get_time() * 5.0).sin() as f32 * 0.1;

        for sweet in &sweets {
            let x_pos = offset_x + sweet.x as f32 * sq_size;
            let y_pos = offset_y + sweet.y as f32 * sq_size;

            let animated_size = sq_size * anim_scale;

            let center_offset = (sq_size - animated_size) / 2.0;

            draw_texture_ex(
                &apple,
                x_pos + center_offset,
                y_pos + center_offset,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(animated_size, animated_size)),
                    ..Default::default()
                },
            );
        }

        next_frame().await
    }
}
