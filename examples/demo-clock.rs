mod demo;

use ::{
    chrono::{Datelike, Local, Timelike},
    nvg::prelude::*,
    std::f32::consts::PI,
};

struct DemoClock;

impl<R: Renderer> demo::Demo<R> for DemoClock {
    fn update(&mut self, width: f32, height: f32, ctx: &mut Context<R>) -> anyhow::Result<()> {
        let dt = Local::now();
        let hour = dt.hour();
        let am = hour < 12;
        let hour = (hour % 12) as f32;
        let minute = dt.minute() as f32;
        let second = dt.second() as f32;
        let year = dt.year();
        let month = dt.month();
        let day = dt.day();

        let clock_size = width.min(height) - 2.0;

        let font_size = 24.0;
        // upper-left corner
        let origin = (0.0, 0.0);
        let dial_center = (width / 2.0, height / 2.0);
        let dial_radius = clock_size / 2.0;
        let second_hand_len = dial_radius * 0.9;
        let minute_hand_len = dial_radius * 0.8;
        let hour_hand_len = dial_radius * 0.6;

        let two_pi = 2.0 * PI;
        let radians_per_sec = two_pi / 60.0;
        let radians_per_hour = two_pi / 12.0;

        let white = Color::rgba(1.0, 1.0, 1.0, 1.0);
        let silver = Color::rgb_i(196, 199, 206);
        let darksilver = Color::rgb_i(148, 152, 161);
        let darkgray = Color::rgb_i(169, 169, 169);
        let dial_color = Color::rgba(0.2, 0.0, 0.8, 1.0);

        for (h, sigil) in ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12"]
            .iter()
            .skip(1)
            .enumerate()
        {
            let j = h as f32;
            let x = dial_center.0 + (second_hand_len * (j * radians_per_hour).sin());
            let y = dial_center.1 - (second_hand_len * (j * radians_per_hour).cos());

            ctx.fill_paint(silver);
            ctx.font_size(font_size);
            ctx.text_align(Align::CENTER | Align::MIDDLE);
            ctx.text((x, y), sigil)?;
        }

        for m in (1u8..61).filter(|m| m % 5 != 0).map(f32::from) {
            let ticks_radius = dial_radius * 0.925;
            let tick_len = 3.0;
            let tick_width = 1.0;
            ctx.begin_path();
            ctx.reset_transform();
            ctx.transform(Transform::translate(dial_center.0, dial_center.1));
            ctx.transform(Transform::rotate(m * radians_per_sec));
            ctx.move_to((0.0, -ticks_radius));
            ctx.line_to((0.0, -ticks_radius - tick_len));
            ctx.global_composite_operation(CompositeOperation::Basic(BasicCompositeOperation::Lighter));
            ctx.stroke_paint(white);
            ctx.stroke_width(tick_width);
            ctx.stroke()?;
        }

        ctx.fill_paint(silver);

        ctx.text_align(Align::CENTER | Align::BASELINE);
        ctx.reset_transform();
        ctx.text(
            (dial_center.0, dial_center.1 + dial_radius * 0.7 - font_size),
            &format!("{}:{:02}:{:02} {}", hour, minute, second, if am { "AM" } else { "PM" }),
        )?;
        ctx.text(
            (dial_center.0, dial_center.1 + dial_radius * 0.7),
            &format!("{:4}-{:02}-{:02}", year, month, day),
        )?;

        // draw dial
        ctx.begin_path();
        ctx.translate(dial_center.0, dial_center.1);
        ctx.circle(origin, dial_radius);
        ctx.stroke_width(3.0);
        ctx.stroke_paint(silver);
        ctx.fill_paint(dial_color);
        ctx.fill()?;
        ctx.stroke()?;

        let mut draw_hand = |theta, length: f32, width| {
            ctx.stroke_width(width);
            ctx.begin_path();
            ctx.reset_transform();
            ctx.translate(dial_center.0, dial_center.1);
            ctx.rotate(theta);
            ctx.move_to(origin);
            ctx.line_to((0.0, -length));
            ctx.stroke_paint(white);
            ctx.stroke()
        };

        let hour_angle = (((hour * 60.0 + minute) / 60.0) / 12.0) * two_pi;
        let minute_angle = minute * radians_per_sec;
        let second_angle = second * radians_per_sec;

        draw_hand(second_angle, second_hand_len, 1.0)?;
        draw_hand(minute_angle, minute_hand_len, 3.0)?;
        draw_hand(hour_angle, hour_hand_len, 5.0)?;

        ctx.begin_path();
        let boss_rad = 6.0;
        ctx.reset_transform();
        ctx.translate(dial_center.0, dial_center.1);
        ctx.circle(origin, boss_rad);
        ctx.stroke_width(1.0);
        ctx.stroke_paint(darkgray);
        ctx.global_composite_operation(CompositeOperation::Basic(BasicCompositeOperation::SrcOver));
        ctx.fill_paint(Gradient::Radial {
            center: origin.into(),
            in_radius: 0.0,
            out_radius: boss_rad,
            inner_color: silver,
            outer_color: darksilver,
        });
        ctx.fill()?;
        ctx.stroke()?;

        Ok(())
    }
}

fn main() {
    demo::run(DemoClock, "demo-clock");
}
