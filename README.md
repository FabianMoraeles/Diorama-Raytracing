
## Authors

- [@FabianMoraeles](https://github.com/FabianMoraeles)

# Diorama - Visualizador de Diorama 3D

Un renderizador de dioramas basado en voxels construido con Rust y Raylib, con efectos visuales avanzados incluyendo reflexiones dinámicas, refracción, iluminación basada en el sol y materiales de vidrio realistas.

## Características

- **Renderizado Voxel 3D**: Escena 3D basada en bloques estilo Minecraft
- **Bloques Texturizados**: 9 tipos de bloques diferentes con texturas PNG personalizadas
- **Efectos Avanzados de Vidrio**: 
  - Desenfoque/refracción de lente de aumento
  - Reflexiones dinámicas basadas en bloques cercanos
  - Patrones de luz cáustica conscientes del sol
- **Iluminación Realista**: Sombras y reflexiones direccionales basadas en el sol
- **Skybox Atmosférico**: Cielo degradado con nubes y sol
- **Reflexiones en el Suelo**: Bloques reflejados bajo el plano del suelo
- **Cámara Orbital**: Rotación completa de 360° con controles de zoom

---

## Controles de Cámara

### Controles de Ratón
- **Clic Izquierdo + Arrastrar**: Rotar la cámara alrededor del diorama
  - Movimiento horizontal: Rotar alrededor del eje Y
  - Movimiento vertical: Inclinar la cámara arriba/abajo
- **Rueda del Ratón**: Acercar/alejar zoom
  - Scroll arriba: Acercarse
  - Scroll abajo: Alejarse

### Controles de Teclado
- **W**: Acercar la cámara al diorama
- **S**: Alejar la cámara del diorama
- **A**: Rotar la cámara a la izquierda alrededor de la escena
- **D**: Rotar la cámara a la derecha alrededor de la escena
- **R**: Restablecer la cámara a la posición y ángulo predeterminados

### Especificaciones de la Cámara
- **Distancia Predeterminada**: 25 unidades desde el centro
- **Rango de Zoom**: 5 a 50 unidades
- **Ángulo Predeterminado**: 45° horizontal, -30° vertical
- **Límites Verticales**: -89° a 89° (previene bloqueo gimbal)
- **Punto Objetivo**: Centro del diorama en (4.0, 6.0, 5.0)

---

## Compilación y Ejecución

### Prerequisitos
```bash
# Rust toolchain (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Dependencias de Raylib (Linux)
sudo apt-get install libasound2-dev mesa-common-dev libx11-dev libxrandr-dev libxi-dev xorg-dev libgl1-mesa-dev libglu1-mesa-dev
```

### Instrucciones de Compilación
```bash
# Clonar el repositorio
git clone <tu-url-repo>
cd McDiorama

# Compilar en modo release (recomendado para rendimiento)
cargo build --release

# Ejecutar la aplicación
cargo run --release
```

### Configuración de Texturas
Coloca tus archivos de texturas en el directorio `textures/`:
```
McDiorama/
├── src/
├── textures/
│   ├── Red Terracotta.png
│   ├── Red Wool.png
│   ├── White Glass.png
│   ├── Grass.png
│   ├── Concrete Powder.png
│   ├── Oak Wood.png
│   ├── Green Wool.png
│   ├── Leaves.png
│   └── Mangrove Wood.png
└── Cargo.toml
```

**Nota**: Los nombres de archivo de las texturas deben coincidir exactamente (incluyendo espacios).

---

## Tipos de Bloques

El diorama utiliza 9 tipos de bloques diferentes, cada uno representado por un carácter en los datos del diorama:

| Carácter | Tipo de Bloque | Descripción |
|----------|----------------|-------------|
| `T` | Terracotta | Bloques de terracota roja |
| `R` | Red Wool | Bloques de lana roja |
| `W` | White Glass | **Especial**: Vidrio con refracción/reflexión |
| `G` | Grass | Bloques de césped |
| `C` | Concrete Powder | Polvo de concreto gris |
| `O` | Oak Wood | Tablones de madera de roble |
| `V` | Green Wool | Bloques de lana verde |
| `L` | Leaves | Bloques de hojas |
| `M` | Mangrove Wood | Madera marrón rojizo oscuro |
| `#` | Aire | Espacio vacío (sin bloque) |

---

## Efectos Visuales

### Material de Vidrio (White Glass)
Los bloques de vidrio blanco presentan efectos ópticos avanzados:

**Efecto de Refracción/Desenfoque:**
- Los objetos vistos a través del vidrio aparecen borrosos
- Múltiples imágenes fantasma simulan distorsión de lente
- Fuerza de refracción del 15% dobla las trayectorias de luz
- Crea apariencia auténtica de lente de aumento

**Reflexiones Dinámicas:**
- Refleja bloques cercanos en superficies de vidrio
- Intensidad de reflexión modulada por ángulo del sol
- Reflexiones más brillantes para superficies orientadas al sol
- Se desvanece con la distancia (hasta 3 unidades)

**Patrones de Luz Cáustica:**
- Puntos de luz enfocados debajo de los bloques de vidrio
- Sigue la dirección del sol
- La intensidad varía con la altura del sol
- Simula refracción de luz a través del vidrio

**Resaltados Fresnel:**
- Reflexiones de borde brillantes en superficies de vidrio
- Posicionados según la dirección del sol
- Simula reflexión superficial en ángulos rasantes

### Sistema de Iluminación

**Iluminación Direccional Basada en el Sol:**
- Posición del sol: (centro.x + 30, centro.y + 35, centro.z - 25)
- Todas las sombras se proyectan alejándose del sol
- Longitud de sombra proporcional a la altura del bloque
- Intensidad de reflexiones basada en ángulo del sol

**Sombras:**
- Proyección dinámica de sombras en el plano del suelo
- Sombras direccionales siguiendo el vector del sol
- Semitransparentes (alfa: 80)
- El desplazamiento aumenta con la altura del bloque

### Reflexiones en el Suelo
- Bloques reflejados bajo el plano del suelo (y = -0.5)
- Refleja bloques con y ≤ 3.0
- Desvanecimiento degradado según altura
- Más oscuros y transparentes que los originales

### Skybox
- Planos degradados multicapa (horizonte a cielo)
- 8 formaciones de nubes volumétricas
- Sol brillante con efecto de halo
- Apariencia perfecta desde todos los ángulos

---

## Rendimiento

**Configuración Recomendada:**
- Ejecutar en modo `--release` para rendimiento óptimo
- Objetivo: 60 FPS a resolución 1024x768
- El rendimiento escala con cantidad de bloques y bloques de vidrio

**Notas de Optimización:**
- Los bloques de vidrio son computacionalmente costosos debido a:
  - Efecto de desenfoque (5 copias por bloque cercano)
  - Reflexiones dinámicas (escanea bloques cercanos)
  - Cálculos de cáusticos
- Reduce la cantidad de bloques de vidrio si experimentas caídas de FPS

---

## Arquitectura Técnica

### Componentes Principales

**Sistema de Bloques:**
- Enum `BlockType` con 9 variantes
- Cada bloque tiene: ruta de textura, color base, color mejorado
- Parseado desde representación en string ASCII art

**Pipeline de Renderizado:**
1. Renderizado de skybox (fondo)
2. Sol y nubes
3. Proyección de sombras (primer pase)
4. Renderizado principal de bloques con texturas
5. Efectos de vidrio (refracción, reflexión, cáusticos)
6. Reflexiones en el suelo
7. Contornos de wireframe
8. Superposición de UI

**Sistema de Cámara:**
- Implementación de cámara orbital
- Sistema de coordenadas esféricas
- Controles suaves de ratón y teclado
- Restricciones de distancia configurables

### Propiedades de Materiales

Cada material tiene:
- **Albedo**: Color base de textura
- **Transparencia**: Canal alfa (vidrio: variable, otros: opacos)
- **Reflectividad**: Dinámica para vidrio, estática para otros
- **Refracción**: Solo bloques de vidrio (simulación de IOR mediante offset)

---

## Formato de Datos del Diorama

El diorama está definido como arte ASCII con capas separadas por líneas en blanco:

```rust
let diorama_data = r#"
########    <- Capa 0 (nivel del suelo)
##TTTT##
#TMMMMT#
...

########    <- Capa 1
#TTTTTT#
...
"#;
```

- Cada capa representa un corte horizontal
- Los caracteres mapean a tipos de bloques
- `#` representa aire (sin bloque)
- Capas apiladas verticalmente (eje Y)

---

## Solución de Problemas

**Texturas no se cargan:**
- Verifica que los archivos de textura existan en el directorio `textures/`
- Comprueba que los nombres de archivo coincidan exactamente (sensible a mayúsculas)
- Asegúrate del formato PNG
- La salida de consola muestra el estado de carga

**Texturas negras:**
- Verifica el formato del archivo de textura (debe ser PNG válido)
- Verifica dimensiones de textura (se recomienda potencia de 2)
- Asegúrate de que el modo de color sea RGB o RGBA

**Problemas de rendimiento:**
- Compila en modo release: `cargo run --release`
- Reduce el tamaño de ventana en el código fuente
- Limita la cantidad de bloques de vidrio en los datos del diorama

**Controles de cámara no funcionan:**
- Mantén presionado el botón izquierdo del ratón mientras arrastras
- Asegúrate de que la ventana tenga foco
- Verifica que la entrada del teclado no esté capturada por el SO

---

## Requisitos del Sistema

**Mínimos:**
- CPU: Dual-core 2.0 GHz
- RAM: 2 GB
- GPU: Compatible con OpenGL 3.3
- SO: Windows 10, Linux (Ubuntu 20.04+), macOS 10.15+

**Recomendados:**
- CPU: Quad-core 3.0 GHz
- RAM: 4 GB
- GPU: GPU dedicada con OpenGL 4.0+
- SO: Windows 11, Linux (Ubuntu 22.04+), macOS 12+

---

## Dependencias

```toml
[dependencies]
raylib = "5.5.1"
```

**Características de Raylib:**
- Renderizado 3D
- Carga de texturas
- Manejo de entrada
- Gestión de ventanas

---

## Estructura del Proyecto

```
McDiorama/
├── src/
│   └── main.rs              # Código principal de la aplicación
├── textures/                # Texturas de bloques (archivos PNG)
│   ├── Red Terracotta.png
│   └── ...
├── Cargo.toml              # Dependencias de Rust
├── Cargo.lock              # Archivo de bloqueo de dependencias
└── README.md               # Este archivo
```

---

## Créditos

**Tecnología:**
- Lenguaje de Programación Rust
- Raylib (bindings raylib-rs)

**Inspiración:**
- Estética voxel de Minecraft
- Técnicas de renderizado físicamente basado
- Programación gráfica en tiempo real

---

## Licencia

[Especifica tu licencia aquí]

---

## Mejoras Futuras

Mejoras potenciales:
- Refracción basada en shaders (screen-space)
- Shadow mapping en tiempo real
- Oclusión ambiental
- Efectos de post-procesamiento (bloom, tone mapping)
- Edición interactiva del diorama
- Sistema de animación
- Múltiples presets de escenas
- Simulación de agua
- Efectos de partículas

---

## Contacto

💼 LinkedIn: linkedin.com/in/fabianmdev/
✉️ Email: fabimoradav2004@gmail.com

Para bugs, problemas o solicitudes de características, por favor abre un issue en el repositorio del proyecto.