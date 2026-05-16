use dpg_adapter::{FileMetadataSource, MetadataSource};
use dpg_rules::validate;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args();
    let _bin = args.next();

    let Some(path) = args.next() else {
        eprintln!("Usage: dpg-cli <metadata-file>");
        return ExitCode::from(2);
    };

    if args.next().is_some() {
        eprintln!("Unexpected extra arguments. Usage: dpg-cli <metadata-file>");
        return ExitCode::from(2);
    }

    let source = FileMetadataSource::new(path);
    let model = match source.load() {
        Ok(model) => model,
        Err(err) => {
            eprintln!("Load failed: {err}");
            return ExitCode::from(1);
        }
    };

    let violations = validate(&model);
    if violations.is_empty() {
        println!("Validation passed for dataset '{}'", model.dataset.name);
        ExitCode::SUCCESS
    } else {
        eprintln!("Validation failed with {} issue(s):", violations.len());
        for violation in &violations {
            eprintln!("- [{}] {}", violation.code, violation.message);
        }
        ExitCode::from(1)
    }
}
