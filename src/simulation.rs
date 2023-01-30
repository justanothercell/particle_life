use crate::world::World;

pub(crate) fn tick(world: &mut World, delta_ms: u128) {
    let delta = delta_ms as f32 / 100.0;
    // UPDATE SPEED
    for x in 0..world.chunks.len() {
        for y in 0..world.chunks[x].len() {
            let chunk: *const _ = &world.chunks[x][y];
            for p in &mut world.chunks[x][y].particles{
                for (x, y) in unsafe {match (p.x > 5.0, p.y > 5.0) {
                    (false, false) => vec![(*chunk).particles.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>(),
                                           (*(*chunk).west).particles.iter().map(|p| (p.x-10.0, p.y)).collect::<Vec<_>>(),
                                           (*(*(*chunk).west).north).particles.iter().map(|p| (p.x-10.0, p.y-10.0)).collect::<Vec<_>>(),
                                           (*(*chunk).north).particles.iter().map(|p| (p.x, p.y-10.0)).collect::<Vec<_>>()],
                    (false, true) => vec![(*chunk).particles.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>(),
                                          (*(*chunk).west).particles.iter().map(|p| (p.x-10.0, p.y)).collect::<Vec<_>>(),
                                          (*(*(*chunk).west).south).particles.iter().map(|p| (p.x-10.0, p.y+10.0)).collect::<Vec<_>>(),
                                          (*(*chunk).south).particles.iter().map(|p| (p.x, p.y+10.0)).collect::<Vec<_>>()],
                    (true, false) => vec![(*chunk).particles.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>(),
                                          (*(*chunk).east).particles.iter().map(|p| (p.x+10.0, p.y)).collect::<Vec<_>>(),
                                          (*(*(*chunk).east).north).particles.iter().map(|p| (p.x+10.0, p.y-10.0)).collect::<Vec<_>>(),
                                          (*(*chunk).north).particles.iter().map(|p| (p.x, p.y-10.0)).collect::<Vec<_>>()],
                    (true, true) => vec![(*chunk).particles.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>(),
                                         (*(*chunk).east).particles.iter().map(|p| (p.x+10.0, p.y)).collect::<Vec<_>>(),
                                         (*(*(*chunk).east).south).particles.iter().map(|p| (p.x+10.0, p.y+10.0)).collect::<Vec<_>>(),
                                         (*(*chunk).south).particles.iter().map(|p| (p.x, p.y+10.0)).collect::<Vec<_>>()],
                }}.into_iter().flatten().collect::<Vec<_>>() {
                    if (p.x, p.y) != (x, y) {
                        let dx = x - p.x;
                        let dy = y - p.y;

                        let mut dp3 = dx.powi(2) + dy.powi(2);
                        if dp3 < 2.0 {
                            dp3 = -f32::max(dp3, 0.1)
                        }
                        p.vx += dx * delta * 0.2 / dp3;
                        p.vy += dy * delta * 0.2 / dp3;
                    }
                    p.vx *= 0.9999;
                    p.vy *= 0.9999;
                }
            }
        }
    }
    // UPDATE POS
    for x in 0..world.chunks.len() {
        for y in 0..world.chunks[x].len() {
            for p in &mut world.chunks[x][y].particles {
                p.x += p.vx;
                p.y += p.vy;
            }
        }
    }
    // UPDATE OWNING CHUNK
    for x in 0..world.chunks.len() {
        for y in 0..world.chunks[x].len() {
            let chunk: *const _ = &world.chunks[x][y];
            world.chunks[x][y].particles.retain_mut(|p| {
                if p.x < 0.0 {
                    p.x += 10.0;
                    let p: *const _ = p;
                    unsafe { (*(*chunk).west).particles.push(std::ptr::read(p)); }
                    return false
                }
                if p.y < 0.0 {
                    p.y += 10.0;
                    let p: *const _ = p;
                    unsafe { (*(*chunk).north).particles.push(std::ptr::read(p)); }
                    return false
                }
                if p.x > 10.0 {
                    p.x -= 10.0;
                    let p: *const _ = p;
                    unsafe { (*(*chunk).east).particles.push(std::ptr::read(p)); }
                    return false
                }
                if p.y > 10.0 {
                    p.y -= 10.0;
                    let p: *const _ = p;
                    unsafe { (*(*chunk).south).particles.push(std::ptr::read(p)); }
                    return false
                }
                true
            })
        }
    }
}