use crate::compiler;
use crate::io;
use crate::vm;

use clap::ArgMatches;

pub fn run(matches: ArgMatches) {
    let filename = matches.value_of("file").unwrap().to_string();
    let source_code = io::read_input(&filename).unwrap();
    let mut compiler = compiler::Compiler::new(&source_code);
    let mut bf_vm = vm::Machine::new(
        compiler.compile(),
        Box::new(std::io::BufReader::new(std::io::stdin())),
        Box::new(std::io::BufWriter::new(std::io::stdout())),
    );
    bf_vm.execute();
}
