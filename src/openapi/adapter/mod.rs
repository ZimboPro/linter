mod adapter_impl;
mod edges;
mod entrypoints;
mod properties;
mod vertex;

#[cfg(test)]
mod tests;
mod utils;

pub use adapter_impl::Adapter;
pub use vertex::Vertex;
