use raylib::prelude::*;
use std::collections::HashMap;

// ===========================================
// TIPOS DE BLOQUES: cada caracter del mapa
// corresponde a uno de estos tipos
// ===========================================
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum BlockType {
    Terracotta,
    RedWool,
    WhiteGlass,
    Grass,
    ConcretePowder,
    OakWood,
    GreenWool,
    Leaves,
    MangroveWood,
}

impl BlockType {
    // Convierte un caracter del mapa a BlockType
    fn from_char(c: char) -> Option<Self> {
        match c {
            'T' => Some(BlockType::Terracotta),
            'R' => Some(BlockType::RedWool),
            'W' => Some(BlockType::WhiteGlass),
            'G' => Some(BlockType::Grass),
            'C' => Some(BlockType::ConcretePowder),
            'O' => Some(BlockType::OakWood),
            'V' => Some(BlockType::GreenWool),
            'L' => Some(BlockType::Leaves),
            'M' => Some(BlockType::MangroveWood),
            _ => None,
        }
    }

    // Color base cuando no hay textura
    fn get_color(&self) -> Color {
        match self {
            BlockType::Terracotta => Color::new(160, 83, 65, 255),      // Red terracotta
            BlockType::RedWool => Color::new(176, 46, 38, 255),         // Red wool
            BlockType::WhiteGlass => Color::new(255, 255, 255, 180),    // White glass (semi-transparente)
            BlockType::Grass => Color::new(91, 164, 81, 255),           // Grass green
            BlockType::ConcretePowder => Color::new(169, 168, 159, 255), // Concrete gray
            BlockType::OakWood => Color::new(162, 130, 78, 255),        // Oak wood brown
            BlockType::GreenWool => Color::new(94, 124, 22, 255),       // Green wool
            BlockType::Leaves => Color::new(68, 119, 68, 255),          // Leaves green
            BlockType::MangroveWood => Color::new(117, 86, 71, 255),    // Mangrove wood
        }
    }

    // Ruta esperada para la textura en disco
    fn get_texture_path(&self) -> &'static str {
        match self {
            BlockType::Terracotta => "textures/Red Terracotta.png",
            BlockType::RedWool => "textures/Red Wool.png",
            BlockType::WhiteGlass => "textures/White Glass.png",
            BlockType::Grass => "textures/Grass.png",
            BlockType::ConcretePowder => "textures/Concrete Powder.png",
            BlockType::OakWood => "textures/Oak Wood.png",
            BlockType::GreenWool => "textures/Green Wool.png",
            BlockType::Leaves => "textures/Leaves.png",
            BlockType::MangroveWood => "textures/Mangrove Wood.png",
        }
    }

    // Color más brillante para efectos (fallback visual)
    fn get_enhanced_color(&self) -> Color {
        match self {
            BlockType::Terracotta => Color::new(200, 100, 80, 255),       // Brighter terracotta
            BlockType::RedWool => Color::new(220, 60, 50, 255),           // Brighter red
            BlockType::WhiteGlass => Color::new(255, 255, 255, 200),      // Brighter white
            BlockType::Grass => Color::new(120, 200, 100, 255),           // Brighter grass
            BlockType::ConcretePowder => Color::new(200, 200, 190, 255),  // Brighter concrete
            BlockType::OakWood => Color::new(200, 160, 100, 255),         // Brighter oak
            BlockType::GreenWool => Color::new(120, 160, 40, 255),        // Brighter green wool
            BlockType::Leaves => Color::new(90, 160, 90, 255),            // Brighter leaves
            BlockType::MangroveWood => Color::new(150, 110, 90, 255),     // Brighter mangrove
        }
    }
}

// Bloque: tipo y posición 3D
struct Block {
    block_type: BlockType,
    position: Vector3,
}

// Cámara orbital simple con ángulos y distancia
struct OrbitCamera {
    target: Vector3,
    distance: f32,
    angle_x: f32,
    angle_y: f32,
    min_distance: f32,
    max_distance: f32,
}

impl OrbitCamera {
    // Constructor de la cámara
    fn new(target: Vector3, distance: f32) -> Self {
        Self {
            target,
            distance,
            angle_x: 45.0,
            angle_y: -30.0,
            min_distance: 5.0,
            max_distance: 50.0,
        }
    }

