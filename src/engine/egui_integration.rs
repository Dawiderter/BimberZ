use std::{collections::HashMap, sync::Arc};

pub struct BimberzEguiViewport {
    state: egui_winit::State,
    id: egui::ViewportId,
    prev_frame_output: Option<egui::ViewportOutput>,
    commands: Vec<egui::ViewportCommand>,
    needs_recreating: bool,
    window: Arc<winit::window::Window>,
}

pub struct BimberzEguiState {
    pub ctx: egui::Context,
    state: HashMap<winit::window::WindowId, BimberzEguiViewport>,
    viewports: HashMap<egui::ViewportId, winit::window::WindowId>,
}

#[derive(Debug, Default)]
pub struct BimberzEguiViewportDelta {
    pub added: Vec<Arc<winit::window::Window>>,
    pub removed: Vec<Arc<winit::window::Window>>,
}

impl BimberzEguiState {
    pub fn from_root(window: Arc<winit::window::Window>) -> Self {
        let ctx = egui::Context::default();
        ctx.set_embed_viewports(false);
        ctx.set_zoom_factor(0.7);

        let window_id = window.id();
        let root_vp = BimberzEguiViewport::from_window(ctx.clone(), egui::ViewportId::ROOT, window);

        Self {
            ctx,
            state: HashMap::from([(window_id, root_vp)]),
            viewports: HashMap::from([(egui::ViewportId::ROOT, window_id)]),
        }
    }

    pub fn apply_prev_output<T>(
        &mut self,
        id: &winit::window::WindowId,
        raw_input: &mut egui::RawInput,
        elwt: &winit::event_loop::EventLoopWindowTarget<T>,
    ) -> Option<Arc<winit::window::Window>> {
        let state = &self.state[id];
        let info = raw_input.viewports.get_mut(&state.id).unwrap();
        self.state
            .get_mut(id)
            .unwrap()
            .apply_prev_output(info, elwt)
    }

    pub fn apply_event(
        &mut self,
        id: &winit::window::WindowId,
        event: &winit::event::WindowEvent,
    ) -> egui_winit::EventResponse {
        let Some(state) = self.state.get_mut(id) else {
            return egui_winit::EventResponse::default();
        };
        state.state.on_window_event(&state.window, event)
    }

    pub fn take_input(&mut self, id: &winit::window::WindowId) -> egui::RawInput {
        let state = self.state.get_mut(id).unwrap();
        let mut input = state.state.take_egui_input(&state.window);
        self.update_info(&mut input);
        input
    }

    pub fn handle_platform_output(
        &mut self,
        id: &winit::window::WindowId,
        output: egui::PlatformOutput,
    ) {
        let state = self.state.get_mut(id).unwrap();
        state.state.handle_platform_output(&state.window, output);
    }

    pub fn handle_viewport_output<T>(
        &mut self,
        output_map: egui::ViewportIdMap<egui::ViewportOutput>,
        elwt: &winit::event_loop::EventLoopWindowTarget<T>,
    ) -> BimberzEguiViewportDelta {
        let mut delta = BimberzEguiViewportDelta::default();

        self.viewports.retain(|id, window_id| {
            let window_exists = output_map.contains_key(id);

            if !window_exists {
                let state = self.state.remove(window_id).unwrap();
                delta.removed.push(state.window);
            }

            window_exists
        });

        for (id, output) in output_map {
            if let Some(window_id) = self.viewports.get(&id) {
                let state = self.state.get_mut(window_id).unwrap();
                state.set_frame_output(output);
            } else {
                let (viewport, window) =
                    BimberzEguiViewport::from_output(self.ctx.clone(), id, output, elwt);
                self.viewports.insert(id, window.id());
                self.state.insert(window.id(), viewport);
                delta.added.push(window);
            }
        }

        delta
    }

    pub fn call(&self, window_id: &winit::window::WindowId) {
        if let Some(callback) = self.state[window_id]
            .prev_frame_output
            .as_ref()
            .and_then(|out| out.viewport_ui_cb.as_ref())
        {
            callback(&self.ctx)
        }
    }

    fn update_info(&self, raw_input: &mut egui::RawInput) {
        for (id, info) in &mut raw_input.viewports {
            let window_id = self.viewports[id];
            let window = &self.state[&window_id].window;
            egui_winit::update_viewport_info(info, &self.ctx, window, false);
        }
    }
}

impl BimberzEguiViewport {
    pub fn from_window(
        ctx: egui::Context,
        id: egui::ViewportId,
        window: Arc<winit::window::Window>,
    ) -> Self {
        Self {
            state: egui_winit::State::new(ctx, id, &window, None, None),
            prev_frame_output: None,
            id,
            commands: Vec::new(),
            needs_recreating: false,
            window,
        }
    }

    pub fn from_output<T>(
        ctx: egui::Context,
        id: egui::ViewportId,
        output: egui::ViewportOutput,
        elwt: &winit::event_loop::EventLoopWindowTarget<T>,
    ) -> (Self, Arc<winit::window::Window>) {
        let window = Arc::new(egui_winit::create_window(&ctx, elwt, &output.builder).unwrap());
        let state = egui_winit::State::new(ctx, id, &window, None, None);
        (
            Self {
                id,
                state,
                prev_frame_output: Some(output),
                commands: Vec::new(),
                needs_recreating: false,
                window: window.clone(),
            },
            window,
        )
    }

    pub fn set_frame_output(&mut self, mut output: egui::ViewportOutput) {
        if let Some(mut prev_output) = self.prev_frame_output.take() {
            std::mem::swap(&mut prev_output.builder, &mut output.builder);
            let (commands, needs_recreating) = output.builder.patch(prev_output.builder);
            self.prev_frame_output = Some(output);
            self.commands = commands;
            self.needs_recreating = needs_recreating;
        } else {
            self.prev_frame_output = Some(output);
            self.needs_recreating = true;
        }
    }

    pub fn apply_prev_output<T>(
        &mut self,
        info: &mut egui::ViewportInfo,
        elwt: &winit::event_loop::EventLoopWindowTarget<T>,
    ) -> Option<Arc<winit::window::Window>> {
        use egui::ahash::{HashSet, HashSetExt};

        let ctx = self.state.egui_ctx();

        let prev_output = &self.prev_frame_output.as_ref()?;

        if self.needs_recreating {
            let window =
                Arc::new(egui_winit::create_window(ctx, elwt, &prev_output.builder).unwrap());
            egui_winit::update_viewport_info(info, ctx, &window, true);
            self.needs_recreating = false;
            self.window = window.clone();
            Some(window)
        } else {
            egui_winit::process_viewport_commands(
                ctx,
                info,
                self.commands
                    .iter()
                    .cloned()
                    .chain(prev_output.commands.iter().cloned()),
                &self.window,
                &mut HashSet::new(),
            );
            None
        }
    }
}
