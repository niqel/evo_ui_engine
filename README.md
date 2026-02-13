# evo_ui_engine
**La evolución de las interfaces gráficas**  
_Un engine declarativo, funcional, sin árboles, basado en acetatos sobre una escena._

---

## ✨ ¿Qué es `evo_ui_engine`?

`evo_ui_engine` es un motor gráfico multiplataforma para construir interfaces de usuario funcionales, reactivas y altamente desacopladas. Está diseñado en Rust y pensado para integrarse fácilmente en entornos **nativos** o **WebAssembly**.

Inspirado por los principios de la programación funcional y la composición inmutable, `evo_ui_engine` reinventa cómo construimos interfaces gráficas eliminando la noción tradicional de árbol de widgets.

---

## 🧠 Filosofía

En lugar de representar la interfaz como un árbol jerárquico, `evo_ui_engine` utiliza el concepto de **acetatos superpuestos sobre una escena**. Cada acetato es un componente inmutable, funcional y autónomo, que responde a eventos y genera salidas sin modificar su estado interno.

- 🟦 **Acetato**: entidad funcional que representa una parte de la UI.
- 🖼 **Escena**: superficie inmutable sobre la que se proyectan los acetatos.
- 🧠 **Renderer**: actor que traduce y renderiza la escena.
- ⚡ **Tick + Eventos**: controlan el tiempo y la reacción de los acetatos sin modificar su naturaleza funcional.
- 🔁 **Animaciones**: representadas como secuencias de eventos `AnimationFrame(n)`.

---

## 🧩 Componentes Clave

### 🎭 Acetate
Un `Acetate` es una estructura funcional con entrada/salida bien definida:
- Se construye/destruye por completo en cada reacción a un evento.
- No mantiene estado interno mutable.
- Solo conoce su `design`, su `area`, y reacciona a eventos si está suscrito a ellos.

### 🌌 Scene
La `Scene` es el universo visual donde se colocan todos los acetatos.
- Es inmutable y estática.
- Solo se reconstruye si sus acetatos cambian.

### 🔄 Event System
Todo cambio en la UI es desencadenado por un sistema de eventos puramente funcional:
- `SystemEvent` (del sistema operativo)
- `InternalEvent` (abstracto e independiente)
- `Event` (usado por los acetatos)

### 📸 Snapshot
El `SnapshotBuilder` convierte una `Scene` en una `Snapshot` optimizada para ser pintada por el `Renderer`.

### 🧠 Actors Funcionales
Cada operación está a cargo de un actor con una única responsabilidad:
- `Renderer::render(snapshot)`
- `Animator::animate(event)`
- `SnapshotBuilder::build(scene)`
- `EventRouter::interpret(system_event)`
- `Ticker::tick()`

---

## ✅ Ventajas

- 🚫 **Sin árboles**: no hay jerarquías ni estructuras padre-hijo.
- 🧬 **Inmutabilidad total**: cada cambio es una nueva construcción.
- 🎯 **Predecible y testeable**: no hay efectos colaterales.
- 🔀 **Animaciones por eventos**: sin mutaciones internas.
- 🧩 **Modular y escalable**: puedes dividir los acetatos como prefieras.
- 🌐 **Multiplataforma y WebAssembly**: ideal para apps nativas o en navegador.

---

## 📦 Integración

`evo_ui_engine` puede integrarse con backends gráficos como:

- [Vello](https://github.com/linebender/vello) (rendering acelerado por GPU)
- [Winit](https://github.com/rust-windowing/winit) (eventos del sistema)
- WebAssembly (mediante `wasm-bindgen`)

---

## 🚀 Estado Actual

`evo_ui_engine` se encuentra en desarrollo activo. Ya cuenta con:

- Diseño arquitectónico funcional y modular.
- Sistema de acetatos, escena y renderizado definido.
- Diagramas UML y flujo de actores disponible.

---

## 🧠 Ejemplo Conceptual

```rust
let tick = ticker.tick();
let system_event = winit_event();
let internal = event_router.interpret(system_event);
let events = input_mapper.translate(internal);
let animations = animator.animate(events);
let updated_acetates = react_acetates(events + animations);
let scene = Scene::from(updated_acetates);
let snapshot = snapshot_builder.build(scene);
renderer.render(snapshot);
```

## 🧾 UI TOML v0

Ejemplo minimo con defaults (z=0, border transparente, border_thickness=0.0):

```toml
[scene]
width = 800
height = 600

[[acetate]]
id = "hero"
x = 40
y = 40
w = 320
h = 180
fill = "#2d9cdb"
text = "Hola"
```


🧪 Roadmap

 Modelo funcional completo
 Sistema de eventos desacoplado
 Acetatos superpuestos
 Soporte de entradas complejas (teclado, mouse, etc)
 Acetatos especializados (texto, botones, listas)
 Herramientas de debugging y diseño
 
 
 “El universo es inmutable. Lo que cambia es lo que aparece en él.”
— evo_ui_engine
