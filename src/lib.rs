#[cfg(not(test))]
compile_error!("test not set");

#[cfg(test)]
compile_error!("test set");
