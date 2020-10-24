// a bunch o buttons to select the world
use makepad_render::*;
use crate::skybox::SkyBox;

#[derive(Clone)]
pub struct TreeWorld {
    pub view: View,
    pub tree_area: Area,
    pub sky_box: SkyBox
}

impl TreeWorld {
    pub fn new(cx: &mut Cx) -> Self {
        Self {
            view: View::new(cx),
            tree_area: Area::Empty,
            sky_box: SkyBox::new(cx),
        }
    }
    
    pub fn style(cx: &mut Cx) {
        live_body!(cx, r#"
            self::color: #f00;
            self::angle: 0.5;
            self::alpha: 0.1;
            self::max_depth: 12.0; // careful here. set it too high and it will hang.
            self::shader: Shader {
                
                use makepad_render::shader_std::prelude::*;
                use makepad_worlds::worldview::uniforms::*;
                
                default_geometry: makepad_render::shader_std::quad_2d;
                geometry geom: vec2;
                
                instance in_path: float;
                instance depth: float;
                
                fn vertex() -> vec4 {
                    let pos = vec2(0.0, -0.5);
                    let scale = vec2(0.2, 0.2);
                    let dir = vec2(0.0, 0.8);
                    let smaller = vec2(.85, 0.85);
                    let path = in_path;
                    let nodesize = vec2(1.);
                    let z = 0.0;
                    for i from 0 to 14 {
                        if float(i) >= depth {
                            break;
                        }
                        
                        let turn_right = mod (path, 2.);
                        let angle = 50.*self::angle;
                        if (turn_right > 0.) {
                            angle = -1.0 * angle;
                            z += 0.1 * scale.x;
                        }
                        else{
                            z -= 0.1 * scale.x;
                        }
                        angle += sin(time + 10. * pos.x) * 5.;
                        
                        dir = Math::rotate_2d(dir, angle * TORAD);
                        pos += dir * scale;
                        scale = scale * smaller;
                        path = floor(path / 2.);
                    }
                    let size = vec2(0.01, 0.01);
                    
                    let m = Math::rotate_2d(
                        vec2(1.0, 0.2) * (geom.xy * nodesize - vec2(1.0, 0.5)),
                        atan(
                            dir.y,
                            dir.x
                        ) 
                    ); 
                    
                    let v = vec4(
                        m * scale.xy + pos.xy,
                        -1.5+z,
                        1.
                    ); 
                    
                    return camera_projection * (camera_view * view_transform * v);
                }
                
                fn pixel() -> vec4 {
                    return vec4(self::color.xyz * self::alpha, self::alpha);
                }
            }
        "#)
    }
    
    pub fn handle_tree_world(&mut self, _cx: &mut Cx, _event: &mut Event) {
        // lets see.
        
    }
    
    pub fn draw_tree_world(&mut self, cx: &mut Cx) {
        self.sky_box.draw_sky_box(cx);
        
        let shader = live_shader!(cx, self::shader);
        self.tree_area = cx.new_instance(shader, None, 0).into();
        
        fn recur(shader: Shader, pself: &mut TreeWorld, cx: &mut Cx, path: f32, depth: f32, max_depth: f32) {
            let inst = cx.new_instance(shader, None, 1);
            let data = [path, depth];
            inst.push_slice(cx, &data);
            if depth > max_depth {return}
            recur(shader, pself, cx, path, depth + 1.0, max_depth);
            recur(shader, pself, cx, path + (2.0f32).powf(depth), depth + 1.0, max_depth);
        }
        recur(shader, self, cx, 0., 0., live_float!(cx, self::max_depth));
    }
}