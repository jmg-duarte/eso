use crate::io;
use crate::vm;

use clap::ArgMatches;

pub fn run(matches: ArgMatches) {
    let filename = matches.value_of("file").unwrap().to_string();
    let source_code = io::read_input(&filename).unwrap();
    let mut bf_vm = vm::Machine::new(
        &source_code,
        Box::new(std::io::stdin()),
        Box::new(std::io::stdout()),
    );

    bf_vm.execute();
}
