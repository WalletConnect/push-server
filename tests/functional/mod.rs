/// Functional tests that cover Echo Server when running with a database and all
/// other expectations as a complete system
#[cfg(feature = "multitenant")]
mod multitenant;
#[cfg(not(feature = "multitenant"))]
mod singletenant;
mod stores;