    // Calcula la posición de la cámara a partir de ángulos y distancia
    fn get_position(&self) -> Vector3 {
        let rad_x = self.angle_x.to_radians();
        let rad_y = self.angle_y.to_radians();
        
        Vector3::new(
            self.target.x + self.distance * rad_y.cos() * rad_x.cos(),
            self.target.y + self.distance * rad_y.sin(),
            self.target.z + self.distance * rad_y.cos() * rad_x.sin(),
        )
    }

    // Actualiza la cámara según input (mouse, rueda, WASD, reset)
    fn update(&mut self, rl: &RaylibHandle, mouse_delta: Vector2) {
        // Rotación con click izquierdo y arrastre
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            self.angle_x += mouse_delta.x * 0.3;
            self.angle_y += mouse_delta.y * 0.3;
            self.angle_y = self.angle_y.clamp(-89.0, 89.0);
        }

        // Zoom con rueda del mouse
        let wheel_move = rl.get_mouse_wheel_move();
        if wheel_move != 0.0 {
            self.distance -= wheel_move * 2.0;
            self.distance = self.distance.clamp(self.min_distance, self.max_distance);
        }

        // Controles WASD: acercar/alejar y rotar horizontalmente
        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.distance -= 0.5;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.distance += 0.5;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.angle_x -= 2.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.angle_x += 2.0;
        }
        
        // Reset de cámara
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            self.angle_x = 45.0;
            self.angle_y = -30.0;
            self.distance = 25.0;
        }

        self.distance = self.distance.clamp(self.min_distance, self.max_distance);
    }
}

// ===========================================
// PARSEO DEL DIORAMA: texto multilínea con
// capas separadas por líneas en blanco
// ===========================================
fn parse_diorama() -> Vec<Block> {
    let diorama_data = r#"########
##TTTT##
#TMMMMT#
#TMMMMT#
#TMMMMT#
#TMMMMT#
G#TTTT#G
G######G
G#####G#
#GGGG#G#
##GGGG##

########
#TTTTTT#
#TMMMMT#
#TMMMMT#
#TMMMMT#
#TMMMMT#
#TTTTTT#
#GGMMGG#
#GGGGG##
#####G##
########

##TTTT##
#T####T#
T######T
T######T
T######T
T######T
#T####T#
##T##T##
########
########
########

#TTTTTT#
T######T
W######W
T######T
T######T
W######W
T######T
#TT##TT#
########
########
########

#TTTTTT#
T######T
W######W
T######R
T######R
W######W
T######R
#TTTTRT#
########
########
########

#TTTTTT#
T######T
W######W
T######R
T######R
W######W
T######C
#TRTRRR#
########
########
########

#TTTTTT#
T######T
T######C
T######R
C######C
T######C
T######C
#RTCRCC#
########
########
########

########
#TTTTTR#
#TTTTRR#
#TTRTRT#
#RRTOCR#
#RTRCRC#
#TRCRCC#
########
########
########
########

########
########
####VL##
####L###
###O####
########
########
########
########
########
########

#####L##
####LL##
#####V##
########
###O####
##O#####
########
########
########
########
########

#####L##
#####VL#
########
########
########
##O#####
########
########
########
########
########

#####V##
########
########
########
########
########
########
########
########
########
########"#;

    let mut blocks = Vec::new();
    // Separar capas por doble salto de línea
    let layers: Vec<&str> = diorama_data.split("\n\n").collect();
    
    for (layer_index, layer) in layers.iter().enumerate() {
        let lines: Vec<&str> = layer.lines().collect();
        
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if let Some(block_type) = BlockType::from_char(ch) {
                    // x = columna, y = capa (layer_index), z = fila
                    blocks.push(Block {
                        block_type,
                        position: Vector3::new(col as f32, layer_index as f32, row as f32),
                    });
                }
            }
        }
    }
    
    blocks
}

