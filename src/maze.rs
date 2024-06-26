use std::cell::RefCell;
use std::rc::Rc;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;
use sdl2::{EventPump};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use crate::maze::DrawTypes::{ALL, PATHS, WALLS};
use crate::node::Node;
use crate::utils::{AdditionalDrawMethods, AdditionalPointMethods};

#[derive(PartialEq, Eq)]
pub enum GenerationTypes {
    DEPTHFIRST,
    HUNTANDKILL,
    TEMPLATE
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DrawTypes {
    ALL,
    WALLS,
    PATHS
}

pub struct Maze {
    width: u32,
    height: u32,
    scale: f32,
    draw_mode: DrawTypes,
    pub nodes: Rc<RefCell<Vec<Vec<Rc<RefCell<Node>>>>>>,
    canvas: Option<Rc<RefCell<WindowCanvas>>>,
    events: Option<Rc<RefCell<EventPump>>>
}

impl Maze {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let mut nodes = Vec::new();
        for i in 0..height {
            nodes.push(Vec::new());
            for j in 0..width {
                nodes[i as usize].push(Rc::new(RefCell::new(Node::new(i, j))));
            }
        }
        let mut m = Maze {
            width,
            height,
            scale: 0.7,
            draw_mode: WALLS,
            nodes: Rc::new(RefCell::new(nodes)),
            canvas: None,
            events: None
        };
        m.setup()?;
        Ok(m)
    }

    pub fn clear(&self) {
        let b = self.nodes.clone();
        let mut nodes = b.borrow_mut();
        for i in nodes.iter() {
            for j in i.iter() {
                j.borrow_mut().conn = false;
                j.borrow_mut().paths.clear();
                j.borrow_mut().target = None;
            }
        }
    }

