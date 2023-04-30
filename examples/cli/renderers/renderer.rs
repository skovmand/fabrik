//// The Renderer is an enum allowing main.rs to build a renderer and pass it to the `solve`
//// functions. It contains the options for rendering sudokus in the example. It implements
//// SudokuRenderer so it can be passed into a function with those trait bounds, and it just
//// delegates to the internal renderers.

use crate::{renderers::SudokuRenderer, Board};

use super::{DelayedRenderer, ResultOnlyRenderer};

pub enum Renderer {
    Delayed(DelayedRenderer),
    FinalResultOnly(ResultOnlyRenderer),
}

impl SudokuRenderer for Renderer {
    fn setup(&self, filename: &str) {
        match self {
            Renderer::Delayed(renderer) => renderer.setup(filename),
            Renderer::FinalResultOnly(renderer) => renderer.setup(filename),
        }
    }

    fn display_step(&self, board: &Board) {
        match self {
            Renderer::Delayed(renderer) => renderer.display_step(board),
            Renderer::FinalResultOnly(renderer) => renderer.display_step(board),
        }
    }

    fn display_final_result(&self, board: &Board) {
        match self {
            Renderer::Delayed(renderer) => renderer.display_final_result(board),
            Renderer::FinalResultOnly(renderer) => renderer.display_final_result(board),
        }
    }

    fn teardown(&self) {
        match self {
            Renderer::Delayed(renderer) => renderer.teardown(),
            Renderer::FinalResultOnly(renderer) => renderer.teardown(),
        }
    }
}
