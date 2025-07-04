use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    import makepad_projects_test_2::my_widget::MyWidget;

    App = {{App}} {
        ui: <Window>{ 
            show_bg: true
            width: Fill,
            height: Fill
            
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return #000;
                }
            }
            
            body = <View>{
                width: Fill,
                height: Fill,
                align: {x: 0.5, y: 0.5}
                
                <MyWidget> {
                    width: 500, height: 500,
                    draw: {
                        fn pixel(self) -> vec4 {
                            let uv = self.pos - 0.5;
                            let dist = length(uv);
                            let t = self.time;
                            
                            if dist < 0.25 * sin(t / 5.0) {
                                return vec4(vec3(0.0, 0.5, 1.0), 1.0);
                            } else if dist < 0.3 * sin(t / 5.0) {
                                return vec4(vec3(0.9, 0.0, 0.0), 1.0);
                            } else {
                                return vec4(0.0, 0.0, 0.0, 0.0);
                            }
                            
                            // Create a circle with radius 0.4
                            // Smooth the edge slightly
                            //let circle = smoothstep(0.41, 0.4, dist);

                            
                            
                            // Blue circle on black background
                            //return vec4(vec3(0.0, 0.5, 1.0) * circle, 1.0);
                        }
                    }
                }
            }
        }
    }
}

app_main!(App); 

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::my_widget::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
} 