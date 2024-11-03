use wgpu_gallary::run;
use pollster::block_on;

fn main() {
    block_on(run());
}