use c_str_macro::c_str;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

mod line;

use crate::shader::Shader;
use line::Line;

pub struct Stage {
    z_translation: f32,
    line1: Line,
    line2: Line,
    line3: Line,
    line4: Line,

    line5: Line,
    line6: Line,
    line7: Line,
    line8: Line,

    line9: Line,
    line10: Line,
    line11: Line,
    line12: Line,

    line13: Line,
    line14: Line,
    line15: Line,
    line16: Line,

    line17: Line,
    line18: Line,
    line19: Line,
    line20: Line,

    line21: Line,
    line22: Line,
    line23: Line,
    line24: Line,

    line25: Line,
    line26: Line,
    line27: Line,
    line28: Line,

    line29: Line,
    line30: Line,
    line31: Line,
    line32: Line,

    line33: Line,
    line34: Line,
    line35: Line,
    line36: Line,

    line37: Line,
    line38: Line,
    line39: Line,
    line40: Line,

    line41: Line,
    line42: Line,
    line43: Line,
    line44: Line,

    line45: Line,
    line46: Line,
    line47: Line,
    line48: Line,

    line49: Line,
    line50: Line,
    line51: Line,
    line52: Line,

    line53: Line,
    line54: Line,
    line55: Line,
    line56: Line,
}

impl Stage {
    pub fn new() -> Stage {
        let line1 = Line::new(-1.0, -1.25, -5.0, -1.0, -1.25, 10.0);
        let line2 = Line::new(-3.0, -1.25, -5.0, -3.0, -1.25, 10.0);
        let line3 = Line::new(-1.0, -3.0, -5.0, -1.0, -3.0, 10.0);
        let line4 = Line::new(-3.0, -3.0, -5.0, -3.0, -3.0, 10.0);

        let line5 = Line::new(-1.0, -1.25, 10.0, -3.0, -1.25, 10.0);
        let line6 = Line::new(-1.0, -3.0, 10.0, -3.0, -3.0, 10.0);
        let line7 = Line::new(-1.0, -1.25, 10.0, -1.0, -3.0, 10.0);
        let line8 = Line::new(-3.0, -1.25, 10.0, -3.0, -3.0, 10.0);

        let line9 = Line::new(-1.0, -1.25, 8.75, -3.0, -1.25, 8.75);
        let line10 = Line::new(-1.0, -3.0, 8.75, -3.0, -3.0, 8.75);
        let line11 = Line::new(-1.0, -1.25, 8.75, -1.0, -3.0, 8.75);
        let line12 = Line::new(-3.0, -1.25, 8.75, -3.0, -3.0, 8.75);

        let line13 = Line::new(-1.0, -1.25, 7.5, -3.0, -1.25, 7.5);
        let line14 = Line::new(-1.0, -3.0, 7.5, -3.0, -3.0, 7.5);
        let line15 = Line::new(-1.0, -1.25, 7.5, -1.0, -3.0, 7.5);
        let line16 = Line::new(-3.0, -1.25, 7.5, -3.0, -3.0, 7.5);

        let line17 = Line::new(-1.0, -1.25, 6.25, -3.0, -1.25, 6.25);
        let line18 = Line::new(-1.0, -3.0, 6.25, -3.0, -3.0, 6.25);
        let line19 = Line::new(-1.0, -1.25, 6.25, -1.0, -3.0, 6.25);
        let line20 = Line::new(-3.0, -1.25, 6.25, -3.0, -3.0, 6.25);

        let line21 = Line::new(-1.0, -1.25, 5.0, -3.0, -1.25, 5.0);
        let line22 = Line::new(-1.0, -3.0, 5.0, -3.0, -3.0, 5.0);
        let line23 = Line::new(-1.0, -1.25, 5.0, -1.0, -3.0, 5.0);
        let line24 = Line::new(-3.0, -1.25, 5.0, -3.0, -3.0, 5.0);

        let line25 = Line::new(-1.0, -1.25, 3.85, -3.0, -1.25, 3.85);
        let line26 = Line::new(-1.0, -3.0, 3.85, -3.0, -3.0, 3.85);
        let line27 = Line::new(-1.0, -1.25, 3.85, -1.0, -3.0, 3.85);
        let line28 = Line::new(-3.0, -1.25, 3.85, -3.0, -3.0, 3.85);

        let line29 = Line::new(-1.0, -1.25, 2.5, -3.0, -1.25, 2.5);
        let line30 = Line::new(-1.0, -3.0, 2.5, -3.0, -3.0, 2.5);
        let line31 = Line::new(-1.0, -1.25, 2.5, -1.0, -3.0, 2.5);
        let line32 = Line::new(-3.0, -1.25, 2.5, -3.0, -3.0, 2.5);

        let line33 = Line::new(-1.0, -1.25, 1.25, -3.0, -1.25, 1.25);
        let line34 = Line::new(-1.0, -3.0, 1.25, -3.0, -3.0, 1.25);
        let line35 = Line::new(-1.0, -1.25, 1.25, -1.0, -3.0, 1.25);
        let line36 = Line::new(-3.0, -1.25, 1.25, -3.0, -3.0, 1.25);

        let line37 = Line::new(-1.0, -1.25, 0.0, -3.0, -1.25, 0.0);
        let line38 = Line::new(-1.0, -3.0, 0.0, -3.0, -3.0, 0.0);
        let line39 = Line::new(-1.0, -1.25, 0.0, -1.0, -3.0, 0.0);
        let line40 = Line::new(-3.0, -1.25, 0.0, -3.0, -3.0, 0.0);

        let line41 = Line::new(-1.0, -1.25, -1.25, -3.0, -1.25, -1.25);
        let line42 = Line::new(-1.0, -3.0, -1.25, -3.0, -3.0, -1.25);
        let line43 = Line::new(-1.0, -1.25, -1.25, -1.0, -3.0, -1.25);
        let line44 = Line::new(-3.0, -1.25, -1.25, -3.0, -3.0, -1.25);

        let line45 = Line::new(-1.0, -1.25, -2.5, -3.0, -1.25, -2.5);
        let line46 = Line::new(-1.0, -3.0, -2.5, -3.0, -3.0, -2.5);
        let line47 = Line::new(-1.0, -1.25, -2.5, -1.0, -3.0, -2.5);
        let line48 = Line::new(-3.0, -1.25, -2.5, -3.0, -3.0, -2.5);

        let line49 = Line::new(-1.0, -1.25, -3.75, -3.0, -1.25, -3.75);
        let line50 = Line::new(-1.0, -3.0, -3.75, -3.0, -3.0, -3.75);
        let line51 = Line::new(-1.0, -1.25, -3.75, -1.0, -3.0, -3.75);
        let line52 = Line::new(-3.0, -1.25, -3.75, -3.0, -3.0, -3.75);

        let line53 = Line::new(-1.0, -1.25, -5.0, -3.0, -1.25, -5.0);
        let line54 = Line::new(-1.0, -3.0, -5.0, -3.0, -3.0, -5.0);
        let line55 = Line::new(-1.0, -1.25, -5.0, -1.0, -3.0, -5.0);
        let line56 = Line::new(-3.0, -1.25, -5.0, -3.0, -3.0, -5.0);

        Stage{
            z_translation: 0.0,

            line1,
            line2,
            line3,
            line4,

            line5,
            line6,
            line7,
            line8,

            line9,
            line10,
            line11,
            line12,

            line13,
            line14,
            line15,
            line16,

            line17,
            line18,
            line19,
            line20,

            line21,
            line22,
            line23,
            line24,

            line25,
            line26,
            line27,
            line28,

            line29,
            line30,
            line31,
            line32,

            line33,
            line34,
            line35,
            line36,

            line37,
            line38,
            line39,
            line40,

            line41,
            line42,
            line43,
            line44,

            line45,
            line46,
            line47,
            line48,

            line49,
            line50,
            line51,
            line52,

            line53,
            line54,
            line55,
            line56,
        }
    }