    pub fn gen(&self, t: GenerationTypes) {
        let mut rng = thread_rng();
        let nodes = self.nodes.clone();
        match t {
            GenerationTypes::DEPTHFIRST => {
                println!("dfs");
                let mut x = rng.gen_range(0..self.width);
                let mut y = rng.gen_range(0..self.height);
                let mut backtraces: Vec<(u32, u32)> = Vec::new();
                let mut counter = 0;
                'l: loop {
                    counter += 1;
                    nodes.borrow_mut()[x as usize][y as usize].borrow_mut().conn = true;
                    let neighbours = self.get_neighbours(x, y);
                    let mut open: Vec<Rc<RefCell<Node>>> = neighbours.into_iter().filter(|t| {!t.borrow().conn}).collect();
                    let next = open.pop();
                    if let Some(next) = next {
                        if open.len() > 0 {
                            backtraces.push((x, y));
                        }
                        let mut nref = nodes.borrow_mut();
                        let mut current = nref[x as usize][y as usize].clone();
                        next.borrow_mut().target = Some(current.clone());
                        current.borrow_mut().paths.push(next.clone());
                        let mut current = current.borrow_mut();
                        x = next.borrow().x;
                        y = next.borrow().y;
                        current.conn = true;
                    } else {
                        let next = backtraces.pop();
                        if let Some(next) = next {
                            x = next.0;
                            y = next.1;
                        } else {
                            break 'l;
                        }
                    }
                }
            }
            GenerationTypes::HUNTANDKILL => {}
            GenerationTypes::TEMPLATE => {
                let nodes = nodes.borrow_mut();
                for i in 0..self.height {
                    for j in 0..self.width {
                        if i > 0 {
                            let refNode = nodes[i as usize - 1][j as usize].clone();
                            let currNode = nodes[i as usize][j as usize].clone();
                            currNode.borrow_mut().target = Some(refNode.clone());
                            refNode.borrow_mut().paths.push(currNode.clone());
                        } else if j > 0 {
                            let refNode = nodes[i as usize][j as usize - 1].clone();
                            let currNode = nodes[i as usize][j as usize].clone();
                            currNode.borrow_mut().target = Some(refNode.clone());
                            refNode.borrow_mut().paths.push(currNode.clone());
                        }
                    }
                }
                nodes[0][0].borrow_mut().target = None;
            }
        }
    }

    pub fn shift(&self, root: Option<(u32, u32)>) -> (u32, u32) {
        let b = self.nodes.clone();
        let nodes = b.borrow();
        let rootNode: Rc<RefCell<Node>>;
        let mut x: u32;
        let mut y: u32;
        if let Some(root) = root {
            let s = nodes[root.0 as usize][root.1 as usize].clone();
            if s.borrow().target.is_none() {
                rootNode = s;
            } else {
                rootNode = self.find_root(root.0, root.1);
            }
        } else {
            let x = thread_rng().gen_range(0..self.width);
            let y = thread_rng().gen_range(0..self.height);
            rootNode = self.find_root(x, y);
        }
        x = rootNode.borrow().x;
        y = rootNode.borrow().y;
        let neigh = self.get_neighbours(x, y);
        if let Some(next) = neigh.last() {
            if !next.borrow().paths.contains(&rootNode) {
                next.borrow_mut().paths.push(rootNode.clone());
            }
            rootNode.borrow_mut().target = Some(next.clone());
            let cN = next.borrow_mut().target.take();
            if let Some(cN) = cN {
                cN.borrow_mut().paths.retain(|x2| {x2 != next});
            }
            x = next.borrow().x;
            y = next.borrow().y;
        }
        (x,y)
    }

    pub fn find_root(&self, x: u32, y: u32) -> Rc<RefCell<Node>> {
        let b = self.nodes.clone();
        let nodes = b.borrow();
        let mut x = x;
        let mut y = y;
        let mut s = nodes[x as usize][y as usize].clone();
        while let Some(new) = &RefCell::clone(&s).borrow().target {
            s = new.clone();
        }
        // loop {
        //     println!("{:?} - {:?}", x, y);
        //     println!("{:?}", s.borrow().paths.len());
        //     if s.borrow().target.is_none() {
        //         break;
        //     } else {
        //         if let Some(new) = &s.borrow().target {
        //             x = new.borrow().x;
        //             y = new.borrow().y;
        //         }
        //         s = nodes[x as usize][y as usize].clone();
        //     }
        // }
        s
    }

    pub fn get_neighbours(&self, x: u32, y: u32) -> Vec<Rc<RefCell<Node>>> {
        let mut vec: Vec<u32> = (0..4).collect();
        vec.shuffle(&mut thread_rng());
        let mut ret = Vec::new();
        let b = self.nodes.clone();
        let nodes = b.borrow();
        'l: loop {
            if let Some(dir) = vec.pop() {
                let mut neighbour = None;
                match dir {
                    0 => {
                        if let Some(coll) = nodes.get((x + 1) as usize) {
                            neighbour = coll.get(y as usize);
                        }
                    },
                    1 => {
                        if x <= 0 {
                            neighbour = None;
                        } else if let Some(coll) = nodes.get((x - 1) as usize) {
                            neighbour = coll.get(y as usize);
                        }
                    },
                    2 => {
                        if let Some(coll) = nodes.get(x as usize) {
                            neighbour = coll.get((y + 1) as usize);
                        }
                    },
                    3 => {
                        if y <= 0 {
                            neighbour = None;
                        } else if let Some(coll) = nodes.get(x as usize) {
                            neighbour = coll.get((y - 1) as usize);
                        }
                    },
                    _ => {}
                }
                if let Some(neighbour) = neighbour {
                    ret.push(neighbour.clone());
                }
            } else { break 'l; }
        }
        ret
    }

    fn setup(&mut self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video_subsys = sdl_context.video()?;
        let window = video_subsys
            .window(
                "maze",
                ((self.width * 100 + 100) as f32 * self.scale) as u32,
                ((self.height * 100 + 100) as f32 * self.scale) as u32,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        self.canvas = Some(Rc::new(RefCell::new(canvas)));
        self.events = Some(Rc::new(RefCell::new(sdl_context.event_pump()?)));
        Ok(())
    }

    pub fn draw(&self) -> Result<(), String> {
        if let Some(canvas) = &self.canvas {
            canvas.borrow_mut().set_draw_color(Color::RGB(0, 0, 0));
            canvas.borrow_mut().clear();
            let scale = self.scale;
            for v in self.nodes.borrow_mut().iter() {
                for n in v {
                    let a = n.borrow();
                    if [ALL, PATHS].contains(&self.draw_mode) {
                        canvas.borrow_mut().set_draw_color(Color::RGB(0, 0, 200));
                        canvas.borrow_mut().draw_circle(Point::p(a.x, a.y, scale),
                                                        (10_f32 * scale) as i32)?;
                        canvas.borrow_mut().set_draw_color(Color::RGB(200, 0, 0));
                        if a.conn {
                            canvas.borrow_mut().draw_circle(Point::p(a.x, a.y, scale),
                                                            (7f32 * scale) as i32)?;
                        }
                    }
                    let mut dir = [None; 4];
                    if let Some(p) = &a.target {
                        let ax = a.x as f32;
                        let ay = a.y as f32;
                        let px = p.borrow().x as f32;
                        let py = p.borrow().y as f32;
                        if [ALL, PATHS].contains(&self.draw_mode) {
                            canvas.borrow_mut().draw_line(Point::p(ax as u32, ay as u32, scale),
                                                          Point::p(px as u32, py as u32, scale))?;
                        }
                        if px > ax {
                            if [ALL, PATHS].contains(&self.draw_mode) {
                                canvas.borrow_mut().fill_rect(Rect::new(((ax * 100.0 + 180.0) * scale) as i32,
                                                                        ((ay * 100.0 + 97.0) * scale) as i32,
                                                                        (7.0 * scale) as u32,
                                                                        (7.0 * scale) as u32))?;
                            }
                            dir[0] = Some(1);
                        }
                        if px < ax {
                            if [ALL, PATHS].contains(&self.draw_mode) {
                                canvas.borrow_mut().fill_rect(Rect::new(((ax * 100.0 + 20.0) * scale) as i32,
                                                                        ((ay * 100.0 + 97.0) * scale) as i32,
                                                                        (7.0 * scale) as u32,
                                                                        (7.0 * scale) as u32))?;
                            }
                            dir[1] = Some(1);
                        }
                        if py > ay {
                            if [ALL, PATHS].contains(&self.draw_mode) {
                                canvas.borrow_mut().fill_rect(Rect::new(((ax * 100.0 + 97.0) * scale) as i32,
                                                                        ((ay * 100.0 + 180.0) * scale) as i32,
                                                                        (7.0 * scale) as u32,
                                                                        (7.0 * scale) as u32))?;
                            }
                            dir[2] = Some(1);
                        }
                        if py < ay {
                            if [ALL, PATHS].contains(&self.draw_mode) {
                                canvas.borrow_mut().fill_rect(Rect::new(((ax * 100.0 + 97.0) * scale) as i32,
                                                                        ((ay * 100.0 + 20.0) * scale) as i32,
                                                                        (7.0 * scale) as u32,
                                                                        (7.0 * scale) as u32))?;
                            }
                            dir[3] = Some(1);
                        }
                    }
                    if [ALL, WALLS].contains(&self.draw_mode) {
                        for n in &a.paths {
                            let ax = a.x;
                            let ay = a.y;
                            let px = n.borrow().x;
                            let py = n.borrow().y;
                            if px > ax {
                                dir[0] = Some(1);
                            }
                            if px < ax {
                                dir[1] = Some(1);
                            }
                            if py > ay {
                                dir[2] = Some(1);
                            }
                            if py < ay {
                                dir[3] = Some(1);
                            }
                        }
                        canvas.borrow_mut().set_draw_color(Color::RGB(230, 230, 230));
                        for i in 0..dir.len() {
                            if dir[i].is_none() {
                                match i {
                                    0 => {
                                        canvas.borrow_mut().draw_line(Point::new(((a.x * 100 + 150) as f32 * scale) as i32,
                                                                                 ((a.y * 100 + 150) as f32 * scale) as i32),
                                                                      Point::new(((a.x * 100 + 150) as f32 * scale) as i32,
                                                                                 ((a.y * 100 + 50) as f32 * scale) as i32))?;
                                    },
                                    1 => {
                                        canvas.borrow_mut().draw_line(Point::new(((a.x * 100 + 50) as f32 * scale) as i32,
                                                                                 ((a.y * 100 + 150) as f32 * scale) as i32),
                                                                      Point::new(((a.x * 100 + 50) as f32 * scale) as i32,
                                                                                 ((a.y * 100 + 50) as f32 * scale) as i32))?;
                                    },
                                    2 => {
                                        canvas.borrow_mut().draw_line(Point::new(((a.x * 100 + 150) as f32 * scale) as i32,
                                                                                 ((a.y * 100 + 150) as f32 * scale) as i32),
                                                                      Point::new(((a.x * 100 + 50) as f32 * scale) as i32,
                                                                                 ((a.y * 100 + 150) as f32 * scale) as i32))?;
                                    },
                                    3 => {
                                        canvas.borrow_mut().draw_line(Point::new(((a.x * 100 + 150) as f32 * scale) as i32,
                                                                                 ((a.y * 100 + 50) as f32 * scale) as i32),
                                                                      Point::new(((a.x * 100 + 50) as f32 * scale) as i32,
                                                                                 ((a.y * 100 + 50) as f32 * scale) as i32))?;
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            canvas.borrow_mut().present();
        }
        Ok(())
    }

    pub fn main_loop(&mut self) -> Result<(), String> {
        let drawMethods = [ALL, WALLS, PATHS];
        let mut drawIndex = 0;
        if let Some(events) = &self.events {
            let mut sdown = false;
            let mut root = (0, 0);
            'main: loop {
                if sdown {
                    let n = self.shift(Some(root));
                    self.draw()?;
                    root = n;
                }
                for event in events.borrow_mut().poll_iter() {
                    match event {
                        Event::Quit { .. } => break 'main,

                        Event::KeyDown {
                            keycode: Some(keycode),
                            ..
                        } => {
                            if keycode == Keycode::Escape {
                                break 'main;
                            } else if keycode == Keycode::Space {
                                println!("generating");
                                self.gen(GenerationTypes::DEPTHFIRST);
                                println!("drawing");
                                self.draw()?;
                                println!("done")
                            } else if keycode == Keycode::S {
                                sdown = !sdown;
                            } else if keycode == Keycode::T {
                                self.gen(GenerationTypes::TEMPLATE);
                                self.draw()?;
                            } else if keycode == Keycode::C {
                                self.clear();
                                self.draw()?;
                            } else if keycode == Keycode::R {
                                self.shift(None);
                                self.draw()?;
                            } else if keycode == Keycode::Up {
                                if drawIndex == 2 {
                                    drawIndex = 0;
                                } else {
                                    drawIndex += 1;
                                }
                                self.draw_mode = drawMethods[drawIndex];
                                self.draw()?;
                            } else if keycode == Keycode::Down {
                                if drawIndex == 0 {
                                    drawIndex = 2;
                                } else {
                                    drawIndex -= 1;
                                }
                                self.draw_mode = drawMethods[drawIndex];
                                self.draw()?;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}