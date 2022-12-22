use std::collections::HashMap;
use std::fs::File;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct LineID {
    start: u16,
    end: u16,
}

fn main() {
    // open the file into array.
    let decoder = png::Decoder::new(File::open("test.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let _bytes = &buf[..info.buffer_size()];
    
    let _lines = map_of_lines();
}

fn bresenham(x0: u16, y0: u16, x1: u16, y1: u16) -> Vec<Point> {
    let (mut cx, mut cy, dx, dy, sx, sy, mut err): (i16, i16, i16, i16, i16, i16, i16);
    
    cx = x0 as i16;
    cy = y0 as i16;
    dx = (x1 as i16 - x0 as i16).abs();
    dy = (y1 as i16 - y0 as i16).abs();
    
    if cx < x1 as i16 {
		sx = 1;
	} else {
		sx = -1;
	}
	if cy < y1 as i16 {
		sy = 1;
	} else {
		sy = -1;
	}
	err = dx - dy;
	
	let mut line = Vec::new();
    
    loop {
		line.push(Point { x: cx as u16, y: cy as u16 });
		if cx == x1 as i16 && cy == y1 as i16 {
			return line;
		}
		let e2 = 2 * err;
		if e2 > 0-dy {
			err = err - dy;
			cx = cx + sx;
		}
		if e2 < dx {
			err = err + dx;
			cy = cy + sy;
		}
	}
}

fn map_of_lines() -> HashMap<LineID, Vec<Point>> {
    let circle_points = 200;
    let mut circle_coords = Vec::new();
    
    for i in 0..circle_points {
        let (x, y): (u16, u16);
        let angle = std::f64::consts::PI * 2.0 / circle_points as f64 * i as f64;
        x = constrain((angle.cos() * 200.0 + 200.0) as usize, 399) as u16;
        y = constrain((angle.sin() * 200.0 + 200.0) as usize, 399) as u16;
        circle_coords.push(Point { x: x, y: y });
    }
    
    let mut lines: HashMap<LineID, Vec<Point>> = HashMap::new();
    for a in 0..circle_coords.len() {
        for b in a+1..circle_coords.len() {
            let pair = LineID { start: a as u16, end: b as u16 };
            let (x0, y0, x1, y1): (u16, u16, u16, u16);
            x0 = circle_coords[a].x;
            y0 = circle_coords[a].y;
            x1 = circle_coords[b].x;
            y1 = circle_coords[b].y;
            lines.insert(pair, bresenham(x0, y0, x1, y1));
        }
    }
    
    return lines;
}

fn constrain(num: usize, max: usize) -> usize {
    if max < num {
        return max;
    }
    return num;
}

