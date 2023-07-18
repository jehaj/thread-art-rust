use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

// IMAGE_SIZE is the diameter of the frame in mm, if the thread width
// is 1 mm. If it is .5 mm, then IMAGE_SIZE should be doubled.
//     IMAGE_SIZE = frame_width(mm) / thread_width(mm).
// E.g. a 40 cm frame with a .5 mm wide thread should have:
//     IMAGE_SIZE = 400 / 0.5 = 800
// which affects the result, so consider changing number of wraps, 
// bWhichrightness factor etc.
const IMAGE_SIZE: u16 = 400;
const IMAGE_AREA: usize = IMAGE_SIZE as usize * IMAGE_SIZE as usize;
const WRAPS: u16 = 2500 - 1;
const CIRCLE_POINTS: u16 = 235;
const MINIMUM_DIFFERENCE: u8 = 20;
const BRIGHTNESS_FACTOR: u8 = 34;

// represents a point/coordinate on the image.
#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: u16,
    y: u16,
}

// represents a line from start point to end point.
#[derive(Debug, PartialEq, Eq, Hash)]
struct LineID {
    start: u16,
    end: u16,
}

fn main() {
    // open the file into array.
    let args: Vec<String> = env::args().collect();
    let input_path = Path::new(&args[1]);
    let mut decoder = png::Decoder::new(File::open(input_path).unwrap());
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let size = info.buffer_size();
    let bytes = &mut buf[..size];
    if size != IMAGE_AREA {
        // the image is not the correct size
        std::process::exit(1);
    }

    let circle_coords = circle_coords();
    let lines = map_of_lines(&circle_coords);

    let mut point_index = 0;
    let mut point_list = vec![point_index];
    let mut used_lines = HashSet::new();

    for _ in 0..WRAPS {
        let mut max_weight = 0;
        let mut max_line = &vec![Point { x: 0, y: 0 }];
        let mut max_point_index = 0;

        for next_point_index in 0..circle_coords.len() {
            if point_index == next_point_index {
                continue;
            }

            let difference = (next_point_index as isize - point_index as isize).abs();
            if difference < MINIMUM_DIFFERENCE as isize
                || difference > (circle_coords.len() - MINIMUM_DIFFERENCE as usize) as isize
            {
                continue;
            }
            if used_lines.contains(&get_line_id(next_point_index as u16, point_index as u16)) {
                continue;
            }
            let line = lines
                .get(&get_line_id(next_point_index as u16, point_index as u16))
                .unwrap();
            let mut weight = line.len() * 255;
            for pos in line {
                let pixel = bytes[IMAGE_SIZE as usize * pos.y as usize + pos.x as usize];
                weight = std::cmp::max(weight as isize - pixel as isize, 0) as usize;
            }

            weight = weight / line.len();

            if weight > max_weight {
                max_weight = weight;
                max_line = line;
                max_point_index = next_point_index;
            }
        }

        used_lines.insert(get_line_id(max_point_index as u16, point_index as u16));
        point_list.push(max_point_index);
        point_index = max_point_index;

        for pos in max_line {
            let pixel_value = bytes[IMAGE_SIZE as usize * pos.y as usize + pos.x as usize];
            let value = std::cmp::min(255, pixel_value as usize + BRIGHTNESS_FACTOR as usize) as u8;
            bytes[IMAGE_SIZE as usize * pos.y as usize + pos.x as usize] = value;
        }
    }

    let mut output: [u8; IMAGE_AREA] =
        [255; IMAGE_AREA];
    for i in 1..point_list.len() {
        let p1 = point_list[i - 1] as u16;
        let p2 = point_list[i] as u16;
        let line = lines.get(&get_line_id(p1, p2)).unwrap();
        for pos in line {
            let c_value = output[IMAGE_SIZE as usize * pos.y as usize + pos.x as usize];
            let value = std::cmp::max(c_value as isize - 20 as isize, 0) as u8;
            output[IMAGE_SIZE as usize * pos.y as usize + pos.x as usize] = value;
        }
    }

    // write the points to file
    let points_path = input_path.parent().unwrap().join("RESULT.txt");
    let points_file = File::create(points_path).unwrap();
    let mut points_writer = BufWriter::new(points_file);
    points_writer
        .write((CIRCLE_POINTS.to_string() + "\n").as_bytes())
        .unwrap();

    for point in point_list {
        let value = point.to_string() + ",";
        points_writer.write(value.as_bytes()).unwrap();
    }

    // write the resulting image
    let output_path = Path::new(&args[2]);
    let output_file = File::create(output_path).unwrap();
    let ref mut output_writer = BufWriter::new(output_file);

    let mut encoder = png::Encoder::new(output_writer, IMAGE_SIZE as u32, IMAGE_SIZE as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&output).unwrap();
}

fn bresenham(x0: u16, y0: u16, x1: u16, y1: u16) -> Vec<Point> {
    let (mut cx, mut cy, dx, dy, sx, sy, mut err);

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
        line.push(Point {
            x: cx as u16,
            y: cy as u16,
        });
        if cx == x1 as i16 && cy == y1 as i16 {
            return line;
        }
        let e2 = 2 * err;
        if e2 > 0 - dy {
            err = err - dy;
            cx = cx + sx;
        }
        if e2 < dx {
            err = err + dx;
            cy = cy + sy;
        }
    }
}

fn circle_coords() -> Vec<Point> {
    let mut circle_coords = Vec::new();

    for i in 0..CIRCLE_POINTS {
        let (x, y);
        let angle = std::f64::consts::PI * 2.0 / CIRCLE_POINTS as f64 * i as f64;
        let size_half = (IMAGE_SIZE / 2) as f64;
        x = constrain(
            (angle.cos() * size_half + size_half) as usize,
            (IMAGE_SIZE - 1) as usize,
        ) as u16;
        y = constrain(
            (angle.sin() * size_half + size_half) as usize,
            (IMAGE_SIZE - 1) as usize,
        ) as u16;
        circle_coords.push(Point { x: x, y: y });
    }

    return circle_coords;
}

fn map_of_lines(circle_coords: &Vec<Point>) -> HashMap<LineID, Vec<Point>> {
    let mut lines: HashMap<LineID, Vec<Point>> = HashMap::new();
    for a in 0..circle_coords.len() {
        for b in a + 1..circle_coords.len() {
            let pair = LineID {
                start: a as u16,
                end: b as u16,
            };
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

fn get_line_id(a: u16, b: u16) -> LineID {
    if a < b {
        return LineID { start: a, end: b };
    } else {
        return LineID { start: b, end: a };
    }
}