// ===========================================
// MAIN: inicializa Raylib, carga texturas/modelos,
// y render loop
// ===========================================
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1024, 768)
        .title("Voxel Diorama")
        .build();

    rl.set_target_fps(60);

    // Parsear diorama desde texto
    let blocks = parse_diorama();
    
    // Centro del diorama (target de la cámara)
    let center = Vector3::new(4.0, 6.0, 5.0);
    let mut orbit_camera = OrbitCamera::new(center, 25.0);

    // Mapas para texturas y modelos (pueden ser None si no se cargó)
    let mut textures: HashMap<BlockType, Option<Texture2D>> = HashMap::new();
    let mut textured_models: HashMap<BlockType, Option<Model>> = HashMap::new();
    
    println!("=== Texture Loading Status ===");
    
    // Intentar cargar cada textura en disco
    for block_type in [
        BlockType::Terracotta,
        BlockType::RedWool,
        BlockType::WhiteGlass,
        BlockType::Grass,
        BlockType::ConcretePowder,
        BlockType::OakWood,
        BlockType::GreenWool,
        BlockType::Leaves,
        BlockType::MangroveWood,
    ] {
        let texture_path = block_type.get_texture_path();
        println!("Attempting to load: {}", texture_path);
        
        match rl.load_texture(&thread, texture_path) {
            Ok(texture) => {
                println!("✓ Texture LOADED: {}", texture_path);
                
                // Crear un mesh cubo y generar modelo desde él
                let mesh = Mesh::gen_mesh_cube(&thread, 1.0, 1.0, 1.0);
                
                match rl.load_model_from_mesh(&thread, unsafe { mesh.make_weak() }) {
                    Ok(mut model) => {
                        // Intentar asociar la textura al material del modelo
                        unsafe {
                            let material = model.materials_mut().as_mut_ptr();
                            if !material.is_null() {
                                let maps = (*material).maps;
                                if !maps.is_null() {
                                    // Asignar textura al mapa ALBEDO
                                    (*maps.add(raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize)).texture = *texture;
                                    println!("✓ Texture applied to model: {:?}", block_type);
                                } else {
                                    println!("✗ Material maps is null for: {:?}", block_type);
                                }
                            } else {
                                println!("✗ Material is null for: {:?}", block_type);
                            }
                        }
                        
                        // Guardar textura y modelo
                        textures.insert(block_type, Some(texture));
                        textured_models.insert(block_type, Some(model));
                    }
                    Err(e) => {
                        // Si falla crear el modelo, guardamos la textura y ponemos None en model
                        println!("✗ Failed to create model for {}: {:?}", texture_path, e);
                        textures.insert(block_type, Some(texture));
                        textured_models.insert(block_type, None);
                    }
                }
            }
            Err(e) => {
                // No se encontró textura en disco -> fallback a color
                println!("✗ Texture NOT FOUND: {} - Error: {:?}", texture_path, e);
                textures.insert(block_type, None);
                textured_models.insert(block_type, None);
            }
        }
    }
    
    let loaded_count = textured_models.values().filter(|opt| opt.is_some()).count();
    println!("=== Summary: {}/9 textures loaded successfully ===", loaded_count);

    // Bucle principal
    while !rl.window_should_close() {
        // Input / actualización de cámara
        let mouse_delta = rl.get_mouse_delta();
        orbit_camera.update(&rl, mouse_delta);

        // Preparar cámara 3D
        let camera_pos = orbit_camera.get_position();
        let camera = Camera3D::perspective(
            camera_pos,
            orbit_camera.target,
            Vector3::new(0.0, 1.0, 0.0),
            60.0,
        );

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(135, 206, 235, 255)); // Cielo azul de fondo

        {
            let mut d3d = d.begin_mode3D(camera);

            // ------------------------------
            // SKYBOX SIMPLE: planos en diferentes alturas
            // y algunas nubes y sol como referencia visual
            // ------------------------------
            d3d.draw_plane(
                Vector3::new(center.x, -10.0, center.z),
                Vector2::new(200.0, 200.0),
                Color::new(200, 220, 240, 255)
            );
            d3d.draw_plane(
                Vector3::new(center.x, 30.0, center.z),
                Vector2::new(200.0, 200.0),
                Color::new(135, 180, 230, 255)
            );
            d3d.draw_plane(
                Vector3::new(center.x, 50.0, center.z),
                Vector2::new(200.0, 200.0),
                Color::new(70, 130, 200, 255)
            );
            
            // Nubes como cubos semitransparentes
            let cloud_positions = [
                (12.0, 18.0, 15.0),
                (2.0, 20.0, 8.0),
                (-5.0, 19.0, 12.0),
                (8.0, 21.0, 3.0),
                (15.0, 17.0, -2.0),
                (-3.0, 22.0, -5.0),
                (5.0, 23.0, 18.0),
                (-8.0, 19.0, 5.0),
            ];
            
            for (x, y, z) in &cloud_positions {
                // Cuerpo principal de la nube
                d3d.draw_cube(
                    Vector3::new(*x, *y, *z),
                    3.0, 1.5, 2.0,
                    Color::new(255, 255, 255, 200)
                );
                // "Puf" adicionales para dar forma
                d3d.draw_cube(
                    Vector3::new(x + 1.0, y + 0.3, *z),
                    2.0, 1.2, 1.5,
                    Color::new(255, 255, 255, 180)
                );
                d3d.draw_cube(
                    Vector3::new(x - 1.0, y + 0.2, *z),
                    2.0, 1.0, 1.5,
                    Color::new(255, 255, 255, 160)
                );
            }
            
            // Sol y su resplandor
            d3d.draw_sphere(
                Vector3::new(center.x + 30.0, center.y + 35.0, center.z - 25.0),
                4.0,
                Color::new(255, 250, 200, 255)
            );
            d3d.draw_sphere(
                Vector3::new(center.x + 30.0, center.y + 35.0, center.z - 25.0),
                6.0,
                Color::new(255, 240, 180, 80)
            );

            // ------------------------------
            // DIBUJAR BLOQUES DEL DIORAMA
            // - sombra simple
            // - textura modelo si existe
            // - caso vidrio: efectos extras
            // - fallback: cubo coloreado
            // ------------------------------
            for block in &blocks {
                let pos = Vector3::new(
                    block.position.x,
                    block.position.y,
                    block.position.z,
                );

                // Sombra proyectada simple en el "suelo"
                let shadow_pos = Vector3::new(
                    pos.x + 0.3, // ligero desplazamiento para simular dirección de luz
                    -0.49,       // justo por encima del plano del suelo
                    pos.z + 0.3,
                );
                d3d.draw_cube(
                    shadow_pos,
                    1.1, 0.01, 1.1,
                    Color::new(0, 0, 0, 80) // sombra semitransparente
                );

                // Intentar dibujar modelo texturizado si se cargó
                if let Some(Some(model)) = textured_models.get(&block.block_type) {
                    d3d.draw_model(model, pos, 1.0, Color::WHITE);
                    
                    // Efectos especiales para bloques de vidrio
                    if block.block_type == BlockType::WhiteGlass {
                        // Capa exterior translúcida (frosted)
                        d3d.draw_cube(pos, 1.0, 1.0, 1.0, Color::new(220, 240, 255, 100));
                        
                        // Núcleo interno para efecto "lente"
                        d3d.draw_cube(pos, 0.7, 0.7, 0.7, Color::new(180, 220, 255, 150));
                        
                        // Punto central brillante
                        d3d.draw_cube(pos, 0.4, 0.4, 0.4, Color::new(255, 255, 255, 200));
                        
                        // REFLEXIONES DINÁMICAS: muestrear bloques cercanos y dibujar versiones pequeñas/translúcidas
                        for other_block in &blocks {
                            if other_block.block_type != BlockType::WhiteGlass {
                                let dist = ((other_block.position.x - block.position.x).powi(2) +
                                           (other_block.position.y - block.position.y).powi(2) +
                                           (other_block.position.z - block.position.z).powi(2)).sqrt();
                                
                                // Solo reflejar objetos en un radio pequeño
                                if dist < 3.0 && dist > 0.1 {
                                    let dir_x = other_block.position.x - block.position.x;
                                    let dir_y = other_block.position.y - block.position.y;
                                    let dir_z = other_block.position.z - block.position.z;
                                    
                                    // Posición de la "reflexión" sobre la superficie del vidrio
                                    let reflection_pos = Vector3::new(
                                        block.position.x + dir_x * 0.5,
                                        block.position.y + dir_y * 0.5,
                                        block.position.z + dir_z * 0.5,
                                    );
                                    
                                    // Color base del objeto reflejado (más brillante)
                                    let base_color = other_block.block_type.get_enhanced_color();
                                    let fade = (150.0 * (1.0 - dist / 3.0)) as u8;
                                    
                                    d3d.draw_cube(
                                        reflection_pos,
                                        0.3, 0.3, 0.3,
                                        Color::new(
                                            base_color.r,
                                            base_color.g,
                                            base_color.b,
                                            fade
                                        )
                                    );
                                }
                            }
                        }
                        
                        // Luz caústica debajo del vidrio
                        d3d.draw_cube(
                            Vector3::new(pos.x, pos.y - 1.2, pos.z),
                            1.5, 0.05, 1.5,
                            Color::new(200, 230, 255, 100)
                        );
                        
                        // Puntos de resaltado en los bordes (simulación simple de Fresnel)
                        let highlight_positions = [
                            Vector3::new(pos.x + 0.45, pos.y, pos.z),
                            Vector3::new(pos.x - 0.45, pos.y, pos.z),
                            Vector3::new(pos.x, pos.y + 0.45, pos.z),
                            Vector3::new(pos.x, pos.y, pos.z + 0.45),
                        ];
                        
                        for highlight_pos in &highlight_positions {
                            d3d.draw_cube(
                                *highlight_pos,
                                0.15, 0.15, 0.15,
                                Color::new(255, 255, 255, 180)
                            );
                        }
                    }
                } else if let Some(Some(_texture)) = textures.get(&block.block_type) {
                    // Si tenemos textura pero no modelo 3D, usamos un cubo simple de fallback
                    d3d.draw_cube(pos, 1.0, 1.0, 1.0, Color::new(255, 255, 100, 255));
                } else {
                    // Sin textura -> dibujar color brillante por defecto
                    d3d.draw_cube(pos, 1.0, 1.0, 1.0, block.block_type.get_enhanced_color());
                }

                // Contorno del cubo para definición visual
                d3d.draw_cube_wires(
                    pos,
                    1.0,
                    1.0,
                    1.0,
                    Color::new(80, 80, 80, 120),
                );
            }
            
            // ------------------------------
            // REFLEJOS EN EL SUELO: para bloques cerca del suelo
            // ------------------------------
            for block in &blocks {
                if block.position.y <= 3.0 { // solo los cercanos al suelo
                    // Calcular posición reflejada (simetría respecto a -0.5)
                    let reflected_y = -0.5 - (block.position.y + 0.5);
                    let pos = Vector3::new(
                        block.position.x,
                        reflected_y,
                        block.position.z,
                    );
                    
                    // Fade según distancia al suelo
                    let fade = ((3.0 - block.position.y) / 3.0 * 60.0) as u8;
                    
                    if let Some(Some(model)) = textured_models.get(&block.block_type) {
                        d3d.draw_model(model, pos, 1.0, Color::new(80, 80, 100, fade));
                    } else {
                        let base_color = block.block_type.get_enhanced_color();
                        d3d.draw_cube(
                            pos, 
                            1.0, 1.0, 1.0, 
                            Color::new(
                                base_color.r / 4,
                                base_color.g / 4,
                                base_color.b / 4,
                                fade
                            )
                        );
                    }
                    
                    // Contorno del reflejo (más tenue)
                    d3d.draw_cube_wires(
                        pos,
                        1.0, 1.0, 1.0,
                        Color::new(40, 40, 50, fade / 2),
                    );
                }
            }

            // Plano de suelo de referencia
            d3d.draw_plane(
                Vector3::new(center.x, -0.5, center.z),
                Vector2::new(20.0, 20.0),
                Color::new(150, 200, 150, 120),
            );
        } // fin begin_mode3D

        // ------------------------------
        // UI: instrucciones y estado
        // ------------------------------
        d.draw_text("Controls:", 10, 10, 20, Color::BLACK);
        d.draw_text("Mouse: Click & drag to rotate", 10, 35, 16, Color::BLACK);
        d.draw_text("Scroll: Zoom in/out", 10, 55, 16, Color::BLACK);
        d.draw_text("W/S: Move closer/further", 10, 75, 16, Color::BLACK);
        d.draw_text("A/D: Strafe left/right", 10, 95, 16, Color::BLACK);
        d.draw_text("R: Reset camera", 10, 115, 16, Color::BLACK);
        
        // Mostrar distancia actual de cámara
        d.draw_text(
            &format!("Distance: {:.1}", orbit_camera.distance),
            10,
            140,
            16,
            Color::BLACK,
        );

        // Mostrar cuántas texturas se cargaron
        let loaded_count = textured_models.values().filter(|opt| opt.is_some()).count();
        d.draw_text(
            &format!("Textures loaded: {}/9", loaded_count),
            10,
            165,
            16,
            if loaded_count > 0 { Color::GREEN } else { Color::RED },
        );
    } // fin while
}
