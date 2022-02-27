use glfw;
use std::sync::mpsc::Receiver;

pub trait CameraController {
    fn poll_window_events(
        &mut self,
        glfw_instance: &mut glfw::Glfw,
        events: &Receiver<(f64, glfw::WindowEvent)>,
    ) {
        glfw_instance.poll_events();
        for (_, event) in glfw::flush_messages(events) {
            self.handle_event(event);
        }
    }
    fn handle_event(&mut self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
                self.handle_key_event(key, scancode, action, modifiers)
            }
            glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                self.handle_mouse_button_event(button, action, modifiers)
            }
            glfw::WindowEvent::CursorPos(x, y) => self.handle_cursor_pos_event(x, y),
            glfw::WindowEvent::CursorEnter(enter) => self.handle_cursor_enter_event(enter),
            _ => {}
        }
    }
    fn handle_key_event(
        &mut self,
        key: glfw::Key,
        scancode: glfw::Scancode,
        action: glfw::Action,
        modifiers: glfw::Modifiers,
    ) {
    }
    fn handle_mouse_button_event(
        &mut self,
        mouse_button: glfw::MouseButton,
        action: glfw::Action,
        modifiers: glfw::Modifiers,
    ) {
    }
    fn handle_cursor_pos_event(&mut self, x: f64, y: f64) {}
    fn handle_cursor_enter_event(&mut self, enter: bool) {}
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//|================================| Base Camera Controller |===================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////
