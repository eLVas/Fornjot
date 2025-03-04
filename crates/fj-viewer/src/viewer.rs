use fj_interop::model::Model;
use fj_math::Aabb;
use tracing::warn;

use crate::{
    camera::{Camera, FocusPoint},
    graphics::{DrawConfig, Renderer},
    input::InputHandler,
    InputEvent, NormalizedScreenPosition, RendererInitError, Screen,
    ScreenSize,
};

/// The Fornjot model viewer
pub struct Viewer {
    camera: Camera,
    cursor: Option<NormalizedScreenPosition>,
    draw_config: DrawConfig,
    focus_point: Option<FocusPoint>,
    renderer: Renderer,
    model: Option<Model>,
}

impl Viewer {
    /// Construct a new instance of `Viewer`
    pub async fn new(screen: &impl Screen) -> Result<Self, RendererInitError> {
        let renderer = Renderer::new(screen).await?;

        Ok(Self {
            camera: Camera::default(),
            cursor: None,
            draw_config: DrawConfig::default(),
            focus_point: None,
            renderer,
            model: None,
        })
    }

    /// Access the cursor
    pub fn cursor(&mut self) -> &mut Option<NormalizedScreenPosition> {
        &mut self.cursor
    }

    /// Toggle the "draw model" setting
    pub fn toggle_draw_model(&mut self) {
        self.draw_config.draw_model = !self.draw_config.draw_model;
    }

    /// Toggle the "draw mesh" setting
    pub fn toggle_draw_mesh(&mut self) {
        self.draw_config.draw_mesh = !self.draw_config.draw_mesh;
    }

    /// Handle the model being updated
    pub fn handle_model_update(&mut self, model: Model) {
        self.renderer.update_geometry((&model.mesh).into());

        let aabb = model.aabb;
        if self.model.replace(model).is_none() {
            self.camera.init_planes(&aabb);
        }
    }

    /// Handle an input event
    pub fn handle_input_event(&mut self, event: InputEvent) {
        if let Some(focus_point) = self.focus_point {
            InputHandler::handle_event(event, focus_point, &mut self.camera);
        }
    }

    /// Handle the screen being resized
    pub fn handle_screen_resize(&mut self, screen_size: ScreenSize) {
        self.renderer.handle_resize(screen_size);
    }

    /// Compute and store a focus point, unless one is already stored
    pub fn add_focus_point(&mut self) {
        if let Some(model) = &self.model {
            if self.focus_point.is_none() {
                self.focus_point =
                    Some(self.camera.focus_point(self.cursor, model));
            }
        }
    }

    /// Remove the stored focus point
    pub fn remove_focus_point(&mut self) {
        self.focus_point = None;
    }

    /// Draw the graphics
    pub fn draw(&mut self) {
        let aabb = self
            .model
            .as_ref()
            .map(|shape| shape.aabb)
            .unwrap_or_else(Aabb::default);

        self.camera.update_planes(&aabb);

        if let Err(err) = self.renderer.draw(&self.camera, &self.draw_config) {
            warn!("Draw error: {}", err);
        }
    }
}
