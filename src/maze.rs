use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;
use rand::seq::SliceRandom;
use sdl2::{EventPump};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use crate::node::Node;
use crate::utils::{AdditionalDrawMethods, AdditionalPointMethods};

pub enum GenerationTypes {
    DEPTHFIRST,
    HUNTANDKILL,
    TEMPLATE
}

pub struct Maze<'a> {
    width: u32,
    height: u32,
    pub nodes: Rc<RefCell<Vec<Vec<Option<Rc<RefCell<Node<'a>>>>>>>>,
    canvas: Rc<RefCell<WindowCanvas>>,
    events: Rc<RefCell<EventPump>>
}

impl<'a> Maze<'a> {
    pub fn new(width: u32, height: u32) -> Self {
        let t = Maze::setup(width, height).unwrap();
        let mut nodes: Vec<Vec<Option<Rc<RefCell<Node>>>>> = Vec::new();
        for i in 0..height {
            nodes.push(Vec::new());
            for j in 0..width {
                nodes[i as usize].push(None);
            }
        }
        let m = Maze {
            width,
            height,
            nodes: Rc::new(RefCell::new(nodes)),
            canvas: Rc::new(RefCell::new(t.1)),
            events: Rc::new(RefCell::new(t.0))
        };
        m
    }

    pub fn gen(&'a self, t: GenerationTypes) {
        let mut rng = rand::thread_rng();
        match t {
            GenerationTypes::DEPTHFIRST => {
                let mut count = 0;
                let points = self.width * self.height;
                let mut x = rng.gen_range(0..self.width);
                let mut y = rng.gen_range(0..self.height);
                let backtraces: Vec<(u32, u32)> = Vec::new();
                while count <= points {
                    let mut current = self.nodes.borrow_mut()[x as usize][y as usize].as_ref().unwrap();
                    let neighbours = Maze::get_neighbours(&self.nodes.borrow_mut(), x, y);

                    count += 1;
                }
            }
            GenerationTypes::HUNTANDKILL => {}
            GenerationTypes::TEMPLATE => {
                for i in 0..self.height {
                    for j in 0..self.width {
                        if i > 0 {
                            if let Some(refNode) = &self.nodes.borrow_mut().to_owned()[i as usize - 1][j as usize] {
                                if let Some(currNode) = &self.nodes.borrow_mut()[i as usize][j as usize] {
                                    currNode.borrow_mut().paths.push(refNode);
                                }
                            }
                        } else if j > 0 {
                            if let Some(x) = &self.nodes.borrow_mut().to_owned()[i as usize][j as usize - 1] {
                                if let Some(currNode) = &self.nodes.borrow_mut()[i as usize][j as usize] {
                                    currNode.borrow_mut().paths.push(&x);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_neighbours(nodes: &'a Vec<Vec<Option<Rc<RefCell<Node<'a>>>>>>, x: u32, y: u32) -> Vec<&'a Rc<RefCell<Node<'a>>>> {
        let mut rng = rand::thread_rng();
        let mut vec: Vec<u32> = (0..4).collect();
        vec.shuffle(&mut rng);
        let mut ret = Vec::new();
        'l: loop {
            if let Some(dir) = vec.pop() {
                let mut neighbour = None;
                match dir {
                    0 => {
                        if let Some(x) = nodes.get((x + 1) as usize) {
                            neighbour = x.get(y as usize);
                        }
                    },
                    1 => {
                        if x <= 0 {
                            neighbour = None;
                        } else if let Some(x) = nodes.get((x - 1) as usize) {
                            neighbour = x.get(y as usize);
                        }
                    },
                    2 => {
                        if let Some(x) = nodes.get(x as usize) {
                            neighbour = x.get((y + 1) as usize);
                        }
                    },
                    3 => {
                        if y <= 0 {
                            neighbour = None;
                        } else if let Some(x) = nodes.get(x as usize) {
                            neighbour = x.get((y - 1) as usize);
                        }
                    },
                    _ => {}
                }
                if let Some(Some(neighbour)) = neighbour {
                    ret.push(neighbour);
                }
            } else { break 'l; }
        }
        ret
    }

    fn setup(width: u32, height: u32) -> Result<(EventPump, WindowCanvas), String> {
        let sdl_context = sdl2::init()?;
        let video_subsys = sdl_context.video()?;
        let window = video_subsys
            .window(
                "maze",
                width * 100 + 100,
                height * 100 + 100,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        Ok((sdl_context.event_pump()?,canvas))
    }

    pub fn draw(&'a self) -> Result<(), String> {
        self.canvas.borrow_mut().set_draw_color(Color::RGB(0, 0, 200));
        for v in self.nodes.borrow_mut().iter() {
            for n in v {
                if let Some(a) = n {
                    let a = a.borrow();
                    self.canvas.borrow_mut().draw_circle(Point::p(a.x, a.y), 10)?;
                    for p in &a.paths {
                        let ax = a.x as i32;
                        let ay = a.y as i32;
                        let px = p.borrow().x as i32;
                        let py = p.borrow().y as i32;
                        self.canvas.borrow_mut().draw_line(Point::p(ax as u32, ay as u32), Point::p(px as u32, py as u32))?;
                        if px > ax {
                            self.canvas.borrow_mut().draw_rect(Rect::new(ax * 100 + 180, ay * 100 + 97, 7, 7))?
                        }
                        if px < ax {
                            self.canvas.borrow_mut().draw_rect(Rect::new(ax * 100 + 20, ay * 100 + 97, 7, 7))?
                        }
                        if py > ay {
                            self.canvas.borrow_mut().draw_rect(Rect::new(ax * 100 + 97, ay * 100 + 180, 7, 7))?
                        }
                        if py < ay {
                            self.canvas.borrow_mut().draw_rect(Rect::new(ax * 100 + 97, ay * 100 + 20, 7, 7))?
                        }
                    }
                }
            }
        }
        self.canvas.borrow_mut().present();
        Ok(())
    }

    pub fn main_loop(&'a self) -> Result<(), String> {
        let mut rng = rand::thread_rng();
        'main: loop {
            for event in self.events.borrow_mut().poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,

                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => {
                        if keycode == Keycode::Escape {
                            break 'main;
                        } else if keycode == Keycode::R {
                            self.canvas.borrow_mut().set_draw_color(Color::RGB(rng.gen(), rng.gen(),rng.gen()));
                            self.canvas.borrow_mut().clear();
                            self.canvas.borrow_mut().present();
                        } else if keycode == Keycode::Space {
                            self.nodes.borrow_mut()[4][5] = Some(Rc::new(RefCell::new(Node::new(4, 5))));
                            let neighs = Self::get_neighbours(&self.nodes.borrow_mut().to_owned(), 5, 5);
                            for n in neighs {
                                println!("{:?} - {:?}", n.borrow().x, n.borrow().y);
                            }
                            self.draw()?;
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}