    pub fn draw(&mut self, shader: &Shader, delay: u128) {
        let x_translation = 0.0;
        let y_translation = 0.0;
        self.z_translation = (self.z_translation + delay as f32 / 15.0 % 1.25) % 1.25;
        let translation = Matrix4::from_translation(
            Vector3::new(
                x_translation,
                y_translation,
                self.z_translation
            )
        );

        let model_matrix = translation;
        unsafe {
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_bool(c_str!("useTexture"), false);
        }


        self.line1.draw();
        self.line2.draw();
        self.line3.draw();
        self.line4.draw();

        self.line5.draw();
        self.line6.draw();
        self.line7.draw();
        self.line8.draw();

        self.line9.draw();
        self.line10.draw();
        self.line11.draw();
        self.line12.draw();

        self.line13.draw();
        self.line14.draw();
        self.line15.draw();
        self.line16.draw();

        self.line17.draw();
        self.line18.draw();
        self.line19.draw();
        self.line20.draw();

        self.line21.draw();
        self.line22.draw();
        self.line23.draw();
        self.line24.draw();

        self.line25.draw();
        self.line26.draw();
        self.line27.draw();
        self.line28.draw();

        self.line29.draw();
        self.line30.draw();
        self.line31.draw();
        self.line32.draw();

        self.line33.draw();
        self.line34.draw();
        self.line35.draw();
        self.line36.draw();

        self.line37.draw();
        self.line38.draw();
        self.line39.draw();
        self.line40.draw();

        self.line41.draw();
        self.line42.draw();
        self.line43.draw();
        self.line44.draw();

        self.line45.draw();
        self.line46.draw();
        self.line47.draw();
        self.line48.draw();

        self.line49.draw();
        self.line50.draw();
        self.line51.draw();
        self.line52.draw();

        self.line53.draw();
        self.line54.draw();
        self.line55.draw();
        self.line56.draw();
    }
}
