use std::ptr::{null_mut};

pub(crate) struct World{
    pub(crate) chunks: Vec<Vec<Chunk>>
}

impl World {
    pub(crate) fn new(w: usize, h: usize) -> Self{
        let mut chunks = vec![];
        for _ in 0..w {
            let mut column = vec![];
            for _ in 0..h {
                column.push(Chunk {
                    north: null_mut(),
                    south: null_mut(),
                    west: null_mut(),
                    east: null_mut(),
                    particles: vec![],
                })
            }
            chunks.push(column)
        }
        for x in 0..w {
            for y in 0..h {
                chunks[x][y].north = &mut chunks[x][(y+h-1)%h];
                chunks[x][y].south = &mut chunks[x][(y+1)%h];
                chunks[x][y].west = &mut chunks[(x+w-1)%w][y];
                chunks[x][y].east = &mut chunks[(x+1)%w][y];
            }
        }
        Self {
            chunks
        }
    }

    pub(crate) fn add_particle(&mut self, mut p: Particle) {
        let cx = p.x as usize / 10 % self.chunks.len();
        let cy = p.y as usize / 10 % self.chunks[0].len();
        p.x %= 10.0;
        p.y %= 10.0;
        self.chunks[cx][cy].particles.push(p)
    }
}

pub(crate) struct Chunk {
    pub(crate) north: *mut Chunk,
    pub(crate) south: *mut Chunk,
    pub(crate) west: *mut Chunk,
    pub(crate) east: *mut Chunk,
    pub(crate) particles: Vec<Particle>
}


pub(crate) struct Particle{
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) vx: f32,
    pub(crate) vy: f32
}