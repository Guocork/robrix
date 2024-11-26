use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    TypingAnimation = {{TypingAnimation}} {
        width: 24,
        height: 20,
        flow: Down,
        show_bg: true,
        draw_bg: {
            uniform freq: 3.0,  // Animation frequency
            uniform dot_radius: 1.6, // Dot radius
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = vec4(0.0, 0.0, 0.0, 1.0);
                let amplitude = self.rect_size.y * 0.3;
                let center_y = self.rect_size.y * 0.5;
                // Creates dotting animation to right using Sine function
                let phi = sin(self.time * self.freq);
                // Create three circle SDFs
                if phi < 0.02 {
                    return sdf.result;
                }
                sdf.circle(
                    self.rect_size.x * 0.25, 
                    center_y, 
                    self.dot_radius
                );
                sdf.fill(color);
                if phi < 0.4 {
                    return sdf.result;
                }
                sdf.circle(
                    self.rect_size.x * 0.5, 
                    center_y, 
                    self.dot_radius
                );
                sdf.fill(color);
                if phi < 0.75 {
                    return sdf.result;
                }
                sdf.circle(
                    self.rect_size.x * 0.75, 
                    center_y, 
                    self.dot_radius
                );
                sdf.fill(color);
                return sdf.result;
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct TypingAnimation {
    #[deref] view: View,
    #[live] time: f32,
    #[rust] next_frame: NextFrame,
    #[rust] is_play: bool,
}
impl Widget for TypingAnimation {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Some(ne) = self.next_frame.is_event(event) {
            self.time += ne.time as f32;       
            self.time = (self.time.round() as u32 % 360) as f32;
            self.redraw(cx);
            if !self.is_play {
                return
            }
            self.next_frame = cx.new_next_frame();
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}


impl TypingAnimationRef {
    /// Starts animation of the bouncing dots.
    pub fn animate(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.is_play = true;
            inner.next_frame = cx.new_next_frame();
        }
    }
    /// Stops animation of the bouncing dots.
    pub fn stop_animation(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.is_play = false;
        }
    }
}
