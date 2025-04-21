fn problem0() -> anyhow::Result<()> {
    // first we make a canvas for GIF image
    let mut canvas = del_canvas::canvas_gif::Canvas::new(
        "problem0.gif",
        (300usize, 300usize),
        &vec![
            0xffffffi32, // color_0 is white
            0xff0000i32, // color_1 is red
        ],
    )?;
    let num_frame = 30;
    for i_frame in 0..num_frame {
        canvas.clear(0); // clear canvas with color_0
        let time = i_frame as f32 / num_frame as f32;
        let radian = time * 2f32 * std::f32::consts::PI;
        // position in the pixel coordinate
        let p0 = [150f32, 150f32];
        // position in the pixel coordinate
        let p1 = [
            p0[0] + 100f32 * radian.cos(),
            p0[1] - 100f32 * radian.sin(),
        ];
        // function to draw line into the canvas
        del_canvas::rasterize::line::draw_dda(&mut canvas.data, canvas.width, &p0, &p1, 1);
        canvas.write(); // save current frame
    }
    Ok(())
}

fn problem1() -> anyhow::Result<()> {
    // first we make a canvas for GIF image
    let mut canvas = del_canvas::canvas_gif::Canvas::new(
        "problem1.gif",
        (300usize, 300usize),
        &vec![
            0xffffffi32, // color_0 is white
            0xff0000i32, // color_1 is red
            0x00ff00i32, // color_2 is green
            0x0000ffi32, // color_3 is blue
        ],
    )?;
    let num_frame: i32 = 60;
    let mut trajectory = Vec::<[f32;2]>::new();
    for i_frame in 0..num_frame {
        canvas.clear(0); // clear canvas with color_0
        let time: f32 = i_frame as f32 / num_frame as f32;
        let radian: f32 = time * 2f32 * std::f32::consts::PI;
        // center of the image
        let p0: [f32; 2] = [150f32, 150f32];
        // center
        let p1: [f32; 2] = [
            p0[0] + 100f32 * radian.cos(),
            p0[1] - 100f32 * radian.sin(),
        ];
        del_canvas::rasterize::line::draw_dda(&mut canvas.data, canvas.width, &p0, &p1, 1);
        //-------------------------------
        // write some code below.
        // modify the defninition of p2
        let p2: [f32; 2] = [
            p1[0] + 50f32 * (2f32*radian.cos()*radian.cos() - 1f32),
            p1[1] - 50f32 * (2f32 *radian.sin()*radian.cos()),
        ];
        del_canvas::rasterize::line::draw_dda(&mut canvas.data, canvas.width, &p1, &p2, 2);
        trajectory.push(p2); // hint
        // draw trajectoty using for loop below.
        for idx in 0..trajectory.len().saturating_sub(1) {
            let p2_pres = trajectory[idx];
            let p2_next = trajectory[idx + 1];
            del_canvas::rasterize::line::draw_dda(&mut canvas.data, canvas.width, &p2_pres, &p2_next, 3);
        }
        // ---------------------
        // no further edit from here
        canvas.write(); // save current frame
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    problem0()?;
    problem1()?;
    Ok(())
}
